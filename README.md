# W Monitor Collectors

Read-only Rust host collection shared by [W Monitor Public](https://github.com/agi-wang/w-monitor-public) and W Monitor Private. MIT licensed.

The library contains CPU, memory, disk capacity and I/O, GPU, temperatures, process summaries, network interfaces and connection observations, and mount state. It returns the existing JSON collector shapes and uses the caller's `sysinfo` snapshots and rate caches. Scheduling, identities, reporting transports and application configuration belong to each consumer.

There is no command dispatcher, control listener, remote terminal/file operation, privilege escalation, installer or automatic updater. Local diagnostic subprocesses use fixed read-only queries with bounded output, deadlines and process-group cleanup. Mount health reads kernel/service snapshots without probing potentially blocked mount points. This crate depends only on `sysinfo`, `parking_lot`, `serde_json`, `libc` and `sha2` (stable temperature sensor identifiers).

## Network accounting

`hardware::collect_network` implements accounting version 2: minimum-metric default IPv4/IPv6 route interfaces define egress; master/lower observations are deduplicated. Loopback, container interfaces and tunnels remain separately classified and are not added to egress. Missing route evidence produces null totals, not zero or a guess based on addresses. Group counts are observations, not globally deduplicated traffic. A default interface can also carry LAN/NAT traffic, so egress does not mean destination-filtered Internet bandwidth.

## Use

Both consumers pin the same reviewed Git revision:

```toml
[dependencies]
w-monitor-collectors = { git = "https://github.com/agi-wang/w-monitor-collectors", rev = "REVIEWED_40_CHARACTER_COMMIT" }
```

Use `hardware`, `traffic` and `mount_health` modules directly. Existing Private collector checks retain their original bodies and call the same implementation through thin re-exports; there are no production fallback copies. Hidden parser/sampling helpers remain exported for those existing checks.

Changing this library does not update either deployed product. Each consumer explicitly updates its pin and lockfile, runs its own checks, and releases independently. Private keeps its control/update layer; Public keeps its read-only Agent and manual-update policy.

## Checks

Rust is pinned in `rust-toolchain.toml`. Run `cargo fmt --all --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo check --locked --target x86_64-unknown-linux-musl`, and `cargo package --locked --no-verify`. GitHub CI also checks the dependency boundary. No application deployment is attached to this repository.
