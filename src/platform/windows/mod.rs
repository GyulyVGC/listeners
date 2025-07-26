use std::collections::HashSet;

use proto_listener::ProtoListener;

use crate::Listener;

mod c_iphlpapi;
mod proto_listener;
mod socket_table;
mod statics;
mod tcp6_table;
mod tcp_table;
mod udp6_table;
mod udp_table;

pub(crate) fn get_all() -> crate::Result<HashSet<Listener>> {
    let mut listeners = HashSet::new();

    for proto_listener in ProtoListener::get_all() {
        if let Some(listener) = proto_listener.to_listener() {
            listeners.insert(listener);
        }
    }

    Ok(listeners)
}
