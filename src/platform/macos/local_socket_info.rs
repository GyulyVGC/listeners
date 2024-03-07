use std::net::IpAddr;

#[derive(Debug)]
pub(super) struct LocalSocketInfo {
    addr: IpAddr,
    port: u16,
}

impl LocalSocketInfo {
    pub(super) fn new(addr: IpAddr, port: u16) -> Self {
        LocalSocketInfo { addr, port }
    }
}
