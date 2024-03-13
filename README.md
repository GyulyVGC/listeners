# Listene***rs***

[![Crates](https://img.shields.io/crates/v/listeners?&logo=rust)](https://crates.io/crates/listeners)
[![Docs](https://docs.rs/listeners/badge.svg)](https://docs.rs/listeners/latest/)
[![CI](https://github.com/gyulyvgc/listeners/workflows/CI/badge.svg)](https://github.com/GyulyVGC/listeners/actions/)

**Rust library to get processes listening on a TCP port in a cross-platform way.**

## Motivation

Despite some Rust libraries to get process information already exist,
none of them satisfies my need to get process ID and name of TCP listeners in a cross-platform way.

Some examples of existing libraries:
- [netstat2](https://crates.io/crates/netstat2): doesn't provide the process name (and it's unmaintained)
- [libproc](https://crates.io/crates/libproc): only for Linux and macOS
- [sysinfo](https://crates.io/crates/sysinfo): doesn't expose the sockets used by each process

This library wants to fill this gap . 

It aims to be: 
- **Cross-platform**: it currently works on Windows, Linux and macOS
- **Performant**: it uses low-level system APIs
- **Simple**: it provides a single function to get all the listening processes
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

listeners = "0.1"
```

Get the listener processes:

``` rust
let listeners = listeners::get_all().unwrap();

for l in listeners {
    println!("{l}");
}
```

Output:

``` text
PID: 1088       Process name: rustrover                 Socket: 127.0.0.1:63342
PID: 609        Process name: Microsoft SharePoint      Socket: [::1]:42050
PID: 160        Process name: mysqld                    Socket: [::]:33060
PID: 160        Process name: mysqld                    Socket: [::]:3306
PID: 460        Process name: rapportd                  Socket: 0.0.0.0:50928
PID: 460        Process name: rapportd                  Socket: [::]:50928 
```
