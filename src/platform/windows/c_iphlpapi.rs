use std::ffi::{c_int, c_ulong, c_void};

#[link(name = "iphlpapi")]
extern "system" {
    pub fn GetExtendedTcpTable(
        pTcpTable: *mut c_void,
        pdwSize: *mut c_ulong,
        bOrder: c_int,
        ulAf: c_ulong,
        TableClass: c_ulong,
        Reserved: c_ulong,
    ) -> c_ulong;
}