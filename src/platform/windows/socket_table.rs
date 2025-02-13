use std::ffi::{c_ulong, c_void};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::platform::target_os::c_iphlpapi::GetExtendedTcpTable;
use crate::platform::target_os::statics::FALSE;
use crate::platform::target_os::tcp_listener::TcpListener;
use crate::platform::windows::statics::{
    AF_INET, AF_INET6, ERROR_INSUFFICIENT_BUFFER, LISTEN, NO_ERROR, TCP_TABLE_OWNER_PID_ALL,
};
use crate::platform::windows::tcp6_table::Tcp6Table;
use crate::platform::windows::tcp_table::TcpTable;
use crate::platform::windows::udp_table::UdpTable;
use crate::platform::windows::udp6_table::Udp6Table;
use crate::Protocol;

use super::c_iphlpapi::GetExtendedUdpTable;
use super::statics::UDP_TABLE_OWNER_PID;



pub(super) trait SocketTable {
    fn get_table() -> crate::Result<Vec<u8>>;
    fn get_rows_count(table: &[u8]) -> usize;
    fn get_tcp_listener(table: &[u8], index: usize) -> Option<TcpListener>;
}

impl SocketTable for TcpTable {
    fn get_table() -> crate::Result<Vec<u8>> {
        get_tcp_table(AF_INET)
    }

    fn get_rows_count(table: &[u8]) -> usize {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<TcpTable>()) };
        table.rows_count as usize
    }

    fn get_tcp_listener(table: &[u8], index: usize) -> Option<TcpListener> {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<TcpTable>()) };
        let rows_ptr = std::ptr::addr_of!(table.rows[0]);
        let row = unsafe { &*rows_ptr.add(index) };
        // if row.state == LISTEN { // get all states
        Some(TcpListener::new(
                IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                u16::from_be(u16::try_from(row.local_port).ok()?),
                row.owning_pid,
                Protocol::TCP
            ))
        }
}

impl SocketTable for Tcp6Table {
    fn get_table() -> crate::Result<Vec<u8>> {
        get_tcp_table(AF_INET6)
    }

    fn get_rows_count(table: &[u8]) -> usize {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<Tcp6Table>()) };
        table.rows_count as usize
    }

    fn get_tcp_listener(table: &[u8], index: usize) -> Option<TcpListener> {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<Tcp6Table>()) };
        let rows_ptr = std::ptr::addr_of!(table.rows[0]);
        let row = unsafe { &*rows_ptr.add(index) };
        // if row.state == LISTEN { 
        Some(TcpListener::new(
                IpAddr::V6(Ipv6Addr::from(row.local_addr)),
                u16::from_be(u16::try_from(row.local_port).ok()?),
                row.owning_pid,
                Protocol::TCP
        ))
    }
}

impl SocketTable for UdpTable {
    fn get_table() -> crate::Result<Vec<u8>> {
        get_udp_table(AF_INET)
    }

    fn get_rows_count(table: &[u8]) -> usize {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<UdpTable>()) };
        table.rows_count as usize
    }

    fn get_tcp_listener(table: &[u8], index: usize) -> Option<TcpListener> {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<UdpTable>()) };
        let rows_ptr = std::ptr::addr_of!(table.rows[0]);
        let row = unsafe { &*rows_ptr.add(index) };
        Some(TcpListener::new(
            IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
            u16::from_be(u16::try_from(row.local_port).ok()?),
            row.owning_pid,
            Protocol::UDP
        ))
    }
}

impl SocketTable for Udp6Table {
    fn get_table() -> crate::Result<Vec<u8>> {
        get_udp_table(AF_INET6)
    }

    fn get_rows_count(table: &[u8]) -> usize {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<Udp6Table>()) };
        table.rows_count as usize
    }

    fn get_tcp_listener(table: &[u8], index: usize) -> Option<TcpListener> {
        #[allow(clippy::cast_ptr_alignment)]
        let table = unsafe { &*(table.as_ptr().cast::<Udp6Table>()) };
        let rows_ptr = std::ptr::addr_of!(table.rows[0]);
        let row = unsafe { &*rows_ptr.add(index) };
        Some(TcpListener::new(
            IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
            u16::from_be(u16::try_from(row.local_port).ok()?),
            row.owning_pid,
            Protocol::UDP
        ))
    }
}


// fn get_protocol_table(address_family: c_ulong, protocol: Protocol) -> crate::Result<Vec<u8>> {
//     match protocol {
//         Protocol::TCP | Protocol::TCP6 => get_tcp_table(address_family),
//         Protocol::UDP | Protocol::UDP6 => get_udp_table(address_family),
//     }
// }

fn get_udp_table(address_family: c_ulong) -> crate::Result<Vec<u8>> {
    let mut table_size: c_ulong = 0;
    let mut err_code = unsafe {
        GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut table_size,
            FALSE,
            address_family,
            UDP_TABLE_OWNER_PID,
            0,
        )
    };
    let mut table = Vec::<u8>::new();
    let mut iterations = 0;
    while err_code == ERROR_INSUFFICIENT_BUFFER {
        table = Vec::<u8>::with_capacity(table_size as usize);
        err_code = unsafe {
            GetExtendedUdpTable(
                table.as_mut_ptr().cast::<c_void>(),
                &mut table_size,
                FALSE,
                address_family,
                UDP_TABLE_OWNER_PID,
                0,
            )
        };
        iterations += 1;
        if iterations > 100 {
            return Err("Failed to allocate buffer".into());
        }
    }
    if err_code == NO_ERROR {
        Ok(table)
    } else {
        Err("Failed to get UDP table".into())
    }
}

fn get_tcp_table(address_family: c_ulong) -> crate::Result<Vec<u8>> {
    let mut table_size: c_ulong = 0;
    let mut err_code = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut table_size,
            FALSE,
            address_family,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    let mut table = Vec::<u8>::new();
    let mut iterations = 0;
    while err_code == ERROR_INSUFFICIENT_BUFFER {
        table = Vec::<u8>::with_capacity(table_size as usize);
        err_code = unsafe {
            GetExtendedTcpTable(
                table.as_mut_ptr().cast::<c_void>(),
                &mut table_size,
                FALSE,
                address_family,
                TCP_TABLE_OWNER_PID_ALL,
                0,
            )
        };
        iterations += 1;
        if iterations > 100 {
            return Err("Failed to allocate buffer".into());
        }
    }
    if err_code == NO_ERROR {
        Ok(table)
    } else {
        Err("Failed to get TCP table".into())
    }
}
