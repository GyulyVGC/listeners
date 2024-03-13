use std::collections::HashSet;
use crate::Listener;
use crate::platform::windows::socket_table::SocketTable;
use crate::platform::windows::tcp6_table::Tcp6Table;
use crate::platform::windows::tcp_listener::TcpListener;
use crate::platform::windows::tcp_table::TcpTable;

mod c_iphlpapi;
mod socket_table;
mod statics;
mod tcp6_table;
mod tcp_listener;
mod tcp_table;

pub fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    let tcp_listeners = entries::<TcpTable>();
    let tcp6_listeners = entries::<Tcp6Table>();

    for tcp_listener in tcp_listeners.iter().flatten().chain(tcp6_listeners.iter().flatten()) {
        if let Some(listener) = tcp_listener.to_listener() {
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}

fn entries<Table: SocketTable>() -> crate::Result<Vec<TcpListener>> {
    let mut tcp_listeners = Vec::new();
    let table = Table::get_table()?;
    for i in 0..Table::get_rows_count(&table) {
        if let Some(tcp_listener) = Table::get_tcp_listener(&table, i) {
            tcp_listeners.push(tcp_listener);
        }
    }
    Ok(tcp_listeners)
}
