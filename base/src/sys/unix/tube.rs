// Copyright 2021 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::os::unix::prelude::AsRawFd;
use std::os::unix::prelude::RawFd;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use crate::descriptor::AsRawDescriptor;
use crate::descriptor_reflection::deserialize_with_descriptors;
use crate::descriptor_reflection::SerializeDescriptors;
use crate::handle_eintr;
use crate::tube::Error;
use crate::tube::RecvTube;
use crate::tube::Result;
use crate::tube::SendTube;
use crate::RawDescriptor;
use crate::ReadNotifier;
use crate::ScmSocket;
use crate::UnixSeqpacket;
use crate::SCM_SOCKET_MAX_FD_COUNT;

// This size matches the inline buffer size of CmsgBuffer.
const TUBE_MAX_FDS: usize = 32;

/// Bidirectional tube that support both send and recv.
#[derive(Serialize, Deserialize)]
pub struct Tube {
    socket: ScmSocket<UnixSeqpacket>,
}

impl Tube {
    /// Create a pair of connected tubes. Request is sent in one direction while response is in the
    /// other direction.
    pub fn pair() -> Result<(Tube, Tube)> {
        let (socket1, socket2) = UnixSeqpacket::pair().map_err(Error::Pair)?;
        let tube1 = Tube::try_from(socket1)?;
        let tube2 = Tube::try_from(socket2)?;
        Ok((tube1, tube2))
    }

    /// DO NOT USE this method directly as it will become private soon (b/221484449). Use a
    /// directional Tube pair instead.
    #[deprecated]
    pub fn try_clone(&self) -> Result<Self> {
        self.socket
            .inner()
            .try_clone()
            .map_err(Error::Clone)?
            .try_into()
    }

    /// Sends a message via a Tube.
    /// The number of file descriptors that this method can send is limited to `TUBE_MAX_FDS`.
    /// If you want to send more descriptors, use `send_with_max_fds` instead.
    pub fn send<T: Serialize>(&self, msg: &T) -> Result<()> {
        self.send_with_max_fds(msg, TUBE_MAX_FDS)
    }

    /// Sends a message with at most `max_fds` file descriptors via a Tube.
    /// Note that `max_fds` must not exceed `SCM_SOCKET_MAX_FD_COUNT` (= 253).
    pub fn send_with_max_fds<T: Serialize>(&self, msg: &T, max_fds: usize) -> Result<()> {
        if max_fds > SCM_SOCKET_MAX_FD_COUNT {
            return Err(Error::SendTooManyFds);
        }
        let msg_serialize = SerializeDescriptors::new(&msg);
        let msg_json = serde_json::to_vec(&msg_serialize).map_err(Error::Json)?;
        let msg_descriptors = msg_serialize.into_descriptors();

        if msg_descriptors.len() > max_fds {
            return Err(Error::SendTooManyFds);
        }

        handle_eintr!(self.socket.send_with_fds(&msg_json, &msg_descriptors))
            .map_err(Error::Send)?;
        Ok(())
    }

    /// Recieves a message from a Tube.
    /// If the sender sent file descriptors more than TUBE_MAX_FDS with `send_with_max_fds`, use
    /// `recv_with_max_fds` instead.
    pub fn recv<T: DeserializeOwned>(&self) -> Result<T> {
        self.recv_with_max_fds(TUBE_MAX_FDS)
    }

    /// Recieves a message with at most `max_fds` file descriptors from a Tube.
    pub fn recv_with_max_fds<T: DeserializeOwned>(&self, max_fds: usize) -> Result<T> {
        if max_fds > SCM_SOCKET_MAX_FD_COUNT {
            return Err(Error::RecvTooManyFds);
        }

        // WARNING: The `cros_async` and `base_tokio` tube wrappers both assume that, if the tube
        // is readable, then a call to `Tube::recv` will not block (which ought to be true since we
        // use SOCK_SEQPACKET and a single recvmsg call currently).

        let msg_size =
            handle_eintr!(self.socket.inner().next_packet_size()).map_err(Error::Recv)?;
        // This buffer is the right size, as the size received in next_packet_size() represents the
        // size of only the message itself and not the file descriptors. The descriptors are stored
        // separately in msghdr::msg_control.
        let mut msg_json = vec![0u8; msg_size];

        let (msg_json_size, msg_descriptors) =
            handle_eintr!(self.socket.recv_with_fds(&mut msg_json, max_fds))
                .map_err(Error::Recv)?;

        if msg_json_size == 0 {
            return Err(Error::Disconnected);
        }

        deserialize_with_descriptors(
            || serde_json::from_slice(&msg_json[0..msg_json_size]),
            msg_descriptors,
        )
        .map_err(Error::Json)
    }

    pub fn set_send_timeout(&self, timeout: Option<Duration>) -> Result<()> {
        self.socket
            .inner()
            .set_write_timeout(timeout)
            .map_err(Error::SetSendTimeout)
    }

    pub fn set_recv_timeout(&self, timeout: Option<Duration>) -> Result<()> {
        self.socket
            .inner()
            .set_read_timeout(timeout)
            .map_err(Error::SetRecvTimeout)
    }

    #[cfg(feature = "proto_tube")]
    fn send_proto<M: protobuf::Message>(&self, msg: &M) -> Result<()> {
        let bytes = msg.write_to_bytes().map_err(Error::Proto)?;
        let no_fds: [RawFd; 0] = [];

        handle_eintr!(self.socket.send_with_fds(&bytes, &no_fds)).map_err(Error::Send)?;

        Ok(())
    }

    #[cfg(feature = "proto_tube")]
    fn recv_proto<M: protobuf::Message>(&self) -> Result<M> {
        let msg_size =
            handle_eintr!(self.socket.inner().next_packet_size()).map_err(Error::Recv)?;
        let mut msg_bytes = vec![0u8; msg_size];

        let (msg_bytes_size, _) =
            handle_eintr!(self.socket.recv_with_fds(&mut msg_bytes, TUBE_MAX_FDS))
                .map_err(Error::Recv)?;

        if msg_bytes_size == 0 {
            return Err(Error::Disconnected);
        }

        protobuf::Message::parse_from_bytes(&msg_bytes).map_err(Error::Proto)
    }
}

impl TryFrom<UnixSeqpacket> for Tube {
    type Error = Error;

    fn try_from(socket: UnixSeqpacket) -> Result<Self> {
        Ok(Tube {
            socket: socket.try_into().map_err(Error::ScmSocket)?,
        })
    }
}

impl AsRawDescriptor for Tube {
    fn as_raw_descriptor(&self) -> RawDescriptor {
        self.socket.as_raw_descriptor()
    }
}

impl AsRawFd for Tube {
    fn as_raw_fd(&self) -> RawFd {
        self.socket.inner().as_raw_descriptor()
    }
}

impl ReadNotifier for Tube {
    fn get_read_notifier(&self) -> &dyn AsRawDescriptor {
        &self.socket
    }
}

impl AsRawDescriptor for SendTube {
    fn as_raw_descriptor(&self) -> RawDescriptor {
        self.0.as_raw_descriptor()
    }
}

impl AsRawDescriptor for RecvTube {
    fn as_raw_descriptor(&self) -> RawDescriptor {
        self.0.as_raw_descriptor()
    }
}

/// Wrapper for Tube used for sending and receiving protos - avoids extra overhead of serialization
/// via serde_json. Since protos should be standalone objects we do not support sending of file
/// descriptors as a normal Tube would.
#[cfg(feature = "proto_tube")]
pub struct ProtoTube(Tube);

#[cfg(feature = "proto_tube")]
impl ProtoTube {
    pub fn pair() -> Result<(ProtoTube, ProtoTube)> {
        Tube::pair().map(|(t1, t2)| (ProtoTube(t1), ProtoTube(t2)))
    }

    pub fn send_proto<M: protobuf::Message>(&self, msg: &M) -> Result<()> {
        self.0.send_proto(msg)
    }

    pub fn recv_proto<M: protobuf::Message>(&self) -> Result<M> {
        self.0.recv_proto()
    }
}

#[cfg(feature = "proto_tube")]
impl From<Tube> for ProtoTube {
    fn from(tube: Tube) -> Self {
        ProtoTube(tube)
    }
}

#[cfg(all(feature = "proto_tube", test))]
#[allow(unused_variables)]
mod tests {
    // not testing this proto specifically, just need an existing one to test the ProtoTube.
    use protos::cdisk_spec::ComponentDisk;

    use super::*;

    #[test]
    fn tube_serializes_and_deserializes() {
        let (pt1, pt2) = ProtoTube::pair().unwrap();
        let proto = ComponentDisk {
            file_path: "/some/cool/path".to_string(),
            offset: 99,
            ..ComponentDisk::new()
        };

        pt1.send_proto(&proto).unwrap();

        let recv_proto = pt2.recv_proto().unwrap();

        assert!(proto.eq(&recv_proto));
    }
}
