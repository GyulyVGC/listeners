use std::ffi::{c_int, c_void};
use std::{mem, ptr};

use super::c_libproc::proc_listpids;
use super::statics::PROC_ALL_PIDS;

#[derive(Debug, Copy, Clone)]
pub(super) struct ProcPid(c_int);

impl ProcPid {
    fn new(n: c_int) -> Self {
        ProcPid(n)
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

    pub(super) fn get_all() -> crate::Result<Vec<ProcPid>> {
        let number_of_pids;

        unsafe {
            number_of_pids = proc_listpids(PROC_ALL_PIDS, 0, ptr::null_mut(), 0);
        }

        if number_of_pids <= 0 {
            return Err("Failed to list processes".into());
        }

        let mut pids: Vec<c_int> = Vec::new();
        pids.resize_with(
            usize::try_from(number_of_pids).unwrap_or(4096),
            Default::default,
        );

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

        Ok(pids
            .into_iter()
            .filter(|f| *f > 0)
            .map(ProcPid::new)
            .collect())
    }
}
