//! Hardware detection platform abstraction.

use crate::error::HardwareError;
use hackintosh_studio_core::hardware::HardwareReport;
use std::path::Path;

/// Trait for platform-specific hardware detection.
pub trait HardwareDetector: Send + Sync {
    /// Detect all hardware and populate a report.
    fn detect(&self) -> Result<HardwareReport, HardwareError>;

    /// Detect hardware with existing report as base (for merging).
    fn detect_into(&self, report: &mut HardwareReport) -> Result<(), HardwareError>;

    /// Get ACPI table summaries.
    fn get_acpi_tables(
        &self,
    ) -> Result<Vec<hackintosh_studio_core::hardware::AcpiTableSummary>, HardwareError>;

    /// Dump raw ACPI tables to a directory.
    fn dump_acpi_tables(&self, dir: &Path) -> Result<(), HardwareError>;

    /// Platform name for diagnostics.
    fn platform_name(&self) -> &'static str;
}

/// Platform capability flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformCapabilities {
    pub cpu_detection: bool,
    pub motherboard_detection: bool,
    pub gpu_detection: bool,
    pub memory_detection: bool,
    pub storage_detection: bool,
    pub network_detection: bool,
    pub audio_detection: bool,
    pub usb_detection: bool,
    pub pci_detection: bool,
    pub acpi_access: bool,
    pub smbios_access: bool,
}

impl Default for PlatformCapabilities {
    fn default() -> Self {
        Self {
            cpu_detection: true,
            motherboard_detection: true,
            gpu_detection: true,
            memory_detection: true,
            storage_detection: true,
            network_detection: true,
            audio_detection: true,
            usb_detection: true,
            pci_detection: true,
            acpi_access: true,
            smbios_access: false,
        }
    }
}

/// Get the platform detector for the current OS.
#[cfg(target_os = "linux")]
pub fn create_detector() -> Box<dyn HardwareDetector> {
    Box::new(crate::platform::linux::LinuxDetector::new())
}

#[cfg(target_os = "windows")]
pub fn create_detector() -> Box<dyn HardwareDetector> {
    Box::new(crate::platform::windows::WindowsDetector::new())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub fn create_detector() -> Box<dyn HardwareDetector> {
    Box::new(crate::platform::unsupported::UnsupportedDetector)
}

pub mod error;
pub mod opcore_import;
pub mod platform;
