use std::collections::HashMap;
use std::fs::File;
use std::os::fd::{AsFd, BorrowedFd, RawFd};
use std::path::Path;
use std::str::FromStr;

use rustix::fs::{Mode, OFlags};

use crate::platform::linux::proc_fd::ProcFd;
use crate::platform::linux::proc_info::ProcInfo;
use crate::platform::linux::statics::O_PATH_MAYBE;

pub(super) fn build_inode_proc_map(proc_fds: Vec<ProcFd>) -> crate::Result<HashMap<u64, ProcInfo>> {
    let mut map: HashMap<u64, ProcInfo> = HashMap::new();

    for proc_fd in proc_fds {
        let dir_fd = rustix::fs::openat(
            proc_fd.as_fd(),
            "fd",
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )?;
        let mut dir = rustix::fs::Dir::read_from(&dir_fd)?;
        dir.rewind();

        let mut socket_inodes = Vec::new();
        while let Some(Ok(entry)) = dir.next() {
            let name = entry.file_name().to_string_lossy();
            if RawFd::from_str(&name).is_ok() {
                if let Ok(socket_inode) = get_socket_inode(dir_fd.as_fd(), name.as_ref()) {
                    socket_inodes.push(socket_inode);
                }
            }
        }

        let stat = rustix::fs::openat(
            proc_fd.as_fd(),
            "stat",
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
        )?;

        if let Ok(proc_info) = ProcInfo::from_file(File::from(stat)) {
            for inode in socket_inodes {
                map.insert(inode, proc_info.clone());
            }
        }
    }

    Ok(map)
}

fn get_socket_inode<P: AsRef<Path>>(dir_fd: BorrowedFd, path: P) -> crate::Result<u64> {
    let p = path.as_ref();

    let flags = OFlags::NOFOLLOW | OFlags::CLOEXEC | *O_PATH_MAYBE;
    let file = rustix::fs::openat(dir_fd, p, flags, Mode::empty())?;
    let link = rustix::fs::readlinkat(&file, "", Vec::new())?;

    let link_os = link.to_string_lossy();

    if !link_os.starts_with('/') && link_os.contains(':') {
        let mut s = link_os.split(':');
        let fd_type = s.next().ok_or("Failed to get fd type")?;
        if fd_type == "socket" {
            let mut inode_str = s.next().ok_or("Failed to get inode")?;
            inode_str = inode_str.strip_prefix('[').ok_or("Failed to get inode")?;
            inode_str = inode_str.strip_suffix(']').ok_or("Failed to get inode")?;
            let inode = u64::from_str(inode_str)?;
            return Ok(inode);
        }
    }

    Err("Not a socket inode".into())
}
