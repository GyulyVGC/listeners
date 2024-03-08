#[repr(C)]
pub(super) struct CProcFdInfo {
    pub(super) proc_fd: i32,
    pub(super) proc_fd_type: u32,
}
