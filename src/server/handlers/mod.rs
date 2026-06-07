#[cfg(not(windows))]
mod dummy;
#[cfg(windows)]
mod windows;

#[cfg(not(windows))]
pub use dummy::DummyHandler as PlatformHandler;
#[cfg(windows)]
pub use windows::WindowsHandler as PlatformHandler;
