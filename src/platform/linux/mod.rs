use std::collections::HashSet;

use helpers::build_inode_proc_map;
use proc_fd::ProcFd;
use tcp_listener::TcpListener;

use crate::Listener;

mod helpers;
mod proc_fd;
mod proc_info;
mod statics;
mod tcp_listener;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    let inode_proc_map = build_inode_proc_map(ProcFd::get_all()?)?;

    for tcp_listener in TcpListener::get_all()? {
        if let Some(p) = inode_proc_map.get(&tcp_listener.inode()) {
            let listener = Listener::new(p.pid(), p.name(), tcp_listener.local_addr(), tcp_listener.protocol());
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_get_all() {
//         let listeners = crate::get_all().unwrap();
//         assert!(!listeners.is_empty());
//
//         // let out = std::process::Command::new("netstat")
//         //     .args(["-plnt"])
//         //     .output()
//         //     .unwrap();
//         // for l in String::from_utf8(out.stdout).unwrap().lines() {
//         //     println!("{}", l);
//         // }
//     }
// }
