use std::ffi::c_int;
use std::os::raw::{c_uint, c_void};
use std::{mem, ptr};

#[derive(Debug)]
pub(super) struct Pid(c_int);

impl Pid {
    fn new(n: c_int) -> Self {
        Pid(n)
    }

    pub(super) fn get_all() -> crate::Result<Vec<Pid>> {
        let number_of_pids;
        let proc_type_all = 1 as c_uint;

        unsafe {
            number_of_pids = proc_listpids(proc_type_all, 0, ptr::null_mut(), 0);
        }

        if number_of_pids < 0 {
            return Err("Failed to list processes".into());
        }

        let mut pids: Vec<std::os::raw::c_int> = Vec::new();
        pids.resize_with(number_of_pids as usize, Default::default);

        let return_code = unsafe {
            proc_listpids(
                proc_type_all,
                0,
                pids.as_mut_ptr() as *mut c_void,
                (pids.len() * mem::size_of::<std::os::raw::c_int>()) as i32,
            )
        };

        if return_code <= 0 {
            return Err("Failed to list processes".into());
        }

        // Sometimes the OS returns excessive zero elements, so we truncate them.
        Ok(pids
            .into_iter()
            .filter(|f| *f > 0)
            .map(|n| Pid::new(n))
            .collect())
    }
}

extern "C" {
    fn proc_listpids(
        type_: u32,
        typeinfo: u32,
        buffer: *mut c_void,
        buffersize: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
