use socket_info::SocketInfo;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};

mod ffi;
mod socket_info;

use crate::{Listener, Process, Protocol};

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let lsockets = SocketInfo::get_all_listening();

    let processes = ffi::get_processes()?;

    let mut listeners = HashSet::new();

    for process in processes {
        let sockets = ffi::get_process_all_sockets(process.pid);

        for socket in sockets.into_iter().filter(|s| lsockets.contains(s)) {
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
    let Some(lsocket) = SocketInfo::get_listening_on_port(port, protocol) else {
        return Err("No process found listening on the specified port and protocol".into());
    };

    let processes = ffi::get_processes()?;

    for process in processes {
        let sockets = ffi::get_process_all_sockets(process.pid);

        if let Some(socket) = sockets.into_iter().find(|s| lsocket.eq(s)) {
            return Ok(process);
        }
    }

    Err("No process found listening on the specified port and protocol".into())
}
