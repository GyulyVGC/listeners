use crate::platform::linux::proc_fd::ProcFd;
use crate::platform::linux::proc_info::ProcInfo;
use crate::platform::linux::statics::KERNEL;
use rustix::fs::{Mode, OFlags};
use std::collections::HashMap;
use std::fs::File;
use std::os::fd::{AsFd, BorrowedFd, RawFd};
use std::path::Path;
use std::str::FromStr;

pub(super) fn build_inode_proc_map(
    proc_fds: &Vec<ProcFd>,
) -> Result<HashMap<u64, ProcInfo>, String> {
    let mut map: HashMap<u64, ProcInfo> = HashMap::new();

    for proc_fd in proc_fds {
        let dir_fd = rustix::fs::openat(
            proc_fd.as_fd(),
            "fd",
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(|e| e.to_string())?;
        let mut dir = rustix::fs::Dir::read_from(&dir_fd).map_err(|e| e.to_string())?;
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
        )
        .map_err(|e| e.to_string())?;

        if let Ok(proc_info) = ProcInfo::from_file(File::from(stat)) {
            for inode in socket_inodes {
                map.insert(inode, proc_info.clone());
            }
        }
    }

    Ok(map)
}

fn get_socket_inode<P: AsRef<Path>>(dir_fd: BorrowedFd, path: P) -> Result<u64, String> {
    let p = path.as_ref();
    // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
    let flags = match &*KERNEL {
        Some(v) if v < &String::from("3.6.0") => OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Some(_) | None => OFlags::NOFOLLOW | OFlags::PATH | OFlags::CLOEXEC,
    };
    let file = rustix::fs::openat(dir_fd, p, flags, Mode::empty()).map_err(|e| e.to_string())?;
    let link = rustix::fs::readlinkat(&file, "", Vec::new()).map_err(|e| e.to_string())?;

    let link_os = link.to_string_lossy();

    if !link_os.starts_with('/') && link_os.contains(':') {
        let mut s = link_os.split(':');
        let fd_type = s.next().ok_or("Failed to get fd type")?;
        if fd_type == "socket" {
            let mut inode_str = s.next().ok_or("Failed to get inode")?;
            inode_str = inode_str.strip_prefix('[').ok_or("Failed to get inode")?;
            inode_str = inode_str.strip_suffix(']').ok_or("Failed to get inode")?;
            let inode = u64::from_str(inode_str).map_err(|e| e.to_string())?;
            return Ok(inode);
        }
    }

    Err("Not a socket inode".to_string())
}
