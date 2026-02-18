pub(crate) use target_os::get_all;
pub(crate) use target_os::get_process_by_port;

mod ffi;

#[cfg(target_os = "openbsd")]
mod helpers;
#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
mod pid_name_path_cache;

#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(target_os = "freebsd")]
use freebsd as target_os;

#[cfg(target_os = "netbsd")]
mod netbsd;
#[cfg(target_os = "netbsd")]
use netbsd as target_os;

#[cfg(target_os = "openbsd")]
mod openbsd;
#[cfg(target_os = "openbsd")]
use openbsd as target_os;

#[cfg(not(any(target_os = "freebsd", target_os = "netbsd", target_os = "openbsd")))]
mod target_os {
    use crate::{Listener, Process, Protocol};
    use std::collections::HashSet;

    pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
        Err("This BSD system isn't supported yet".into())
    }

    pub(crate) fn get_process_by_port(_port: u16, _protocol: Protocol) -> crate::Result<Process> {
        Err("This BSD system isn't supported yet".into())
    }
}
