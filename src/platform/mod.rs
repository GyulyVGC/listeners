pub use target_os::get_all;

/* ---------- windows ---------- */
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as target_os;

/* ----------- macos ----------- */
#[cfg(any(target_os = "macos", target_os = "openbsd"))]
mod macos;
#[cfg(any(target_os = "macos", target_os = "openbsd"))]
use macos as target_os;

/* ----------- linux ----------- */
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as target_os;
