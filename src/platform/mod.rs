pub(crate) use target_os::get_all;

/* ---------- windows ---------- */
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as target_os;

/* ----------- macos ----------- */
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as target_os;

/* ----------- linux ----------- */
#[cfg(target_os = "linux")]
mod macos;
#[cfg(target_os = "linux")]
use macos as target_os;

/* ----------- other ----------- */
#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "windows")
))]
mod bsd;
#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "windows")
))]
use bsd as target_os;
