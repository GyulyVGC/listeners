use rustix::fs::OFlags;

pub(super) const ROOT: &str = "/proc";

pub(super) static O_PATH_MAYBE: std::sync::LazyLock<OFlags> = std::sync::LazyLock::new(|| {
    let kernel = std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_owned())
        .ok();

    // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
    match kernel {
        Some(v) if v.as_str() < "3.6.0" => OFlags::empty(),
        Some(_) | None => OFlags::PATH,
    }
});
