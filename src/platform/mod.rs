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

/* ---------- bsd-like ---------- */

#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
mod bsd;
#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
use bsd as target_os;

/* -------- unsupported -------- */
#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "freebsd"),
    not(target_os = "openbsd"),
    not(target_os = "netbsd")
))]
mod unsupported;
#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "macos"),
    not(target_os = "windows"),
    not(target_os = "freebsd"),
    not(target_os = "openbsd"),
    not(target_os = "netbsd")
))]
use unsupported as target_os;
