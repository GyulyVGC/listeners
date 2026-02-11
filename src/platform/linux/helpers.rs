use std::collections::HashMap;
use std::fs::File;
use std::os::fd::{AsFd, BorrowedFd, RawFd};
use std::path::Path;
use std::str::FromStr;

use rustix::fs::{Mode, OFlags};

use crate::platform::linux::proc_fd::ProcFd;
use crate::platform::linux::proc_info::ProcInfo;
use crate::platform::linux::statics::O_PATH_MAYBE;

pub(super) fn build_inode_proc_map() -> crate::Result<HashMap<u64, ProcInfo>> {
    let proc_fds = ProcFd::get_all()?;
    let mut map: HashMap<u64, ProcInfo> = HashMap::new();

    for proc_fd in proc_fds {
        let dirfd = proc_fd.as_fd();
        let path = "fd";
        let Ok(dir_fd) = rustix::fs::openat(
            dirfd,
            path,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        ) else {
            continue;
        };
        let Ok(mut dir) = rustix::fs::Dir::read_from(&dir_fd) else {
            continue;
        };
        dir.rewind();

        let mut socket_inodes = Vec::new();
        for entry_res in dir {
            if let Ok(entry) = entry_res {
                let name = entry.file_name().to_string_lossy();
                if RawFd::from_str(&name).is_ok()
                    && let Ok(socket_inode) = get_socket_inode(dir_fd.as_fd(), name.as_ref())
                {
                    socket_inodes.push(socket_inode);
                }
            }
        }

        let Ok(stat) = rustix::fs::openat(
            proc_fd.as_fd(),
            "stat",
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
        ) else {
            continue;
        };

        if let Ok(proc_info) = ProcInfo::from_file(File::from(stat)) {
            for inode in socket_inodes {
                map.insert(inode, proc_info.clone());
            }
        }
    }

    Ok(map)
}

pub(super) fn get_proc_by_inode(inode: u64) -> crate::Result<ProcInfo> {
    let proc_fds = ProcFd::get_all()?;

    for proc_fd in proc_fds {
        let dirfd = proc_fd.as_fd();
        let path = "fd";
        let Ok(dir_fd) = rustix::fs::openat(
            dirfd,
            path,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        ) else {
            continue;
        };
        let Ok(mut dir) = rustix::fs::Dir::read_from(&dir_fd) else {
            continue;
        };
        dir.rewind();

        let mut inode_found = false;
        for entry_res in dir {
            if let Ok(entry) = entry_res {
                let name = entry.file_name().to_string_lossy();
                if RawFd::from_str(&name).is_ok()
                    && let Ok(socket_inode) = get_socket_inode(dir_fd.as_fd(), name.as_ref())
                    && socket_inode == inode
                {
                    inode_found = true;
                    break;
                }
            }
        }

        if !inode_found {
            continue;
        }

        let Ok(stat) = rustix::fs::openat(
            proc_fd.as_fd(),
            "stat",
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
        ) else {
            continue;
        };

        if let Ok(proc_info) = ProcInfo::from_file(File::from(stat)) {
            return Ok(proc_info);
        }
    }

    Err("No process found with the specified socket inode".into())
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
