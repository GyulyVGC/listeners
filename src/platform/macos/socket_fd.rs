use crate::platform::macos::c_proc_fd_info::CProcFdInfo;
use crate::platform::macos::libproc::proc_pidinfo;
use crate::platform::macos::pid::Pid;
use crate::platform::macos::statics::{FD_TYPE_SOCKET, PROC_PID_LIST_FDS};
use std::ffi::c_void;
use std::{mem, ptr};

pub(super) struct SocketFd(i32);

impl SocketFd {
    fn new(fd: i32) -> Self {
        SocketFd(fd)
    }

    pub(super) fn fd(&self) -> i32 {
        self.0
    }

    pub(super) fn get_all_of_pid(pid: Pid) -> crate::Result<Vec<Self>> {
        let buffer_size =
            unsafe { proc_pidinfo(pid.as_c_int(), PROC_PID_LIST_FDS, 0, ptr::null_mut(), 0) };

        if buffer_size <= 0 {
            return Err("Failed to list file descriptors".into());
        }

        let number_of_fds = usize::try_from(buffer_size)? / mem::size_of::<CProcFdInfo>();

        let mut fds: Vec<CProcFdInfo> = Vec::new();
        fds.resize_with(number_of_fds, CProcFdInfo::default);

        let return_code = unsafe {
            proc_pidinfo(
                pid.as_c_int(),
                PROC_PID_LIST_FDS,
                0,
                fds.as_mut_ptr().cast::<c_void>(),
                buffer_size,
            )
        };

        if return_code <= 0 {
            return Err("Failed to list file descriptors".into());
        }

        Ok(fds
            .iter()
            .filter(|fd| fd.fd_type() == FD_TYPE_SOCKET)
            .map(|fd| Self::new(fd.fd()))
            .collect())
    }
}
