use socket_info::SocketInfo;
use std::collections::HashSet;

mod ffi;
mod socket_info;

use crate::{Listener, Process, Protocol};

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for process in ffi::get_processes()? {
        for socket in ffi::get_all_sockets_of_pid(process.pid) {
            listeners.insert(Listener::new(
                process.pid,
                process.name.clone(),
                process.path.clone(),
                socket.address,
                socket.protocol,
            ));
        }
    }

    Ok(listeners)
}

pub(crate) fn get_process_by_port(port: u16, protocol: Protocol) -> crate::Result<Process> {
    for process in ffi::get_processes()? {
        if let Some(socket) = ffi::get_socket_by_port_of_pid(process.pid, port, protocol) {
            return Ok(process);
        }
    }

    Err("No process found listening on the specified port and protocol".into())
}
