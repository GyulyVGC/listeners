use std::ffi::c_uchar;
use std::os::raw::c_ulong;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Tcp6Table {
    pub rows_count: c_ulong,
    pub rows: [Tcp6Row; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Tcp6Row {
    pub local_addr: [c_uchar; 16],
    pub local_scope_id: c_ulong,
    pub local_port: c_ulong,
    pub remote_addr: [c_uchar; 16],
    pub remote_scope_id: c_ulong,
    pub remote_port: c_ulong,
    pub state: c_ulong,
    pub owning_pid: c_ulong,
}
