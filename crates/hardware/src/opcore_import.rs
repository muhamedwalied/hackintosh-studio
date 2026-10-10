//! OpCore-Simplify Report.json import adapter.
//!
//! This module provides functionality to import hardware reports from OpCore-Simplify's
//! Report.json format into our native HardwareReport domain model.
//!
//! The OpCore-Simplify schema (from report_validator.py) defines a different structure
//! than our native report. This adapter maps known mappable fields and preserves
//! unknown/unmapped details where practical.
//!
//! Note: Complete reverse export to OpCore-Simplify's required schema is not supported
//! because our native model contains more detailed fields that don't map cleanly to
//! their simplified schema. This adapter is for IMPORT ONLY.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use crate::error::HardwareError;
use hackintosh_studio_core::hardware::*;

/// OpCore-Simplify Report.json schema structures for deserialization.
/// Field names use exact serde renames to match the official schema (case-sensitive, spaces preserved).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreReport {
    #[serde(rename = "Motherboard")]
    pub motherboard: Option<OpCoreMotherboard>,
    #[serde(rename = "BIOS")]
    pub bios: Option<OpCoreBios>,
    #[serde(rename = "CPU")]
    pub cpu: Option<OpCoreCpu>,
    #[serde(rename = "GPU")]
    pub gpu: Option<HashMap<String, OpCoreGpu>>,
    #[serde(rename = "Monitor")]
    pub monitor: Option<HashMap<String, OpCoreMonitor>>,
    #[serde(rename = "Network")]
    pub network: Option<HashMap<String, OpCoreNetwork>>,
    #[serde(rename = "Sound")]
    pub sound: Option<HashMap<String, OpCoreSound>>,
    #[serde(rename = "USB Controllers")]
    pub usb_controllers: Option<HashMap<String, OpCoreUsbController>>,
    #[serde(rename = "Input")]
    pub input: Option<HashMap<String, OpCoreInput>>,
    #[serde(rename = "Storage Controllers")]
    pub storage_controllers: Option<HashMap<String, OpCoreStorageController>>,
    #[serde(rename = "Biometric")]
    pub biometric: Option<HashMap<String, OpCoreBiometric>>,
    #[serde(rename = "Bluetooth")]
    pub bluetooth: Option<HashMap<String, OpCoreBluetooth>>,
    #[serde(rename = "SD Controller")]
    pub sd_controller: Option<HashMap<String, OpCoreSdController>>,
    #[serde(rename = "System Devices")]
    pub system_devices: Option<HashMap<String, OpCoreSystemDevice>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreMotherboard {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Chipset")]
    pub chipset: Option<String>,
    #[serde(rename = "Platform")]
    pub platform: Option<String>, // "Desktop" or "Laptop"
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreBios {
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "Release Date")]
    pub release_date: Option<String>,
    #[serde(rename = "System Type")]
    pub system_type: Option<String>,
    #[serde(rename = "Firmware Type")]
    pub firmware_type: Option<String>, // "UEFI" or "BIOS"
    #[serde(rename = "Secure Boot")]
    pub secure_boot: Option<String>,   // "Enabled" or "Disabled"
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreCpu {
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,     // "Intel" or "AMD"
    #[serde(rename = "Processor Name")]
    pub processor_name: Option<String>,
    #[serde(rename = "Codename")]
    pub codename: Option<String>,
    #[serde(rename = "Core Count")]
    pub core_count: Option<String>,       // string but numeric
    #[serde(rename = "CPU Count")]
    pub cpu_count: Option<String>,        // string but numeric (logical threads)
    #[serde(rename = "SIMD Features")]
    pub simd_features: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreGpu {
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,      // "Intel", "AMD", "NVIDIA"
    #[serde(rename = "Codename")]
    pub codename: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,         // 4-char hex
    #[serde(rename = "Device Type")]
    pub device_type: Option<String>,       // "Integrated GPU", "Discrete GPU", "Unknown"
    #[serde(rename = "Subsystem ID")]
    pub subsystem_id: Option<String>,
    #[serde(rename = "PCI Path")]
    pub pci_path: Option<String>,
    #[serde(rename = "ACPI Path")]
    pub acpi_path: Option<String>,
    #[serde(rename = "Resizable BAR")]
    pub resizable_bar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreMonitor {
    #[serde(rename = "Connector Type")]
    pub connector_type: Option<String>,
    #[serde(rename = "Resolution")]
    pub resolution: Option<String>,
    #[serde(rename = "Connected GPU")]
    pub connected_gpu: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreNetwork {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,          // "PCI", "USB", "ACPI", "ROOT"
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
    #[serde(rename = "Subsystem ID")]
    pub subsystem_id: Option<String>,
    #[serde(rename = "PCI Path")]
    pub pci_path: Option<String>,
    #[serde(rename = "ACPI Path")]
    pub acpi_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreSound {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
    #[serde(rename = "Subsystem ID")]
    pub subsystem_id: Option<String>,
    #[serde(rename = "Audio Endpoints")]
    pub audio_endpoints: Option<Vec<String>>,
    #[serde(rename = "Controller Device ID")]
    pub controller_device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreUsbController {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
    #[serde(rename = "Subsystem ID")]
    pub subsystem_id: Option<String>,
    #[serde(rename = "PCI Path")]
    pub pci_path: Option<String>,
    #[serde(rename = "ACPI Path")]
    pub acpi_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreInput {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device")]
    pub device: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
    #[serde(rename = "Device Type")]
    pub device_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreStorageController {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
    #[serde(rename = "Subsystem ID")]
    pub subsystem_id: Option<String>,
    #[serde(rename = "PCI Path")]
    pub pci_path: Option<String>,
    #[serde(rename = "ACPI Path")]
    pub acpi_path: Option<String>,
    #[serde(rename = "Disk Drives")]
    pub disk_drives: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreBiometric {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device")]
    pub device: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreBluetooth {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreSdController {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
    #[serde(rename = "Subsystem ID")]
    pub subsystem_id: Option<String>,
    #[serde(rename = "PCI Path")]
    pub pci_path: Option<String>,
    #[serde(rename = "ACPI Path")]
    pub acpi_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreSystemDevice {
    #[serde(rename = "Bus Type")]
    pub bus_type: Option<String>,
    #[serde(rename = "Device")]
    pub device: Option<String>,
    #[serde(rename = "Device ID")]
    pub device_id: Option<String>,
    #[serde(rename = "Subsystem ID")]
    pub subsystem_id: Option<String>,
    #[serde(rename = "PCI Path")]
    pub pci_path: Option<String>,
    #[serde(rename = "ACPI Path")]
    pub acpi_path: Option<String>,
}

/// Result of importing an OpCore-Simplify report.
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// The converted native hardware report.
    pub report: HardwareReport,
    /// Fields that could not be mapped from the source report.
    pub unmapped_fields: Vec<String>,
    /// Warnings about data loss or approximation during import.
    pub warnings: Vec<String>,
}

/// Import an OpCore-Simplify Report.json file into a native HardwareReport.
pub fn import_opcore_report<P: AsRef<Path>>(path: P) -> Result<ImportResult, HardwareError> {
    let content = std::fs::read_to_string(&path)
        .map_err(|e| HardwareError::permission_denied("read report file", e))?;
    
    let opcore_report: OpCoreReport = serde_json::from_str(&content)
        .map_err(|e| HardwareError::parse(format!("invalid OpCore-Simplify JSON: {}", e)))?;
    
    // Validate required sections per schema
    validate_required_sections(&opcore_report)?;
    
    let mut report = HardwareReport::new();
    let mut unmapped = Vec::new();
    let mut warnings = Vec::new();

    // Import motherboard info
    if let Some(mb) = opcore_report.motherboard {
        if let Some(name) = mb.name {
            report.motherboard.model = HardwareField::detected(name);
        }
        if let Some(chipset) = mb.chipset {
            report.motherboard.chipset = HardwareField::detected(chipset);
        }
        if let Some(platform) = mb.platform {
            report.motherboard.form_factor = HardwareField::detected(platform);
        }
    }

    // Import BIOS info
    if let Some(bios) = opcore_report.bios {
        if let Some(version) = bios.version {
            report.motherboard.bios_version = HardwareField::detected(version);
        }
        if let Some(date) = bios.release_date {
            report.motherboard.bios_date = HardwareField::detected(date);
        }
        // Firmware Type (UEFI/BIOS) does NOT map to bios_vendor - leave unmapped
        if bios.firmware_type.is_some() {
            unmapped.push("BIOS.Firmware Type (UEFI/BIOS) - no semantic equivalent in native bios_vendor".to_string());
        }
        // System Type, Secure Boot - no semantic equivalents
        if bios.system_type.is_some() {
            unmapped.push("BIOS.System Type - no semantic equivalent in native model".to_string());
        }
        if bios.secure_boot.is_some() {
            unmapped.push("BIOS.Secure Boot - no semantic equivalent in native model".to_string());
        }
    }

    // Import CPU info
    if let Some(cpu) = opcore_report.cpu {
        if let Some(mfr) = cpu.manufacturer {
            let vendor_str = match mfr.as_str() {
                "Intel" => "GenuineIntel",
                "AMD" => "AuthenticAMD",
                _ => &mfr,
            };
            report.cpu.vendor = HardwareField::detected(vendor_str.to_string());
        }
        if let Some(name) = cpu.processor_name {
            report.cpu.brand = HardwareField::detected(name);
        }
        if let Some(codename) = cpu.codename {
            report.cpu.codename = HardwareField::detected(codename);
        }
        if let Some(core_count) = cpu.core_count {
            if let Ok(cores) = core_count.parse::<u32>() {
                report.cpu.physical_cores = HardwareField::detected(cores);
            }
        }
        if let Some(cpu_count) = cpu.cpu_count {
            if let Ok(threads) = cpu_count.parse::<u32>() {
                report.cpu.logical_threads = HardwareField::detected(threads);
            }
        }
        if let Some(features) = cpu.simd_features {
            let feature_list: Vec<String> = features
                .split(&[',', ' '][..])
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !feature_list.is_empty() {
                report.cpu.features = HardwareField::detected(feature_list);
            }
        }
        // CPU family/model/stepping, cache, frequencies, microcode not in source schema
        unmapped.push("CPU family/model/stepping, cache, frequencies, microcode - not in OpCore schema".to_string());
    }

    // Import GPUs
    if let Some(gpus) = opcore_report.gpu {
        for (key, gpu) in gpus {
            let mut gpu_info = GpuInfo::default();
            let mut has_identifying_info = false;
            
            if let Some(mfr) = gpu.manufacturer {
                gpu_info.vendor = HardwareField::detected(mfr.clone());
                has_identifying_info = true;
                // Do NOT fabricate PCI vendor ID from manufacturer alone
                // gpu_info.vendor_id left Unavailable unless we have actual Device ID
            }
            if let Some(codename) = gpu.codename {
                gpu_info.codename = HardwareField::detected(codename);
                has_identifying_info = true;
            }
            if let Some(dev_id) = gpu.device_id {
                gpu_info.device_id = HardwareField::detected(dev_id.to_uppercase());
                has_identifying_info = true;
            }
            if let Some(dev_type) = gpu.device_type {
                gpu_info.is_integrated = HardwareField::detected(dev_type == "Integrated GPU");
                has_identifying_info = true;
            }
            // Subsystem ID does NOT map to GPU revision - leave unmapped
            if gpu.subsystem_id.is_some() {
                unmapped.push(format!("GPU '{}': Subsystem ID - no semantic equivalent in native revision", key));
            }
            if let Some(pci_path) = gpu.pci_path {
                gpu_info.pci_location = HardwareField::detected(pci_path);
                has_identifying_info = true;
            }
            if gpu.acpi_path.is_some() {
                unmapped.push(format!("GPU '{}': ACPI Path - not in native model", key));
            }
            if gpu.resizable_bar.is_some() {
                unmapped.push(format!("GPU '{}': Resizable BAR - not in native model", key));
            }
            
            if has_identifying_info {
                report.gpus.push(gpu_info);
            } else {
                unmapped.push(format!("GPU '{}': no identifying fields", key));
            }
        }
    }

    // Import Network interfaces
    if let Some(network) = opcore_report.network {
        for (key, net) in network {
            let mut iface = NetworkInterface::default();
            iface.name = HardwareField::detected(key.clone());
            if let Some(dev_id) = net.device_id {
                iface.pci_ids = HardwareField::detected(PciIds {
                    vendor_id: String::new(), // Can't split device_id without vendor
                    device_id: dev_id.to_uppercase(),
                    class_code: String::new(),
                    subclass_code: String::new(),
                });
            }
            if let Some(subsys) = net.subsystem_id {
                // Subsystem ID mapped to subclass_code field with note
                iface.pci_ids = HardwareField::inferred(
                    PciIds {
                        vendor_id: String::new(),
                        device_id: String::new(),
                        class_code: String::new(),
                        subclass_code: subsys,
                    },
                    format!("subsystem ID from OpCore report: {}", subsys)
                );
            }
            if let Some(bus_type) = net.bus_type {
                iface.type_ = HardwareField::detected(bus_type);
            }
            if net.pci_path.is_some() {
                unmapped.push(format!("Network '{}': PCI Path - not in native model", key));
            }
            if net.acpi_path.is_some() {
                unmapped.push(format!("Network '{}': ACPI Path - not in native model", key));
            }
            report.network.push(iface);
        }
    }

    // Import USB controllers
    if let Some(usb) = opcore_report.usb_controllers {
        for (key, ctrl) in usb {
            let mut usb_ctrl = UsbController::default();
            usb_ctrl.name = HardwareField::detected(key.clone());
            if let Some(dev_id) = ctrl.device_id {
                usb_ctrl.pci_ids = HardwareField::detected(PciIds {
                    vendor_id: String::new(),
                    device_id: dev_id.to_uppercase(),
                    class_code: String::new(),
                    subclass_code: String::new(),
                });
            }
            if let Some(subsys) = ctrl.subsystem_id {
                unmapped.push(format!("USB Controller '{}': Subsystem ID - no semantic equivalent in native model", key));
            }
            if let Some(bus_type) = ctrl.bus_type {
                usb_ctrl.driver = HardwareField::detected(bus_type);
            }
            if ctrl.pci_path.is_some() {
                unmapped.push(format!("USB Controller '{}': PCI Path - not in native model", key));
            }
            if ctrl.acpi_path.is_some() {
                unmapped.push(format!("USB Controller '{}': ACPI Path - not in native model", key));
            }
            report.usb_controllers.push(usb_ctrl);
        }
    }

    // Import Audio/Sound
    if let Some(sound) = opcore_report.sound {
        for (key, snd) in sound {
            let mut audio = AudioDevice::default();
            audio.name = HardwareField::detected(key.clone());
            if let Some(dev_id) = snd.device_id {
                audio.pci_ids = HardwareField::detected(PciIds {
                    vendor_id: String::new(),
                    device_id: dev_id.to_uppercase(),
                    class_code: String::new(),
                    subclass_code: String::new(),
                });
            }
            if let Some(subsys) = snd.subsystem_id {
                unmapped.push(format!("Sound '{}': Subsystem ID - no semantic equivalent in native model", key));
            }
            if let Some(codec) = snd.controller_device_id {
                audio.codec = HardwareField::detected(codec);
            }
            if let Some(endpoints) = snd.audio_endpoints {
                if !endpoints.is_empty() {
                    unmapped.push(format!("Sound '{}': Audio Endpoints ({:?}) - not in native model", key, endpoints));
                }
            }
            if snd.bus_type.is_some() {
                unmapped.push(format!("Sound '{}': Bus Type - not in native model", key));
            }
            report.audio.push(audio);
        }
    }

    // Import Storage controllers
    if let Some(storage) = opcore_report.storage_controllers {
        for (key, ctrl) in storage {
            let mut store = StorageDevice::default();
            store.name = HardwareField::detected(key.clone());
            if let Some(dev_id) = ctrl.device_id {
                store.pci_ids = HardwareField::detected(PciIds {
                    vendor_id: String::new(),
                    device_id: dev_id.to_uppercase(),
                    class_code: String::new(),
                    subclass_code: String::new(),
                });
            }
            if let Some(subsys) = ctrl.subsystem_id {
                unmapped.push(format!("Storage Controller '{}': Subsystem ID - no semantic equivalent in native model", key));
            }
            if let Some(bus_type) = ctrl.bus_type {
                store.interface = HardwareField::detected(bus_type);
            }
            if ctrl.pci_path.is_some() {
                unmapped.push(format!("Storage Controller '{}': PCI Path - not in native model", key));
            }
            if ctrl.acpi_path.is_some() {
                unmapped.push(format!("Storage Controller '{}': ACPI Path - not in native model", key));
            }
            if let Some(disks) = ctrl.disk_drives {
                if !disks.is_empty() {
                    unmapped.push(format!("Storage Controller '{}': Disk Drives ({:?}) - not in native model", key, disks));
                }
            }
            report.storage.push(store);
        }
    }

    // Track unmapped sections
    if opcore_report.monitor.is_some() {
        unmapped.push("Monitor section (not in native model)".to_string());
    }
    if opcore_report.input.is_some() {
        unmapped.push("Input devices section (not in native model)".to_string());
    }
    if opcore_report.biometric.is_some() {
        unmapped.push("Biometric devices section (not in native model)".to_string());
    }
    if opcore_report.bluetooth.is_some() {
        unmapped.push("Bluetooth section (not in native model)".to_string());
    }
    if opcore_report.sd_controller.is_some() {
        unmapped.push("SD Controller section (not in native model)".to_string());
    }
    if opcore_report.system_devices.is_some() {
        unmapped.push("System Devices section (not in native model)".to_string());
    }

    // Add warnings about limitations
    warnings.push("OpCore-Simplify import: native report has more detailed fields than OpCore schema; some data loss is expected".to_string());
    warnings.push("OpCore-Simplify import: PCI class codes, driver names, and ACPI paths are not in source schema".to_string());
    warnings.push("OpCore-Simplify import: CPU family/model/stepping, cache, frequencies, microcode not in source schema".to_string());
    warnings.push("OpCore-Simplify import: Memory modules, storage partitions, ACPI tables not in source schema".to_string());
    warnings.push("OpCore-Simplify import: Manufacturer alone does not fabricate PCI vendor ID; subsystem ID does not map to revision".to_string());

    Ok(ImportResult {
        report,
        unmapped_fields: unmapped,
        warnings,
    })
}

/// Validate that required sections per OpCore schema are present.
fn validate_required_sections(report: &OpCoreReport) -> Result<(), HardwareError> {
    let mut missing = Vec::new();
    
    if report.motherboard.is_none() {
        missing.push("Motherboard");
    }
    if report.bios.is_none() {
        missing.push("BIOS");
    }
    if report.cpu.is_none() {
        missing.push("CPU");
    }
    if report.gpu.is_none() {
        missing.push("GPU");
    }
    if report.network.is_none() {
        missing.push("Network");
    }
    if report.usb_controllers.is_none() {
        missing.push("USB Controllers");
    }
    
    if !missing.is_empty() {
        return Err(HardwareError::invalid_data(format!(
            "OpCore-Simplify report missing required sections: {}",
            missing.join(", ")
        )));
    }
    Ok(())
}

/// Check if a JSON file appears to be an OpCore-Simplify Report.json.
/// This is a heuristic based on required top-level keys.
pub fn is_opcore_report<P: AsRef<Path>>(path: P) -> Result<bool, HardwareError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| HardwareError::permission_denied("read report file", e))?;
    
    let value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| HardwareError::parse(format!("invalid JSON: {}", e)))?;
    
    // Check for required top-level keys from OpCore schema (exact case-sensitive names)
    let required_keys = ["Motherboard", "BIOS", "CPU", "GPU", "Network", "USB Controllers"];
    let has_required = required_keys.iter().all(|k| value.get(*k).is_some());
    
    Ok(has_required)
}