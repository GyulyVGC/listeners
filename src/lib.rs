#![doc = include_str!("../README.md")]

use std::collections::HashSet;
use std::fmt::Display;
use std::net::SocketAddr;

mod platform;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A process listening on a socket.
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Listener {
    /// The listening process.
    pub process: Process,
    /// The socket this listener is listening on.
    pub socket: SocketAddr,
    /// The protocol used.
    pub protocol: Protocol,
}

/// A process, characterized by its PID and name.
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Process {
    /// Process ID.
    pub pid: u32,
    /// Process name.
    pub name: String,
    /// Process path.
    pub path: String,
}

/// The protocol used by the listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// Transmission Control Protocol.
    TCP,
    /// User Datagram Protocol.
    UDP,
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

/// Returns the list of [Process]es listening on a given port.
///
/// # Arguments
///
/// * `port` - The port to look for.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ``` no_run
#[doc = include_str!("../examples/get_processes_by_port.rs")]
/// ```
///
/// Output:
/// ``` text
/// PID: 2123       Process name: Telegram
/// ```
pub fn get_processes_by_port(port: u16) -> Result<HashSet<Process>> {
    platform::get_all().map(|listeners| {
        listeners
            .into_iter()
            .filter(|listener| listener.socket.port() == port)
            .map(|listener| listener.process)
            .collect()
    })
}

/// Returns the list of ports listened to by a process given its PID.
///
/// # Arguments
///
/// * `pid` - The PID of the process.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ``` no_run
#[doc = include_str!("../examples/get_ports_by_pid.rs")]
/// ```
///
/// Output:
/// ``` text
/// 3306
/// 33060
/// ```
pub fn get_ports_by_pid(pid: u32) -> Result<HashSet<u16>> {
    platform::get_all().map(|listeners| {
        listeners
            .into_iter()
            .filter(|listener| listener.process.pid == pid)
            .map(|listener| listener.socket.port())
            .collect()
    })
}

/// Returns the list of ports listened to by a process given its name.
///
/// # Arguments
///
/// * `name` - The name of the process.
///
/// # Errors
///
/// This function returns an error if it fails to retrieve listeners for the current platform.
///
/// # Example
///
///  ``` no_run
#[doc = include_str!("../examples/get_ports_by_process_name.rs")]
/// ```
///
/// Output:
/// ``` text
/// 3306
/// 33060
/// ```
pub fn get_ports_by_process_name(name: &str) -> Result<HashSet<u16>> {
    platform::get_all().map(|listeners| {
        listeners
            .into_iter()
            .filter(|listener| listener.process.name == name)
            .map(|listener| listener.socket.port())
            .collect()
    })
}

impl Listener {
    fn new(pid: u32, name: String, path: String, socket: SocketAddr, protocol: Protocol) -> Self {
        let process = Process::new(pid, name, path);
        Self {
            process,
            socket,
            protocol,
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
        } = self;
        let process = process.to_string();
        write!(f, "{process:<55} Socket: {socket:<30} Protocol: {protocol}",)
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

    use crate::{Listener, Process, Protocol};

    #[test]
    fn test_v4_listener_to_string() {
        let listener = Listener::new(
            455,
            "rapportd".to_string(),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 51189),
            Protocol::TCP,
        );
        assert_eq!(
            listener.to_string(),
            "PID: 455        Process name: rapportd                  Socket: 0.0.0.0:51189                  Protocol: TCP"
        );
    }

    #[test]
    fn test_v6_listener_to_string() {
        let listener = Listener::new(
            160,
            "mysqld".to_string(),
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 3306),
            Protocol::UDP,
        );
        assert_eq!(
            listener.to_string(),
            "PID: 160        Process name: mysqld                    Socket: [::]:3306                      Protocol: UDP"
        );
    }

    #[test]
    fn test_process_to_string() {
        let process = Process::new(611, "Microsoft SharePoint".to_string());
        assert_eq!(
            process.to_string(),
            "PID: 611        Process name: Microsoft SharePoint"
        );
    }
}
