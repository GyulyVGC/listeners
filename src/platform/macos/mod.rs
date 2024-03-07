mod helpers;
mod pid;
mod proc_fd_info;
mod statics;

// use netstat2::get_sockets_info;

use crate::platform::macos::helpers::proc_pidinfo;
use crate::platform::macos::proc_fd_info::ProcFdInfo;
use crate::platform::macos::statics::{FD_TYPE_SOCKET, PROC_PID_LIST_FDS};
use pid::Pid;
use std::ffi::c_void;
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

        println!("PID: {}", pid.as_c_int());
        println!("FDS: {fds:?}");
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
    let number_of_fds = buffer_size as usize / mem::size_of::<ProcFdInfo>();

    let mut fds: Vec<ProcFdInfo> = Vec::new();
    fds.resize_with(number_of_fds, || ProcFdInfo {
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
