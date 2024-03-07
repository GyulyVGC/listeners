#[repr(C)]
#[derive(Debug)]
pub(super) struct CProcFdInfo {
    pub proc_fd: i32,
    pub proc_fd_type: u32,
}
