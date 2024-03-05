use once_cell::sync::Lazy;
use rustix::fs::{Mode, OFlags};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::fd::{AsFd, BorrowedFd, OwnedFd, RawFd};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::Listener;

const ROOT: &str = "/proc";

static KERNEL: Lazy<Option<String>> = Lazy::new(|| {
    std::fs::read_to_string("/proc/sys/kernel/osrelease")
        .and_then(|s| Ok(s.trim().to_owned()))
        .ok()
});

pub(crate) fn hi() -> Result<HashSet<Listener>, String> {
    let mut listeners = HashSet::new();

    let processes = get_all_processes()?;

    let socket_inode_process_map = build_inode_process_map(processes)?;

    for tcp_listener in get_tcp_table()? {
        if let Some(p) = socket_inode_process_map.get(&tcp_listener.inode) {
            let listener = Listener {
                pid: p.pid,
                pname: p.name.clone(),
                socket: tcp_listener.local_addr,
            };
            listeners.insert(listener);
        }
    }
    Ok(listeners)
}

fn get_all_processes() -> Result<Vec<Process>, String> {
    let root = Path::new(ROOT);
    let dir = rustix::fs::openat(
        rustix::fs::CWD,
        root,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    ).map_err(|e| e.to_string())?;
    let dir = rustix::fs::Dir::read_from(dir).map_err(|e| e.to_string())?;

    let mut processes: Vec<Process> = vec![];
    for entry in dir {
        if let Ok(e) = entry {
            if let Ok(pid) = i32::from_str(&e.file_name().to_string_lossy()) {
                let proc_root = PathBuf::from(root).join(pid.to_string());

                // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
                let flags = match &*KERNEL {
                    Some(v) if v < &String::from("3.6.0") => OFlags::DIRECTORY | OFlags::CLOEXEC,
                    Some(_) | None => OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
                };
                let file =
                    rustix::fs::openat(rustix::fs::CWD, &proc_root, flags, Mode::empty()).map_err(|e| e.to_string())?;

                let pid_res = proc_root
                    .as_path()
                    .components()
                    .last()
                    .and_then(|c| match c {
                        std::path::Component::Normal(s) => Some(s),
                        _ => None,
                    })
                    .and_then(|s| s.to_string_lossy().parse::<u32>().ok())
                    .or_else(|| {
                        rustix::fs::readlinkat(rustix::fs::CWD, &proc_root, Vec::new())
                            .ok()
                            .and_then(|s| s.to_string_lossy().parse::<u32>().ok())
                    });
                let pid = pid_res.ok_or("Failed to parse pid")?;

                processes.push(Process::new(pid, file, proc_root));
            }
        }
    }
    Ok(processes)
}

fn build_inode_process_map(processes: Vec<Process>) -> Result<HashMap<u64, PidName>, String> {
    let mut map: HashMap<u64, PidName> = HashMap::new();
    for proc in processes {
        let stat = rustix::fs::openat(
            &proc.fd,
            "stat",
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
        ).map_err(|e| e.to_string())?;
        let dir_fd = rustix::fs::openat(
            &proc.fd,
            "fd",
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        ).map_err(|e| e.to_string())?;
        let mut dir = rustix::fs::Dir::read_from(&dir_fd).map_err(|e| e.to_string())?;
        dir.rewind();
        let mut socket_inodes = Vec::new();
        while let Some(Ok(entry)) = dir.next() {
            let name = entry.file_name().to_string_lossy();
            if RawFd::from_str(&name).is_ok() {
                if let Ok(socket_inode) = get_socket_inode(dir_fd.as_fd(), name.as_ref()) {
                    socket_inodes.push(socket_inode);
                }
            }
        }
        if let Ok(pid_name) = PidName::from_file(File::from(stat)) {
            for inode in socket_inodes {
                map.insert(inode, pid_name.clone());
            }
        }
    }
    Ok(map)
}

#[derive(Debug)]
struct Process {
    pid: u32,
    fd: OwnedFd,
    root: PathBuf,
}

impl Process {
    fn new(pid: u32, fd: OwnedFd, root: PathBuf) -> Self {
        Process { pid, fd, root }
    }
}

#[derive(Clone, Debug)]
struct PidName {
    pid: u32,
    name: String,
}

impl PidName {
    fn from_file<R: Read>(mut r: R) -> Result<Self, String> {
        // read in entire thing, this is only going to be 1 line
        let mut buf = Vec::new();
        r.read_to_end(&mut buf).map_err(|e| e.to_string())?;

        let line = String::from_utf8_lossy(&buf);
        let buf = line.trim();

        // find the first opening paren, and split off the first part (pid)
        let start_paren = buf.find('(').ok_or("Failed to find opening paren")?;
        let end_paren = buf.rfind(')').ok_or("Failed to find closing paren")?;
        let pid_s = &buf[..start_paren - 1];
        let comm = buf[start_paren + 1..end_paren].to_string();

        let pid = FromStr::from_str(pid_s).map_err(|e| e.to_string())?;

        Ok(PidName { pid, name: comm })
    }
}

fn get_socket_inode<P: AsRef<Path>>(dir_fd: BorrowedFd, path: P) -> Result<u64, String> {
    let p = path.as_ref();
    // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
    let flags = match &*KERNEL {
        Some(v) if v < &String::from("3.6.0") => OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Some(_) | None => OFlags::NOFOLLOW | OFlags::PATH | OFlags::CLOEXEC,
    };
    let file = rustix::fs::openat(dir_fd, p, flags, Mode::empty()).map_err(|e| e.to_string())?;
    let link = rustix::fs::readlinkat(&file, "", Vec::new()).map_err(|e| e.to_string())?;

    let link_os = link.to_string_lossy();

    fn strip_first_last(s: &str) -> &str {
        let mut c = s.chars();
        // remove the first and last characters
        let _ = c.next();
        let _ = c.next_back();
        c.as_str()
    }

    return if !link_os.starts_with('/') && link_os.contains(':') {
        let mut s = link_os.split(':');
        let fd_type = s.next().ok_or("Failed to get fd type")?;
        if fd_type == "socket" {
            let inode_str = s.next().ok_or("Failed to get inode")?;
            let inode = u64::from_str(strip_first_last(inode_str)).map_err(|e| e.to_string())?;
            Ok(inode)
        } else {
            Err("Not a socket".to_string())
        }
    } else {
        Err("Not a socket".to_string())
    };
}

fn get_tcp_table() -> Result<Vec<TcpListener>, String> {
    let mut table = Vec::new();
    let file = File::open("/proc/net/tcp").map_err(|e| e.to_string())?;
    for line in BufReader::new(file).lines().flatten() {
        if let Ok(l) = TcpListener::from_line(&line) {
            table.push(l)
        }
    }
    Ok(table)
}

#[derive(Debug)]
struct TcpListener {
    local_addr: SocketAddr,
    inode: u64,
}

impl TcpListener {
    const LISTEN_STATE: &'static str = "0A";

    fn from_line(line: &str) -> Result<Self, String> {
        let mut s = line.trim().split_whitespace();

        let local_addr_hex = s.nth(1).ok_or("Failed to get local address")?;
        let Some(Self::LISTEN_STATE) = s.nth(1) else {
            return Err("Not a listening socket".to_string());
        };

        let local_ip_port = local_addr_hex
            .split(':')
            .map(|s| u32::from_str_radix(s, 16))
            .flatten()
            .collect::<Vec<u32>>();

        let ip_n = local_ip_port.get(0).ok_or("Failed to get IP")?;
        let port_n = local_ip_port.get(1).ok_or("Failed to get port")?;
        let ip = Ipv4Addr::from(*ip_n);
        let port = u16::try_from(*port_n).map_err(|e| e.to_string())?;
        let local_addr = SocketAddr::new(IpAddr::V4(ip), port);

        let inode_n = s.nth(5).ok_or("Failed to get inode")?;
        let inode = u64::from_str(inode_n).map_err(|e| e.to_string())?;

        Ok(Self {
            local_addr,
            inode,
        })
    }
}
