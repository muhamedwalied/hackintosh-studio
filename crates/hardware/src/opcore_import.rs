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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreReport {
    pub motherboard: Option<OpCoreMotherboard>,
    pub bios: Option<OpCoreBios>,
    pub cpu: Option<OpCoreCpu>,
    pub gpu: Option<HashMap<String, OpCoreGpu>>,
    pub monitor: Option<HashMap<String, OpCoreMonitor>>,
    pub network: Option<HashMap<String, OpCoreNetwork>>,
    pub sound: Option<HashMap<String, OpCoreSound>>,
    pub usb_controllers: Option<HashMap<String, OpCoreUsbController>>,
    pub input: Option<HashMap<String, OpCoreInput>>,
    pub storage_controllers: Option<HashMap<String, OpCoreStorageController>>,
    pub biometric: Option<HashMap<String, OpCoreBiometric>>,
    pub bluetooth: Option<HashMap<String, OpCoreBluetooth>>,
    pub sd_controller: Option<HashMap<String, OpCoreSdController>>,
    pub system_devices: Option<HashMap<String, OpCoreSystemDevice>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreMotherboard {
    pub name: Option<String>,
    pub chipset: Option<String>,
    pub platform: Option<String>, // "Desktop" or "Laptop"
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreBios {
    pub version: Option<String>,
    pub release_date: Option<String>,
    pub system_type: Option<String>,
    pub firmware_type: Option<String>, // "UEFI" or "BIOS"
    pub secure_boot: Option<String>,   // "Enabled" or "Disabled"
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreCpu {
    pub manufacturer: Option<String>,     // "Intel" or "AMD"
    pub processor_name: Option<String>,
    pub codename: Option<String>,
    pub core_count: Option<String>,       // string but numeric
    pub cpu_count: Option<String>,        // string but numeric (logical threads)
    pub simd_features: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreGpu {
    pub manufacturer: Option<String>,      // "Intel", "AMD", "NVIDIA"
    pub codename: Option<String>,
    pub device_id: Option<String>,         // 4-char hex
    pub device_type: Option<String>,       // "Integrated GPU", "Discrete GPU", "Unknown"
    pub subsystem_id: Option<String>,
    pub pci_path: Option<String>,
    pub acpi_path: Option<String>,
    pub resizable_bar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreMonitor {
    pub connector_type: Option<String>,
    pub resolution: Option<String>,
    pub connected_gpu: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreNetwork {
    pub bus_type: Option<String>,          // "PCI", "USB", "ACPI", "ROOT"
    pub device_id: Option<String>,
    pub subsystem_id: Option<String>,
    pub pci_path: Option<String>,
    pub acpi_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreSound {
    pub bus_type: Option<String>,
    pub device_id: Option<String>,
    pub subsystem_id: Option<String>,
    pub audio_endpoints: Option<Vec<String>>,
    pub controller_device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreUsbController {
    pub bus_type: Option<String>,
    pub device_id: Option<String>,
    pub subsystem_id: Option<String>,
    pub pci_path: Option<String>,
    pub acpi_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreInput {
    pub bus_type: Option<String>,
    pub device: Option<String>,
    pub device_id: Option<String>,
    pub device_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreStorageController {
    pub bus_type: Option<String>,
    pub device_id: Option<String>,
    pub subsystem_id: Option<String>,
    pub pci_path: Option<String>,
    pub acpi_path: Option<String>,
    pub disk_drives: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreBiometric {
    pub bus_type: Option<String>,
    pub device: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreBluetooth {
    pub bus_type: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreSdController {
    pub bus_type: Option<String>,
    pub device_id: Option<String>,
    pub subsystem_id: Option<String>,
    pub pci_path: Option<String>,
    pub acpi_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpCoreSystemDevice {
    pub bus_type: Option<String>,
    pub device: Option<String>,
    pub device_id: Option<String>,
    pub subsystem_id: Option<String>,
    pub pci_path: Option<String>,
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
    let content = std::fs::read_to_string(path)
        .map_err(|e| HardwareError::permission_denied("read report file", e))?;
    
    let opcore_report: OpCoreReport = serde_json::from_str(&content)
        .map_err(|e| HardwareError::parse(format!("invalid OpCore-Simplify JSON: {}", e)))?;
    
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
        if let Some(fw_type) = bios.firmware_type {
            // Map "UEFI"/"BIOS" to our form_factor or add as note
            report.motherboard.bios_vendor = HardwareField::inferred(fw_type.clone(), "firmware type from OpCore report".to_string());
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
            // Parse comma/space-separated features
            let feature_list: Vec<String> = features
                .split(&[',', ' '][..])
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !feature_list.is_empty() {
                report.cpu.features = HardwareField::detected(feature_list);
            }
        }
    }

    // Import GPUs
    if let Some(gpus) = opcore_report.gpu {
        for (key, gpu) in gpus {
            let mut gpu_info = GpuInfo::default();
            
            if let Some(mfr) = gpu.manufacturer {
                gpu_info.vendor = HardwareField::detected(mfr.clone());
                // Map vendor to vendor_id if known
                gpu_info.vendor_id = HardwareField::inferred(
                    match mfr.as_str() {
                        "Intel" => "8086",
                        "AMD" => "1002",
                        "NVIDIA" => "10DE",
                        _ => "0000",
                    }.to_string(),
                    format!("mapped from manufacturer '{}'", mfr)
                );
            }
            if let Some(codename) = gpu.codename {
                gpu_info.codename = HardwareField::detected(codename);
            }
            if let Some(dev_id) = gpu.device_id {
                gpu_info.device_id = HardwareField::detected(dev_id.to_uppercase());
            }
            if let Some(dev_type) = gpu.device_type {
                gpu_info.is_integrated = HardwareField::detected(dev_type == "Integrated GPU");
            }
            if let Some(subsys) = gpu.subsystem_id {
                // Store subsystem ID in notes or as inferred field
                gpu_info.revision = HardwareField::inferred(subsys, "subsystem ID from OpCore report".to_string());
            }
            if let Some(pci_path) = gpu.pci_path {
                gpu_info.pci_location = HardwareField::detected(pci_path);
            }
            
            // Only add GPU if we have at least some identifying info
            if gpu_info.vendor.is_some() || gpu_info.device_id.is_some() || gpu_info.name.is_some() {
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
            if let Some(dev_id) = net.device_id {
                iface.pci_ids = HardwareField::detected(PciIds {
                    vendor_id: String::new(), // Can't split device_id without vendor
                    device_id: dev_id.to_uppercase(),
                    class_code: String::new(),
                    subclass_code: String::new(),
                });
            }
            if let Some(subsys) = net.subsystem_id {
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
            iface.name = HardwareField::detected(key.clone());
            if let Some(bus_type) = net.bus_type {
                iface.type_ = HardwareField::detected(bus_type);
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
            if let Some(bus_type) = ctrl.bus_type {
                usb_ctrl.driver = HardwareField::detected(bus_type);
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
            if let Some(codec) = snd.controller_device_id {
                audio.codec = HardwareField::detected(codec);
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
            if let Some(bus_type) = ctrl.bus_type {
                store.interface = HardwareField::detected(bus_type);
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

    Ok(ImportResult {
        report,
        unmapped_fields: unmapped,
        warnings,
    })
}

/// Check if a JSON file appears to be an OpCore-Simplify Report.json.
/// This is a heuristic based on required top-level keys.
pub fn is_opcore_report<P: AsRef<Path>>(path: P) -> Result<bool, HardwareError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| HardwareError::permission_denied("read report file", e))?;
    
    let value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| HardwareError::parse(format!("invalid JSON: {}", e)))?;
    
    // Check for required top-level keys from OpCore schema
    let required_keys = ["Motherboard", "BIOS", "CPU", "GPU", "Network", "USB Controllers"];
    let has_required = required_keys.iter().all(|k| value.get(*k).is_some());
    
    Ok(has_required)
}