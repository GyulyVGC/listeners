use crate::{Listener, Process, Protocol};
use std::collections::HashSet;

mod ffi;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    let sockets: Vec<_> = ffi::get_tcp_sockets()?
        .into_iter()
        .chain(ffi::get_udp_sockets()?)
        .collect();

    let kvaddr_pid_map = ffi::get_kvaddr_to_pid_table()?;

    for socket in sockets {
        if let Some(pid) = kvaddr_pid_map.get(&socket.kvaddr) {
            listeners.insert(Listener::new(
                *pid as u32,
                ffi::get_process_name(*pid).unwrap_or_default(),
                ffi::get_process_path(*pid).unwrap_or_default(),
                socket.address,
                socket.protocol,
            ));
        }
    }

    Ok(listeners)
}

pub(crate) fn get_process_by_port(port: u16, protocol: Protocol) -> crate::Result<Process> {
    let sockets = match protocol {
        Protocol::TCP => ffi::get_tcp_sockets()?,
        Protocol::UDP => ffi::get_udp_sockets()?,
    };

    let kvaddr_pid_map = ffi::get_kvaddr_to_pid_table()?;

    for socket in sockets {
        if let Some(pid) = kvaddr_pid_map.get(&socket.kvaddr)
            && socket.address.port() == port
        {
            return Ok(Process::new(*pid as u32, String::new(), String::new()));
        }
    }

    Err("No process found listening on the specified port and protocol".into())
}
