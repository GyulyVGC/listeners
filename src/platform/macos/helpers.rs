use std::ffi::{c_int, c_void};

extern "C" {
    pub(super) fn proc_listpids(
        type_: u32,
        typeinfo: u32,
        buffer: *mut c_void,
        buffersize: c_int,
    ) -> c_int;
}

extern "C" {
    pub(super) fn proc_pidinfo(
        pid: c_int,
        flavor: c_int,
        arg: u64,
        buffer: *mut c_void,
        buffersize: c_int,
    ) -> c_int;
}
