use crate::platform::windows::socket_table::SocketTable;
use crate::platform::windows::tcp_table::TcpTable;

mod c_iphlpapi;
mod socket_table;
mod statics;
mod tcp_listener;
mod tcp_table;

pub fn get_all() {
    // let sis = netstat2::get_sockets_info().unwrap();
    let mut iterators = Vec::with_capacity(2);
    // iterators.push(SocketTableIterator::new::<MIB_TCPTABLE_OWNER_PID>()?);
    entries::<TcpTable>();
}

fn entries<Table: SocketTable>() {
    let mut tcp_listeners = Vec::new();
    let table = Table::get_table()?;
    for i in 0..Table::get_rows_count(&table) {
        if let Some(tcp_listener) = Table::get_tcp_listener(&table, i) {
            tcp_listeners.push(tcp_listener);
            println!("{:?}", tcp_listener);
        }
    }
}

fn get_name_from_pid(pid: u32) -> Option<String> {
    use std::mem::size_of;
    use std::mem::zeroed;
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
    };

    let h = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).unwrap() };

    let mut process = unsafe { zeroed::<PROCESSENTRY32>() };
    process.dwSize = size_of::<PROCESSENTRY32>() as u32;

    if unsafe { Process32First(h, &mut process) }.is_ok() {
        loop {
            if unsafe { Process32Next(h, &mut process) }.is_ok() {
                let id: u32 = process.th32ProcessID;
                if id == pid {
                    break;
                }
            } else {
                return None;
            }
        }
    }

    unsafe { CloseHandle(h).unwrap() };

    let name = process.szExeFile;
    let mut temp: Vec<u8> = vec![];
    let len = name.iter().position(|&x| x == 0).unwrap();

    for i in name.iter() {
        temp.push(*i as u8);
    }
    Some(String::from_utf8(temp[0..len].to_vec()).unwrap_or_default())
}
