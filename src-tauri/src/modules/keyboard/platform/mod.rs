#[cfg(target_os = "linux")]
pub mod linux;

// Re-export platform functions
#[cfg(target_os = "linux")]
pub use linux::*;
