/* automatically generated by tools/bindgen-all-the-things */

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Added by virtio_sys/bindgen.sh
use zerocopy::FromBytes;
use zerocopy::Immutable;
use zerocopy::IntoBytes;
use zerocopy::KnownLayout;

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const VIRTIO_NET_F_CSUM: u32 = 0;
pub const VIRTIO_NET_F_GUEST_CSUM: u32 = 1;
pub const VIRTIO_NET_F_CTRL_GUEST_OFFLOADS: u32 = 2;
pub const VIRTIO_NET_F_MTU: u32 = 3;
pub const VIRTIO_NET_F_MAC: u32 = 5;
pub const VIRTIO_NET_F_GUEST_TSO4: u32 = 7;
pub const VIRTIO_NET_F_GUEST_TSO6: u32 = 8;
pub const VIRTIO_NET_F_GUEST_ECN: u32 = 9;
pub const VIRTIO_NET_F_GUEST_UFO: u32 = 10;
pub const VIRTIO_NET_F_HOST_TSO4: u32 = 11;
pub const VIRTIO_NET_F_HOST_TSO6: u32 = 12;
pub const VIRTIO_NET_F_HOST_ECN: u32 = 13;
pub const VIRTIO_NET_F_HOST_UFO: u32 = 14;
pub const VIRTIO_NET_F_MRG_RXBUF: u32 = 15;
pub const VIRTIO_NET_F_STATUS: u32 = 16;
pub const VIRTIO_NET_F_CTRL_VQ: u32 = 17;
pub const VIRTIO_NET_F_CTRL_RX: u32 = 18;
pub const VIRTIO_NET_F_CTRL_VLAN: u32 = 19;
pub const VIRTIO_NET_F_CTRL_RX_EXTRA: u32 = 20;
pub const VIRTIO_NET_F_GUEST_ANNOUNCE: u32 = 21;
pub const VIRTIO_NET_F_MQ: u32 = 22;
pub const VIRTIO_NET_F_CTRL_MAC_ADDR: u32 = 23;
pub const VIRTIO_NET_F_DEVICE_STATS: u32 = 50;
pub const VIRTIO_NET_F_VQ_NOTF_COAL: u32 = 52;
pub const VIRTIO_NET_F_NOTF_COAL: u32 = 53;
pub const VIRTIO_NET_F_GUEST_USO4: u32 = 54;
pub const VIRTIO_NET_F_GUEST_USO6: u32 = 55;
pub const VIRTIO_NET_F_HOST_USO: u32 = 56;
pub const VIRTIO_NET_F_HASH_REPORT: u32 = 57;
pub const VIRTIO_NET_F_GUEST_HDRLEN: u32 = 59;
pub const VIRTIO_NET_F_RSS: u32 = 60;
pub const VIRTIO_NET_F_RSC_EXT: u32 = 61;
pub const VIRTIO_NET_F_STANDBY: u32 = 62;
pub const VIRTIO_NET_F_SPEED_DUPLEX: u32 = 63;
pub const VIRTIO_NET_F_GSO: u32 = 6;
pub const VIRTIO_NET_S_LINK_UP: u32 = 1;
pub const VIRTIO_NET_S_ANNOUNCE: u32 = 2;
pub const VIRTIO_NET_RSS_HASH_TYPE_IPv4: u32 = 1;
pub const VIRTIO_NET_RSS_HASH_TYPE_TCPv4: u32 = 2;
pub const VIRTIO_NET_RSS_HASH_TYPE_UDPv4: u32 = 4;
pub const VIRTIO_NET_RSS_HASH_TYPE_IPv6: u32 = 8;
pub const VIRTIO_NET_RSS_HASH_TYPE_TCPv6: u32 = 16;
pub const VIRTIO_NET_RSS_HASH_TYPE_UDPv6: u32 = 32;
pub const VIRTIO_NET_RSS_HASH_TYPE_IP_EX: u32 = 64;
pub const VIRTIO_NET_RSS_HASH_TYPE_TCP_EX: u32 = 128;
pub const VIRTIO_NET_RSS_HASH_TYPE_UDP_EX: u32 = 256;
pub const VIRTIO_NET_HDR_F_NEEDS_CSUM: u32 = 1;
pub const VIRTIO_NET_HDR_F_DATA_VALID: u32 = 2;
pub const VIRTIO_NET_HDR_F_RSC_INFO: u32 = 4;
pub const VIRTIO_NET_HDR_GSO_NONE: u32 = 0;
pub const VIRTIO_NET_HDR_GSO_TCPV4: u32 = 1;
pub const VIRTIO_NET_HDR_GSO_UDP: u32 = 3;
pub const VIRTIO_NET_HDR_GSO_TCPV6: u32 = 4;
pub const VIRTIO_NET_HDR_GSO_UDP_L4: u32 = 5;
pub const VIRTIO_NET_HDR_GSO_ECN: u32 = 128;
pub const VIRTIO_NET_HASH_REPORT_NONE: u32 = 0;
pub const VIRTIO_NET_HASH_REPORT_IPv4: u32 = 1;
pub const VIRTIO_NET_HASH_REPORT_TCPv4: u32 = 2;
pub const VIRTIO_NET_HASH_REPORT_UDPv4: u32 = 3;
pub const VIRTIO_NET_HASH_REPORT_IPv6: u32 = 4;
pub const VIRTIO_NET_HASH_REPORT_TCPv6: u32 = 5;
pub const VIRTIO_NET_HASH_REPORT_UDPv6: u32 = 6;
pub const VIRTIO_NET_HASH_REPORT_IPv6_EX: u32 = 7;
pub const VIRTIO_NET_HASH_REPORT_TCPv6_EX: u32 = 8;
pub const VIRTIO_NET_HASH_REPORT_UDPv6_EX: u32 = 9;
pub const VIRTIO_NET_OK: u32 = 0;
pub const VIRTIO_NET_ERR: u32 = 1;
pub const VIRTIO_NET_CTRL_RX: u32 = 0;
pub const VIRTIO_NET_CTRL_RX_PROMISC: u32 = 0;
pub const VIRTIO_NET_CTRL_RX_ALLMULTI: u32 = 1;
pub const VIRTIO_NET_CTRL_RX_ALLUNI: u32 = 2;
pub const VIRTIO_NET_CTRL_RX_NOMULTI: u32 = 3;
pub const VIRTIO_NET_CTRL_RX_NOUNI: u32 = 4;
pub const VIRTIO_NET_CTRL_RX_NOBCAST: u32 = 5;
pub const VIRTIO_NET_CTRL_MAC: u32 = 1;
pub const VIRTIO_NET_CTRL_MAC_TABLE_SET: u32 = 0;
pub const VIRTIO_NET_CTRL_MAC_ADDR_SET: u32 = 1;
pub const VIRTIO_NET_CTRL_VLAN: u32 = 2;
pub const VIRTIO_NET_CTRL_VLAN_ADD: u32 = 0;
pub const VIRTIO_NET_CTRL_VLAN_DEL: u32 = 1;
pub const VIRTIO_NET_CTRL_ANNOUNCE: u32 = 3;
pub const VIRTIO_NET_CTRL_ANNOUNCE_ACK: u32 = 0;
pub const VIRTIO_NET_CTRL_MQ: u32 = 4;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_SET: u32 = 0;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_MIN: u32 = 1;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_MAX: u32 = 32768;
pub const VIRTIO_NET_CTRL_MQ_RSS_CONFIG: u32 = 1;
pub const VIRTIO_NET_CTRL_MQ_HASH_CONFIG: u32 = 2;
pub const VIRTIO_NET_CTRL_GUEST_OFFLOADS: u32 = 5;
pub const VIRTIO_NET_CTRL_GUEST_OFFLOADS_SET: u32 = 0;
pub const VIRTIO_NET_CTRL_NOTF_COAL: u32 = 6;
pub const VIRTIO_NET_CTRL_NOTF_COAL_TX_SET: u32 = 0;
pub const VIRTIO_NET_CTRL_NOTF_COAL_RX_SET: u32 = 1;
pub const VIRTIO_NET_CTRL_NOTF_COAL_VQ_SET: u32 = 2;
pub const VIRTIO_NET_CTRL_NOTF_COAL_VQ_GET: u32 = 3;
pub const VIRTIO_NET_CTRL_STATS: u32 = 8;
pub const VIRTIO_NET_CTRL_STATS_QUERY: u32 = 0;
pub const VIRTIO_NET_CTRL_STATS_GET: u32 = 1;
pub const VIRTIO_NET_STATS_TYPE_CVQ: u64 = 4294967296;
pub const VIRTIO_NET_STATS_TYPE_RX_BASIC: u32 = 1;
pub const VIRTIO_NET_STATS_TYPE_RX_CSUM: u32 = 2;
pub const VIRTIO_NET_STATS_TYPE_RX_GSO: u32 = 4;
pub const VIRTIO_NET_STATS_TYPE_RX_SPEED: u32 = 8;
pub const VIRTIO_NET_STATS_TYPE_TX_BASIC: u32 = 65536;
pub const VIRTIO_NET_STATS_TYPE_TX_CSUM: u32 = 131072;
pub const VIRTIO_NET_STATS_TYPE_TX_GSO: u32 = 262144;
pub const VIRTIO_NET_STATS_TYPE_TX_SPEED: u32 = 524288;
pub const VIRTIO_NET_STATS_TYPE_REPLY_CVQ: u32 = 32;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_BASIC: u32 = 0;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_CSUM: u32 = 1;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_GSO: u32 = 2;
pub const VIRTIO_NET_STATS_TYPE_REPLY_RX_SPEED: u32 = 3;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_BASIC: u32 = 16;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_CSUM: u32 = 17;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_GSO: u32 = 18;
pub const VIRTIO_NET_STATS_TYPE_REPLY_TX_SPEED: u32 = 19;
pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;
pub type __virtio16 = u16;
pub type __virtio32 = u32;
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_config {
    pub mac: [u8; 6usize],
    pub status: __virtio16,
    pub max_virtqueue_pairs: __virtio16,
    pub mtu: __virtio16,
    pub speed: __le32,
    pub duplex: u8,
    pub rss_max_key_size: u8,
    pub rss_max_indirection_table_length: __le16,
    pub supported_hash_types: __le32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1 {
    pub flags: u8,
    pub gso_type: u8,
    pub hdr_len: __virtio16,
    pub gso_size: __virtio16,
    pub __bindgen_anon_1: virtio_net_hdr_v1__bindgen_ty_1,
    pub num_buffers: __virtio16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union virtio_net_hdr_v1__bindgen_ty_1 {
    pub __bindgen_anon_1: virtio_net_hdr_v1__bindgen_ty_1__bindgen_ty_1,
    pub csum: virtio_net_hdr_v1__bindgen_ty_1__bindgen_ty_2,
    pub rsc: virtio_net_hdr_v1__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_hdr_v1__bindgen_ty_1__bindgen_ty_1 {
    pub csum_start: __virtio16,
    pub csum_offset: __virtio16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_hdr_v1__bindgen_ty_1__bindgen_ty_2 {
    pub start: __virtio16,
    pub offset: __virtio16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_hdr_v1__bindgen_ty_1__bindgen_ty_3 {
    pub segments: __le16,
    pub dup_acks: __le16,
}
impl Default for virtio_net_hdr_v1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for virtio_net_hdr_v1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_net_hdr_v1_hash {
    pub hdr: virtio_net_hdr_v1,
    pub hash_value: __le32,
    pub hash_report: __le16,
    pub padding: __le16,
}
impl Default for virtio_net_hdr_v1_hash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct virtio_net_hdr {
    pub flags: u8,
    pub gso_type: u8,
    pub hdr_len: __virtio16,
    pub gso_size: __virtio16,
    pub csum_start: __virtio16,
    pub csum_offset: __virtio16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct virtio_net_hdr_mrg_rxbuf {
    pub hdr: virtio_net_hdr,
    pub num_buffers: __virtio16,
}
#[repr(C, packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_hdr {
    pub class: u8,
    pub cmd: u8,
}
pub type virtio_net_ctrl_ack = u8;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_mq {
    pub virtqueue_pairs: __virtio16,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct virtio_net_rss_config {
    pub hash_types: __le32,
    pub indirection_table_mask: __le16,
    pub unclassified_queue: __le16,
    pub indirection_table: [__le16; 1usize],
    pub max_tx_vq: __le16,
    pub hash_key_length: u8,
    pub hash_key_data: __IncompleteArrayField<u8>,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct virtio_net_hash_config {
    pub hash_types: __le32,
    pub reserved: [__le16; 4usize],
    pub hash_key_length: u8,
    pub hash_key_data: __IncompleteArrayField<u8>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_coal_tx {
    pub tx_max_packets: __le32,
    pub tx_usecs: __le32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_coal_rx {
    pub rx_max_packets: __le32,
    pub rx_usecs: __le32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_coal {
    pub max_packets: __le32,
    pub max_usecs: __le32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_coal_vq {
    pub vqn: __le16,
    pub reserved: __le16,
    pub coal: virtio_net_ctrl_coal,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_capabilities {
    pub supported_stats_types: [__le64; 1usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_queue_stats {
    pub stats: [virtio_net_ctrl_queue_stats__bindgen_ty_1; 1usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_ctrl_queue_stats__bindgen_ty_1 {
    pub vq_index: __le16,
    pub reserved: [__le16; 3usize],
    pub types_bitmap: [__le64; 1usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_reply_hdr {
    pub type_: u8,
    pub reserved: u8,
    pub vq_index: __le16,
    pub reserved1: __le16,
    pub size: __le16,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_cvq {
    pub hdr: virtio_net_stats_reply_hdr,
    pub command_num: __le64,
    pub ok_num: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_rx_basic {
    pub hdr: virtio_net_stats_reply_hdr,
    pub rx_notifications: __le64,
    pub rx_packets: __le64,
    pub rx_bytes: __le64,
    pub rx_interrupts: __le64,
    pub rx_drops: __le64,
    pub rx_drop_overruns: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_tx_basic {
    pub hdr: virtio_net_stats_reply_hdr,
    pub tx_notifications: __le64,
    pub tx_packets: __le64,
    pub tx_bytes: __le64,
    pub tx_interrupts: __le64,
    pub tx_drops: __le64,
    pub tx_drop_malformed: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_rx_csum {
    pub hdr: virtio_net_stats_reply_hdr,
    pub rx_csum_valid: __le64,
    pub rx_needs_csum: __le64,
    pub rx_csum_none: __le64,
    pub rx_csum_bad: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_tx_csum {
    pub hdr: virtio_net_stats_reply_hdr,
    pub tx_csum_none: __le64,
    pub tx_needs_csum: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_rx_gso {
    pub hdr: virtio_net_stats_reply_hdr,
    pub rx_gso_packets: __le64,
    pub rx_gso_bytes: __le64,
    pub rx_gso_packets_coalesced: __le64,
    pub rx_gso_bytes_coalesced: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_tx_gso {
    pub hdr: virtio_net_stats_reply_hdr,
    pub tx_gso_packets: __le64,
    pub tx_gso_bytes: __le64,
    pub tx_gso_segments: __le64,
    pub tx_gso_segments_bytes: __le64,
    pub tx_gso_packets_noseg: __le64,
    pub tx_gso_bytes_noseg: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_rx_speed {
    pub hdr: virtio_net_stats_reply_hdr,
    pub rx_ratelimit_packets: __le64,
    pub rx_ratelimit_bytes: __le64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct virtio_net_stats_tx_speed {
    pub hdr: virtio_net_stats_reply_hdr,
    pub tx_ratelimit_packets: __le64,
    pub tx_ratelimit_bytes: __le64,
}
