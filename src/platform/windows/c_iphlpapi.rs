use std::ffi::{c_int, c_ulong, c_void};

#[allow(non_snake_case)]
#[link(name = "iphlpapi")]
extern "system" {
    pub(super) fn GetExtendedTcpTable(
        pTcpTable: *mut c_void,
        pdwSize: *mut c_ulong,
        bOrder: c_int,
        ulAf: c_ulong,
        TableClass: c_ulong,
        Reserved: c_ulong,
    ) -> c_ulong;
}
