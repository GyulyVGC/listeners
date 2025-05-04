use std::ffi::c_ulong;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(super) struct UdpTable {
    pub(super) rows_count: c_ulong,
    pub(super) rows: [UdpRow; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(super) struct UdpRow {
    pub(super) local_addr: c_ulong,
    pub(super) local_port: c_ulong,
    pub(super) owning_pid: c_ulong,
}
