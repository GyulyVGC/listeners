use std::path::Path;
use std::str::FromStr;
use rustix::fs::{OFlags, Mode};

const ROOT: &str = "/proc";

pub(crate) fn hi() {
    // procfs::process::all_processes().unwrap();
    let root = rustix::fs::openat(
            rustix::fs::CWD,
            Path::new(ROOT),
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty()
        ).unwrap();
    let dir = rustix::fs::Dir::read_from(root).unwrap();
    for entry in dir {
        if let Some(Ok(e)) = entry {
            if let Ok(pid) = i32::from_str(&e.file_name().to_string_lossy()) {
                Some(Process::new_with_root(self.root.join(pid.to_string())));
            }
        }
    }
}
