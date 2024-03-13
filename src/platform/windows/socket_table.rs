use crate::platform::target_os::c_iphlpapi::get_extended_tcp_table;
use crate::platform::target_os::statics::FALSE;
use crate::platform::target_os::tcp_listener::TcpListener;
use crate::platform::windows::statics::{
    AF_INET, ERROR_INSUFFICIENT_BUFFER, LISTEN, NO_ERROR, TCP_TABLE_OWNER_PID_ALL,
};
use crate::platform::windows::tcp_table::{TcpRow, TcpTable};
use std::ffi::{c_ulong, c_void};
use std::net::{IpAddr, Ipv4Addr};

pub trait SocketTable {
    fn get_table() -> crate::Result<Vec<u8>>;
    fn get_rows_count(table: &[u8]) -> usize;
    fn get_tcp_listener(table: &[u8], index: usize) -> Option<TcpListener>;
}

impl SocketTable for TcpTable {
    fn get_table() -> crate::Result<Vec<u8>> {
        get_tcp_table(AF_INET)
    }
    fn get_rows_count(table: &[u8]) -> usize {
        let table = unsafe { &*(table.as_ptr() as *const TcpTable) };
        table.rows_count as usize
    }
    fn get_tcp_listener(table: &[u8], index: usize) -> Option<TcpListener> {
        let table = unsafe { &*(table.as_ptr() as *const TcpTable) };
        let rows_ptr = &table.rows[0] as *const TcpRow;
        let row = unsafe { &*rows_ptr.add(index) };
        if row.state == LISTEN {
            Some(TcpListener::new(
                IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                u16::from_be(row.local_port as u16),
                vec![row.owning_pid],
            ))
        } else {
            None
        }
    }
}

fn get_tcp_table(address_family: c_ulong) -> crate::Result<Vec<u8>> {
    let mut table_size: c_ulong = 0;
    let mut err_code = unsafe {
        get_extended_tcp_table(
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
            get_extended_tcp_table(
                table.as_mut_ptr() as *mut c_void,
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
