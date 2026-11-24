use super::SocketInfo;
use crate::{Protocol, SocketState};
use std::{
    collections::HashSet,
    ffi::{c_char, c_int, c_uint},
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    ptr,
};

/// How many times to retry a sysctl read when the data grows between the size query and the read.
const SYSCTL_RETRIES: usize = 3;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(in crate::platform::bsd) struct ProcessInfo {
    pub(in crate::platform::bsd) pid: i32,
    pub(in crate::platform::bsd) name: String,
}

impl From<&libc::kinfo_proc> for ProcessInfo {
    fn from(proc: &libc::kinfo_proc) -> Self {
        ProcessInfo {
            pid: proc.p_pid,
            name: c_chars_to_string(&proc.p_comm),
        }
    }
}

pub(in crate::platform::bsd) fn get_all_processes() -> io::Result<HashSet<ProcessInfo>> {
    let mib = [
        libc::CTL_KERN,
        libc::KERN_PROC,
        libc::KERN_PROC_ALL,
        0,
        elem_size::<libc::kinfo_proc>()?,
        c_int::MAX,
    ];

    let procs = sysctl_array::<libc::kinfo_proc>(&mib)?;

    Ok(procs.iter().map(ProcessInfo::from).collect())
}

pub(in crate::platform::bsd) fn get_sockets(pid: i32) -> io::Result<Vec<SocketInfo>> {
    let mib = [
        libc::CTL_KERN,
        libc::KERN_FILE,
        libc::KERN_FILE_BYPID,
        pid,
        elem_size::<libc::kinfo_file>()?,
        c_int::MAX,
    ];

    let files = sysctl_array::<libc::kinfo_file>(&mib)?;

    Ok(files.iter().filter_map(socket_info).collect())
}

/// Converts a `kinfo_file` entry into a [`SocketInfo`].
///
/// Returns `None` for anything that is not an IPv4/IPv6 TCP or UDP socket.
fn socket_info(file: &libc::kinfo_file) -> Option<SocketInfo> {
    if i32::try_from(file.f_type) != Ok(libc::DTYPE_SOCKET) {
        return None;
    }

    let (protocol, state) = match i32::try_from(file.so_protocol) {
        Ok(libc::IPPROTO_TCP) => (
            Protocol::TCP,
            SocketState::from_bsd(i32::try_from(file.t_state).unwrap_or(-1)),
        ),
        Ok(libc::IPPROTO_UDP) => (Protocol::UDP, SocketState::Unknown),
        _ => return None,
    };

    let ip = match i32::try_from(file.so_family) {
        Ok(libc::AF_INET) => IpAddr::V4(Ipv4Addr::from(file.inp_laddru[0].to_ne_bytes())),
        Ok(libc::AF_INET6) => IpAddr::V6(Ipv6Addr::from(ipv6_octets(&file.inp_laddru))),
        _ => return None,
    };

    // The kernel stores the 16-bit port, in network byte order, in a 32-bit field.
    let port = u16::from_be(u16::try_from(file.inp_lport).ok()?);

    Some(SocketInfo {
        address: SocketAddr::new(ip, port),
        protocol,
        state,
    })
}

/// Rebuilds the 16 address bytes from the four 32-bit words the kernel copies them into.
///
/// The words hold the address in memory order, so their native-endian bytes are the octets.
fn ipv6_octets(words: &[u32; 4]) -> [u8; 16] {
    let mut octets = [0u8; 16];
    for (chunk, word) in octets.chunks_exact_mut(4).zip(words) {
        chunk.copy_from_slice(&word.to_ne_bytes());
    }
    octets
}

/// Converts a fixed-size C character array to a `String`.
///
/// Stops at the first NUL, or uses the whole array if it has none, so it never reads past the array.
fn c_chars_to_string(chars: &[c_char]) -> String {
    let bytes: Vec<u8> = chars
        .iter()
        .map(|c| c.to_ne_bytes()[0])
        .take_while(|&b| b != 0)
        .collect();

    String::from_utf8_lossy(&bytes).into_owned()
}

/// Returns `size_of::<T>()` as a `c_int`, for use as a sysctl element size.
fn elem_size<T>() -> io::Result<c_int> {
    c_int::try_from(size_of::<T>()).map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))
}

/// Reads a sysctl that returns an array of `T`.
///
/// Queries the required size first, then reads into a buffer of that size. If the data grew
/// in between and the kernel reports `ENOMEM`, it retries up to [`SYSCTL_RETRIES`] times.
///
/// `T` must be a plain C struct for which any bit pattern is valid, such as the `kinfo_*`
/// structs from `libc`.
fn sysctl_array<T: Copy>(mib: &[c_int]) -> io::Result<Vec<T>> {
    let elem_size = size_of::<T>();
    let mib_len =
        c_uint::try_from(mib.len()).map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?;

    for _ in 0..SYSCTL_RETRIES {
        let mut needed: libc::size_t = 0;

        // SAFETY: a null `oldp` asks the kernel only for the required size, which it writes to
        // `needed`. `mib` is valid for `mib_len` elements.
        let rc = unsafe {
            libc::sysctl(
                mib.as_ptr(),
                mib_len,
                ptr::null_mut(),
                &raw mut needed,
                ptr::null_mut(),
                0,
            )
        };
        if rc != 0 {
            return Err(io::Error::last_os_error());
        }

        if needed == 0 || elem_size == 0 {
            return Ok(Vec::new());
        }

        let capacity = needed.div_ceil(elem_size);
        let mut buf: Vec<T> = Vec::with_capacity(capacity);
        let mut len_bytes: libc::size_t = capacity * elem_size;

        // SAFETY: `buf` has room for `len_bytes` bytes. The kernel writes at most that many and
        // stores the number of bytes written back into `len_bytes`.
        let rc = unsafe {
            libc::sysctl(
                mib.as_ptr(),
                mib_len,
                buf.as_mut_ptr().cast(),
                &raw mut len_bytes,
                ptr::null_mut(),
                0,
            )
        };

        if rc == 0 {
            // SAFETY: the kernel initialized the first `len_bytes` bytes, which never exceeds the
            // capacity. `T` is plain data, so every whole element in that range is valid.
            unsafe { buf.set_len(len_bytes / elem_size) };
            return Ok(buf);
        }

        let err = io::Error::last_os_error();
        if err.raw_os_error() != Some(libc::ENOMEM) {
            return Err(err);
        }
    }

    Err(io::Error::from_raw_os_error(libc::EOVERFLOW))
}
