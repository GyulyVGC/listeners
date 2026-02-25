# Changelog

All releases with the relative changes are documented in this file.

## [UNRELEASED]
### Added
- OpenBSD and NetBSD support ([#39](https://github.com/GyulyVGC/listeners/pull/39))
### Changed
- Update process name retrieval on Windows for improved performance ([#43](https://github.com/GyulyVGC/listeners/pull/43) — fixes [#42](https://github.com/GyulyVGC/listeners/issues/42))
- Update benchmarks to be even more comprehensive: test multiple processes each listening on multiple ports ([#41](https://github.com/GyulyVGC/listeners/pull/41) — fixes [#35](https://github.com/GyulyVGC/listeners/issues/35))
### Fixed
- Use more loose version requirements to avoid dependency conflicts in projects using this library

## [0.4.0] - 2026-02-14
### Added
- FreeBSD support ([#32](https://github.com/GyulyVGC/listeners/pull/32))
- New API `get_process_by_port` to efficiently get the process listening on a specific port and protocol ([#34](https://github.com/GyulyVGC/listeners/pull/34) — fixes [#25](https://github.com/GyulyVGC/listeners/issues/25))
- `IS_OS_SUPPORTED` constant to check if the consumer platform is supported by the library
- Benchmarks on all supported platforms, which helped to considerably improve this library performance ([#31](https://github.com/GyulyVGC/listeners/pull/31))
### Changed
- Only open one `ProcFd` at a time on Linux ([#30](https://github.com/GyulyVGC/listeners/pull/30))
### Removed
- `get_ports_by_pid`, `get_ports_by_process_name`, and `get_processes_by_port`, since they were not optimized because internally simply based on `get_all`

## [0.3.0] - 2025-08-02
### Added
- Added `path` field to `Process` struct, making it possible to obtain the executables' full path ([#23](https://github.com/GyulyVGC/listeners/pull/23))
- New `Protocol` enum
- Added `protocol` field to `Listener` struct, indicating whether the listener uses TCP or UDP
### Changed
- The library now retrieves all the processes listening on TCP/UDP ports, instead of just the TCP-based ones in `LISTEN` state ([#13](https://github.com/GyulyVGC/listeners/pull/13) — fixes [#5](https://github.com/GyulyVGC/listeners/issues/5)) 

## [0.2.1] - 2024-07-12
### Fixed
- Linux permission denied issue ([#10](https://github.com/GyulyVGC/listeners/pull/10) — fixes [#9](https://github.com/GyulyVGC/listeners/issues/9))

## [0.2.0] - 2024-03-27
### Added
- New APIs to get the listening processes in a more granular way
  - `get_ports_by_pid`
  - `get_ports_by_process_name`
  - `get_processes_by_port`
- New `Process` struct to represent a process identified by its PID and name
### Changed
- `Listener` struct now has a `process` field of type `Process`, which takes place of the old fields `pid` and `name`

## [0.1.0] - 2024-03-14
### Added
- Support for Windows, Linux and macOS
- `get_all` API to get all the listening processes
 