use std::collections::HashSet;

use proc_name::ProcName;
use proc_pid::ProcPid;
use proto_listener::ProtoListener;
use socket_fd::SocketFd;

use crate::Listener;
use crate::platform::macos::proc_path::ProcPath;

mod c_libproc;
mod c_proc_fd_info;
mod c_socket_fd_info;
mod proc_name;
mod proc_path;
mod proc_pid;
mod proto_listener;
mod socket_fd;
mod statics;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for pid in ProcPid::get_all().into_iter().flatten() {
        for fd in SocketFd::get_all_of_pid(pid).iter().flatten() {
            if let Ok(proto_listener) = ProtoListener::from_pid_fd(pid, fd) {
                if let Ok(ProcName(name)) = ProcName::from_pid(pid) {
                    let ProcPath(path) = ProcPath::from_pid(pid);
                    let Ok(pid_u_32) = pid.as_u_32() else {
                        continue;
                    };
                    let listener = Listener::new(
                        pid_u_32,
                        name,
                        path,
                        proto_listener.socket_addr(),
                        proto_listener.protocol(),
                    );
                    listeners.insert(listener);
                }
            }
        }
    }

    Ok(listeners)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_get_all() {
//         let listeners = crate::get_all().unwrap();
//         assert!(!listeners.is_empty());
//
//         // let out = std::process::Command::new("netstat")
//         //     .args(["-p", "tcp", "-van"])
//         //     .output()
//         //     .unwrap();
//         // for l in String::from_utf8(out.stdout).unwrap().lines().filter(|l| l.contains("LISTEN")) {
//         //     println!("{}", l);
//         // }
//     }
// }
