use once_cell::sync::Lazy;
use procfs::process::{Stat};
use rustix::fs::{AtFlags, Mode, OFlags};
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::os::fd::{AsFd, BorrowedFd, OwnedFd, RawFd};
use std::path::{Path, PathBuf};
use std::str::FromStr;

const ROOT: &str = "/proc";

static KERNEL: Lazy<Option<&str>> = Lazy::new(|| {
    std::fs::read_to_string("/proc/sys/kernel/osreleas")
        .and_then(|s| Ok(s.trim()))
        .ok()
});

pub(crate) fn hi() {}

fn get_all_processes() -> Vec<Process> {
    procfs::process::all_processes().unwrap();

    let root = rustix::fs::openat(
        rustix::fs::CWD,
        Path::new(ROOT),
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();
    let dir = rustix::fs::Dir::read_from(root).unwrap();

    let mut processes: Vec<Process> = vec![];
    for entry in dir {
        if let Ok(e) = entry {
            if let Ok(pid) = i32::from_str(&e.file_name().to_string_lossy()) {
                let proc_root = PathBuf::from(ROOT).join(pid.to_string());

                // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
                let flags = match *KERNEL {
                    Some(v) if v < "3.6.0" => OFlags::DIRECTORY | OFlags::CLOEXEC,
                    Some(_) | None => OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
                };
                let file = rustix::fs::openat(rustix::fs::CWD, &proc_root, flags, Mode::empty()).unwrap();

                let pidres = proc_root
                    .as_path()
                    .components()
                    .last()
                    .and_then(|c| match c {
                        std::path::Component::Normal(s) => Some(s),
                        _ => None,
                    })
                    .and_then(|s| s.to_string_lossy().parse::<i32>().ok())
                    .or_else(|| {
                        rustix::fs::readlinkat(rustix::fs::CWD, &proc_root, Vec::new())
                            .ok()
                            .and_then(|s| s.to_string_lossy().parse::<i32>().ok())
                    });
                let pid = pidres.unwrap();

                processes.push(Process::new(pid, file, proc_root));
            }
        }
    }
    processes
}

fn build_inode_process_map(processes: Vec<Process>) -> HashMap<u64, PidName> {
    let mut map: HashMap<u64, PidName> = HashMap::new();
    for process in processes {
        let read = rustix::fs::openat(
            process.fd,
            "stat",
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
        ).unwrap();
        let dir_fd = rustix::fs::openat(
            &process.fd,
            "fd",
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .unwrap();
        let mut dir = rustix::fs::Dir::read_from(&dir_fd).unwrap();
        let mut socket_inodes = Vec::new();
        if let Some(Ok(entry)) = dir.next() {
            let name = entry.file_name().to_string_lossy();
            if let Ok(fd) = RawFd::from_str(&name) {
                if let Some(socket_inode) =
                    get_socket_inodes(process.root, dir_fd.as_fd(), name.as_ref(), fd)
                {
                    socket_inodes.push(socket_inode);
                }
            }
        }
        if let Some(pid_name) = PidName::from_read(read) {
            for inode in socket_inodes {
                map.insert(inode, pid_name.clone());
            }
        }
    }
    map
}

struct Process {
    pid: i32,
    fd: OwnedFd,
    root: PathBuf,
}

impl Process {
    fn new(pid: i32, fd: OwnedFd, root: PathBuf) -> Self {
        Process { pid, fd, root }
    }
}

struct PidName {
    pid: i32,
    name: String,
}

impl PidName {
    fn from_read<R: Read>(mut r: R) -> Option<Self> {
        // read in entire thing, this is only going to be 1 line
        let mut buf = Vec::with_capacity(512);
        r.read_to_end(&mut buf).unwrap();

        let line = String::from_utf8_lossy(&buf);
        let buf = line.trim();

        // find the first opening paren, and split off the first part (pid)
        let start_paren = buf.find('(').unwrap();
        let end_paren = buf.rfind(')').unwrap();
        let pid_s = &buf[..start_paren - 1];
        let comm = buf[start_paren + 1..end_paren].to_string();

        let pid = FromStr::from_str(pid_s).unwrap();

        Some(PidName { pid, name: comm })
    }
}

fn get_socket_inodes<P: AsRef<Path>, Q: AsRef<Path>>(
    base: P,
    dirfd: BorrowedFd,
    path: Q,
    fd: i32,
) -> Option<u64> {
    let p = path.as_ref();
    let root = base.as_ref().join(p);
    // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
    let flags = match *KERNEL {
        Ok(v) if v < "3.6.0" => OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Ok(_) | Err(_) => OFlags::NOFOLLOW | OFlags::PATH | OFlags::CLOEXEC,
    };
    let file = rustix::fs::openat(dirfd, p, flags, Mode::empty()).unwrap();
    let link = rustix::fs::readlinkat(&file, "", Vec::new())
        .map_err(io::Error::from)
        .unwrap();
    let md = rustix::fs::statat(&file, "", AtFlags::SYMLINK_NOFOLLOW | AtFlags::EMPTY_PATH)
        .map_err(io::Error::from)
        .unwrap();

    let link_os = link.to_string_lossy();

    fn strip_first_last(s: &str) -> &str {
        let mut c = s.chars();
        // remove the first and last characters
        let _ = c.next();
        let _ = c.next_back();
        c.as_str()
    }

    return if !link_os.starts_with('/') && link_os.contains(':') {
        let mut s = link_os.split(':');
        let fd_type = s.next().unwrap();
        if fd_type == "socket" {
            let inode = s.next().unwrap();
            let inode = u64::from_str(strip_first_last(inode)).unwrap();
            Some(inode)
        } else {
            None
        }
    } else {
        None
    };
}
