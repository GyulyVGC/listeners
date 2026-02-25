use crate::{Listener, Process, Protocol};
use std::collections::HashSet;

use super::ffi::openbsd;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    let processes = openbsd::get_all_processes()?;

    for process in processes {
        let sockets = openbsd::get_sockets(process.pid).unwrap_or_default();

        for socket in sockets {
            listeners.insert(Listener::new(
                process.pid.cast_unsigned(),
                process.name.clone(),
                String::new(),
                socket.address,
                socket.protocol,
            ));
        }
    }

    Ok(listeners)
}

pub(crate) fn get_process_by_port(port: u16, protocol: Protocol) -> crate::Result<Process> {
    let processes = openbsd::get_all_processes()?;

    for process in processes {
        let sockets = openbsd::get_sockets(process.pid).unwrap_or_default();

        for socket in sockets {
            if socket.address.port() == port && socket.protocol == protocol {
                return Ok(Process::new(
                    process.pid.cast_unsigned(),
                    process.name,
                    String::new(),
                ));
            }
        }
    }

    Err("No process found listening on the specified port and protocol".into())
}
