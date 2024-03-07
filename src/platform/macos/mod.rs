mod c_proc_fd_info;
mod c_socket_fd_info;
mod helpers;
mod local_socket_info;
mod pid;
mod statics;

use crate::platform::macos::c_proc_fd_info::CProcFdInfo;
use crate::platform::macos::c_socket_fd_info::{CSocketFdInfo, InSockinfo};
use crate::platform::macos::helpers::{proc_pidfdinfo, proc_pidinfo};
use crate::platform::macos::local_socket_info::LocalSocketInfo;
use crate::platform::macos::statics::{
    FD_TYPE_SOCKET, PROC_PID_FD_SOCKET_INFO, PROC_PID_LIST_FDS, SOCKET_STATE_LISTEN,
};
use byteorder::{ByteOrder, NetworkEndian};
use pid::Pid;
use std::ffi::{c_int, c_void};
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::{mem, ptr};

pub fn get_all() {
    // returns: local socket address, socket state, associated PIDs (but NOT process names)
    // netstat2::get_sockets_info(
    //     netstat2::AddressFamilyFlags::IPV4,
    //     netstat2::ProtocolFlags::TCP,
    // )
    // .unwrap_or_default()
    // .iter()
    // .for_each(|s| {
    //     println!("{:?}", s);
    // });

    let pids = Pid::get_all().unwrap();

    for pid in pids {
        let fds = get_socket_fds_of_pid(pid).unwrap();
        for fd in fds {
            if let Ok(local_socket_info) = get_fd_info(pid, fd) {
                println!("PID: {}", pid.as_c_int());
                // println!("FD: {fd}");
                println!("{local_socket_info:?}", );
                println!();
            }
        }
    }
}

// fn get_sockets_info() {
//     let pids = Pid::get_all().unwrap();
//
//     let mut results = vec![];
//
//     for pid in pids {
//         let fds = match list_all_fds_for_pid(pid) {
//             Ok(fds) => fds,
//             Err(e) => {
//                 continue;
//             }
//         };
//
//         for fd in fds {
//             if fd.proc_fdtype == ProcFDType::Socket {
//                 let fd_information = match get_fd_information(pid, fd) {
//                     Ok(fd_information) => fd_information,
//                     Err(e) => {
//                         results.push(Err(e));
//                         continue;
//                     }
//                 };
//
//                 match fd_information {
//                     FDInformation::SocketInfo(sinfo) => {
//                         if sinfo.psi.soi_protocol == IPPROTO_TCP as i32 {
//                             if let Some(row) = parse_tcp_socket_info(pid, fd, sinfo) {
//                                 results.push(Ok(SocketInfo {
//                                     protocol_socket_info: ProtocolSocketInfo::Tcp(row),
//                                     associated_pids: vec![pid as u32],
//                                 }));
//                             }
//                         }
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     }
//
//     Ok(results.into_iter())
// }

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
    }
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
        // 30 => {
        //     // AF_INET6
        //     let addr = unsafe { &saddr.insi_laddr.ina_6.__u6_addr.__u6_addr8 };
        //     let mut ipv6_addr = [0_u16; 8];
        //     NetworkEndian::read_u16_into(addr, &mut ipv6_addr);
        //     Ok(IpAddr::V6(Ipv6Addr::from(ipv6_addr)))
        // }
        _ => Err("Unsupported socket family".into()),
    }
}
