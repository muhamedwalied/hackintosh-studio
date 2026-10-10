use crate::error::CoreError;
use crate::hardware::HardwareReport;
use std::path::Path;

/// Serialize a hardware report to JSON file.
pub fn write_hardware_report_json<P: AsRef<Path>>(
    report: &HardwareReport,
    path: P,
) -> Result<(), CoreError> {
    let json = serde_json::to_string_pretty(report)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Deserialize a hardware report from JSON file.
pub fn read_hardware_report_json<P: AsRef<Path>>(path: P) -> Result<HardwareReport, CoreError> {
    let json = std::fs::read_to_string(path)?;
    let report = serde_json::from_str(&json)?;
    Ok(report)
}

/// Serialize any serializable type to JSON file.
pub fn write_json<T: serde::Serialize, P: AsRef<Path>>(
    value: &T,
    path: P,
) -> Result<(), CoreError> {
    let json = serde_json::to_string_pretty(value)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Deserialize any deserializable type from JSON file.
pub fn read_json<T: serde::de::DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, CoreError> {
    let json = std::fs::read_to_string(path)?;
    let value = serde_json::from_str(&json)?;
    Ok(value)
}
