use std::fmt::Display;
use std::net::SocketAddr;

pub use platform::get_all;

mod platform;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A struct representing a process that is listening on a socket
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Listener {
    /// The process ID of the listener process
    pub pid: u32,
    /// The name of the listener process
    pub pname: String,
    /// The local socket this listener is listening on
    pub socket: SocketAddr,
}

impl Listener {
    fn new(pid: u32, pname: String, socket: SocketAddr) -> Self {
        Self { pid, pname, socket }
    }
}

impl Display for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PID: {:<10} Process name: {:<25} Socket: {:<25}",
            self.pid, self.pname, self.socket
        )
    }
}
