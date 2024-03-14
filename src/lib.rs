#![doc = include_str!("../README.md")]

use std::fmt::Display;
use std::net::SocketAddr;

pub use platform::get_all;

mod platform;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A process listening on a TCP port.
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Listener {
    /// Process ID.
    pub pid: u32,
    /// Process name.
    pub name: String,
    /// The TCP socket this process is listening on.
    pub socket: SocketAddr,
}

impl Listener {
    fn new(pid: u32, name: String, socket: SocketAddr) -> Self {
        Self { pid, name, socket }
    }
}

impl Display for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PID: {:<10} Process name: {:<25} Socket: {}",
            self.pid, self.name, self.socket
        )
    }
}
