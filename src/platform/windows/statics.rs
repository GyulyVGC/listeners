use std::ffi::{c_int, c_ulong};

pub(super) const TCP_TABLE_OWNER_PID_ALL: c_ulong = 5;
pub(super) const UDP_TABLE_OWNER_PID: c_ulong = 1;
pub(super) const FALSE: c_int = 0;
pub(super) const ERROR_INSUFFICIENT_BUFFER: c_ulong = 0x7A;
pub(super) const NO_ERROR: c_ulong = 0;
pub(super) const AF_INET: c_ulong = 2;
pub(super) const AF_INET6: c_ulong = 23;
pub(super) const LISTEN: c_ulong = 2;
