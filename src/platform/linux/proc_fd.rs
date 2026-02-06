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

    pub(super) fn get_all() -> crate::Result<impl Iterator<Item = ProcFd>> {
        let root = Path::new(ROOT);
        let dir = rustix::fs::openat(
            rustix::fs::CWD,
            root,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )?;
        let dir = rustix::fs::Dir::read_from(dir)?;

        Ok(ProcFdsIter {
            iter: dir.flatten(),
        })
    }
}

struct ProcFdsIter {
    iter: std::iter::Flatten<rustix::fs::Dir>,
}

impl Iterator for ProcFdsIter {
    type Item = ProcFd;

    fn next(&mut self) -> Option<Self::Item> {
        let root = Path::new(ROOT);

        for entry in self.iter.by_ref() {
            if let Ok(pid) = i32::from_str(&entry.file_name().to_string_lossy()) {
                let proc_root = PathBuf::from(root).join(pid.to_string());

                let flags = OFlags::DIRECTORY | OFlags::CLOEXEC | *O_PATH_MAYBE;
                let Ok(file) =
                    rustix::fs::openat(rustix::fs::CWD, &proc_root, flags, Mode::empty())
                else {
                    continue;
                };

                return Some(ProcFd::new(file));
            }
        }

        None
    }
}
