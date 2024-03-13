use std::net::IpAddr;

#[derive(Debug)]
pub(super) struct TcpListener {
    local_addr: IpAddr,
    local_port: u16,
    pid: u32,
}

impl TcpListener {
    pub(super) fn new(local_addr: IpAddr, local_port: u16, pid: u32) -> Self {
        Self {
            local_addr,
            local_port,
            pid,
        }
    }

    pub(super) fn local_addr(&self) -> IpAddr {
        self.local_addr
    }

    pub(super) fn local_port(&self) -> u16 {
        self.local_port
    }

    pub(super) fn pids(&self) -> u32 {
        self.pid
    }
}
