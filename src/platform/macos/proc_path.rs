use std::ffi::c_void;

use crate::platform::macos::c_libproc::proc_pidpath;
use crate::platform::macos::proc_pid::ProcPid;
use crate::platform::macos::statics::PROC_PID_PATH_INFO_MAXSIZE;

#[derive(Default)]
pub(super) struct ProcPath(pub(super) String);

impl ProcPath {
    fn new(path: String) -> Self {
        ProcPath(path)
    }

    pub(super) fn from_pid(pid: ProcPid) -> crate::Result<Self> {
        let mut buf: Vec<u8> = Vec::with_capacity(PROC_PID_PATH_INFO_MAXSIZE);
        let buffer_ptr = buf.as_mut_ptr().cast::<c_void>();
        let buffer_size = u32::try_from(buf.capacity())?;

        let ret;
        unsafe {
            ret = proc_pidpath(pid.as_c_int(), buffer_ptr, buffer_size);
        };

        if ret <= 0 {
            return Err("Failed to get process path".into());
        }

        unsafe {
            buf.set_len(usize::try_from(ret)?);
        }

        match String::from_utf8(buf) {
            Ok(path) => Ok(Self::new(path)),
            Err(_) => Err("Invalid UTF sequence for process path".into()),
        }
    }
}
