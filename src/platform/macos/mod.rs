use std::collections::HashSet;

use proc_name::ProcName;
use proc_pid::ProcPid;
use socket_fd::SocketFd;
use tcp_listener::TcpListener;

use crate::Listener;

mod c_libproc;
mod c_proc_fd_info;
mod c_socket_fd_info;
mod proc_name;
mod proc_pid;
mod socket_fd;
mod statics;
mod tcp_listener;

pub fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for pid in ProcPid::get_all()? {
        let fds = SocketFd::get_all_of_pid(pid)?;
        for fd in fds {
            if let Ok(tcp_listener) = TcpListener::from_pid_fd(pid, &fd) {
                let ProcName(name) = ProcName::from_pid(pid)?;
                let listener = Listener::new(pid.as_u_32()?, name, tcp_listener.socket_addr());
                listeners.insert(listener);
            }
        }
    }

    Ok(listeners)
}
