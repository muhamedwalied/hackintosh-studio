//! Windows hardware detection placeholder (to be implemented in later phase).

use crate::error::HardwareError;
use hackintosh_studio_core::hardware::*;

pub struct WindowsDetector;

impl crate::HardwareDetector for WindowsDetector {
    fn detect(&self) -> Result<HardwareReport, HardwareError> {
        Err(HardwareError::not_implemented(
            "Windows hardware detection not yet implemented",
        ))
    }

    fn detect_into(&self, _report: &mut HardwareReport) -> Result<(), HardwareError> {
        Err(HardwareError::not_implemented(
            "Windows hardware detection not yet implemented",
        ))
    }

    fn get_acpi_tables(&self) -> Result<Vec<AcpiTableSummary>, HardwareError> {
        Err(HardwareError::not_implemented(
            "Windows ACPI access not yet implemented",
        ))
    }

    fn dump_acpi_tables(&self, _dir: &Path) -> Result<(), HardwareError> {
        Err(HardwareError::not_implemented(
            "Windows ACPI dump not yet implemented",
        ))
    }

    fn platform_name(&self) -> &'static str {
        "windows"
    }
}
