// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) getusername

#[cfg(unix)]
pub use self::unix::get_username;

#[cfg(windows)]
pub use self::windows::get_username;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

// For non-Unix, non-Windows targets (e.g. wasm32-wasip2), read $USER env var.
#[cfg(not(any(unix, windows)))]
use std::{ffi::OsString, io};

#[cfg(not(any(unix, windows)))]
pub fn get_username() -> io::Result<OsString> {
    Ok(std::env::var_os("USER").unwrap_or_else(|| OsString::from("user")))
}
