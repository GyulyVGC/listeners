mod c_proc_fd_info;
mod c_socket_fd_info;
mod helpers;
mod local_socket_info;
mod pid;
mod statics;

use crate::platform::macos::c_proc_fd_info::CProcFdInfo;
use crate::platform::macos::c_socket_fd_info::{CSocketFdInfo, InSockinfo};
use crate::platform::macos::helpers::{proc_name, proc_pidfdinfo, proc_pidinfo};
use crate::platform::macos::local_socket_info::LocalSocketInfo;
use crate::platform::macos::statics::{
    FD_TYPE_SOCKET, PROC_PID_FD_SOCKET_INFO, PROC_PID_LIST_FDS, SOCKET_STATE_LISTEN,
};
use crate::platform::target_os::statics::PROC_PID_PATH_INFO_MAXSIZE;
use byteorder::{ByteOrder, NetworkEndian};
use pid::Pid;
use std::ffi::{c_int, c_void};
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::{mem, ptr};
use std::collections::HashSet;
use crate::Listener;

pub fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for pid in Pid::get_all()? {
        let fds = get_socket_fds_of_pid(pid).unwrap();
        for fd in fds {
            if let Ok(local_socket_info) = get_fd_info(pid, fd) {
                let proc_name = get_proc_name(pid)?;
                let listener = Listener::new(pid.as_u_32(), proc_name, local_socket_info.socket_addr());
                listeners.insert(listener);
            }
        }
    }

    Ok(listeners)
}

fn get_socket_fds_of_pid(pid: Pid) -> crate::Result<Vec<i32>> {
    let buffer_size =
        unsafe { proc_pidinfo(pid.as_c_int(), PROC_PID_LIST_FDS, 0, ptr::null_mut(), 0) };

    if buffer_size <= 0 {
        return Err("Failed to list file descriptors".into());
    }

    #[allow(clippy::cast_sign_loss)]
    let number_of_fds = buffer_size as usize / mem::size_of::<CProcFdInfo>();

    let mut fds: Vec<CProcFdInfo> = Vec::new();
    fds.resize_with(number_of_fds, || CProcFdInfo {
        proc_fd: 0,
        proc_fd_type: 0,
    });

    let return_code = unsafe {
        proc_pidinfo(
            pid.as_c_int(),
            PROC_PID_LIST_FDS,
            0,
            fds.as_mut_ptr().cast::<c_void>(),
            buffer_size,
        )
    };

    if return_code <= 0 {
        return Err("Failed to list file descriptors".into());
    }

    Ok(fds
        .iter()
        .filter(|fd| fd.proc_fd_type == FD_TYPE_SOCKET)
        .map(|fd| fd.proc_fd)
        .collect())
}

pub fn get_fd_info(pid: Pid, fd: i32) -> crate::Result<LocalSocketInfo> {
    let mut sinfo: MaybeUninit<CSocketFdInfo> = MaybeUninit::uninit();

    let return_code = unsafe {
        proc_pidfdinfo(
            pid.as_c_int(),
            fd,
            PROC_PID_FD_SOCKET_INFO,
            sinfo.as_mut_ptr().cast::<c_void>(),
            mem::size_of::<CSocketFdInfo>() as i32,
        )
    };

    if return_code < 0 {
        return Err("Failed to get file descriptor information".into());
    }

    return if let Some(local_socket_info) = parse_tcp_socket_info(unsafe { sinfo.assume_init() }) {
        Ok(local_socket_info)
    } else {
        Err("Failed to parse TCP socket information".into())
    };
}

fn parse_tcp_socket_info(sinfo: CSocketFdInfo) -> Option<LocalSocketInfo> {
    let sock_info = sinfo.psi;
    let family = sock_info.soi_family;

    let tcp_in = unsafe { sock_info.soi_proto.pri_tcp };

    if tcp_in.tcpsi_state != SOCKET_STATE_LISTEN {
        return None;
    }

    let tcp_sockaddr_in = tcp_in.tcpsi_ini;
    let lport_bytes: [u8; 4] = i32::to_le_bytes(tcp_sockaddr_in.insi_lport);
    let local_address = get_local_addr(family, tcp_sockaddr_in).ok()?;

    let socket_info = LocalSocketInfo::new(local_address, NetworkEndian::read_u16(&lport_bytes));

    Some(socket_info)
}

fn get_local_addr(family: c_int, saddr: InSockinfo) -> crate::Result<IpAddr> {
    match family {
        2 => {
            // AF_INET
            let addr = unsafe { saddr.insi_laddr.ina_46.i46a_addr4.s_addr };
            Ok(IpAddr::V4(Ipv4Addr::from(u32::from_be(addr))))
        }
        30 => {
            // AF_INET6
            let addr = unsafe { &saddr.insi_laddr.ina_6.__u6_addr.__u6_addr8 };
            let mut ipv6_addr = [0_u16; 8];
            NetworkEndian::read_u16_into(addr, &mut ipv6_addr);
            Ok(IpAddr::V6(Ipv6Addr::from(ipv6_addr)))
        }
        _ => Err("Unsupported socket family".into()),
    }
}

fn get_proc_name(pid: Pid) -> crate::Result<String> {
    let mut buf: Vec<u8> = Vec::with_capacity(PROC_PID_PATH_INFO_MAXSIZE);
    let buffer_ptr = buf.as_mut_ptr().cast::<c_void>();
    let buffer_size = buf.capacity() as u32;

    let ret;
    unsafe {
        ret = proc_name(pid.as_c_int(), buffer_ptr, buffer_size);
    };

    if ret <= 0 || ret > buffer_size as c_int {
        return Err("Failed to get process name".into());
    }

    unsafe {
        buf.set_len(ret as usize);
    }

    match String::from_utf8(buf) {
        Ok(name) => Ok(name),
        Err(_) => Err("Invalid UTF sequence for process name".into()),
    }
}
