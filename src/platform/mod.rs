/* ---------- windows ---------- */
#[cfg(target_os = "windows")]
pub use windows::get_all;
#[cfg(target_os = "windows")]
mod windows;

/* ----------- macos ----------- */
#[cfg(target_os = "macos")]
pub use macos::get_all;
#[cfg(target_os = "macos")]
mod macos;

/* ----------- linux ----------- */
#[cfg(target_os = "linux")]
pub use linux::get_all;
#[cfg(target_os = "linux")]
mod linux;
