use super::ffi::netbsd;
use super::pid_name_path_cache::ProcNamesPathsCache;
use crate::{Listener, Process, Protocol};
use std::collections::HashSet;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut proc_cache = ProcNamesPathsCache::new();

    let mut listeners = HashSet::new();

    let sockets: Vec<_> = netbsd::get_tcp_sockets()?
        .into_iter()
        .chain(netbsd::get_tcp6_sockets()?)
        .chain(netbsd::get_udp_sockets()?)
        .chain(netbsd::get_udp6_sockets()?)
        .collect();

    let kvaddr_pid_map = netbsd::get_kvaddr_to_pid_table()?;

    for socket in sockets {
        if let Some(pid) = kvaddr_pid_map.get(&socket.kvaddr)
            && let Some((name, path)) = proc_cache.get(*pid)
        {
            listeners.insert(Listener::new(
                (*pid).cast_unsigned(),
                name,
                path,
                socket.address,
                socket.protocol,
            ));
        }
    }

    Ok(listeners)
}

pub(crate) fn get_process_by_port(port: u16, protocol: Protocol) -> crate::Result<Process> {
    let mut sockets_on_port: Vec<_> = match protocol {
        Protocol::TCP => netbsd::get_tcp_sockets()?
            .into_iter()
            .chain(netbsd::get_tcp6_sockets()?)
            .collect(),
        Protocol::UDP => netbsd::get_udp_sockets()?
            .into_iter()
            .chain(netbsd::get_udp6_sockets()?)
            .collect(),
    };

    sockets_on_port.retain(|socket| socket.address.port() == port);

    if sockets_on_port.is_empty() {
        return Err("No process found listening on the specified port and protocol".into());
    }

    let kvaddr_pid_map = netbsd::get_kvaddr_to_pid_table()?;

    for socket in sockets_on_port {
        if let Some(pid) = kvaddr_pid_map.get(&socket.kvaddr)
            && let Ok(name) = netbsd::get_process_name(*pid)
        {
            return Ok(Process::new(
                (*pid).cast_unsigned(),
                name,
                netbsd::get_process_path(*pid).unwrap_or_default(),
            ));
        }
    }

    Err("No process found listening on the specified port and protocol".into())
}
