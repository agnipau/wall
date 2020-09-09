#[cfg(target_os = "linux")]
pub mod xlib;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(windows)]
pub mod windows;
