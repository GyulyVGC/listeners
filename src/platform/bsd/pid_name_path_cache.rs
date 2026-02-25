use std::collections::{HashMap, hash_map::Entry};

#[cfg(target_os = "freebsd")]
use super::ffi::freebsd as platform;

#[cfg(target_os = "netbsd")]
use super::ffi::netbsd as platform;

pub(super) struct ProcNamesPathsCache {
    cache: HashMap<i32, Option<(String, String)>>,
}

impl ProcNamesPathsCache {
    pub(super) fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub(super) fn get(&mut self, pid: i32) -> Option<(String, String)> {
        if let Entry::Vacant(e) = self.cache.entry(pid) {
            let name = platform::get_process_name(pid).ok();
            let path = Some(platform::get_process_path(pid).unwrap_or_default());

            e.insert(name.zip(path));
        }

        self.cache.get(&pid).cloned().flatten()
    }
}
