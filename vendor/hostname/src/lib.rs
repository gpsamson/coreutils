use std::ffi::OsString;
use std::io;

pub fn get() -> io::Result<OsString> {
    // Read $HOSTNAME env var with fallback to "localhost".
    let hostname = std::env::var_os("HOSTNAME").unwrap_or_else(|| OsString::from("localhost"));
    Ok(hostname)
}

pub fn set(_hostname: impl AsRef<std::ffi::OsStr>) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "setting hostname is unsupported in this build",
    ))
}
