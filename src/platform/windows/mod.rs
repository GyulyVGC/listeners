use crate::Listener;
use std::collections::HashSet;
use tcp_listener::TcpListener;

mod c_iphlpapi;
mod socket_table;
mod statics;
mod tcp6_table;
mod tcp_listener;
mod tcp_table;

/// Returns the list of all processes listening on a TCP port.
///
/// # Errors
///
/// This function returns an error if it fails to get the list of processes for the current platform.
///
/// # Example
///
///  ``` rust
/// let listeners = listeners::get_all().unwrap();
///
/// for l in listeners {
///    println!("{l}");
/// }
/// ```
///
/// Output:
/// ``` text
/// PID: 1088       Process name: rustrover                 Socket: [::7f00:1]:63342
/// PID: 609        Process name: Microsoft SharePoint      Socket: [::1]:42050
/// PID: 160        Process name: mysqld                    Socket: [::]:33060
/// PID: 160        Process name: mysqld                    Socket: [::]:3306
/// PID: 460        Process name: rapportd                  Socket: 0.0.0.0:50928
/// PID: 460        Process name: rapportd                  Socket: [::]:50928
/// ```
pub fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for tcp_listener in TcpListener::get_all() {
        if let Some(listener) = tcp_listener.to_listener() {
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}
