use std::collections::HashSet;
use std::io::{Error, ErrorKind};

mod ffi;
mod socket_info;

use crate::{Listener, Process, Protocol};

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let lsockets: HashSet<_> = ffi::get_listening_sockets_tcp()?
        .into_iter()
        .chain(ffi::get_listening_sockets_tcp6()?)
        .chain(ffi::get_listening_sockets_udp()?)
        .chain(ffi::get_listening_sockets_udp6()?)
        .collect();

    let processes: HashSet<_> = ffi::get_processes()?.into_iter().collect();

    let mut listeners = HashSet::new();

    for process in processes {
        let sockets = ffi::get_process_all_sockets(process.pid)?;

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
    let lsockets: HashSet<_> = match protocol {
        Protocol::TCP => ffi::get_listening_sockets_tcp()?
            .into_iter()
            .chain(ffi::get_listening_sockets_tcp6()?)
            .collect(),
        Protocol::UDP => ffi::get_listening_sockets_udp()?
            .into_iter()
            .chain(ffi::get_listening_sockets_udp6()?)
            .collect(),
    };

    let processes: HashSet<_> = ffi::get_processes()?.into_iter().collect();

    for process in processes {
        let sockets = ffi::get_process_all_sockets(process.pid)?;

        for socket in sockets.into_iter().filter(|s| lsockets.contains(s)) {
            if socket.address.port() == port {
                return Ok(process);
            }
        }
    }

    Err(Box::new(Error::from(ErrorKind::NotFound)))
}
