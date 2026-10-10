//! Platform-specific hardware detection implementations.

pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub mod unsupported;

/// Trait for platform-specific hardware detection (re-exported for crate users).
pub use crate::HardwareDetector;
pub use crate::PlatformCapabilities;
pub use crate::create_detector;
