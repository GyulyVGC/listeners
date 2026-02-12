pub(crate) use target_os::get_all;
pub(crate) use target_os::get_process_by_port;

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
mod linux;
#[cfg(target_os = "linux")]
use linux as target_os;

/* ---------- freebsd ---------- */
#[cfg(target_os = "freebsd")]
mod freebsd;
#[cfg(target_os = "freebsd")]
use freebsd as target_os;

/* -------- unsupported -------- */
#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "freebsd")
))]
mod unsupported;
#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "freebsd")
))]
use unsupported as target_os;
