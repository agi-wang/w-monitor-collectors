//! Read-only host collection shared by W Monitor Private and Public.
//! Contains no identity, transport, inbound listeners, control dispatcher,
//! remote file/terminal operations, installer or automatic update code.
pub mod hardware;
pub mod mount_health;
pub mod traffic;

use std::time::{Duration, Instant};
const MAX_COLLECTOR_STDOUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_COLLECTOR_STDERR_BYTES: usize = 256 * 1024;

pub(crate) fn run_cmd_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Option<std::process::Output> {
    run_command_with_timeout(std::process::Command::new(program).args(args), timeout)
}

pub(crate) fn run_command_with_timeout(
    command: &mut std::process::Command,
    timeout: Duration,
) -> Option<std::process::Output> {
    use std::process::Stdio;
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command.spawn().ok()?;
    let stdout = child.stdout.take().map(|pipe| {
        std::thread::spawn(move || read_bounded_and_drain(pipe, MAX_COLLECTOR_STDOUT_BYTES))
    });
    let stderr = child.stderr.take().map(|pipe| {
        std::thread::spawn(move || read_bounded_and_drain(pipe, MAX_COLLECTOR_STDERR_BYTES))
    });

    let deadline = Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(None) | Err(_) => {
                #[cfg(unix)]
                unsafe {
                    // The child starts its own process group, so helpers it
                    // spawned cannot outlive the collector timeout.
                    libc::kill(-(child.id() as i32), libc::SIGKILL);
                }
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
        }
    };
    let stdout = stdout
        .and_then(|handle| handle.join().ok())
        .unwrap_or_default();
    let stderr = stderr
        .and_then(|handle| handle.join().ok())
        .unwrap_or_default();
    status.map(|status| std::process::Output {
        status,
        stdout,
        stderr,
    })
}

fn read_bounded_and_drain(mut reader: impl std::io::Read, max_bytes: usize) -> Vec<u8> {
    let mut captured = Vec::with_capacity(max_bytes.min(64 * 1024));
    let mut chunk = [0u8; 16 * 1024];
    loop {
        let count = match reader.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(count) => count,
        };
        let remaining = max_bytes.saturating_sub(captured.len());
        captured.extend_from_slice(&chunk[..count.min(remaining)]);
        // Continue draining after the capture limit. Closing a full pipe early
        // would SIGPIPE an otherwise healthy collector and distort its status.
    }
    captured
}
