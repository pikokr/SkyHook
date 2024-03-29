use crate::types::{Error, Event};

// ------- LINUX -------

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub fn run(callback: fn(Event)) -> Result<(), Error> {
    linux::start(callback)
}

#[cfg(target_os = "linux")]
pub fn stop() -> Result<(), Error> {
    linux::stop()
}

#[cfg(target_os = "linux")]
pub fn is_running() -> bool {
    linux::is_running()
}

// ------- WINDOWS -------

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub fn run(callback: fn(Event)) -> Result<(), Error> {
    windows::start(callback)
}

#[cfg(target_os = "windows")]
pub fn stop() -> Result<(), Error> {
    windows::stop()
}

#[cfg(target_os = "windows")]
pub fn is_running() -> bool {
    windows::is_running()
}

// ------- MACOS -------

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub fn run(callback: fn(Event)) -> Result<(), Error> {
    macos::start(callback)
}

#[cfg(target_os = "macos")]
pub fn stop() -> Result<(), Error> {
    macos::stop()
}

#[cfg(target_os = "macos")]
pub fn is_running() -> bool {
    macos::is_running()
}
