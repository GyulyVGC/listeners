# listene*rs*

[![Stars](https://img.shields.io/github/stars/GyulyVGC/listeners?logo=github&style=flat)](https://github.com/GyulyVGC/listeners)
[![Downloads](https://img.shields.io/crates/d/listeners.svg)](https://crates.io/crates/listeners)
[![Codecov](https://codecov.io/gh/GyulyVGC/listeners/graph/badge.svg?token=GSDVTT75C2)](https://codecov.io/gh/GyulyVGC/listeners)
[![CI](https://github.com/gyulyvgc/listeners/workflows/CI/badge.svg)](https://github.com/GyulyVGC/listeners/actions/)
[![Docs](https://docs.rs/listeners/badge.svg)](https://docs.rs/listeners/latest/)
[![Crates](https://img.shields.io/crates/v/listeners?&logo=rust)](https://crates.io/crates/listeners)

**Cross-platform library for Rust to [_efficiently_](#benchmarks) find out processes listening on network ports.**

## Motivation

Despite some Rust libraries to get process information already exist,
none of them correlates process ID and name to open network ports in a cross-platform way.

Some examples of existing libraries:
- [netstat2](https://crates.io/crates/netstat2): doesn't provide the process name (and it's unmaintained)
- [libproc](https://crates.io/crates/libproc): only for Linux and macOS
- [sysinfo](https://crates.io/crates/sysinfo): doesn't expose the sockets used by each process

This library wants to fill this gap, and it aims to be: 
- **Cross-platform**: it currently supports Windows, Linux, macOS, FreeBSD, OpenBSD and NetBSD
- **Performant**: it focuses on performance (see [benchmarks](#benchmarks)) by internally using low-level system APIs
- **Simple**: it exposes intuitive APIs to get details about the listening processes
- **Lightweight**: it has only the strictly necessary dependencies

## Roadmap

- [x] Windows
- [x] Linux
- [x] macOS
- [x] FreeBSD
- [x] OpenBSD
- [x] NetBSD
- [ ] Android
- [ ] iOS
- [ ] Other?

## Usage

Add this to your `Cargo.toml`:

``` toml
[dependencies]

listeners = "0.4"
```

Get all the listening processes:

``` rust
if let Ok(listeners) = listeners::get_all() {
    for l in listeners {
        println!("{l}");
    }
}
```

Output:

``` text
PID: 440        Process name: ControlCenter             Socket: 0.0.0.0:0                      Protocol: UDP
PID: 456        Process name: rapportd                  Socket: [::]:49158                     Protocol: TCP
PID: 456        Process name: rapportd                  Socket: 0.0.0.0:49158                  Protocol: TCP
PID: 456        Process name: rapportd                  Socket: 0.0.0.0:0                      Protocol: UDP
PID: 485        Process name: sharingd                  Socket: 0.0.0.0:0                      Protocol: UDP   
PID: 516        Process name: WiFiAgent                 Socket: 0.0.0.0:0                      Protocol: UDP
PID: 1480       Process name: rustrover                 Socket: [::7f00:1]:63342               Protocol: TCP
PID: 2123       Process name: Telegram                  Socket: 192.168.1.102:49659            Protocol: TCP
PID: 2123       Process name: Telegram                  Socket: 192.168.1.102:49656            Protocol: TCP
PID: 2156       Process name: Google Chrome             Socket: 0.0.0.0:0                      Protocol: UDP
PID: 2167       Process name: Google Chrome Helper      Socket: 192.168.1.102:60834            Protocol: UDP
PID: 2167       Process name: Google Chrome Helper      Socket: 192.168.1.102:53220            Protocol: UDP
PID: 2167       Process name: Google Chrome Helper      Socket: 192.168.1.102:59216            Protocol: UDP 
```
 
For more examples of usage, including how to get listening processes in a more granular way,
check the [`examples`](https://github.com/GyulyVGC/listeners/tree/main/examples) folder.

## Benchmarks

Below you can find exhaustive benchmarks measuring the performance of this library APIs
on all supported platforms, varying the system load (number of running processes and open ports).

The benchmarks include:
- `listeners::get_all`: get all the listening processes and their sockets
- `listeners::get_process_by_port`: get the process listening on a specific port
  - active port: the tested port is randomly selected among the open ports
  - inactive port: the tested port is not open

Benchmarks are run on GitHub Actions runners, and results are generated with the help of [`criterion`](https://crates.io/crates/criterion).

### Windows

<details>

<summary>See benchmarks</summary>

| System load | <code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;listeners::get_all&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</code>                   | `listeners::get_process_by_port`<br>(active port)                                                                                            | `listeners::get_process_by_port`<br>(inactive port)                                                                                            |
|-------------|---------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| low         | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_all_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_process_by_active_port_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_process_by_inactive_port_low.svg)    |
| medium      | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_all_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_process_by_active_port_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_process_by_inactive_port_medium.svg) |
| high        | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_all_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_process_by_active_port_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/windows_get_process_by_inactive_port_high.svg)   |

</details>

### Linux

<details>

<summary>See benchmarks</summary>

| System load | <code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;listeners::get_all&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</code>                 | `listeners::get_process_by_port`<br>(active port)                                                                                          | `listeners::get_process_by_port`<br>(inactive port)                                                                                          |
|-------------|-------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| low         | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_all_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_process_by_active_port_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_process_by_inactive_port_low.svg)    |
| medium      | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_all_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_process_by_active_port_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_process_by_inactive_port_medium.svg) |
| high        | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_all_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_process_by_active_port_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/linux_get_process_by_inactive_port_high.svg)   |

</details>

### macOS

<details>

<summary>See benchmarks</summary>

| System load | <code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;listeners::get_all&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</code>                 | `listeners::get_process_by_port`<br>(active port)                                                                                          | `listeners::get_process_by_port`<br>(inactive port)                                                                                          |
|-------------|-------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| low         | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_all_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_process_by_active_port_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_process_by_inactive_port_low.svg)    |
| medium      | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_all_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_process_by_active_port_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_process_by_inactive_port_medium.svg) |
| high        | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_all_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_process_by_active_port_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/macos_get_process_by_inactive_port_high.svg)   |

</details>

### FreeBSD

<details>

<summary>See benchmarks</summary>

| System load | <code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;listeners::get_all&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</code>                   | `listeners::get_process_by_port`<br>(active port)                                                                                            | `listeners::get_process_by_port`<br>(inactive port)                                                                                            |
|-------------|---------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| low         | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_all_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_process_by_active_port_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_process_by_inactive_port_low.svg)    |
| medium      | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_all_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_process_by_active_port_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_process_by_inactive_port_medium.svg) |
| high        | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_all_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_process_by_active_port_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/freebsd_get_process_by_inactive_port_high.svg)   |

</details>

### OpenBSD

<details>

<summary>See benchmarks</summary>

| System load | <code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;listeners::get_all&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</code>                   | `listeners::get_process_by_port`<br>(active port)                                                                                            | `listeners::get_process_by_port`<br>(inactive port)                                                                                            |
|-------------|---------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| low         | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_all_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_process_by_active_port_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_process_by_inactive_port_low.svg)    |
| medium      | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_all_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_process_by_active_port_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_process_by_inactive_port_medium.svg) |
| high        | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_all_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_process_by_active_port_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/openbsd_get_process_by_inactive_port_high.svg)   |

</details>

### NetBSD

<details>

<summary>See benchmarks</summary>

| System load | <code>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;listeners::get_all&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</code>                  | `listeners::get_process_by_port`<br>(active port)                                                                                           | `listeners::get_process_by_port`<br>(inactive port)                                                                                           |
|-------------|--------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| low         | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_all_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_process_by_active_port_low.svg)    | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_process_by_inactive_port_low.svg)    |
| medium      | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_all_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_process_by_active_port_medium.svg) | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_process_by_inactive_port_medium.svg) |
| high        | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_all_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_process_by_active_port_high.svg)   | ![](https://raw.githubusercontent.com/GyulyVGC/listeners/refs/heads/main/resources/benchmarks/netbsd_get_process_by_inactive_port_high.svg)   |

</details>
