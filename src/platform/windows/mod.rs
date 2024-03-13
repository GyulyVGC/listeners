use crate::Listener;
use socket_table::SocketTable;
use std::collections::HashSet;
use tcp_listener::TcpListener;

mod c_iphlpapi;
mod socket_table;
mod statics;
mod tcp6_table;
mod tcp_listener;
mod tcp_table;

pub fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for tcp_listener in TcpListener::get_all() {
        if let Some(listener) = tcp_listener.to_listener() {
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}
