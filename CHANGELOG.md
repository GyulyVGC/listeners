# Changelog

All releases with the relative changes are documented in this file.

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
 