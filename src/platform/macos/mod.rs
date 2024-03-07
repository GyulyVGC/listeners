mod helpers;
mod pid;
mod statics;

use pid::Pid;

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

    println!("ALL PIDS: {pids:?}");
}

// fn get_sockets_info() {
//     let pids = list_pids(ProcType::ProcAllPIDS)?;
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
