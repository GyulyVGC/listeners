mod helpers;
mod proc_fd;
mod proc_info;
mod statics;
mod tcp_listener;

use crate::Listener;
use helpers::build_inode_proc_map;
use proc_fd::ProcFd;
use std::collections::HashSet;
use tcp_listener::TcpListener;

pub(crate) fn get_all_listeners() -> Result<HashSet<Listener>, String> {
    let mut listeners = HashSet::new();

    let proc_fds = ProcFd::get_all()?;

    let inode_proc_map = build_inode_proc_map(&proc_fds)?;

    for tcp_listener in TcpListener::get_all()? {
        if let Some(p) = inode_proc_map.get(&tcp_listener.inode()) {
            let listener = Listener::new(p.pid(), p.name(), tcp_listener.local_addr());
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}
