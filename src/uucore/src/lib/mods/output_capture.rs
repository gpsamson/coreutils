// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//! Thread-local stdout/stderr capture for WASIp2 component usage.
//!
//! When running as a WASIp2 component, `uumain()` writes to `std::io::stdout()`
//! which goes to the host, not back to the caller via the WIT return value.
//! This module provides a capture mechanism: before calling `uumain()`, the
//! caller activates capture, and after, extracts the buffered bytes.
//!
//! On native targets this module is not compiled (gated on `target_arch = "wasm32"`).

use std::cell::RefCell;
use std::io::Write;

thread_local! {
    static STDOUT_BUF: RefCell<Option<Vec<u8>>> = const { RefCell::new(None) };
    static STDERR_BUF: RefCell<Option<Vec<u8>>> = const { RefCell::new(None) };
}

/// Begin capturing stdout. Any subsequent writes to `CaptureWriter::stdout()`
/// will be buffered instead of going to `std::io::stdout()`.
pub fn capture_stdout() {
    STDOUT_BUF.with(|buf| {
        *buf.borrow_mut() = Some(Vec::new());
    });
}

/// Begin capturing stderr.
pub fn capture_stderr() {
    STDERR_BUF.with(|buf| {
        *buf.borrow_mut() = Some(Vec::new());
    });
}

/// Extract the captured stdout bytes, ending the capture.
/// Returns an empty Vec if capture was not active.
pub fn take_stdout() -> Vec<u8> {
    STDOUT_BUF.with(|buf| buf.borrow_mut().take().unwrap_or_default())
}

/// Extract the captured stderr bytes, ending the capture.
/// Returns an empty Vec if capture was not active.
pub fn take_stderr() -> Vec<u8> {
    STDERR_BUF.with(|buf| buf.borrow_mut().take().unwrap_or_default())
}

/// Returns a writer that either appends to the capture buffer (if active)
/// or falls through to `std::io::stdout()`.
pub fn stdout() -> Box<dyn Write> {
    if STDOUT_BUF.with(|buf| buf.borrow().is_some()) {
        Box::new(CaptureWriter { is_stdout: true })
    } else {
        Box::new(std::io::stdout())
    }
}

/// Returns a writer that either appends to the capture buffer (if active)
/// or falls through to `std::io::stderr()`.
pub fn stderr() -> Box<dyn Write> {
    if STDERR_BUF.with(|buf| buf.borrow().is_some()) {
        Box::new(CaptureWriter { is_stdout: false })
    } else {
        Box::new(std::io::stderr())
    }
}

struct CaptureWriter {
    is_stdout: bool,
}

impl Write for CaptureWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let cell = if self.is_stdout {
            &STDOUT_BUF
        } else {
            &STDERR_BUF
        };
        cell.with(|c| {
            if let Some(ref mut v) = *c.borrow_mut() {
                v.extend_from_slice(buf);
                Ok(buf.len())
            } else {
                // Capture ended mid-write; fall through to real output.
                if self.is_stdout {
                    std::io::stdout().write(buf)
                } else {
                    std::io::stderr().write(buf)
                }
            }
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
