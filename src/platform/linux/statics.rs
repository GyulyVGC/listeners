use once_cell::sync::Lazy;

pub(super) const ROOT: &str = "/proc";

pub(super) static KERNEL: Lazy<Option<String>> = Lazy::new(|| {
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_owned())
        .ok()
});
