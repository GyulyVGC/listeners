mod common;
pub use common::hi_cross;

/* windows */
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::hi;

/* macos */
// #[cfg(target_os = "macos")]
// mod macos;
// #[cfg(target_os = "macos")]
// use macos::hi;

/* linux */
// #[cfg(target_os = "linux")]
mod linux;
// #[cfg(target_os = "linux")]
use linux::hi;
