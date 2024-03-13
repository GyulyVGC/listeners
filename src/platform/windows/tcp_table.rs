use std::ffi::c_ulong;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TcpTable {
    pub rows_count: c_ulong,
    pub rows: [TcpRow; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TcpRow {
    pub state: c_ulong,
    pub local_addr: c_ulong,
    pub local_port: c_ulong,
    pub remote_addr: c_ulong,
    pub remote_port: c_ulong,
    pub owning_pid: c_ulong,
}
