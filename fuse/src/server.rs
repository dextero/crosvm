// Copyright 2019 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cmp::max;
use std::cmp::min;
use std::convert::TryInto;
use std::ffi::CStr;
use std::io;
use std::mem::size_of;
use std::mem::MaybeUninit;
use std::os::unix::io::AsRawFd;
use std::time::Duration;

use base::error;
use base::pagesize;
use base::Protection;
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::IntoBytes;

use crate::filesystem::Context;
use crate::filesystem::DirEntry;
use crate::filesystem::DirectoryIterator;
use crate::filesystem::Entry;
use crate::filesystem::FileSystem;
use crate::filesystem::GetxattrReply;
use crate::filesystem::IoctlReply;
use crate::filesystem::ListxattrReply;
use crate::filesystem::ZeroCopyReader;
use crate::filesystem::ZeroCopyWriter;
use crate::sys::*;
use crate::Error;
use crate::Result;

const DIRENT_PADDING: [u8; 8] = [0; 8];

const SELINUX_XATTR_CSTR: &[u8] = b"security.selinux\0";

/// A trait for reading from the underlying FUSE endpoint.
pub trait Reader: io::Read {
    fn read_struct<T: IntoBytes + FromBytes>(&mut self) -> Result<T> {
        let mut out = T::new_zeroed();
        self.read_exact(out.as_mut_bytes())
            .map_err(Error::DecodeMessage)?;
        Ok(out)
    }
}

impl<R: Reader> Reader for &'_ mut R {}

/// A trait for writing to the underlying FUSE endpoint. The FUSE device expects the write
/// operation to happen in one write transaction. Since there are cases when data needs to be
/// generated earlier than the header, it implies the writer implementation to keep an internal
/// buffer. The buffer then can be flushed once header and data are both prepared.
pub trait Writer: io::Write {
    /// The type passed in to the closure in `write_at`. For most implementations, this should be
    /// `Self`.
    type ClosureWriter: Writer + ZeroCopyWriter;

    /// Allows a closure to generate and write data at the current writer's offset. The current
    /// writer is passed as a mutable reference to the closure. As an example, this provides an
    /// adapter for the read implementation of a filesystem to write directly to the final buffer
    /// without generating the FUSE header first.
    ///
    /// Notes: An alternative implementation would be to return a slightly different writer for the
    /// API client to write to the offset. Since the API needs to be called for more than one time,
    /// it imposes some complexity to deal with borrowing and mutability. The current approach
    /// simply does not need to create a different writer, thus no need to deal with the mentioned
    /// complexity.
    fn write_at<F>(&mut self, offset: usize, f: F) -> io::Result<usize>
    where
        F: Fn(&mut Self::ClosureWriter) -> io::Result<usize>;

    /// Checks if the writer can still accept certain amount of data.
    fn has_sufficient_buffer(&self, size: u32) -> bool;
}

impl<W: Writer> Writer for &'_ mut W {
    type ClosureWriter = W::ClosureWriter;

    fn write_at<F>(&mut self, offset: usize, f: F) -> io::Result<usize>
    where
        F: Fn(&mut Self::ClosureWriter) -> io::Result<usize>,
    {
        (**self).write_at(offset, f)
    }

    fn has_sufficient_buffer(&self, size: u32) -> bool {
        (**self).has_sufficient_buffer(size)
    }
}

/// A trait for memory mapping for DAX.
///
/// For some transports (like virtio) it may be possible to share a region of memory with the
/// FUSE kernel driver so that it can access file contents directly without issuing read or
/// write requests.  In this case the driver will instead send requests to map a section of a
/// file into the shared memory region.
pub trait Mapper {
    /// Maps `size` bytes starting at `file_offset` bytes from within the given `fd` at `mem_offset`
    /// bytes from the start of the memory region with `prot` protections. `mem_offset` must be
    /// page aligned.
    ///
    /// # Arguments
    /// * `mem_offset` - Page aligned offset into the memory region in bytes.
    /// * `size` - Size of memory region in bytes.
    /// * `fd` - File descriptor to mmap from.
    /// * `file_offset` - Offset in bytes from the beginning of `fd` to start the mmap.
    /// * `prot` - Protection of the memory region.
    fn map(
        &self,
        mem_offset: u64,
        size: usize,
        fd: &dyn AsRawFd,
        file_offset: u64,
        prot: Protection,
    ) -> io::Result<()>;

    /// Unmaps `size` bytes at `offset` bytes from the start of the memory region. `offset` must be
    /// page aligned.
    ///
    /// # Arguments
    /// * `offset` - Page aligned offset into the arena in bytes.
    /// * `size` - Size of memory region in bytes.
    fn unmap(&self, offset: u64, size: u64) -> io::Result<()>;
}

impl<M: Mapper> Mapper for &M {
    fn map(
        &self,
        mem_offset: u64,
        size: usize,
        fd: &dyn AsRawFd,
        file_offset: u64,
        prot: Protection,
    ) -> io::Result<()> {
        (**self).map(mem_offset, size, fd, file_offset, prot)
    }

    fn unmap(&self, offset: u64, size: u64) -> io::Result<()> {
        (**self).unmap(offset, size)
    }
}

pub struct Server<F: FileSystem + Sync> {
    fs: F,
}

impl<F: FileSystem + Sync> Server<F> {
    pub fn new(fs: F) -> Server<F> {
        Server { fs }
    }

    pub fn handle_message<R: Reader + ZeroCopyReader, W: Writer + ZeroCopyWriter, M: Mapper>(
        &self,
        mut r: R,
        w: W,
        mapper: M,
    ) -> Result<usize> {
        let in_header: InHeader = r.read_struct()?;
        cros_tracing::trace_simple_print!("fuse server: handle_message: in_header={:?}", in_header);

        if in_header.len
            > size_of::<InHeader>() as u32 + size_of::<WriteIn>() as u32 + self.fs.max_buffer_size()
        {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }
        match Opcode::n(in_header.opcode) {
            Some(Opcode::Lookup) => self.lookup(in_header, r, w),
            Some(Opcode::Forget) => self.forget(in_header, r), // No reply.
            Some(Opcode::Getattr) => self.getattr(in_header, r, w),
            Some(Opcode::Setattr) => self.setattr(in_header, r, w),
            Some(Opcode::Readlink) => self.readlink(in_header, w),
            Some(Opcode::Symlink) => self.symlink(in_header, r, w),
            Some(Opcode::Mknod) => self.mknod(in_header, r, w),
            Some(Opcode::Mkdir) => self.mkdir(in_header, r, w),
            Some(Opcode::Unlink) => self.unlink(in_header, r, w),
            Some(Opcode::Rmdir) => self.rmdir(in_header, r, w),
            Some(Opcode::Rename) => self.rename(in_header, r, w),
            Some(Opcode::Link) => self.link(in_header, r, w),
            Some(Opcode::Open) => self.open(in_header, r, w),
            Some(Opcode::Read) => self.read(in_header, r, w),
            Some(Opcode::Write) => self.write(in_header, r, w),
            Some(Opcode::Statfs) => self.statfs(in_header, w),
            Some(Opcode::Release) => self.release(in_header, r, w),
            Some(Opcode::Fsync) => self.fsync(in_header, r, w),
            Some(Opcode::Setxattr) => self.setxattr(in_header, r, w),
            Some(Opcode::Getxattr) => self.getxattr(in_header, r, w),
            Some(Opcode::Listxattr) => self.listxattr(in_header, r, w),
            Some(Opcode::Removexattr) => self.removexattr(in_header, r, w),
            Some(Opcode::Flush) => self.flush(in_header, r, w),
            Some(Opcode::Init) => self.init(in_header, r, w),
            Some(Opcode::Opendir) => self.opendir(in_header, r, w),
            Some(Opcode::Readdir) => self.readdir(in_header, r, w),
            Some(Opcode::Releasedir) => self.releasedir(in_header, r, w),
            Some(Opcode::Fsyncdir) => self.fsyncdir(in_header, r, w),
            Some(Opcode::Getlk) => self.getlk(in_header, r, w),
            Some(Opcode::Setlk) => self.setlk(in_header, r, w),
            Some(Opcode::Setlkw) => self.setlkw(in_header, r, w),
            Some(Opcode::Access) => self.access(in_header, r, w),
            Some(Opcode::Create) => self.create(in_header, r, w),
            Some(Opcode::Interrupt) => self.interrupt(in_header),
            Some(Opcode::Bmap) => self.bmap(in_header, r, w),
            Some(Opcode::Destroy) => self.destroy(),
            Some(Opcode::Ioctl) => self.ioctl(in_header, r, w),
            Some(Opcode::Poll) => self.poll(in_header, r, w),
            Some(Opcode::NotifyReply) => self.notify_reply(in_header, r, w),
            Some(Opcode::BatchForget) => self.batch_forget(in_header, r, w),
            Some(Opcode::Fallocate) => self.fallocate(in_header, r, w),
            Some(Opcode::Readdirplus) => self.readdirplus(in_header, r, w),
            Some(Opcode::Rename2) => self.rename2(in_header, r, w),
            Some(Opcode::Lseek) => self.lseek(in_header, r, w),
            Some(Opcode::CopyFileRange) => self.copy_file_range(in_header, r, w),
            Some(Opcode::ChromeOsTmpfile) => self.chromeos_tmpfile(in_header, r, w),
            Some(Opcode::SetUpMapping) => self.set_up_mapping(in_header, r, w, mapper),
            Some(Opcode::RemoveMapping) => self.remove_mapping(in_header, r, w, mapper),
            Some(Opcode::OpenAtomic) => self.open_atomic(in_header, r, w),
            None => reply_error(
                io::Error::from_raw_os_error(libc::ENOSYS),
                in_header.unique,
                w,
            ),
        }
    }

    fn lookup<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let namelen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .ok_or(Error::InvalidHeaderLength)?;

        let mut buf = vec![0; namelen];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let name = bytes_to_cstr(&buf)?;

        match self
            .fs
            .lookup(Context::from(in_header), in_header.nodeid.into(), name)
        {
            Ok(entry) => {
                let out = EntryOut::from(entry);

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn forget<R: Reader>(&self, in_header: InHeader, mut r: R) -> Result<usize> {
        let ForgetIn { nlookup } = r.read_struct()?;

        self.fs
            .forget(Context::from(in_header), in_header.nodeid.into(), nlookup);

        // There is no reply for forget messages.
        Ok(0)
    }

    fn getattr<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let GetattrIn {
            flags,
            dummy: _,
            fh,
        } = r.read_struct()?;

        let handle = if (flags & GETATTR_FH) != 0 {
            Some(fh.into())
        } else {
            None
        };

        match self
            .fs
            .getattr(Context::from(in_header), in_header.nodeid.into(), handle)
        {
            Ok((st, timeout)) => {
                let out = AttrOut {
                    attr_valid: timeout.as_secs(),
                    attr_valid_nsec: timeout.subsec_nanos(),
                    dummy: 0,
                    attr: st.into(),
                };
                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn setattr<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let setattr_in: SetattrIn = r.read_struct()?;

        let handle = if setattr_in.valid & FATTR_FH != 0 {
            Some(setattr_in.fh.into())
        } else {
            None
        };

        let valid = SetattrValid::from_bits_truncate(setattr_in.valid);

        let st: libc::stat64 = setattr_in.into();

        match self.fs.setattr(
            Context::from(in_header),
            in_header.nodeid.into(),
            st,
            handle,
            valid,
        ) {
            Ok((st, timeout)) => {
                let out = AttrOut {
                    attr_valid: timeout.as_secs(),
                    attr_valid_nsec: timeout.subsec_nanos(),
                    dummy: 0,
                    attr: st.into(),
                };
                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn readlink<W: Writer>(&self, in_header: InHeader, w: W) -> Result<usize> {
        match self
            .fs
            .readlink(Context::from(in_header), in_header.nodeid.into())
        {
            Ok(linkname) => {
                // We need to disambiguate the option type here even though it is `None`.
                reply_ok(None::<u8>, Some(&linkname), in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn symlink<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        // Unfortunately the name and linkname are encoded one after another and
        // separated by a nul character.
        let len = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .ok_or(Error::InvalidHeaderLength)?;
        let mut buf = vec![0; len];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let mut iter = buf.split_inclusive(|&c| c == b'\0');
        let name = iter
            .next()
            .ok_or(Error::MissingParameter)
            .and_then(bytes_to_cstr)?;
        let linkname = iter
            .next()
            .ok_or(Error::MissingParameter)
            .and_then(bytes_to_cstr)?;

        let split_pos = name.to_bytes_with_nul().len() + linkname.to_bytes_with_nul().len();
        let security_ctx = parse_selinux_xattr(&buf[split_pos..])?;

        match self.fs.symlink(
            Context::from(in_header),
            linkname,
            in_header.nodeid.into(),
            name,
            security_ctx,
        ) {
            Ok(entry) => {
                let out = EntryOut::from(entry);

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn mknod<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let MknodIn {
            mode, rdev, umask, ..
        } = r.read_struct()?;

        let buflen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<MknodIn>()))
            .ok_or(Error::InvalidHeaderLength)?;
        let mut buf = vec![0; buflen];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let mut iter = buf.split_inclusive(|&c| c == b'\0');
        let name = iter
            .next()
            .ok_or(Error::MissingParameter)
            .and_then(bytes_to_cstr)?;

        let split_pos = name.to_bytes_with_nul().len();
        let security_ctx = parse_selinux_xattr(&buf[split_pos..])?;

        match self.fs.mknod(
            Context::from(in_header),
            in_header.nodeid.into(),
            name,
            mode,
            rdev,
            umask,
            security_ctx,
        ) {
            Ok(entry) => {
                let out = EntryOut::from(entry);

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn mkdir<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let MkdirIn { mode, umask } = r.read_struct()?;

        let buflen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<MkdirIn>()))
            .ok_or(Error::InvalidHeaderLength)?;
        let mut buf = vec![0; buflen];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let mut iter = buf.split_inclusive(|&c| c == b'\0');
        let name = iter
            .next()
            .ok_or(Error::MissingParameter)
            .and_then(bytes_to_cstr)?;

        let split_pos = name.to_bytes_with_nul().len();
        let security_ctx = parse_selinux_xattr(&buf[split_pos..])?;

        match self.fs.mkdir(
            Context::from(in_header),
            in_header.nodeid.into(),
            name,
            mode,
            umask,
            security_ctx,
        ) {
            Ok(entry) => {
                let out = EntryOut::from(entry);

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn chromeos_tmpfile<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let ChromeOsTmpfileIn { mode, umask } = r.read_struct()?;

        let len = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<ChromeOsTmpfileIn>()))
            .ok_or(Error::InvalidHeaderLength)?;
        let mut buf = vec![0; len];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let security_ctx = parse_selinux_xattr(&buf)?;

        match self.fs.chromeos_tmpfile(
            Context::from(in_header),
            in_header.nodeid.into(),
            mode,
            umask,
            security_ctx,
        ) {
            Ok(entry) => {
                let out = EntryOut::from(entry);

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn unlink<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let namelen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .ok_or(Error::InvalidHeaderLength)?;
        let mut name = vec![0; namelen];

        r.read_exact(&mut name).map_err(Error::DecodeMessage)?;

        match self.fs.unlink(
            Context::from(in_header),
            in_header.nodeid.into(),
            bytes_to_cstr(&name)?,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn rmdir<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let namelen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .ok_or(Error::InvalidHeaderLength)?;
        let mut name = vec![0; namelen];

        r.read_exact(&mut name).map_err(Error::DecodeMessage)?;

        match self.fs.rmdir(
            Context::from(in_header),
            in_header.nodeid.into(),
            bytes_to_cstr(&name)?,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn do_rename<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        msg_size: usize,
        newdir: u64,
        flags: u32,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let buflen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(msg_size))
            .ok_or(Error::InvalidHeaderLength)?;
        let mut buf = vec![0; buflen];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        // We want to include the '\0' byte in the first slice.
        let split_pos = buf
            .iter()
            .position(|c| *c == b'\0')
            .map(|p| p + 1)
            .ok_or(Error::MissingParameter)?;

        let (oldname, newname) = buf.split_at(split_pos);

        match self.fs.rename(
            Context::from(in_header),
            in_header.nodeid.into(),
            bytes_to_cstr(oldname)?,
            newdir.into(),
            bytes_to_cstr(newname)?,
            flags,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn rename<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let RenameIn { newdir } = r.read_struct()?;

        self.do_rename(in_header, size_of::<RenameIn>(), newdir, 0, r, w)
    }

    fn rename2<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let Rename2In { newdir, flags, .. } = r.read_struct()?;

        #[allow(clippy::unnecessary_cast)]
        let flags = flags & (libc::RENAME_EXCHANGE | libc::RENAME_NOREPLACE) as u32;

        self.do_rename(in_header, size_of::<Rename2In>(), newdir, flags, r, w)
    }

    fn link<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let LinkIn { oldnodeid } = r.read_struct()?;

        let namelen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<LinkIn>()))
            .ok_or(Error::InvalidHeaderLength)?;
        let mut name = vec![0; namelen];

        r.read_exact(&mut name).map_err(Error::DecodeMessage)?;

        match self.fs.link(
            Context::from(in_header),
            oldnodeid.into(),
            in_header.nodeid.into(),
            bytes_to_cstr(&name)?,
        ) {
            Ok(entry) => {
                let out = EntryOut::from(entry);

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn open<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let OpenIn { flags, .. } = r.read_struct()?;

        match self
            .fs
            .open(Context::from(in_header), in_header.nodeid.into(), flags)
        {
            Ok((handle, opts)) => {
                let out = OpenOut {
                    fh: handle.map(Into::into).unwrap_or(0),
                    open_flags: opts.bits(),
                    ..Default::default()
                };

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn read<R: Reader, W: ZeroCopyWriter + Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        mut w: W,
    ) -> Result<usize> {
        let ReadIn {
            fh,
            offset,
            size,
            read_flags,
            lock_owner,
            flags,
            ..
        } = r.read_struct()?;

        if size > self.fs.max_buffer_size() {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        let owner = if read_flags & READ_LOCKOWNER != 0 {
            Some(lock_owner)
        } else {
            None
        };

        // Skip for the header size to write the data first.
        match w.write_at(size_of::<OutHeader>(), |writer| {
            self.fs.read(
                Context::from(in_header),
                in_header.nodeid.into(),
                fh.into(),
                writer,
                size,
                offset,
                owner,
                flags,
            )
        }) {
            Ok(count) => {
                // Don't use `reply_ok` because we need to set a custom size length for the
                // header.
                let out = OutHeader {
                    len: (size_of::<OutHeader>() + count) as u32,
                    error: 0,
                    unique: in_header.unique,
                };

                w.write_all(out.as_bytes()).map_err(Error::EncodeMessage)?;
                w.flush().map_err(Error::FlushMessage)?;
                Ok(out.len as usize)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn write<R: Reader + ZeroCopyReader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let WriteIn {
            fh,
            offset,
            size,
            write_flags,
            lock_owner,
            flags,
            ..
        } = r.read_struct()?;

        if size > self.fs.max_buffer_size() {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        let owner = if write_flags & WRITE_LOCKOWNER != 0 {
            Some(lock_owner)
        } else {
            None
        };

        let delayed_write = write_flags & WRITE_CACHE != 0;

        match self.fs.write(
            Context::from(in_header),
            in_header.nodeid.into(),
            fh.into(),
            r,
            size,
            offset,
            owner,
            delayed_write,
            flags,
        ) {
            Ok(count) => {
                let out = WriteOut {
                    size: count as u32,
                    ..Default::default()
                };

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn statfs<W: Writer>(&self, in_header: InHeader, w: W) -> Result<usize> {
        match self
            .fs
            .statfs(Context::from(in_header), in_header.nodeid.into())
        {
            Ok(st) => reply_ok(Some(Kstatfs::from(st)), None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn release<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let ReleaseIn {
            fh,
            flags,
            release_flags,
            lock_owner,
        } = r.read_struct()?;

        let flush = release_flags & RELEASE_FLUSH != 0;
        let flock_release = release_flags & RELEASE_FLOCK_UNLOCK != 0;
        let lock_owner = if flush || flock_release {
            Some(lock_owner)
        } else {
            None
        };

        match self.fs.release(
            Context::from(in_header),
            in_header.nodeid.into(),
            flags,
            fh.into(),
            flush,
            flock_release,
            lock_owner,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn fsync<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let FsyncIn {
            fh, fsync_flags, ..
        } = r.read_struct()?;
        let datasync = fsync_flags & 0x1 != 0;

        match self.fs.fsync(
            Context::from(in_header),
            in_header.nodeid.into(),
            datasync,
            fh.into(),
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn setxattr<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let SetxattrIn { size, flags } = r.read_struct()?;

        // The name and value and encoded one after another and separated by a '\0' character.
        let len = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<SetxattrIn>()))
            .ok_or(Error::InvalidHeaderLength)?;
        let mut buf = vec![0; len];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        // We want to include the '\0' byte in the first slice.
        let split_pos = buf
            .iter()
            .position(|c| *c == b'\0')
            .map(|p| p + 1)
            .ok_or(Error::MissingParameter)?;

        let (name, value) = buf.split_at(split_pos);

        if size != value.len() as u32 {
            return Err(Error::InvalidXattrSize(size, value.len()));
        }

        match self.fs.setxattr(
            Context::from(in_header),
            in_header.nodeid.into(),
            bytes_to_cstr(name)?,
            value,
            flags,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn getxattr<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let GetxattrIn { size, .. } = r.read_struct()?;

        let namelen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<GetxattrIn>()))
            .ok_or(Error::InvalidHeaderLength)?;
        let mut name = vec![0; namelen];

        r.read_exact(&mut name).map_err(Error::DecodeMessage)?;

        if size > self.fs.max_buffer_size() {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        match self.fs.getxattr(
            Context::from(in_header),
            in_header.nodeid.into(),
            bytes_to_cstr(&name)?,
            size,
        ) {
            Ok(GetxattrReply::Value(val)) => reply_ok(None::<u8>, Some(&val), in_header.unique, w),
            Ok(GetxattrReply::Count(count)) => {
                let out = GetxattrOut {
                    size: count,
                    ..Default::default()
                };

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn listxattr<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let GetxattrIn { size, .. } = r.read_struct()?;

        if size > self.fs.max_buffer_size() {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        match self
            .fs
            .listxattr(Context::from(in_header), in_header.nodeid.into(), size)
        {
            Ok(ListxattrReply::Names(val)) => reply_ok(None::<u8>, Some(&val), in_header.unique, w),
            Ok(ListxattrReply::Count(count)) => {
                let out = GetxattrOut {
                    size: count,
                    ..Default::default()
                };

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn removexattr<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let namelen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .ok_or(Error::InvalidHeaderLength)?;

        let mut buf = vec![0; namelen];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let name = bytes_to_cstr(&buf)?;

        match self
            .fs
            .removexattr(Context::from(in_header), in_header.nodeid.into(), name)
        {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn flush<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let FlushIn {
            fh,
            unused: _,
            padding: _,
            lock_owner,
        } = r.read_struct()?;

        match self.fs.flush(
            Context::from(in_header),
            in_header.nodeid.into(),
            fh.into(),
            lock_owner,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn init<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        cros_tracing::trace_simple_print!("fuse server: init: in_header={:?}", in_header);
        let InitIn {
            major,
            minor,
            max_readahead,
            flags,
        } = r.read_struct()?;

        if major < KERNEL_VERSION {
            error!("Unsupported fuse protocol version: {}.{}", major, minor);
            return reply_error(
                io::Error::from_raw_os_error(libc::EPROTO),
                in_header.unique,
                w,
            );
        }

        if major > KERNEL_VERSION {
            // Wait for the kernel to reply back with a 7.X version.
            let out = InitOut {
                major: KERNEL_VERSION,
                minor: KERNEL_MINOR_VERSION,
                ..Default::default()
            };

            return reply_ok(Some(out), None, in_header.unique, w);
        }

        if minor < OLDEST_SUPPORTED_KERNEL_MINOR_VERSION {
            error!(
                "Unsupported fuse protocol minor version: {}.{}",
                major, minor
            );
            return reply_error(
                io::Error::from_raw_os_error(libc::EPROTO),
                in_header.unique,
                w,
            );
        }

        let InitInExt { flags2, .. } =
            if (FsOptions::from_bits_truncate(u64::from(flags)) & FsOptions::INIT_EXT).is_empty() {
                InitInExt::default()
            } else {
                r.read_struct()?
            };

        // These fuse features are supported by this server by default.
        let supported = FsOptions::ASYNC_READ
            | FsOptions::PARALLEL_DIROPS
            | FsOptions::BIG_WRITES
            | FsOptions::AUTO_INVAL_DATA
            | FsOptions::HANDLE_KILLPRIV
            | FsOptions::ASYNC_DIO
            | FsOptions::HAS_IOCTL_DIR
            | FsOptions::DO_READDIRPLUS
            | FsOptions::READDIRPLUS_AUTO
            | FsOptions::ATOMIC_O_TRUNC
            | FsOptions::MAX_PAGES
            | FsOptions::MAP_ALIGNMENT
            | FsOptions::INIT_EXT;

        let capable = FsOptions::from_bits_truncate(u64::from(flags) | u64::from(flags2) << 32);

        match self.fs.init(capable) {
            Ok(want) => {
                let mut enabled = capable & (want | supported);

                // HANDLE_KILLPRIV doesn't work correctly when writeback caching is enabled so turn
                // it off.
                if enabled.contains(FsOptions::WRITEBACK_CACHE) {
                    enabled.remove(FsOptions::HANDLE_KILLPRIV);
                }

                // ATOMIC_O_TRUNC doesn't work with ZERO_MESSAGE_OPEN.
                if enabled.contains(FsOptions::ZERO_MESSAGE_OPEN) {
                    enabled.remove(FsOptions::ATOMIC_O_TRUNC);
                }

                let max_write = self.fs.max_buffer_size();
                let max_pages = min(
                    max(max_readahead, max_write) / pagesize() as u32,
                    u16::MAX as u32,
                ) as u16;
                let out = InitOut {
                    major: KERNEL_VERSION,
                    minor: KERNEL_MINOR_VERSION,
                    max_readahead,
                    flags: enabled.bits() as u32,
                    max_background: u16::MAX,
                    congestion_threshold: (u16::MAX / 4) * 3,
                    max_write,
                    time_gran: 1, // nanoseconds
                    max_pages,
                    map_alignment: pagesize().trailing_zeros() as u16,
                    flags2: (enabled.bits() >> 32) as u32,
                    ..Default::default()
                };

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn opendir<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let OpenIn { flags, .. } = r.read_struct()?;

        match self
            .fs
            .opendir(Context::from(in_header), in_header.nodeid.into(), flags)
        {
            Ok((handle, opts)) => {
                let out = OpenOut {
                    fh: handle.map(Into::into).unwrap_or(0),
                    open_flags: opts.bits(),
                    ..Default::default()
                };

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn readdir<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        mut w: W,
    ) -> Result<usize> {
        let ReadIn {
            fh, offset, size, ..
        } = r.read_struct()?;

        if size > self.fs.max_buffer_size() {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        if !w.has_sufficient_buffer(size) {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        // Skip over enough bytes for the header.
        let unique = in_header.unique;
        let result = w.write_at(size_of::<OutHeader>(), |cursor| {
            match self.fs.readdir(
                Context::from(in_header),
                in_header.nodeid.into(),
                fh.into(),
                size,
                offset,
            ) {
                Ok(mut entries) => {
                    let mut total_written = 0;
                    while let Some(dirent) = entries.next() {
                        let remaining = (size as usize).saturating_sub(total_written);
                        match add_dirent(cursor, remaining, &dirent, None) {
                            // No more space left in the buffer.
                            Ok(0) => break,
                            Ok(bytes_written) => {
                                total_written += bytes_written;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(total_written)
                }
                Err(e) => Err(e),
            }
        });

        match result {
            Ok(total_written) => reply_readdir(total_written, unique, w),
            Err(e) => reply_error(e, unique, w),
        }
    }

    fn lookup_dirent_attribute(
        &self,
        in_header: &InHeader,
        dir_entry: &DirEntry,
    ) -> io::Result<Entry> {
        let parent = in_header.nodeid.into();
        let name = dir_entry.name.to_bytes();
        let entry = if name == b"." || name == b".." {
            // Don't do lookups on the current directory or the parent directory.
            // SAFETY: struct only contains integer fields and any value is valid.
            let mut attr = unsafe { MaybeUninit::<libc::stat64>::zeroed().assume_init() };
            attr.st_ino = dir_entry.ino;
            attr.st_mode = dir_entry.type_;

            // We use 0 for the inode value to indicate a negative entry.
            Entry {
                inode: 0,
                generation: 0,
                attr,
                attr_timeout: Duration::from_secs(0),
                entry_timeout: Duration::from_secs(0),
            }
        } else {
            self.fs
                .lookup(Context::from(*in_header), parent, dir_entry.name)?
        };

        Ok(entry)
    }

    fn readdirplus<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        mut w: W,
    ) -> Result<usize> {
        cros_tracing::trace_simple_print!("fuse server: readdirplus: in_header={:?}", in_header);
        let ReadIn {
            fh, offset, size, ..
        } = r.read_struct()?;

        if size > self.fs.max_buffer_size() {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        if !w.has_sufficient_buffer(size) {
            return reply_error(
                io::Error::from_raw_os_error(libc::ENOMEM),
                in_header.unique,
                w,
            );
        }

        // Skip over enough bytes for the header.
        let unique = in_header.unique;
        let result = w.write_at(size_of::<OutHeader>(), |cursor| {
            match self.fs.readdir(
                Context::from(in_header),
                in_header.nodeid.into(),
                fh.into(),
                size,
                offset,
            ) {
                Ok(mut entries) => {
                    let mut total_written = 0;
                    while let Some(dirent) = entries.next() {
                        let mut entry_inode = None;
                        let dirent_result = self
                            .lookup_dirent_attribute(&in_header, &dirent)
                            .and_then(|e| {
                                entry_inode = Some(e.inode);
                                let remaining = (size as usize).saturating_sub(total_written);
                                add_dirent(cursor, remaining, &dirent, Some(e))
                            });

                        match dirent_result {
                            Ok(0) => {
                                // No more space left in the buffer but we need to undo the lookup
                                // that created the Entry or we will end up with mismatched lookup
                                // counts.
                                if let Some(inode) = entry_inode {
                                    self.fs.forget(Context::from(in_header), inode.into(), 1);
                                }
                                break;
                            }
                            Ok(bytes_written) => {
                                total_written += bytes_written;
                            }
                            Err(e) => {
                                if let Some(inode) = entry_inode {
                                    self.fs.forget(Context::from(in_header), inode.into(), 1);
                                }

                                if total_written == 0 {
                                    // We haven't filled any entries yet so we can just propagate
                                    // the error.
                                    return Err(e);
                                }

                                // We already filled in some entries. Returning an error now will
                                // cause lookup count mismatches for those entries so just return
                                // whatever we already have.
                                break;
                            }
                        }
                    }
                    Ok(total_written)
                }
                Err(e) => Err(e),
            }
        });

        match result {
            Ok(total_written) => reply_readdir(total_written, unique, w),
            Err(e) => reply_error(e, unique, w),
        }
    }

    fn releasedir<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let ReleaseIn { fh, flags, .. } = r.read_struct()?;

        match self.fs.releasedir(
            Context::from(in_header),
            in_header.nodeid.into(),
            flags,
            fh.into(),
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn fsyncdir<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let FsyncIn {
            fh, fsync_flags, ..
        } = r.read_struct()?;
        let datasync = fsync_flags & 0x1 != 0;

        match self.fs.fsyncdir(
            Context::from(in_header),
            in_header.nodeid.into(),
            datasync,
            fh.into(),
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn getlk<R: Reader, W: Writer>(&self, in_header: InHeader, mut _r: R, w: W) -> Result<usize> {
        if let Err(e) = self.fs.getlk() {
            reply_error(e, in_header.unique, w)
        } else {
            Ok(0)
        }
    }

    fn setlk<R: Reader, W: Writer>(&self, in_header: InHeader, mut _r: R, w: W) -> Result<usize> {
        if let Err(e) = self.fs.setlk() {
            reply_error(e, in_header.unique, w)
        } else {
            Ok(0)
        }
    }

    fn setlkw<R: Reader, W: Writer>(&self, in_header: InHeader, mut _r: R, w: W) -> Result<usize> {
        if let Err(e) = self.fs.setlkw() {
            reply_error(e, in_header.unique, w)
        } else {
            Ok(0)
        }
    }

    fn access<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let AccessIn { mask, .. } = r.read_struct()?;

        match self
            .fs
            .access(Context::from(in_header), in_header.nodeid.into(), mask)
        {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn create<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let CreateIn {
            flags, mode, umask, ..
        } = r.read_struct()?;

        let buflen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<CreateIn>()))
            .ok_or(Error::InvalidHeaderLength)?;

        let mut buf = vec![0; buflen];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let mut iter = buf.split_inclusive(|&c| c == b'\0');
        let name = iter
            .next()
            .ok_or(Error::MissingParameter)
            .and_then(bytes_to_cstr)?;

        let split_pos = name.to_bytes_with_nul().len();
        let security_ctx = parse_selinux_xattr(&buf[split_pos..])?;

        match self.fs.create(
            Context::from(in_header),
            in_header.nodeid.into(),
            name,
            mode,
            flags,
            umask,
            security_ctx,
        ) {
            Ok((entry, handle, opts)) => {
                let entry_out = EntryOut {
                    nodeid: entry.inode,
                    generation: entry.generation,
                    entry_valid: entry.entry_timeout.as_secs(),
                    attr_valid: entry.attr_timeout.as_secs(),
                    entry_valid_nsec: entry.entry_timeout.subsec_nanos(),
                    attr_valid_nsec: entry.attr_timeout.subsec_nanos(),
                    attr: entry.attr.into(),
                };
                let open_out = OpenOut {
                    fh: handle.map(Into::into).unwrap_or(0),
                    open_flags: opts.bits(),
                    ..Default::default()
                };

                // Kind of a hack to write both structs.
                reply_ok(
                    Some(entry_out),
                    Some(open_out.as_bytes()),
                    in_header.unique,
                    w,
                )
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn interrupt(&self, _in_header: InHeader) -> Result<usize> {
        Ok(0)
    }

    fn bmap<R: Reader, W: Writer>(&self, in_header: InHeader, mut _r: R, w: W) -> Result<usize> {
        if let Err(e) = self.fs.bmap() {
            reply_error(e, in_header.unique, w)
        } else {
            Ok(0)
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn destroy(&self) -> Result<usize> {
        // No reply to this function.
        self.fs.destroy();

        Ok(0)
    }

    fn ioctl<R: Reader, W: Writer>(&self, in_header: InHeader, mut r: R, w: W) -> Result<usize> {
        let IoctlIn {
            fh,
            flags,
            cmd,
            arg,
            in_size,
            out_size,
        } = r.read_struct()?;

        let res = self.fs.ioctl(
            in_header.into(),
            in_header.nodeid.into(),
            fh.into(),
            IoctlFlags::from_bits_truncate(flags),
            cmd,
            arg,
            in_size,
            out_size,
            r,
        );

        match res {
            Ok(reply) => match reply {
                IoctlReply::Retry { input, output } => {
                    retry_ioctl(in_header.unique, input, output, w)
                }
                IoctlReply::Done(res) => finish_ioctl(in_header.unique, res, w),
            },
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn poll<R: Reader, W: Writer>(&self, in_header: InHeader, mut _r: R, w: W) -> Result<usize> {
        if let Err(e) = self.fs.poll() {
            reply_error(e, in_header.unique, w)
        } else {
            Ok(0)
        }
    }

    fn notify_reply<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut _r: R,
        w: W,
    ) -> Result<usize> {
        if let Err(e) = self.fs.notify_reply() {
            reply_error(e, in_header.unique, w)
        } else {
            Ok(0)
        }
    }

    fn batch_forget<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let BatchForgetIn { count, .. } = r.read_struct()?;

        if let Some(size) = (count as usize).checked_mul(size_of::<ForgetOne>()) {
            if size > self.fs.max_buffer_size() as usize {
                return reply_error(
                    io::Error::from_raw_os_error(libc::ENOMEM),
                    in_header.unique,
                    w,
                );
            }
        } else {
            return reply_error(
                io::Error::from_raw_os_error(libc::EOVERFLOW),
                in_header.unique,
                w,
            );
        }

        let mut requests = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let f: ForgetOne = r.read_struct()?;
            requests.push((f.nodeid.into(), f.nlookup));
        }

        self.fs.batch_forget(Context::from(in_header), requests);

        // No reply for forget messages.
        Ok(0)
    }

    fn fallocate<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let FallocateIn {
            fh,
            offset,
            length,
            mode,
            ..
        } = r.read_struct()?;

        match self.fs.fallocate(
            Context::from(in_header),
            in_header.nodeid.into(),
            fh.into(),
            mode,
            offset,
            length,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn lseek<R: Reader, W: Writer>(&self, in_header: InHeader, mut _r: R, w: W) -> Result<usize> {
        if let Err(e) = self.fs.lseek() {
            reply_error(e, in_header.unique, w)
        } else {
            Ok(0)
        }
    }

    fn copy_file_range<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let CopyFileRangeIn {
            fh_src,
            off_src,
            nodeid_dst,
            fh_dst,
            off_dst,
            len,
            flags,
        } = r.read_struct()?;

        match self.fs.copy_file_range(
            Context::from(in_header),
            in_header.nodeid.into(),
            fh_src.into(),
            off_src,
            nodeid_dst.into(),
            fh_dst.into(),
            off_dst,
            len,
            flags,
        ) {
            Ok(count) => {
                let out = WriteOut {
                    size: count as u32,
                    ..Default::default()
                };

                reply_ok(Some(out), None, in_header.unique, w)
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn set_up_mapping<R, W, M>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
        mapper: M,
    ) -> Result<usize>
    where
        R: Reader,
        W: Writer,
        M: Mapper,
    {
        let SetUpMappingIn {
            fh,
            foffset,
            len,
            flags,
            moffset,
        } = r.read_struct()?;
        let flags = SetUpMappingFlags::from_bits_truncate(flags);

        let mut prot = 0;
        if flags.contains(SetUpMappingFlags::READ) {
            prot |= libc::PROT_READ as u32;
        }
        if flags.contains(SetUpMappingFlags::WRITE) {
            prot |= libc::PROT_WRITE as u32;
        }

        let size = if let Ok(s) = len.try_into() {
            s
        } else {
            return reply_error(
                io::Error::from_raw_os_error(libc::EOVERFLOW),
                in_header.unique,
                w,
            );
        };

        match self.fs.set_up_mapping(
            Context::from(in_header),
            in_header.nodeid.into(),
            fh.into(),
            foffset,
            moffset,
            size,
            prot,
            mapper,
        ) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => {
                error!("set_up_mapping failed: {}", e);
                reply_error(e, in_header.unique, w)
            }
        }
    }

    fn remove_mapping<R, W, M>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
        mapper: M,
    ) -> Result<usize>
    where
        R: Reader,
        W: Writer,
        M: Mapper,
    {
        let RemoveMappingIn { count } = r.read_struct()?;

        // `FUSE_REMOVEMAPPING_MAX_ENTRY` is defined as
        // `PAGE_SIZE / sizeof(struct fuse_removemapping_one)` in /kernel/include/uapi/linux/fuse.h.
        let max_entry = pagesize() / std::mem::size_of::<RemoveMappingOne>();

        if max_entry < count as usize {
            return reply_error(
                io::Error::from_raw_os_error(libc::EINVAL),
                in_header.unique,
                w,
            );
        }

        let mut msgs = Vec::with_capacity(count as usize);
        for _ in 0..(count as usize) {
            let msg: RemoveMappingOne = r.read_struct()?;
            msgs.push(msg);
        }

        match self.fs.remove_mapping(&msgs, mapper) {
            Ok(()) => reply_ok(None::<u8>, None, in_header.unique, w),
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }

    fn open_atomic<R: Reader, W: Writer>(
        &self,
        in_header: InHeader,
        mut r: R,
        w: W,
    ) -> Result<usize> {
        let CreateIn {
            flags, mode, umask, ..
        } = r.read_struct()?;

        let buflen = (in_header.len as usize)
            .checked_sub(size_of::<InHeader>())
            .and_then(|l| l.checked_sub(size_of::<CreateIn>()))
            .ok_or(Error::InvalidHeaderLength)?;

        let mut buf = vec![0; buflen];

        r.read_exact(&mut buf).map_err(Error::DecodeMessage)?;

        let mut iter = buf.split_inclusive(|&c| c == b'\0');
        let name = iter
            .next()
            .ok_or(Error::MissingParameter)
            .and_then(bytes_to_cstr)?;

        let split_pos = name.to_bytes_with_nul().len();
        let security_ctx = parse_selinux_xattr(&buf[split_pos..])?;

        match self.fs.atomic_open(
            Context::from(in_header),
            in_header.nodeid.into(),
            name,
            mode,
            flags,
            umask,
            security_ctx,
        ) {
            Ok((entry, handle, opts)) => {
                let entry_out = EntryOut {
                    nodeid: entry.inode,
                    generation: entry.generation,
                    entry_valid: entry.entry_timeout.as_secs(),
                    attr_valid: entry.attr_timeout.as_secs(),
                    entry_valid_nsec: entry.entry_timeout.subsec_nanos(),
                    attr_valid_nsec: entry.attr_timeout.subsec_nanos(),
                    attr: entry.attr.into(),
                };
                let open_out = OpenOut {
                    fh: handle.map(Into::into).unwrap_or(0),
                    open_flags: opts.bits(),
                    ..Default::default()
                };

                // open_out passed the `data` argument, but the two out structs are independent
                // This is a hack to return two out stucts in one fuse reply
                reply_ok(
                    Some(entry_out),
                    Some(open_out.as_bytes()),
                    in_header.unique,
                    w,
                )
            }
            Err(e) => reply_error(e, in_header.unique, w),
        }
    }
}

fn retry_ioctl<W: Writer>(
    unique: u64,
    input: Vec<IoctlIovec>,
    output: Vec<IoctlIovec>,
    mut w: W,
) -> Result<usize> {
    // We don't need to check for overflow here because if adding these 2 values caused an overflow
    // we would have run out of memory before reaching this point.
    if input.len() + output.len() > IOCTL_MAX_IOV {
        return Err(Error::TooManyIovecs(
            input.len() + output.len(),
            IOCTL_MAX_IOV,
        ));
    }

    let len = size_of::<OutHeader>()
        + size_of::<IoctlOut>()
        + (input.len() * size_of::<IoctlIovec>())
        + (output.len() * size_of::<IoctlIovec>());
    let header = OutHeader {
        len: len as u32,
        error: 0,
        unique,
    };
    let out = IoctlOut {
        result: 0,
        flags: IoctlFlags::RETRY.bits(),
        in_iovs: input.len() as u32,
        out_iovs: output.len() as u32,
    };

    let mut total_bytes = size_of::<OutHeader>() + size_of::<IoctlOut>();
    w.write_all(header.as_bytes())
        .map_err(Error::EncodeMessage)?;
    w.write_all(out.as_bytes()).map_err(Error::EncodeMessage)?;
    for i in input.into_iter().chain(output.into_iter()) {
        total_bytes += i.as_bytes().len();
        w.write_all(i.as_bytes()).map_err(Error::EncodeMessage)?;
    }

    w.flush().map_err(Error::FlushMessage)?;
    debug_assert_eq!(len, total_bytes);
    Ok(len)
}

fn finish_ioctl<W: Writer>(unique: u64, res: io::Result<Vec<u8>>, w: W) -> Result<usize> {
    let (out, data) = match res {
        Ok(data) => {
            let out = IoctlOut {
                result: 0,
                ..Default::default()
            };
            (out, Some(data))
        }
        Err(e) => {
            let out = IoctlOut {
                result: -e.raw_os_error().unwrap_or(libc::EIO),
                ..Default::default()
            };
            (out, None)
        }
    };
    reply_ok(Some(out), data.as_ref().map(|d| &d[..]), unique, w)
}

fn reply_readdir<W: Writer>(len: usize, unique: u64, mut w: W) -> Result<usize> {
    let out = OutHeader {
        len: (size_of::<OutHeader>() + len) as u32,
        error: 0,
        unique,
    };

    w.write_all(out.as_bytes()).map_err(Error::EncodeMessage)?;
    w.flush().map_err(Error::FlushMessage)?;
    Ok(out.len as usize)
}

fn reply_ok<T: IntoBytes + Immutable, W: Writer>(
    out: Option<T>,
    data: Option<&[u8]>,
    unique: u64,
    mut w: W,
) -> Result<usize> {
    let mut len = size_of::<OutHeader>();

    if out.is_some() {
        len += size_of::<T>();
    }

    if let Some(data) = data {
        len += data.len();
    }

    let header = OutHeader {
        len: len as u32,
        error: 0,
        unique,
    };

    let mut total_bytes = size_of::<OutHeader>();
    w.write_all(header.as_bytes())
        .map_err(Error::EncodeMessage)?;

    if let Some(out) = out {
        total_bytes += out.as_bytes().len();
        w.write_all(out.as_bytes()).map_err(Error::EncodeMessage)?;
    }

    if let Some(data) = data {
        total_bytes += data.len();
        w.write_all(data).map_err(Error::EncodeMessage)?;
    }

    w.flush().map_err(Error::FlushMessage)?;
    debug_assert_eq!(len, total_bytes);
    Ok(len)
}

fn reply_error<W: Writer>(e: io::Error, unique: u64, mut w: W) -> Result<usize> {
    let header = OutHeader {
        len: size_of::<OutHeader>() as u32,
        error: -e.raw_os_error().unwrap_or(libc::EIO),
        unique,
    };

    w.write_all(header.as_bytes())
        .map_err(Error::EncodeMessage)?;
    w.flush().map_err(Error::FlushMessage)?;

    Ok(header.len as usize)
}

fn bytes_to_cstr(buf: &[u8]) -> Result<&CStr> {
    // Convert to a `CStr` first so that we can drop the '\0' byte at the end
    // and make sure there are no interior '\0' bytes.
    CStr::from_bytes_with_nul(buf).map_err(Error::InvalidCString)
}

fn add_dirent<W: Writer>(
    cursor: &mut W,
    max: usize,
    d: &DirEntry,
    entry: Option<Entry>,
) -> io::Result<usize> {
    // Strip the trailing '\0'.
    let name = d.name.to_bytes();
    if name.len() > u32::MAX as usize {
        return Err(io::Error::from_raw_os_error(libc::EOVERFLOW));
    }

    let dirent_len = size_of::<Dirent>()
        .checked_add(name.len())
        .ok_or_else(|| io::Error::from_raw_os_error(libc::EOVERFLOW))?;

    // Directory entries must be padded to 8-byte alignment.  If adding 7 causes
    // an overflow then this dirent cannot be properly padded.
    let padded_dirent_len = dirent_len
        .checked_add(7)
        .map(|l| l & !7)
        .ok_or_else(|| io::Error::from_raw_os_error(libc::EOVERFLOW))?;

    let total_len = if entry.is_some() {
        padded_dirent_len
            .checked_add(size_of::<EntryOut>())
            .ok_or_else(|| io::Error::from_raw_os_error(libc::EOVERFLOW))?
    } else {
        padded_dirent_len
    };

    if max < total_len {
        Ok(0)
    } else {
        if let Some(entry) = entry {
            cursor.write_all(EntryOut::from(entry).as_bytes())?;
        }

        let dirent = Dirent {
            ino: d.ino,
            off: d.offset,
            namelen: name.len() as u32,
            type_: d.type_,
        };

        cursor.write_all(dirent.as_bytes())?;
        cursor.write_all(name)?;

        // We know that `dirent_len` <= `padded_dirent_len` due to the check above
        // so there's no need for checked arithmetic.
        let padding = padded_dirent_len - dirent_len;
        if padding > 0 {
            cursor.write_all(&DIRENT_PADDING[..padding])?;
        }

        Ok(total_len)
    }
}

/// Parses the value of the desired attribute from the FUSE request input and returns it as an
/// `Ok(CStr)`. Returns `Ok(None)` if `buf` is empty or appears to be a valid request extension that
/// does not contain any security context information.
///
/// # Arguments
///
/// * `buf` - a byte array that contains the contents following any expected byte string parameters
///   of the FUSE request from the server. It begins with a struct `SecctxHeader`, and then the
///   subsequent entry is a struct `Secctx` followed by a nul-terminated string with the xattribute
///   name and then another nul-terminated string with the value for that xattr.
///
/// # Errors
///
/// * `Error::InvalidHeaderLength` - indicates that there is an inconsistency between the size of
///   the data read from `buf` and the stated `size` of the `SecctxHeader`, the respective `Secctx`
///   struct, or `buf` itself.
/// * `Error::DecodeMessage` - indicates that the expected structs cannot be read from `buf`.
/// * `Error::MissingParameter` - indicates that either a security context `name` or `value` is
///   missing from a security context entry.
fn parse_selinux_xattr(buf: &[u8]) -> Result<Option<&CStr>> {
    // Return early if request was not followed by context information
    if buf.is_empty() {
        return Ok(None);
    } else if buf.len() < size_of::<SecctxHeader>() {
        return Err(Error::InvalidHeaderLength);
    }

    // Because the security context data block may have been preceded by variable-length strings,
    // `SecctxHeader` and the subsequent `Secctx` structs may not be correctly byte-aligned
    // within `buf`.
    let (secctx_header, _) = SecctxHeader::read_from_prefix(buf)
        .map_err(|_| Error::DecodeMessage(io::Error::from_raw_os_error(libc::EINVAL)))?;

    // FUSE 7.38 introduced a generic request extension with the same structure as  `SecctxHeader`.
    // A `nr_secctx` value above `MAX_NR_SECCTX` indicates that this data block does not contain
    // any security context information.
    if secctx_header.nr_secctx > MAX_NR_SECCTX {
        return Ok(None);
    }

    let mut cur_secctx_pos = size_of::<SecctxHeader>();
    for _ in 0..secctx_header.nr_secctx {
        // `SecctxHeader.size` denotes the total size for the `SecctxHeader`, each of the
        // `nr_secctx` `Secctx` structs along with the corresponding context name and value,
        // and any additional padding.
        if (cur_secctx_pos + size_of::<Secctx>()) > buf.len()
            || (cur_secctx_pos + size_of::<Secctx>()) > secctx_header.size as usize
        {
            return Err(Error::InvalidHeaderLength);
        }

        let secctx =
            Secctx::read_from_bytes(&buf[cur_secctx_pos..(cur_secctx_pos + size_of::<Secctx>())])
                .map_err(|_| Error::DecodeMessage(io::Error::from_raw_os_error(libc::EINVAL)))?;

        cur_secctx_pos += size_of::<Secctx>();

        let secctx_data = &buf[cur_secctx_pos..]
            .split_inclusive(|&c| c == b'\0')
            .take(2)
            .map(bytes_to_cstr)
            .collect::<Result<Vec<&CStr>>>()?;

        if secctx_data.len() != 2 {
            return Err(Error::MissingParameter);
        }

        let name = secctx_data[0];
        let value = secctx_data[1];

        cur_secctx_pos += name.to_bytes_with_nul().len() + value.to_bytes_with_nul().len();
        if cur_secctx_pos > secctx_header.size as usize {
            return Err(Error::InvalidHeaderLength);
        }

        // `Secctx.size` contains the size of the security context value (not including the
        // corresponding context name).
        if value.to_bytes_with_nul().len() as u32 != secctx.size {
            return Err(Error::InvalidHeaderLength);
        }

        if name.to_bytes_with_nul() == SELINUX_XATTR_CSTR {
            return Ok(Some(value));
        }
    }

    // `SecctxHeader.size` is always the total size of the security context data padded to an
    // 8-byte alignment. If adding 7 causes an overflow, then the `size` field of our header
    // is invalid, so we should return an error.
    let padded_secctx_size = cur_secctx_pos
        .checked_next_multiple_of(8)
        .ok_or(Error::InvalidHeaderLength)?;
    if padded_secctx_size != secctx_header.size as usize {
        return Err(Error::InvalidHeaderLength);
    }

    // None of the `nr_secctx` attributes we parsed had a `name` matching `SELINUX_XATTR_CSTR`.
    // Return `Ok(None)` to indicate that the security context data block was valid but there was no
    // specified selinux label attached to this request.
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_secctx(ctxs: &[(&[u8], &[u8])], size_truncation: u32) -> Vec<u8> {
        let nr_secctx = ctxs.len();
        let total_size = (size_of::<SecctxHeader>() as u32
            + (size_of::<Secctx>() * nr_secctx) as u32
            + ctxs
                .iter()
                .fold(0, |s, &(n, v)| s + n.len() as u32 + v.len() as u32))
        .checked_add(7)
        .map(|l| l & !7)
        .expect("total_size padded to 8-byte boundary")
        .checked_sub(size_truncation)
        .expect("size truncated by bytes < total_size");

        let ctx_data: Vec<_> = ctxs
            .iter()
            .map(|(n, v)| {
                [
                    Secctx {
                        size: v.len() as u32,
                        padding: 0,
                    }
                    .as_bytes(),
                    n,
                    v,
                ]
                .concat()
            })
            .collect::<Vec<_>>()
            .concat();

        [
            SecctxHeader {
                size: total_size,
                nr_secctx: nr_secctx as u32,
            }
            .as_bytes(),
            ctx_data.as_slice(),
        ]
        .concat()
    }

    #[test]
    fn parse_selinux_xattr_empty() {
        let v: Vec<u8> = vec![];
        let res = parse_selinux_xattr(&v);
        assert_eq!(res.unwrap(), None);
    }

    #[test]
    fn parse_selinux_xattr_basic() {
        let sec_value = CStr::from_bytes_with_nul(b"user_u:object_r:security_type:s0\0").unwrap();
        let v = create_secctx(&[(SELINUX_XATTR_CSTR, sec_value.to_bytes_with_nul())], 0);

        let res = parse_selinux_xattr(&v);
        assert_eq!(res.unwrap(), Some(sec_value));
    }

    #[test]
    fn parse_selinux_xattr_find_attr() {
        let foo_value: &CStr =
            CStr::from_bytes_with_nul(b"user_foo:object_foo:foo_type:s0\0").unwrap();
        let sec_value: &CStr =
            CStr::from_bytes_with_nul(b"user_u:object_r:security_type:s0\0").unwrap();
        let v = create_secctx(
            &[
                (b"foo\0", foo_value.to_bytes_with_nul()),
                (SELINUX_XATTR_CSTR, sec_value.to_bytes_with_nul()),
            ],
            0,
        );

        let res = parse_selinux_xattr(&v);
        assert_eq!(res.unwrap(), Some(sec_value));
    }

    #[test]
    fn parse_selinux_xattr_wrong_attr() {
        // Test with an xattr name that looks similar to security.selinux, but has extra
        // characters to ensure that `parse_selinux_xattr` will not return the associated
        // context value to the caller.
        let invalid_selinux_value: &CStr =
            CStr::from_bytes_with_nul(b"user_invalid:object_invalid:invalid_type:s0\0").unwrap();
        let v = create_secctx(
            &[(
                b"invalid.security.selinux\0",
                invalid_selinux_value.to_bytes_with_nul(),
            )],
            0,
        );

        let res = parse_selinux_xattr(&v);
        assert_eq!(res.unwrap(), None);
    }

    #[test]
    fn parse_selinux_xattr_too_short() {
        // Test that parse_selinux_xattr will return an `Error::InvalidHeaderLength` when
        // the total size in the `SecctxHeader` does not encompass the entirety of the
        // associated data.
        let foo_value: &CStr =
            CStr::from_bytes_with_nul(b"user_foo:object_foo:foo_type:s0\0").unwrap();
        let sec_value: &CStr =
            CStr::from_bytes_with_nul(b"user_u:object_r:security_type:s0\0").unwrap();
        let v = create_secctx(
            &[
                (b"foo\0", foo_value.to_bytes_with_nul()),
                (SELINUX_XATTR_CSTR, sec_value.to_bytes_with_nul()),
            ],
            8,
        );

        let res = parse_selinux_xattr(&v);
        assert!(matches!(res, Err(Error::InvalidHeaderLength)));
    }
}
