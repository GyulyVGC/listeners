mod c_proc_fd_info;
mod c_socket_fd_info;
mod libproc;
mod local_socket_info;
mod pid;
mod proc_name;
mod socket_fd;
mod statics;

use crate::platform::macos::local_socket_info::LocalSocket;
use crate::platform::macos::socket_fd::SocketFd;

use crate::platform::macos::proc_name::ProcName;
use crate::Listener;
use pid::Pid;
use std::collections::HashSet;

pub fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for pid in Pid::get_all()? {
        let fds = SocketFd::get_all_of_pid(pid)?;
        for fd in fds {
            if let Ok(local_socket_info) = LocalSocket::from_pid_fd(pid, fd) {
                let ProcName(name) = ProcName::from_pid(pid)?;
                let listener = Listener::new(pid.as_u_32(), name, local_socket_info.socket_addr());
                listeners.insert(listener);
            }
        }
    }

    Ok(listeners)
}
