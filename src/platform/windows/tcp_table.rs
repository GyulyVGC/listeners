use std::ffi::c_ulong;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(super) struct TcpTable {
    pub(super) rows_count: c_ulong,
    pub(super) rows: [TcpRow; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(super) struct TcpRow {
    pub(super) state: c_ulong,
    pub(super) local_addr: c_ulong,
    pub(super) local_port: c_ulong,
    remote_addr: c_ulong,
    remote_port: c_ulong,
    pub(super) owning_pid: c_ulong,
}
