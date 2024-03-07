#[repr(C)]
#[derive(Debug)]
pub(super) struct ProcFdInfo {
    pub proc_fd: i32,
    pub proc_fd_type: u32,
}
