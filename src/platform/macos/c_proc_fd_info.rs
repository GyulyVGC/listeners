#[repr(C)]
#[derive(Default)]
pub(super) struct CProcFdInfo {
    proc_fd: i32,
    proc_fd_type: u32,
}

impl CProcFdInfo {
    pub(super) fn fd(&self) -> i32 {
        self.proc_fd
    }

    pub(super) fn fd_type(&self) -> u32 {
        self.proc_fd_type
    }
}
