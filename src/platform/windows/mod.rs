use std::collections::HashSet;

use proto_listener::{PidNamePathCache, ProtoListener, pname_ppath};

use crate::{Listener, Process, Protocol};

mod proto_listener;
mod socket_table;
mod statics;
mod tcp6_table;
mod tcp_table;
mod udp6_table;
mod udp_table;

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut proto_listeners_cache = PidNamePathCache::new();
    let mut listeners = HashSet::new();

    for proto_listener in ProtoListener::get_all() {
        if let Some(listener) = proto_listeners_cache.get(proto_listener) {
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}

pub(crate) fn get_process_by_port(port: u16, protocol: Protocol) -> crate::Result<Process> {
    let proto_listener = ProtoListener::get_by_port(port, protocol)?;
    let pid = proto_listener.pid;
    pname_ppath(pid)
        .map(|(pname, ppath)| Process::new(pid, pname, ppath))
        .ok_or_else(|| "Could not get process path".into())
}
