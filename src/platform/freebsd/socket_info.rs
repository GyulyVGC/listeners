use super::ffi::CSocketInfo;
use crate::Protocol;
use std::net::SocketAddr;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(super) struct SocketInfo {
    pub(super) address: SocketAddr,
    pub(super) protocol: Protocol,
}

impl From<&CSocketInfo> for SocketInfo {
    fn from(socket: &CSocketInfo) -> Self {
        Self {
            address: socket.to_sockaddr(),
            protocol: match socket.protocol {
                0 => Protocol::TCP,
                _ => Protocol::UDP,
            },
        }
    }
}
