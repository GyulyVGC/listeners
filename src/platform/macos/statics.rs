use std::ffi::c_int;

pub(super) const PROC_ALL_PIDS: u32 = 1;
pub(super) const PROC_PID_LIST_FDS: c_int = 1;
pub(super) const PROC_PID_FD_SOCKET_INFO: c_int = 3;
pub(super) const FD_TYPE_SOCKET: u32 = 2;
// pub(super) const SOCKET_STATE_LISTEN: c_int = 1;
pub(super) const PROC_PID_PATH_INFO_MAXSIZE: usize = 4096;
pub(super) const IPPROTO_TCP: c_int = 6;
pub(super) const IPPROTO_UDP: c_int = 17;

/// `INI_IPV4` / `INI_IPV6` from `<netinet/in_pcb.h>`: `insi_vflag` says which slot of
/// `insi_laddr` holds the address. `soi_family` doesn't — an `AF_INET6` socket bound to
/// an IPv4-mapped address keeps it in the 4-byte `i46a_addr4` slot, not the 16-byte one.
pub(super) const INI_IPV4: u8 = 0x1;
pub(super) const INI_IPV6: u8 = 0x2;
