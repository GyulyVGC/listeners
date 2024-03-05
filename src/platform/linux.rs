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

pub(crate) fn hi() -> HashSet<Listener> {
    let mut listeners = HashSet::new();

    let processes = get_all_processes();

    let socket_inode_process_map = build_inode_process_map(processes);
    for (inode, process) in socket_inode_process_map {
        println!("Inode: {inode} Process: {}", process.name);
    }

    for tcp_listener in get_tcp_table() {
        if let Some(p) = socket_inode_process_map.get(&tcp_listener.inode) {
            let listener = Listener {
                pid: p.pid,
                pname: p.name.clone(),
                socket: tcp_listener.local_addr,
            };
            listeners.insert(listener);
        }
    }
    listeners
}

fn get_all_processes() -> Vec<Process> {
    let root = Path::new("/proc");
    let dir = rustix::fs::openat(
        rustix::fs::CWD,
        root,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .unwrap();
    let dir = rustix::fs::Dir::read_from(dir).unwrap();

    let mut processes: Vec<Process> = vec![];
    for entry in dir {
        if let Ok(e) = entry {
            if let Ok(pid) = i32::from_str(&e.file_name().to_string_lossy()) {
                let proc_root = PathBuf::from(root).join(pid.to_string());

                // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
                let flags = match &*KERNEL {
                    Some(v) if v < &String::from("3.6.0") => OFlags::DIRECTORY | OFlags::CLOEXEC,
                    Some(v) => OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
                    None => OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
                };
                let file =
                    rustix::fs::openat(rustix::fs::CWD, &proc_root, flags, Mode::empty()).unwrap();

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
                let pid = pid_res.unwrap();

                processes.push(Process::new(pid, file, proc_root));
            }
        }
    }
    processes
}

fn build_inode_process_map(processes: Vec<Process>) -> HashMap<u64, PidName> {
    let mut map: HashMap<u64, PidName> = HashMap::new();
    for proc in processes {
        let stat = rustix::fs::openat(
            &proc.fd,
            "stat",
            OFlags::RDONLY | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .unwrap();
        let dir_fd = rustix::fs::openat(
            &proc.fd,
            "fd",
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .unwrap();
        let mut dir = rustix::fs::Dir::read_from(&dir_fd).unwrap();
        dir.rewind();
        let mut socket_inodes = Vec::new();
        while let Some(Ok(entry)) = dir.next() {
            let name = entry.file_name().to_string_lossy();
            if RawFd::from_str(&name).is_ok() {
                if let Some(socket_inode) = get_socket_inodes(dir_fd.as_fd(), name.as_ref()) {
                    socket_inodes.push(socket_inode);
                }
            }
        }
        if let Some(pid_name) = PidName::from_file(File::from(stat)) {
            for inode in socket_inodes {
                map.insert(inode, pid_name.clone());
            }
        }
    }
    map
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
    fn from_file<R: Read>(mut r: R) -> Option<Self> {
        // read in entire thing, this is only going to be 1 line
        let mut buf = Vec::with_capacity(512);
        r.read_to_end(&mut buf).unwrap();

        let line = String::from_utf8_lossy(&buf);
        let buf = line.trim();

        // find the first opening paren, and split off the first part (pid)
        let start_paren = buf.find('(').unwrap();
        let end_paren = buf.rfind(')').unwrap();
        let pid_s = &buf[..start_paren - 1];
        let comm = buf[start_paren + 1..end_paren].to_string();

        let pid = FromStr::from_str(pid_s).unwrap();

        Some(PidName { pid, name: comm })
    }
}

fn get_socket_inodes<P: AsRef<Path>>(dir_fd: BorrowedFd, path: P) -> Option<u64> {
    let p = path.as_ref();
    // for 2.6.39 <= kernel < 3.6 fstat doesn't support O_PATH see https://github.com/eminence/procfs/issues/265
    let flags = match &*KERNEL {
        Some(v) if v < &String::from("3.6.0") => OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Some(_) | None => OFlags::NOFOLLOW | OFlags::PATH | OFlags::CLOEXEC,
    };
    let file = rustix::fs::openat(dir_fd, p, flags, Mode::empty()).unwrap();
    let link = rustix::fs::readlinkat(&file, "", Vec::new()).unwrap();

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
        let fd_type = s.next().unwrap();
        if fd_type == "socket" {
            let inode = s.next().unwrap();
            let inode = u64::from_str(strip_first_last(inode)).unwrap();
            Some(inode)
        } else {
            None
        }
    } else {
        None
    };
}

fn get_tcp_table() -> Vec<TcpListener> {
    let mut table = Vec::new();
    let file = File::open("/proc/net/tcp").unwrap();
    for line in BufReader::new(file).lines().flatten() {
        if let Some(l) = TcpListener::from_line(&line) {
            table.push(l)
        }
    }
    table
}

#[derive(Debug)]
struct TcpListener {
    local_addr: SocketAddr,
    inode: u64,
}

impl TcpListener {
    const LISTEN_STATE: &'static str = "0A";

    fn from_line(line: &str) -> Option<Self> {
        let mut s = line.trim().split_whitespace();
        let local_addr_hex = s.nth(1).unwrap();
        let Some(Self::LISTEN_STATE) = s.nth(1) else {
            return None;
        };

        let local_ip_port = local_addr_hex
            .split(':')
            .map(|s| u32::from_str_radix(s, 0x10).unwrap())
            .collect::<Vec<u32>>();
        let ip = Ipv4Addr::from(local_ip_port[0]);
        let port = u16::try_from(local_ip_port[1]).unwrap();
        let local_addr = SocketAddr::new(IpAddr::V4(ip), port);

        let inode = u64::from_str(s.nth(5).unwrap()).unwrap();

        Some(Self {
            local_addr,
            inode,
        })
    }
}
