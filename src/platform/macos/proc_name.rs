use crate::platform::macos::libproc::proc_name;
use crate::platform::macos::pid::Pid;
use crate::platform::macos::statics::PROC_PID_PATH_INFO_MAXSIZE;
use std::ffi::c_void;

pub(super) struct ProcName(pub(super) String);

impl ProcName {
    fn new(name: String) -> Self {
        ProcName(name)
    }

    pub(super) fn from_pid(pid: Pid) -> crate::Result<Self> {
        let mut buf: Vec<u8> = Vec::with_capacity(PROC_PID_PATH_INFO_MAXSIZE);
        let buffer_ptr = buf.as_mut_ptr().cast::<c_void>();
        let buffer_size = u32::try_from(buf.capacity())?;

        let ret;
        unsafe {
            ret = proc_name(pid.as_c_int(), buffer_ptr, buffer_size);
        };

        if ret <= 0 {
            return Err("Failed to get process name".into());
        }

        unsafe {
            buf.set_len(usize::try_from(ret)?);
        }

        match String::from_utf8(buf) {
            Ok(name) => Ok(Self::new(name)),
            Err(_) => Err("Invalid UTF sequence for process name".into()),
        }
    }
}
