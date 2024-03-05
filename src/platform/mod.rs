mod common;
pub use common::get_all;

/* windows */
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::get_all_listeners;

/* macos */
// #[cfg(target_os = "macos")]
// mod macos;
// #[cfg(target_os = "macos")]
// use macos::get_all_listeners;

/* linux */
// #[cfg(target_os = "linux")]
mod linux;
// #[cfg(target_os = "linux")]
use linux::get_all_listeners;
