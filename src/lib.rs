use procfs::net::TcpState;
use procfs::process::{FDTarget, Process, Stat};
use procfs::ProcResult;
use std::collections::HashMap;

pub fn linux_proc() {
    // get all processes
    let all_procs = procfs::process::all_processes().unwrap();

    // build up a map between socket inodes and processes:
    let mut map: HashMap<u64, Stat> = HashMap::new();
    for p in all_procs {
        let process = p.unwrap();
        if let (Ok(stat), Ok(fds)) = (process.stat(), process.fd()) {
            for fd in fds {
                if let FDTarget::Socket(inode) = fd.unwrap().target {
                    map.insert(inode, stat.clone());
                }
            }
        }
    }

    // get the tcp table
    let tcp = procfs::net::tcp().unwrap();
    let tcp6 = procfs::net::tcp6().unwrap();
    // get the udp table
    let udp = procfs::net::udp().unwrap();
    let udp6 = procfs::net::udp6().unwrap();

    for (collection,title) in [(tcp, "TCP"), (tcp6, "TCP6)")] {
        println!("----- {title} -----");
        println!(
            "{:<26} {:<26} {:<15} {:<8} {}",
            "Local address", "Remote address", "State", "Inode", "PID/Program name"
        );
        for entry in collection {
            // find the process (if any) that has an open FD to this entry's inode
            let local_address = format!("{}", entry.local_address);
            let remote_addr = format!("{}", entry.remote_address);
            let state = format!("{:?}", entry.state);
            if let Some(stat) = map.get(&entry.inode) {
                println!(
                    "{:<26} {:<26} {:<15} {:<12} {}/{}",
                    local_address, remote_addr, state, entry.inode, stat.pid, stat.comm
                );
            } else {
                // We might not always be able to find the process associated with this socket
                println!(
                    "{:<26} {:<26} {:<15} {:<12} -",
                    local_address, remote_addr, state, entry.inode
                );
            }
        }
        println!("\n\n\n");
    }

    for (collection,title) in [(udp, "UDP"), (udp6, "UDP6)")] {
        println!("----- {title} -----\n");
        println!(
            "{:<26} {:<26} {:<15} {:<8} {}",
            "Local address", "Remote address", "State", "Inode", "PID/Program name"
        );
        for entry in collection {
            // find the process (if any) that has an open FD to this entry's inode
            let local_address = format!("{}", entry.local_address);
            let remote_addr = format!("{}", entry.remote_address);
            let state = format!("{:?}", entry.state);
            if let Some(stat) = map.get(&entry.inode) {
                println!(
                    "{:<26} {:<26} {:<15} {:<12} {}/{}",
                    local_address, remote_addr, state, entry.inode, stat.pid, stat.comm
                );
            } else {
                // We might not always be able to find the process associated with this socket
                println!(
                    "{:<26} {:<26} {:<15} {:<12} -",
                    local_address, remote_addr, state, entry.inode
                );
            }
        }
        println!("\n\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
