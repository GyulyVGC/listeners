use std::net::{IpAddr, SocketAddr};

#[derive(Debug)]
pub(super) struct LocalSocketInfo {
    socket_addr: SocketAddr,
}

impl LocalSocketInfo {
    pub(super) fn new(addr: IpAddr, port: u16) -> Self {
        LocalSocketInfo { socket_addr: SocketAddr::new(addr, port) }
    }

    pub(super) fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }
}
