use std::ffi::{c_char, c_int, c_longlong, c_short, c_uchar, c_uint, c_ushort};

#[repr(C)]
pub(super) struct CSocketFdInfo {
    pfi: ProcFileinfo,
    pub(super) psi: SocketInfo,
}

#[repr(C)]
struct ProcFileinfo {
    fi_openflags: u32,
    fi_status: u32,
    fi_offset: c_longlong,
    fi_type: i32,
    fi_guardflags: u32,
}

#[repr(C)]
pub(super) struct SocketInfo {
    soi_stat: VinfoStat,
    soi_so: u64,
    soi_pcb: u64,
    soi_type: c_int,
    soi_protocol: c_int,
    pub(super) soi_family: c_int,
    soi_options: c_short,
    soi_linger: c_short,
    soi_state: c_short,
    soi_qlen: c_short,
    soi_incqlen: c_short,
    soi_qlimit: c_short,
    soi_timeo: c_short,
    soi_error: c_ushort,
    soi_oobmark: u32,
    soi_rcv: SockbufInfo,
    soi_snd: SockbufInfo,
    soi_kind: c_int,
    rfu_1: u32,
    pub(super) soi_proto: SocketInfoBindgenTy1,
}

#[repr(C)]
struct VinfoStat {
    vst_dev: u32,
    vst_mode: u16,
    vst_nlink: u16,
    vst_ino: u64,
    vst_uid: c_uint,
    vst_gid: c_uint,
    vst_atime: i64,
    vst_atimensec: i64,
    vst_mtime: i64,
    vst_mtimensec: i64,
    vst_ctime: i64,
    vst_ctimensec: i64,
    vst_birthtime: i64,
    vst_birthtimensec: i64,
    vst_size: c_longlong,
    vst_blocks: i64,
    vst_blksize: i32,
    vst_flags: u32,
    vst_gen: u32,
    vst_rdev: u32,
    vst_qspare: [i64; 2usize],
}

#[repr(C)]
struct SockbufInfo {
    sbi_cc: u32,
    sbi_hiwat: u32,
    sbi_mbcnt: u32,
    sbi_mbmax: u32,
    sbi_lowat: u32,
    sbi_flags: c_short,
    sbi_timeo: c_short,
}

#[repr(C)]
pub(super) union SocketInfoBindgenTy1 {
    pri_in: InSockinfo,
    pub(super) pri_tcp: TcpSockinfo,
    pri_un: UnSockinfo,
    pri_ndrv: NdrvInfo,
    pri_kern_event: KernEventInfo,
    pri_kern_ctl: KernCtlInfo,
    _bindgen_union_align: [u64; 66usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct InSockinfo {
    insi_fport: c_int,
    pub(super) insi_lport: c_int,
    insi_gencnt: u64,
    insi_flags: u32,
    insi_flow: u32,
    insi_vflag: u8,
    insi_ip_ttl: u8,
    rfu_1: u32,
    insi_faddr: InSockinfoBindgenTy1,
    pub(super) insi_laddr: InSockinfoBindgenTy2,
    insi_v4: InSockinfoBindgenTy3,
    insi_v6: InSockinfoBindgenTy4,
}

#[repr(C)]
#[derive(Copy, Clone)]
union InSockinfoBindgenTy1 {
    ina_46: In4in6Addr,
    ina_6: In6Addr,
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) union InSockinfoBindgenTy2 {
    pub(super) ina_46: In4in6Addr,
    pub(super) ina_6: In6Addr,
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InSockinfoBindgenTy3 {
    in4_tos: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InSockinfoBindgenTy4 {
    in6_hlim: u8,
    in6_cksum: c_int,
    in6_ifindex: c_ushort,
    in6_hops: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct In4in6Addr {
    i46a_pad32: [c_uint; 3usize],
    pub(super) i46a_addr4: InAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct InAddr {
    pub(super) s_addr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct In6Addr {
    pub(super) __u6_addr: In6AddrBindgenTy1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) union In6AddrBindgenTy1 {
    pub(super) __u6_addr8: [c_uchar; 16usize],
    __u6_addr16: [c_ushort; 8usize],
    __u6_addr32: [c_uint; 4usize],
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) struct TcpSockinfo {
    pub(super) tcpsi_ini: InSockinfo,
    pub(super) tcpsi_state: c_int,
    tcpsi_timer: [c_int; 4usize],
    tcpsi_mss: c_int,
    tcpsi_flags: u32,
    rfu_1: u32,
    tcpsi_tp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct UnSockinfo {
    unsi_conn_so: u64,
    unsi_conn_pcb: u64,
    unsi_addr: UnSockinfoBindgenTy1,
    unsi_caddr: UnSockinfoBindgenTy2,
}

#[repr(C)]
#[derive(Copy, Clone)]
union UnSockinfoBindgenTy1 {
    ua_sun: SockaddrUn,
    ua_dummy: [c_char; 255usize],
    _bindgen_union_align: [u8; 255usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SockaddrUn {
    sun_len: c_uchar,
    sun_family: c_uchar,
    sun_path: [c_char; 104usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
union UnSockinfoBindgenTy2 {
    ua_sun: SockaddrUn,
    ua_dummy: [c_char; 255usize],
    _bindgen_union_align: [u8; 255usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NdrvInfo {
    ndrvsi_if_family: u32,
    ndrvsi_if_unit: u32,
    ndrvsi_if_name: [c_char; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct KernEventInfo {
    kesi_vendor_code_filter: u32,
    kesi_class_filter: u32,
    kesi_subclass_filter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct KernCtlInfo {
    kcsi_id: u32,
    kcsi_reg_unit: u32,
    kcsi_flags: u32,
    kcsi_recvbufsize: u32,
    kcsi_sendbufsize: u32,
    kcsi_unit: u32,
    kcsi_name: [c_char; 96usize],
}
