use std::os::fd::OwnedFd;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use rustix::fs::{Mode, OFlags};

use crate::platform::linux::statics::{O_PATH_MAYBE, ROOT};

#[derive(Debug)]
pub(super) struct ProcFd(OwnedFd);

impl ProcFd {
    fn new(fd: OwnedFd) -> Self {
        ProcFd(fd)
    }

    pub(super) fn as_fd(&self) -> &OwnedFd {
        &self.0
    }

    pub(super) fn get_all() -> crate::Result<Vec<ProcFd>> {
        let root = Path::new(ROOT);
        let dir = rustix::fs::openat(
            rustix::fs::CWD,
            root,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )?;
        let dir = rustix::fs::Dir::read_from(dir)?;

        let mut proc_fds: Vec<ProcFd> = vec![];
        for entry in dir.flatten() {
            if let Ok(pid) = i32::from_str(&entry.file_name().to_string_lossy()) {
                let proc_root = PathBuf::from(root).join(pid.to_string());

                let flags = OFlags::DIRECTORY | OFlags::CLOEXEC | *O_PATH_MAYBE;
                let file = rustix::fs::openat(rustix::fs::CWD, &proc_root, flags, Mode::empty())?;

                proc_fds.push(ProcFd::new(file));
            }
        }
        Ok(proc_fds)
    }
}
