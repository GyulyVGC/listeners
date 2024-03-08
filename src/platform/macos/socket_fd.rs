use crate::platform::macos::c_proc_fd_info::CProcFdInfo;
use crate::platform::macos::libproc::proc_pidinfo;
use crate::platform::macos::pid::Pid;
use crate::platform::macos::statics::{FD_TYPE_SOCKET, PROC_PID_LIST_FDS};
use std::ffi::c_void;
use std::{mem, ptr};

pub(super) struct SocketFd {
    fd: i32,
}

impl SocketFd {
    pub(super) fn new(fd: i32) -> Self {
        SocketFd { fd }
    }

    pub(super) fn fd(&self) -> i32 {
        self.fd
    }

    pub(super) fn get_all_of_pid(pid: Pid) -> crate::Result<Vec<Self>> {
        let buffer_size =
            unsafe { proc_pidinfo(pid.as_c_int(), PROC_PID_LIST_FDS, 0, ptr::null_mut(), 0) };

        if buffer_size <= 0 {
            return Err("Failed to list file descriptors".into());
        }

        #[allow(clippy::cast_sign_loss)]
        let number_of_fds = buffer_size as usize / mem::size_of::<CProcFdInfo>();

        let mut fds: Vec<CProcFdInfo> = Vec::new();
        fds.resize_with(number_of_fds, || CProcFdInfo {
            proc_fd: 0,
            proc_fd_type: 0,
        });

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
            .filter(|fd| fd.proc_fd_type == FD_TYPE_SOCKET)
            .map(|fd| Self::new(fd.proc_fd))
            .collect())
    }
}
