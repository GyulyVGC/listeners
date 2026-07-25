use std::ffi::c_int;

pub(super) const PROC_ALL_PIDS: u32 = 1;
pub(super) const PROC_PID_LIST_FDS: c_int = 1;
pub(super) const PROC_PID_FD_SOCKET_INFO: c_int = 3;
pub(super) const FD_TYPE_SOCKET: u32 = 2;
// pub(super) const SOCKET_STATE_LISTEN: c_int = 1;
pub(super) const PROC_PID_PATH_INFO_MAXSIZE: usize = 4096;
pub(super) const IPPROTO_TCP: c_int = 6;
pub(super) const IPPROTO_UDP: c_int = 17;

/// `INI_IPV4` / `INI_IPV6` from `<netinet/in_pcb.h>` — the discriminator that says
/// WHICH slot of `insi_laddr` holds the address. `soi_family` does not answer this:
/// an `AF_INET6` socket bound to an IPv4-mapped address reports `soi_family =
/// AF_INET6` while keeping the address in the 4-byte `i46a_addr4` slot, and the
/// 16-byte `ina_6` slot then holds 12 zero pad bytes followed by the v4 address —
/// which read as an IPv6 address is the deprecated IPv4-COMPATIBLE form, a different
/// address from the IPv4-MAPPED one that was bound.
pub(super) const INI_IPV4: u8 = 0x1;
pub(super) const INI_IPV6: u8 = 0x2;
