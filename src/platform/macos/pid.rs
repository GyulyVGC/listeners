use super::libproc::proc_listpids;
use super::statics::PROC_ALL_PIDS;
use std::ffi::{c_int, c_void};
use std::{mem, ptr};

#[derive(Debug, Copy, Clone)]
pub(super) struct Pid(c_int);

impl Pid {
    fn new(n: c_int) -> Self {
        Pid(n)
    }

    pub(super) fn as_c_int(self) -> c_int {
        self.0
    }

    pub(super) fn as_u_32(self) -> crate::Result<u32> {
        match u32::try_from(self.0) {
            Ok(n) => Ok(n),
            Err(_) => Err("Failed to convert pid to u32".into()),
        }
    }

    pub(super) fn get_all() -> crate::Result<Vec<Pid>> {
        let number_of_pids;

        unsafe {
            number_of_pids = proc_listpids(PROC_ALL_PIDS, 0, ptr::null_mut(), 0);
        }

        if number_of_pids <= 0 {
            return Err("Failed to list processes".into());
        }

        let mut pids: Vec<c_int> = Vec::new();
        pids.resize_with(usize::try_from(number_of_pids)?, Default::default);

        let return_code = unsafe {
            proc_listpids(
                PROC_ALL_PIDS,
                0,
                pids.as_mut_ptr().cast::<c_void>(),
                c_int::try_from(pids.len() * mem::size_of::<c_int>())?,
            )
        };

        if return_code <= 0 {
            return Err("Failed to list processes".into());
        }

        Ok(pids.into_iter().filter(|f| *f > 0).map(Pid::new).collect())
    }
}
