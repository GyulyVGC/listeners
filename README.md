# Listene*rs*

[![Crates](https://img.shields.io/crates/v/listeners?&logo=rust)](https://crates.io/crates/listeners)
[![Downloads](https://img.shields.io/crates/d/listeners.svg)](https://crates.io/crates/listeners)
[![Docs](https://docs.rs/listeners/badge.svg)](https://docs.rs/listeners/latest/)
[![CI](https://github.com/gyulyvgc/listeners/workflows/CI/badge.svg)](https://github.com/GyulyVGC/listeners/actions/)

**Cross-platform library for Rust to find out process listening on network sockets.**

## Motivation

Despite some Rust libraries to get process information already exist,
none of them correlates process ID and name to active network sockets in a cross-platform way.

Some examples of existing libraries:
- [netstat2](https://crates.io/crates/netstat2): doesn't provide the process name (and it's unmaintained)
- [libproc](https://crates.io/crates/libproc): only for Linux and macOS
- [sysinfo](https://crates.io/crates/sysinfo): doesn't expose the sockets used by each process

This library wants to fill this gap, and it aims to be: 
- **Cross-platform**: it currently supports Windows, Linux and macOS
- **Performant**: it internally uses low-level system APIs
- **Simple**: it exposes intuitive APIs to get details about the listening processes
- **Lightweight**: it has only the strictly necessary dependencies

## Roadmap

- [x] Windows
- [x] Linux
- [x] macOS
- [ ] BSD
- [ ] iOS
- [ ] Android

## Usage

Add this to your `Cargo.toml`:

``` toml
[dependencies]

listeners = "0.3"
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
