use std::ffi::{c_char, c_int, c_longlong, c_short, c_uchar, c_uint, c_ushort};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use byteorder::{ByteOrder, NetworkEndian};

use crate::platform::macos::tcp_listener::TcpListener;
use crate::Protocol;

use super::statics::{IPPROT_TCP, IPPROT_UDP, SOCKET_STATE_CLOSED};

#[repr(C)]
pub(super) struct CSocketFdInfo {
    pfi: ProcFileinfo,
    psi: SocketInfo,
}

impl CSocketFdInfo {
    pub(super) fn to_tcp_listener(&self) -> crate::Result<TcpListener> {
        let sock_info = self.psi;
        let family = sock_info.soi_family;
        let ip_protocol = sock_info.soi_protocol;
        
        let general_sock_info = unsafe {sock_info.soi_proto.pri_in};
        let tcp_in = unsafe { sock_info.soi_proto.pri_tcp };

        // get all possible states for tcp
        if tcp_in.tcpsi_state != SOCKET_STATE_CLOSED && ip_protocol == IPPROT_TCP {
            return Err("Socket is closed".into());
        }

        // let tcp_sockaddr_in = tcp_in.tcpsi_ini;
        let lport_bytes: [u8; 4] = i32::to_le_bytes(general_sock_info.insi_lport);
        let local_address = Self::get_local_addr(family, general_sock_info)?;
        let protocol = Self::get_protocol(family, ip_protocol)?;

        let socket_info = TcpListener::new(local_address, NetworkEndian::read_u16(&lport_bytes), protocol);

        Ok(socket_info)
    }

    fn get_local_addr(family: c_int, tcp_sockaddr_in: InSockinfo) -> crate::Result<IpAddr> {
        match family {
            2 => {
                // AF_INET
                let addr = unsafe { tcp_sockaddr_in.insi_laddr.ina_46.i46a_addr4.s_addr };
                Ok(IpAddr::V4(Ipv4Addr::from(u32::from_be(addr))))
            }
            30 => {
                // AF_INET6
                let addr = unsafe { &tcp_sockaddr_in.insi_laddr.ina_6.__u6_addr.__u6_addr8 };
                let mut ipv6_addr = [0_u16; 8];
                NetworkEndian::read_u16_into(addr, &mut ipv6_addr);
                Ok(IpAddr::V6(Ipv6Addr::from(ipv6_addr)))
            }
            _ => Err("Unsupported socket family".into()),
        }
    }

    fn get_protocol(family: c_int, ip_protocol: c_int) -> crate::Result<Protocol> {
        match (family,ip_protocol) {
            (2,IPPROT_TCP) => Ok(Protocol::TCP),
            (30,IPPROT_TCP) => Ok(Protocol::TCP6),
            (2,IPPROT_UDP) => Ok(Protocol::UDP),
            (30,IPPROT_UDP) => Ok(Protocol::UDP6),
            (_,_) => Err("unsupported protocol".into()),
        }
    }
}

#[repr(C)]
#[allow(clippy::struct_field_names)]
struct ProcFileinfo {
    fi_openflags: u32,
    fi_status: u32,
    fi_offset: c_longlong,
    fi_type: i32,
    fi_guardflags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SocketInfo {
    soi_stat: VinfoStat,
    soi_so: u64,
    soi_pcb: u64,
    soi_type: c_int,
    soi_protocol: c_int,
    soi_family: c_int,
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
    soi_proto: SocketInfoBindgenTy1,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(clippy::struct_field_names)]
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
#[derive(Copy, Clone)]
#[allow(clippy::struct_field_names)]
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
#[derive(Copy, Clone)]
union SocketInfoBindgenTy1 {
    pri_in: InSockinfo,
    pri_tcp: TcpSockinfo,
    pri_un: UnSockinfo,
    pri_ndrv: NdrvInfo,
    pri_kern_event: KernEventInfo,
    pri_kern_ctl: KernCtlInfo,
    _bindgen_union_align: [u64; 66usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InSockinfo {
    insi_fport: c_int,
    insi_lport: c_int,
    insi_gencnt: u64,
    insi_flags: u32,
    insi_flow: u32,
    insi_vflag: u8,
    insi_ip_ttl: u8,
    rfu_1: u32,
    insi_faddr: InSockinfoBindgenTy1,
    insi_laddr: InSockinfoBindgenTy2,
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
union InSockinfoBindgenTy2 {
    ina_46: In4in6Addr,
    ina_6: In6Addr,
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InSockinfoBindgenTy3 {
    in4_tos: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(clippy::struct_field_names)]
struct InSockinfoBindgenTy4 {
    in6_hlim: u8,
    in6_cksum: c_int,
    in6_ifindex: c_ushort,
    in6_hops: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct In4in6Addr {
    i46a_pad32: [c_uint; 3usize],
    i46a_addr4: InAddr,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InAddr {
    s_addr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct In6Addr {
    __u6_addr: In6AddrBindgenTy1,
}

#[repr(C)]
#[derive(Copy, Clone)]
union In6AddrBindgenTy1 {
    __u6_addr8: [c_uchar; 16usize],
    __u6_addr16: [c_ushort; 8usize],
    __u6_addr32: [c_uint; 4usize],
    _bindgen_union_align: [u32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct TcpSockinfo {
    tcpsi_ini: InSockinfo,
    tcpsi_state: c_int,
    tcpsi_timer: [c_int; 4usize],
    tcpsi_mss: c_int,
    tcpsi_flags: u32,
    rfu_1: u32,
    tcpsi_tp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(clippy::struct_field_names)]
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
#[allow(clippy::struct_field_names)]
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
#[allow(clippy::struct_field_names)]
struct NdrvInfo {
    ndrvsi_if_family: u32,
    ndrvsi_if_unit: u32,
    ndrvsi_if_name: [c_char; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(clippy::struct_field_names)]
struct KernEventInfo {
    kesi_vendor_code_filter: u32,
    kesi_class_filter: u32,
    kesi_subclass_filter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(clippy::struct_field_names)]
struct KernCtlInfo {
    kcsi_id: u32,
    kcsi_reg_unit: u32,
    kcsi_flags: u32,
    kcsi_recvbufsize: u32,
    kcsi_sendbufsize: u32,
    kcsi_unit: u32,
    kcsi_name: [c_char; 96usize],
}
