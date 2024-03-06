use std::os::fd::OwnedFd;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use rustix::fs::{Mode, OFlags};

use crate::platform::linux::statics::{KERNEL, ROOT};

#[derive(Debug)]
pub(super) struct ProcFd(OwnedFd);

impl ProcFd {
    fn new(fd: OwnedFd) -> Self {
        ProcFd(fd)
    }

    pub(super) fn as_fd(&self) -> &OwnedFd {
        &self.0
    }

    pub(super) fn get_all() -> Result<Vec<ProcFd>, String> {
        let root = Path::new(ROOT);
        let dir = rustix::fs::openat(
            rustix::fs::CWD,
            root,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(|e| e.to_string())?;
        let dir = rustix::fs::Dir::read_from(dir).map_err(|e| e.to_string())?;

        let mut proc_fds: Vec<ProcFd> = vec![];
        for entry in dir.flatten() {
            if let Ok(pid) = i32::from_str(&entry.file_name().to_string_lossy()) {
                let proc_root = PathBuf::from(root).join(pid.to_string());

                // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
                let flags = match &*KERNEL {
                    Some(v) if v < &String::from("3.6.0") => OFlags::DIRECTORY | OFlags::CLOEXEC,
                    Some(_) | None => OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
                };
                let file = rustix::fs::openat(rustix::fs::CWD, &proc_root, flags, Mode::empty())
                    .map_err(|e| e.to_string())?;

                proc_fds.push(ProcFd::new(file));
            }
        }
        Ok(proc_fds)
    }
}
