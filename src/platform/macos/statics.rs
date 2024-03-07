use std::ffi::c_int;

pub(super) const PROC_ALL_PIDS: u32 = 1;
pub(super) const PROC_PID_LIST_FDS: c_int = 1;
pub(super) const FD_TYPE_SOCKET: u32 = 2;
