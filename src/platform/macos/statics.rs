use std::ffi::c_int;

pub(super) const PROC_ALL_PIDS: u32 = 1;
pub(super) const PROC_PID_LIST_FDS: c_int = 1;
pub(super) const PROC_PID_FD_SOCKET_INFO: c_int = 3;
pub(super) const FD_TYPE_SOCKET: u32 = 2;
// pub(super) const SOCKET_STATE_LISTEN: c_int = 1;
pub(super) const SOCKET_STATE_CLOSED: c_int = 0;
pub(super) const PROC_PID_PATH_INFO_MAXSIZE: usize = 4096;
pub(super) const IPPROT_TCP: c_int = 6;
pub(super) const IPPROT_UDP: c_int = 17;
