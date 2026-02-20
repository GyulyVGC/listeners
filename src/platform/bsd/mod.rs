pub(crate) use target_os::get_all;
pub(crate) use target_os::get_process_by_port;

mod ffi;

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
