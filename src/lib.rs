#![doc = include_str!("../README.md")]

use std::collections::HashSet;
use std::fmt::Display;
use std::net::SocketAddr;

mod platform;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Indicates whether the current operating system is supported by this library.
///
/// Currently, the supported operating systems are Windows, Linux, macOS, FreeBSD, OpenBSD and NetBSD.
pub const IS_OS_SUPPORTED: bool = cfg!(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
));

/// A process listening on a socket.
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Listener {
    /// The listening process.
    pub process: Process,
    /// The socket this listener is listening on.
    pub socket: SocketAddr,
    /// The protocol used.
    pub protocol: Protocol,
    /// The state of the socket connection.
    pub state: SocketState,
}

/// An active process.
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Process {
    /// Process ID.
    pub pid: u32,
    /// Process name.
    pub name: String,
    /// Process path.
    pub path: String,
}

/// The network protocol used by a socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// Transmission Control Protocol.
    TCP,
    /// User Datagram Protocol.
    UDP,
}

/// The state of a socket connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocketState {
    /// Connection is open and exchanging data.
    Established,
    /// Initiating a connection.
    SynSent,
    /// Received a connection request.
    SynReceived,
    /// First step of the four-way closing handshake.
    FinWait1,
    /// Second step of the four-way closing handshake.
    FinWait2,
    /// Waiting for remaining packets after close.
    TimeWait,
    /// Socket is not connected.
    Closed,
    /// Received a FIN, waiting to send FIN.
    CloseWait,
    /// Sent FIN, waiting for ACK.
    LastAck,
    /// Listening for incoming connections.
    Listen,
    /// Both sides sent FIN simultaneously.
    Closing,
    /// State is unknown or not applicable (e.g. UDP).
    Unknown,
}

impl SocketState {
    #[cfg(target_os = "linux")]
    pub(crate) fn from_linux(raw: u8) -> Self {
        match raw {
            0x01 => Self::Established,
            0x02 => Self::SynSent,
            0x03 => Self::SynReceived,
            0x04 => Self::FinWait1,
            0x05 => Self::FinWait2,
            0x06 => Self::TimeWait,
            0x07 => Self::Closed,
            0x08 => Self::CloseWait,
            0x09 => Self::LastAck,
            0x0A => Self::Listen,
            0x0B => Self::Closing,
            _ => Self::Unknown,
        }
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn from_windows(raw: u32) -> Self {
        match raw {
            1 => Self::Closed,
            2 => Self::Listen,
            3 => Self::SynSent,
            4 => Self::SynReceived,
            5 => Self::Established,
            6 => Self::FinWait1,
            7 => Self::FinWait2,
            8 => Self::CloseWait,
            9 => Self::Closing,
            10 => Self::LastAck,
            11 => Self::TimeWait,
            _ => Self::Unknown,
        }
    }

    #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub(crate) fn from_bsd(raw: i32) -> Self {
        match raw {
            0 => Self::Closed,
            1 => Self::Listen,
            2 => Self::SynSent,
            3 => Self::SynReceived,
            4 => Self::Established,
            5 => Self::CloseWait,
            6 => Self::FinWait1,
            7 => Self::Closing,
            8 => Self::LastAck,
            9 => Self::FinWait2,
            10 => Self::TimeWait,
            _ => Self::Unknown,
        }
    }
}

/// Returns all the [Listener]s.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ```
#[doc = include_str!("../examples/get_all.rs")]
/// ```
///
/// Output:
/// ``` text
/// PID: 440        Process name: ControlCenter             Socket: 0.0.0.0:0                      Protocol: UDP
/// PID: 456        Process name: rapportd                  Socket: [::]:49158                     Protocol: TCP
/// PID: 456        Process name: rapportd                  Socket: 0.0.0.0:49158                  Protocol: TCP
/// PID: 456        Process name: rapportd                  Socket: 0.0.0.0:0                      Protocol: UDP
/// PID: 485        Process name: sharingd                  Socket: 0.0.0.0:0                      Protocol: UDP
/// PID: 516        Process name: WiFiAgent                 Socket: 0.0.0.0:0                      Protocol: UDP
/// PID: 1480       Process name: rustrover                 Socket: [::7f00:1]:63342               Protocol: TCP
/// PID: 2123       Process name: Telegram                  Socket: 192.168.1.102:49659            Protocol: TCP
/// PID: 2123       Process name: Telegram                  Socket: 192.168.1.102:49656            Protocol: TCP
/// PID: 2156       Process name: Google Chrome             Socket: 0.0.0.0:0                      Protocol: UDP
/// PID: 2167       Process name: Google Chrome Helper      Socket: 192.168.1.102:60834            Protocol: UDP
/// PID: 2167       Process name: Google Chrome Helper      Socket: 192.168.1.102:53220            Protocol: UDP
/// PID: 2167       Process name: Google Chrome Helper      Socket: 192.168.1.102:59216            Protocol: UDP
/// ```
pub fn get_all() -> Result<HashSet<Listener>> {
    platform::get_all()
}

/// Returns the [Process] listening on a given port.
///
/// # Arguments
///
/// * `port` - The port to look for.
/// * `protocol` - The protocol to look for (TCP or UDP).
///
/// # Errors
///
/// This function returns an error if it fails to retrieve the process listening on the given port, or if no process is found.
///
/// # Example
///
///  ``` no_run
#[doc = include_str!("../examples/get_process_by_port.rs")]
/// ```
///
/// Output:
/// ``` text
/// PID: 2123       Process name: Telegram
/// ```
pub fn get_process_by_port(port: u16, protocol: Protocol) -> Result<Process> {
    if port == 0 {
        return Err("Port can't be 0".into());
    }

    platform::get_process_by_port(port, protocol)
}

impl Listener {
    fn new(
        pid: u32,
        name: String,
        path: String,
        socket: SocketAddr,
        protocol: Protocol,
        state: SocketState,
    ) -> Self {
        let process = Process::new(pid, name, path);
        Self {
            process,
            socket,
            protocol,
            state,
        }
    }
}

impl Process {
    fn new(pid: u32, name: String, path: String) -> Self {
        Self { pid, name, path }
    }
}

impl Display for Listener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Listener {
            process,
            socket,
            protocol,
            state,
        } = self;
        let process = process.to_string();
        write!(
            f,
            "{process:<55} Socket: {socket:<30} Protocol: {protocol} State: {state}"
        )
    }
}

impl Display for SocketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketState::Established => write!(f, "ESTABLISHED"),
            SocketState::SynSent => write!(f, "SYN_SENT"),
            SocketState::SynReceived => write!(f, "SYN_RECEIVED"),
            SocketState::FinWait1 => write!(f, "FIN_WAIT1"),
            SocketState::FinWait2 => write!(f, "FIN_WAIT2"),
            SocketState::TimeWait => write!(f, "TIME_WAIT"),
            SocketState::Closed => write!(f, "CLOSED"),
            SocketState::CloseWait => write!(f, "CLOSE_WAIT"),
            SocketState::LastAck => write!(f, "LAST_ACK"),
            SocketState::Listen => write!(f, "LISTEN"),
            SocketState::Closing => write!(f, "CLOSING"),
            SocketState::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Process { pid, name, .. } = self;
        write!(f, "PID: {pid:<10} Process name: {name}")
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use crate::{Listener, Process, Protocol, SocketState};

    #[test]
    fn test_v4_listener_to_string() {
        let listener = Listener::new(
            455,
            "rapportd".to_string(),
            "path/to/rapportd".to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 51189),
            Protocol::TCP,
            SocketState::Listen,
        );
        assert_eq!(
            listener.to_string(),
            "PID: 455        Process name: rapportd                  Socket: 0.0.0.0:51189                  Protocol: TCP State: LISTEN"
        );
    }

    #[test]
    fn test_v6_listener_to_string() {
        let listener = Listener::new(
            160,
            "mysqld".to_string(),
            "path/to/mysqld".to_string(),
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 3306),
            Protocol::UDP,
            SocketState::Unknown,
        );
        assert_eq!(
            listener.to_string(),
            "PID: 160        Process name: mysqld                    Socket: [::]:3306                      Protocol: UDP State: UNKNOWN"
        );
    }

    #[test]
    fn test_process_to_string() {
        let process = Process::new(
            611,
            "Microsoft SharePoint".to_string(),
            "path/to/sharepoint".to_string(),
        );
        assert_eq!(
            process.to_string(),
            "PID: 611        Process name: Microsoft SharePoint"
        );
    }
}
