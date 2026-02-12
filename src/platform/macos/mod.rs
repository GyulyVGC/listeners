use std::collections::HashSet;

use proc_name::ProcName;
use proc_pid::ProcPid;
use proto_listener::ProtoListener;
use socket_fd::SocketFd;

use crate::platform::macos::proc_path::ProcPath;
use crate::{Listener, Process, Protocol};

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

    for pid in ProcPid::get_all()? {
        for fd in SocketFd::get_all_of_pid(pid).iter().flatten() {
            if let Ok(proto_listener) = ProtoListener::from_pid_fd(pid, fd)
                && let Ok(ProcName(name)) = ProcName::from_pid(pid)
            {
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

    Ok(listeners)
}

pub(crate) fn get_process_by_port(port: u16, protocol: Protocol) -> crate::Result<Process> {
    for pid in ProcPid::get_all()? {
        for fd in SocketFd::get_all_of_pid(pid).iter().flatten() {
            if let Ok(proto_listener) = ProtoListener::from_pid_fd(pid, fd)
                && proto_listener.socket_addr().port() == port
                && proto_listener.protocol() == protocol
                && let Ok(ProcName(name)) = ProcName::from_pid(pid)
            {
                let ProcPath(path) = ProcPath::from_pid(pid);
                let Ok(pid_u_32) = pid.as_u_32() else {
                    continue;
                };
                return Ok(Process::new(pid_u_32, name, path));
            }
        }
    }

    Err("No process found listening on the specified port and protocol".into())
}
