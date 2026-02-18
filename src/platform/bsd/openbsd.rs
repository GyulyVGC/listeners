use crate::{Listener, Process, Protocol};
use std::collections::HashSet;

use super::ffi::openbsd;
use super::helpers;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    let processes = openbsd::get_all_processes()?;

    for process in processes {
        let process_path = helpers::locate_process(&process.name)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_default();

        let sockets = openbsd::get_sockets(process.pid)?;

        for socket in sockets {
            listeners.insert(Listener::new(
                process.pid.cast_unsigned(),
                process.name.clone(),
                process_path.clone(),
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
        let process_path = helpers::locate_process(&process.name)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_default();

        let sockets = openbsd::get_sockets(process.pid)?;

        for socket in sockets {
            if socket.address.port() == port && socket.protocol == protocol {
                return Ok(Process::new(
                    process.pid.cast_unsigned(),
                    process.name,
                    process_path,
                ));
            }
        }
    }

    Err("No process found listening on the specified port and protocol".into())
}
