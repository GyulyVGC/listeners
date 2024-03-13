use std::ffi::{c_int, c_ulong, c_void};

#[link(name = "iphlpapi")]
extern "system" {
    pub fn get_extended_tcp_table(
        p_tcp_table: *mut c_void,
        pdw_size: *mut c_ulong,
        b_order: c_int,
        ul_af: c_ulong,
        table_class: c_ulong,
        reserved: c_ulong,
    ) -> c_ulong;
}
