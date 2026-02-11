use std::collections::HashSet;

use helpers::{build_inode_proc_map, get_proc_by_inode};
use proto_listener::ProtoListener;

use crate::{Listener, Process, Protocol};

mod helpers;
mod proc_fd;
mod proc_info;
mod proto_listener;
mod statics;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    let inode_proc_map = build_inode_proc_map()?;

    for proto_listener in ProtoListener::get_all()? {
        if let Some(p) = inode_proc_map.get(&proto_listener.inode()) {
            let listener = Listener::new(
                p.pid(),
                p.name(),
                p.path(),
                proto_listener.local_addr(),
                proto_listener.protocol(),
            );
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}

pub(crate) fn get_process_by_port(port: u16, protocol: Protocol) -> crate::Result<Process> {
    let proto_listener = ProtoListener::get_by_port(port, protocol)?;
    let p = get_proc_by_inode(proto_listener.inode())?;
    return Ok(Process::new(p.pid(), p.name(), p.path()));
}
