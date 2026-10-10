//! Unsupported platform detector.

use crate::error::HardwareError;
use async_trait::async_trait;
use hackintosh_studio_core::hardware::*;

pub struct UnsupportedDetector;

impl crate::HardwareDetector for UnsupportedDetector {
    fn detect(&self) -> Result<HardwareReport, HardwareError> {
        Err(HardwareError::not_implemented(
            "hardware detection not implemented on this platform",
        ))
    }

    fn detect_into(&self, _report: &mut HardwareReport) -> Result<(), HardwareError> {
        Err(HardwareError::not_implemented(
            "hardware detection not implemented on this platform",
        ))
    }

    fn get_acpi_tables(&self) -> Result<Vec<AcpiTableSummary>, HardwareError> {
        Err(HardwareError::not_implemented(
            "ACPI access not implemented on this platform",
        ))
    }

    fn dump_acpi_tables(&self, _dir: &Path) -> Result<(), HardwareError> {
        Err(HardwareError::not_implemented(
            "ACPI dump not implemented on this platform",
        ))
    }

    fn platform_name(&self) -> &'static str {
        "unsupported"
    }
}
