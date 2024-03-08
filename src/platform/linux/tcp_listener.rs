use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

#[derive(Debug)]
pub(super) struct TcpListener {
    local_addr: SocketAddr,
    inode: u64,
}

impl TcpListener {
    const LISTEN_STATE: &'static str = "0A";

    pub(super) fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    pub(super) fn inode(&self) -> u64 {
        self.inode
    }

    pub(super) fn get_all() -> crate::Result<Vec<TcpListener>> {
        let mut table = Vec::new();
        let tcp_table = File::open("/proc/net/tcp")?;
        for line in BufReader::new(tcp_table).lines().flatten() {
            if let Ok(l) = TcpListener::from_tcp_table_entry(&line) {
                table.push(l);
            }
        }
        let tcp6_table = File::open("/proc/net/tcp6")?;
        for line in BufReader::new(tcp6_table).lines().flatten() {
            if let Ok(l) = TcpListener::from_tcp6_table_entry(&line) {
                table.push(l);
            }
        }
        Ok(table)
    }

    fn from_tcp_table_entry(line: &str) -> crate::Result<Self> {
        let mut s = line.split_whitespace();

        let local_addr_hex = s.nth(1).ok_or("Failed to get local address")?;
        let Some(Self::LISTEN_STATE) = s.nth(1) else {
            return Err("Not a listening socket".into());
        };

        let local_ip_port = local_addr_hex
            .split(':')
            .flat_map(|s| u32::from_str_radix(s, 16))
            .collect::<Vec<u32>>();

        let ip_n = local_ip_port.first().ok_or("Failed to get IP")?;
        let port_n = local_ip_port.get(1).ok_or("Failed to get port")?;
        let ip = Ipv4Addr::from(u32::to_be(*ip_n));
        let port = u16::try_from(*port_n)?;
        let local_addr = SocketAddr::new(IpAddr::V4(ip), port);

        let inode_n = s.nth(5).ok_or("Failed to get inode")?;
        let inode = u64::from_str(inode_n)?;

        Ok(Self { local_addr, inode })
    }

    fn from_tcp6_table_entry(line: &str) -> crate::Result<Self> {
        let mut s = line.split_whitespace();

        let local_addr_hex = s.nth(1).ok_or("Failed to get local address")?;
        let Some(Self::LISTEN_STATE) = s.nth(1) else {
            return Err("Not a listening socket".into());
        };

        let local_ip_port = local_addr_hex
            .split(':')
            .flat_map(|s| u128::from_str_radix(s, 16))
            .collect::<Vec<u128>>();

        let ip_n = local_ip_port.first().ok_or("Failed to get IP")?;
        let port_n = local_ip_port.get(1).ok_or("Failed to get port")?;
        let ip = Ipv6Addr::from(u128::to_be(*ip_n));
        let port = u16::try_from(*port_n)?;
        let local_addr = SocketAddr::new(IpAddr::V6(ip), port);

        let inode_n = s.nth(5).ok_or("Failed to get inode")?;
        let inode = u64::from_str(inode_n)?;

        Ok(Self { local_addr, inode })
    }
}
