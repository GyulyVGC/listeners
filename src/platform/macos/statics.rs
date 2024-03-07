use std::ffi::c_int;

pub(super) const PROC_ALL_PIDS: u32 = 1;
pub(super) const PROC_PID_LIST_FDS: c_int = 1;
pub(super) const PROC_PID_FD_SOCKET_INFO: c_int = 3;
pub(super) const FD_TYPE_SOCKET: u32 = 2;
pub(super) const SOCKET_STATE_LISTEN: c_int = 1;
