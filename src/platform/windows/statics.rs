use std::ffi::c_ulong;
use windows::Win32::NetworkManagement::IpHelper::{TCP_TABLE_CLASS, UDP_TABLE_CLASS};

pub(super) const TCP_TABLE_OWNER_PID_ALL: TCP_TABLE_CLASS = TCP_TABLE_CLASS(5);
pub(super) const UDP_TABLE_OWNER_PID: UDP_TABLE_CLASS = UDP_TABLE_CLASS(1);
pub(super) const ERROR_INSUFFICIENT_BUFFER: c_ulong = 0x7A;
pub(super) const NO_ERROR: c_ulong = 0;
pub(super) const AF_INET: c_ulong = 2;
pub(super) const AF_INET6: c_ulong = 23;
