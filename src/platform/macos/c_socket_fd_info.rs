use std::ffi::{c_char, c_int, c_longlong, c_short, c_uchar, c_uint, c_ushort};

#[repr(C)]
pub(super) struct CSocketFdInfo {
    pub pfi: ProcFileinfo,
    pub psi: SocketInfo,
}

#[repr(C)]
struct ProcFileinfo {
    pub fi_openflags: u32,
    pub fi_status: u32,
    pub fi_offset: c_longlong,
    pub fi_type: i32,
    pub fi_guardflags: u32,
}

#[repr(C)]
pub(super) struct SocketInfo {
    pub soi_stat: VinfoStat,
    pub soi_so: u64,
    pub soi_pcb: u64,
    pub soi_type: c_int,
    pub soi_protocol: c_int,
    pub soi_family: c_int,
    pub soi_options: c_short,
    pub soi_linger: c_short,
    pub soi_state: c_short,
    pub soi_qlen: c_short,
    pub soi_incqlen: c_short,
    pub soi_qlimit: c_short,
    pub soi_timeo: c_short,
    pub soi_error: c_ushort,
    pub soi_oobmark: u32,
    pub soi_rcv: SockbufInfo,
    pub soi_snd: SockbufInfo,
    pub soi_kind: c_int,
    pub rfu_1: u32,
    pub soi_proto: SocketInfoBindgenTy1,
}

#[repr(C)]
struct VinfoStat {
    pub vst_dev: u32,
    pub vst_mode: u16,
    pub vst_nlink: u16,
    pub vst_ino: u64,
    pub vst_uid: c_uint,
    pub vst_gid: c_uint,
    pub vst_atime: i64,
    pub vst_atimensec: i64,
    pub vst_mtime: i64,
    pub vst_mtimensec: i64,
    pub vst_ctime: i64,
    pub vst_ctimensec: i64,
    pub vst_birthtime: i64,
    pub vst_birthtimensec: i64,
    pub vst_size: c_longlong,
    pub vst_blocks: i64,
    pub vst_blksize: i32,
    pub vst_flags: u32,
    pub vst_gen: u32,
    pub vst_rdev: u32,
    pub vst_qspare: [i64; 2usize],
}

#[repr(C)]
struct SockbufInfo {
    pub sbi_cc: u32,
    pub sbi_hiwat: u32,
    pub sbi_mbcnt: u32,
    pub sbi_mbmax: u32,
    pub sbi_lowat: u32,
    pub sbi_flags: c_short,
    pub sbi_timeo: c_short,
}

#[repr(C)]
pub(super) union SocketInfoBindgenTy1 {
    pub pri_in: InSockinfo,
    pub pri_tcp: TcpSockinfo,
    pub pri_un: UnSockinfo,
    pub pri_ndrv: NdrvInfo,
    pub pri_kern_event: KernEventInfo,
    pub pri_kern_ctl: KernCtlInfo,
    _bindgen_union_align: [u64; 66usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct InSockinfo {
    pub insi_fport: c_int,
    pub insi_lport: c_int,
    pub insi_gencnt: u64,
    pub insi_flags: u32,
    pub insi_flow: u32,
    pub insi_vflag: u8,
    pub insi_ip_ttl: u8,
    pub rfu_1: u32,
    pub insi_faddr: InSockinfoBindgenTy1,
    pub insi_laddr: InSockinfoBindgenTy2,
    pub insi_v4: InSockinfoBindgenTy3,
    pub insi_v6: InSockinfoBindgenTy4,
}

#[repr(C)]
#[derive(Copy, Clone)]
union InSockinfoBindgenTy1 {
    pub ina_46: In4in6Addr,
    pub ina_6: In6Addr,
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) union InSockinfoBindgenTy2 {
    pub ina_46: In4in6Addr,
    pub ina_6: In6Addr,
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InSockinfoBindgenTy3 {
    pub in4_tos: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InSockinfoBindgenTy4 {
    pub in6_hlim: u8,
    pub in6_cksum: c_int,
    pub in6_ifindex: c_ushort,
    pub in6_hops: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct In4in6Addr {
    pub i46a_pad32: [c_uint; 3usize],
    pub i46a_addr4: InAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct InAddr {
    pub s_addr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct In6Addr {
    pub __u6_addr: In6AddrBindgenTy1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) union In6AddrBindgenTy1 {
    pub __u6_addr8: [c_uchar; 16usize],
    pub __u6_addr16: [c_ushort; 8usize],
    pub __u6_addr32: [c_uint; 4usize],
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TcpSockinfo {
    pub tcpsi_ini: InSockinfo,
    pub tcpsi_state: c_int,
    pub tcpsi_timer: [c_int; 4usize],
    pub tcpsi_mss: c_int,
    pub tcpsi_flags: u32,
    pub rfu_1: u32,
    pub tcpsi_tp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct UnSockinfo {
    pub unsi_conn_so: u64,
    pub unsi_conn_pcb: u64,
    pub unsi_addr: UnSockinfoBindgenTy1,
    pub unsi_caddr: UnSockinfoBindgenTy2,
}

#[repr(C)]
#[derive(Copy, Clone)]
union UnSockinfoBindgenTy1 {
    pub ua_sun: SockaddrUn,
    pub ua_dummy: [c_char; 255usize],
    _bindgen_union_align: [u8; 255usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SockaddrUn {
    pub sun_len: c_uchar,
    pub sun_family: c_uchar,
    pub sun_path: [c_char; 104usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
union UnSockinfoBindgenTy2 {
    pub ua_sun: SockaddrUn,
    pub ua_dummy: [c_char; 255usize],
    _bindgen_union_align: [u8; 255usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NdrvInfo {
    pub ndrvsi_if_family: u32,
    pub ndrvsi_if_unit: u32,
    pub ndrvsi_if_name: [c_char; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct KernEventInfo {
    pub kesi_vendor_code_filter: u32,
    pub kesi_class_filter: u32,
    pub kesi_subclass_filter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct KernCtlInfo {
    pub kcsi_id: u32,
    pub kcsi_reg_unit: u32,
    pub kcsi_flags: u32,
    pub kcsi_recvbufsize: u32,
    pub kcsi_sendbufsize: u32,
    pub kcsi_unit: u32,
    pub kcsi_name: [c_char; 96usize],
}
