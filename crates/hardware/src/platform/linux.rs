//! Linux hardware detection using sysfs, procfs, and standard system interfaces.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::HardwareError;
use hackintosh_studio_core::hardware::*;

/// Linux hardware detector using native system interfaces.
pub struct LinuxDetector {
    sys_root: PathBuf,
    proc_root: PathBuf,
}

impl LinuxDetector {
    /// Create a new Linux detector with default paths.
    pub fn new() -> Self {
        Self {
            sys_root: PathBuf::from("/sys"),
            proc_root: PathBuf::from("/proc"),
        }
    }

    /// Create a detector with custom root paths (useful for testing).
    pub fn with_roots(sys_root: PathBuf, proc_root: PathBuf) -> Self {
        Self {
            sys_root,
            proc_root,
        }
    }

    /// Read a file to string, returning None if not found or unreadable.
    fn read_file_opt(&self, path: &Path) -> Option<String> {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    }

    /// Read a file to string, returning an error with context.
    fn read_file(&self, path: &Path) -> Result<String, HardwareError> {
        fs::read_to_string(path)
            .map(|s| s.trim().to_string())
            .map_err(|e| HardwareError::permission_denied(path.display().to_string(), e))
    }

    /// Parse a key=value line.
    fn parse_key_value(line: &str) -> Option<(String, String)> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let mut parts = line.splitn(2, '=');
        let key = parts.next()?.trim().to_string();
        let value = parts.next()?.trim().to_string();
        Some((key, value))
    }

    /// Parse /proc/cpuinfo.
    fn parse_cpuinfo(&self) -> Result<Vec<HashMap<String, String>>, HardwareError> {
        let content = self.read_file(&self.proc_root.join("cpuinfo"))?;
        let mut cpus = Vec::new();
        let mut current = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                if !current.is_empty() {
                    cpus.push(current);
                    current = HashMap::new();
                }
                continue;
            }
            if let Some((k, v)) = Self::parse_key_value(line) {
                current.insert(k, v);
            }
        }
        if !current.is_empty() {
            cpus.push(current);
        }
        Ok(cpus)
    }

    /// Get CPU vendor from cpuinfo.
    fn cpu_vendor(&self, cpuinfo: &[HashMap<String, String>]) -> HardwareField<String> {
        cpuinfo
            .first()
            .and_then(|c| c.get("vendor_id"))
            .map(|v| HardwareField::detected(v.clone()))
            .unwrap_or_else(|| HardwareField::unavailable("vendor_id not in /proc/cpuinfo"))
    }

    /// Get CPU brand/model from cpuinfo.
    fn cpu_brand(&self, cpuinfo: &[HashMap<String, String>]) -> HardwareField<String> {
        cpuinfo
            .first()
            .and_then(|c| c.get("model name"))
            .map(|v| HardwareField::detected(v.clone()))
            .unwrap_or_else(|| HardwareField::unavailable("model name not in /proc/cpuinfo"))
    }

    /// Get CPU family/model/stepping from cpuinfo.
    fn cpu_family_model_stepping(
        &self,
        cpuinfo: &[HashMap<String, String>],
    ) -> (HardwareField<u8>, HardwareField<u8>, HardwareField<u8>) {
        let first = cpuinfo.first();
        let family = first
            .and_then(|c| c.get("cpu family"))
            .and_then(|v| v.parse().ok())
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable("cpu family not in /proc/cpuinfo"));
        let model = first
            .and_then(|c| c.get("model"))
            .and_then(|v| v.parse().ok())
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable("model not in /proc/cpuinfo"));
        let stepping = first
            .and_then(|c| c.get("stepping"))
            .and_then(|v| v.parse().ok())
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable("stepping not in /proc/cpuinfo"));
        (family, model, stepping)
    }

    /// Get CPU microcode version.
    fn cpu_microcode(&self, cpuinfo: &[HashMap<String, String>]) -> HardwareField<String> {
        cpuinfo
            .first()
            .and_then(|c| c.get("microcode"))
            .map(|v| HardwareField::detected(v.clone()))
            .unwrap_or_else(|| HardwareField::unavailable("microcode not in /proc/cpuinfo"))
    }

    /// Get CPU cores/threads.
    fn cpu_cores_threads(
        &self,
        cpuinfo: &[HashMap<String, String>],
    ) -> (HardwareField<u32>, HardwareField<u32>) {
        let physical_cores = cpuinfo
            .iter()
            .filter_map(|c| c.get("core id"))
            .collect::<std::collections::HashSet<_>>()
            .len() as u32;
        let logical_threads = cpuinfo.len() as u32;

        let physical = if physical_cores > 0 {
            HardwareField::detected(physical_cores)
        } else {
            HardwareField::unavailable("no core id in /proc/cpuinfo")
        };
        let logical = if logical_threads > 0 {
            HardwareField::detected(logical_threads)
        } else {
            HardwareField::unavailable("no processors in /proc/cpuinfo")
        };
        (physical, logical)
    }

    /// Get CPU frequencies.
    fn cpu_frequencies(&self) -> (HardwareField<u32>, HardwareField<u32>) {
        let base = self
            .read_file_opt(
                &self
                    .sys_root
                    .join("devices/system/cpu/cpu0/cpufreq/cpuinfo_min_freq"),
            )
            .and_then(|s| s.parse().ok())
            .map(|khz: u32| HardwareField::detected(khz / 1000))
            .unwrap_or_else(|| HardwareField::unavailable("cpufreq not accessible"));
        let max = self
            .read_file_opt(
                &self
                    .sys_root
                    .join("devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq"),
            )
            .and_then(|s| s.parse().ok())
            .map(|khz: u32| HardwareField::detected(khz / 1000))
            .unwrap_or_else(|| HardwareField::unavailable("cpufreq not accessible"));
        (base, max)
    }

    /// Determine CPU codename from family/model/stepping.
    fn cpu_codename(
        &self,
        vendor: &HardwareField<String>,
        family: &HardwareField<u8>,
        model: &HardwareField<u8>,
    ) -> HardwareField<String> {
        match (vendor.value.as_deref(), family.value, model.value) {
            (Some("GenuineIntel"), Some(f), Some(m)) => {
                let codename = match (f, m) {
                    (6, 0x1A | 0x1E) => "Nehalem",
                    (6, 0x25 | 0x2C | 0x2F) => "Westmere",
                    (6, 0x2A | 0x2E) => "Sandy Bridge",
                    (6, 0x3A | 0x3E) => "Ivy Bridge",
                    (6, 0x3C | 0x3F | 0x45 | 0x46) => "Haswell",
                    (6, 0x3D | 0x47 | 0x4F | 0x56) => "Broadwell",
                    (6, 0x4E | 0x5E) => "Skylake",
                    (6, 0x8E | 0x9E) => "Kaby Lake / Coffee Lake",
                    (6, 0xA5 | 0xA6) => "Comet Lake",
                    (6, 0xA7 | 0xBA | 0xBC) => "Rocket Lake",
                    (6, 0x97 | 0x9A) => "Alder Lake",
                    (6, 0xBE | 0xBF) => "Raptor Lake",
                    (6, 0xB7) => "Raptor Lake",
                    (6, 0xC6 | 0xC7) => "Meteor Lake",
                    (6, 0xD0) => "Arrow Lake",
                    _ => "Unknown Intel",
                };
                HardwareField::inferred(
                    codename.to_string(),
                    format!("family={:?}, model={:?}", family.value, model.value),
                )
            }
            (Some("AuthenticAMD"), Some(f), Some(m)) => {
                let codename = match (f, m) {
                    (0x17, 0x00..=0x0F) => "Zen",
                    (0x17, 0x10..=0x1F) => "Zen+",
                    (0x17, 0x20..=0x2F) => "Zen 2",
                    (0x17, 0x30..=0x3F) => "Zen 3",
                    (0x19, _) => "Zen 4",
                    _ => "Unknown AMD",
                };
                HardwareField::inferred(
                    codename.to_string(),
                    format!("family={:?}, model={:?}", family.value, model.value),
                )
            }
            _ => HardwareField::unavailable("unknown vendor or missing family/model"),
        }
    }

    /// Get CPU cache info.
    fn cpu_cache(&self) -> HardwareField<CpuCache> {
        let mut l1_data = 0;
        let mut l1_inst = 0;
        let mut l2 = 0;
        let mut l3 = 0;

        let cache_dir = self.sys_root.join("devices/system/cpu/cpu0/cache");
        if let Ok(entries) = fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let level = self
                    .read_file_opt(&path.join("level"))
                    .and_then(|s| s.parse().ok());
                let size = self.read_file_opt(&path.join("size")).and_then(|s| {
                    let s = s.trim_end_matches('K').trim_end_matches('k');
                    s.parse::<u32>().ok()
                });
                let cache_type = self.read_file_opt(&path.join("type")).unwrap_or_default();

                if let (Some(lvl), Some(sz)) = (level, size) {
                    match (lvl, cache_type.as_str()) {
                        (1, "Data") => l1_data += sz,
                        (1, "Instruction") => l1_inst += sz,
                        (2, _) => l2 += sz,
                        (3, _) => l3 += sz,
                        _ => {}
                    }
                }
            }
        }

        if l1_data > 0 || l1_inst > 0 || l2 > 0 || l3 > 0 {
            HardwareField::detected(CpuCache {
                l1_data_kb: l1_data,
                l1_instruction_kb: l1_inst,
                l2_kb: l2,
                l3_kb: l3,
            })
        } else {
            HardwareField::unavailable("cache info not in sysfs")
        }
    }

    /// Get CPU features from /proc/cpuinfo flags.
    fn cpu_features(&self, cpuinfo: &[HashMap<String, String>]) -> HardwareField<Vec<String>> {
        cpuinfo
            .first()
            .and_then(|c| c.get("flags"))
            .map(|flags| {
                let features: Vec<String> =
                    flags.split_whitespace().map(|s| s.to_string()).collect();
                HardwareField::detected(features)
            })
            .unwrap_or_else(|| HardwareField::unavailable("flags not in /proc/cpuinfo"))
    }

    /// Detect if CPU is hybrid (big.LITTLE).
    fn cpu_is_hybrid(&self, cpuinfo: &[HashMap<String, String>]) -> HardwareField<bool> {
        // Check for different core types in cpuinfo
        let core_types: std::collections::HashSet<_> =
            cpuinfo.iter().filter_map(|c| c.get("core type")).collect();
        if core_types.len() > 1 {
            HardwareField::detected(true)
        } else {
            HardwareField::inferred(false, "single core type detected".to_string())
        }
    }

    /// Parse /proc/meminfo.
    fn parse_meminfo(&self) -> Result<HashMap<String, u64>, HardwareError> {
        let content = self.read_file(&self.proc_root.join("meminfo"))?;
        let mut map = HashMap::new();
        for line in content.lines() {
            if let Some((k, v)) = Self::parse_key_value(line) {
                let v = v.trim_end_matches(" kB").trim();
                if let Ok(val) = v.parse::<u64>() {
                    map.insert(k, val * 1024); // Convert to bytes
                }
            }
        }
        Ok(map)
    }

    /// Get DMI/SMBIOS information.
    fn read_dmi(&self, path: &str) -> HardwareField<String> {
        let full_path = self.sys_root.join("class/dmi/id").join(path);
        self.read_file_opt(&full_path)
            .filter(|s| !s.is_empty())
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable(format!("DMI {} not readable", path)))
    }

    /// Parse lspci output for PCI devices (fallback if sysfs insufficient).
    fn parse_lspci(&self) -> Vec<PciDevice> {
        // Try to run lspci -nnmm if available
        let output = std::process::Command::new("lspci").args(["-nnmm"]).output();
        let mut devices = Vec::new();

        if let Ok(out) = output {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                for line in stdout.lines() {
                    if let Some(dev) = Self::parse_lspci_line(line) {
                        devices.push(dev);
                    }
                }
            }
        }
        devices
    }

    fn parse_lspci_line(line: &str) -> Option<PciDevice> {
        // Format: "00:00.0 \"Host bridge\" \"Intel Corporation\" \"12th Gen Core Processor Host Bridge/DRAM Registers\" \"8086\" \"4670\""
        let parts: Vec<&str> = line.split('"').collect();
        if parts.len() >= 6 {
            let addr = parts[0].trim();
            let _class = parts[1];
            let vendor_name = parts[3];
            let device_name = parts[5];
            let ids: Vec<&str> = parts[0].split_whitespace().collect();
            let vendor_id = ids.get(2).unwrap_or(&"");
            let device_id = ids.get(3).unwrap_or(&"");

            Some(PciDevice {
                address: HardwareField::detected(addr.to_string()),
                vendor_id: HardwareField::detected(vendor_id.to_string()),
                device_id: HardwareField::detected(device_id.to_string()),
                class_code: HardwareField::unavailable("not parsed"),
                subclass_code: HardwareField::unavailable("not parsed"),
                prog_if: HardwareField::unavailable("not parsed"),
                vendor_name: HardwareField::detected(vendor_name.to_string()),
                device_name: HardwareField::detected(device_name.to_string()),
                driver: HardwareField::unavailable("not detected"),
                subsystem_vendor_id: HardwareField::unavailable("not parsed"),
                subsystem_device_id: HardwareField::unavailable("not parsed"),
            })
        } else {
            None
        }
    }

    /// Get GPU devices from PCI.
    fn get_gpus(&self, pci_devices: &[PciDevice]) -> Vec<GpuInfo> {
        pci_devices.iter()
            .filter(|d| {
                d.class_code.value.as_deref() == Some("0300") || // VGA compatible controller
                d.class_code.value.as_deref() == Some("0380") || // Other display
                d.device_name.value.as_deref().map(|n| n.to_lowercase().contains("gpu") || n.to_lowercase().contains("graphics") || n.to_lowercase().contains("vga")).unwrap_or(false)
            })
            .map(|d| GpuInfo {
                vendor_id: d.vendor_id.clone(),
                device_id: d.device_id.clone(),
                vendor: d.vendor_name.clone(),
                name: d.device_name.clone(),
                codename: HardwareField::unavailable("codename lookup not implemented"),
                revision: HardwareField::unavailable("not detected"),
                vram_mb: HardwareField::unavailable("not detected"),
                is_integrated: HardwareField::unavailable("not detected"),
                pci_location: d.address.clone(),
                driver: d.driver.clone(),
            })
            .collect()
    }

    /// Get audio devices from PCI.
    fn get_audio(&self, pci_devices: &[PciDevice]) -> Vec<AudioDevice> {
        pci_devices
            .iter()
            .filter(|d| d.class_code.value.as_deref() == Some("0403")) // Audio device
            .map(|d| AudioDevice {
                name: d.device_name.clone(),
                pci_ids: HardwareField::detected(PciIds {
                    vendor_id: d.vendor_id.value.clone().unwrap_or_default(),
                    device_id: d.device_id.value.clone().unwrap_or_default(),
                    class_code: d.class_code.value.clone().unwrap_or_default(),
                    subclass_code: d.subclass_code.value.clone().unwrap_or_default(),
                }),
                driver: d.driver.clone(),
                codec: HardwareField::unavailable("codec detection not implemented"),
                layout_id: HardwareField::unavailable("not detected"),
            })
            .collect()
    }

    /// Get network interfaces.
    fn get_network(&self) -> Vec<NetworkInterface> {
        let mut interfaces = Vec::new();
        let net_dir = self.sys_root.join("class/net");
        if let Ok(entries) = fs::read_dir(&net_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "lo" {
                    continue;
                } // Skip loopback

                let path = entry.path();
                let mac = self
                    .read_file_opt(&path.join("address"))
                    .filter(|s| s != "00:00:00:00:00:00")
                    .map(HardwareField::detected)
                    .unwrap_or_else(|| HardwareField::unavailable("MAC not readable"));

                let driver = self
                    .read_file_opt(&path.join("device/driver/module/srcversion"))
                    .map(HardwareField::detected)
                    .unwrap_or_else(|| HardwareField::unavailable("driver not readable"));

                let operstate = self
                    .read_file_opt(&path.join("operstate"))
                    .map(|s| HardwareField::detected(s == "up"))
                    .unwrap_or_else(|| HardwareField::unavailable("operstate not readable"));

                let speed = self
                    .read_file_opt(&path.join("speed"))
                    .and_then(|s| s.parse().ok())
                    .map(HardwareField::detected)
                    .unwrap_or_else(|| HardwareField::unavailable("speed not readable"));

                // Try to get PCI IDs for this interface
                let pci_ids = self
                    .read_file_opt(&path.join("device/uevent"))
                    .and_then(|s| {
                        let mut vendor_id = None;
                        let mut device_id = None;
                        for line in s.lines() {
                            if line.starts_with("PCI_ID=") {
                                let parts: Vec<&str> = line.split('=').nth(1)?.split(':').collect();
                                vendor_id = parts.get(0).map(|s| s.to_string());
                                device_id = parts.get(1).map(|s| s.to_string());
                            }
                        }
                        vendor_id.zip(device_id).map(|(v, d)| {
                            HardwareField::detected(PciIds {
                                vendor_id: v,
                                device_id: d,
                                class_code: String::new(),
                                subclass_code: String::new(),
                            })
                        })
                    })
                    .unwrap_or_else(|| HardwareField::unavailable("PCI IDs not in uevent"));

                interfaces.push(NetworkInterface {
                    name: HardwareField::detected(name),
                    mac,
                    pci_ids,
                    driver,
                    type_: HardwareField::unavailable("type detection not implemented"),
                    is_up: operstate,
                    speed_mbps: speed,
                });
            }
        }
        interfaces
    }

    /// Get storage devices.
    fn get_storage(&self) -> Vec<StorageDevice> {
        let mut devices = Vec::new();
        let block_dir = self.sys_root.join("block");
        if let Ok(entries) = fs::read_dir(&block_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let path = entry.path();

                // Skip partitions (contain numbers after name) and loop/ram devices
                if name
                    .chars()
                    .last()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
                {
                    continue;
                }
                if name.starts_with("loop") || name.starts_with("ram") || name.starts_with("dm-") {
                    continue;
                }

                let model = self
                    .read_file_opt(&path.join("device/model"))
                    .map(HardwareField::detected)
                    .unwrap_or_else(|| HardwareField::unavailable("model not readable"));

                let _vendor = self
                    .read_file_opt(&path.join("device/vendor"))
                    .map(HardwareField::detected)
                    .unwrap_or_else(|| HardwareField::unavailable("vendor not readable"));

                let size = self
                    .read_file_opt(&path.join("size"))
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(|sectors| HardwareField::detected(sectors * 512))
                    .unwrap_or_else(|| HardwareField::unavailable("size not readable"));

                let rotational = self
                    .read_file_opt(&path.join("queue/rotational"))
                    .and_then(|s| s.parse::<u8>().ok())
                    .map(|r| {
                        HardwareField::detected(if r == 1 { "HDD" } else { "SSD" }.to_string())
                    })
                    .unwrap_or_else(|| HardwareField::unavailable("rotational not readable"));

                let device_type = if name.starts_with("nvme") {
                    HardwareField::detected("NVMe".to_string())
                } else if rotational.value.as_deref() == Some("SSD") {
                    HardwareField::detected("SATA SSD".to_string())
                } else if rotational.value.as_deref() == Some("HDD") {
                    HardwareField::detected("HDD".to_string())
                } else {
                    HardwareField::unavailable("type unknown")
                };

                // Get PCI IDs if available
                let pci_ids = self
                    .read_file_opt(&path.join("device/uevent"))
                    .and_then(|s| {
                        let mut vendor_id = None;
                        let mut device_id = None;
                        for line in s.lines() {
                            if line.starts_with("PCI_ID=") {
                                let parts: Vec<&str> = line.split('=').nth(1)?.split(':').collect();
                                vendor_id = parts.get(0).map(|s| s.to_string());
                                device_id = parts.get(1).map(|s| s.to_string());
                            }
                        }
                        vendor_id.zip(device_id).map(|(v, d)| {
                            HardwareField::detected(PciIds {
                                vendor_id: v,
                                device_id: d,
                                class_code: String::new(),
                                subclass_code: String::new(),
                            })
                        })
                    })
                    .unwrap_or_else(|| HardwareField::unavailable("PCI IDs not in uevent"));

                devices.push(StorageDevice {
                    name: HardwareField::detected(name),
                    model,
                    serial: HardwareField::unavailable("serial not implemented"),
                    size_bytes: size,
                    type_: device_type,
                    interface: HardwareField::unavailable("interface not implemented"),
                    pci_ids,
                    partitions: Vec::new(),
                    smart_health: HardwareField::unavailable("SMART not implemented"),
                });
            }
        }
        devices
    }

    /// Get USB controllers and ports.
    fn get_usb(&self) -> (Vec<UsbController>, Vec<UsbPort>) {
        let mut controllers = Vec::new();

        let usb_dir = self.sys_root.join("bus/usb/devices");
        if let Ok(entries) = fs::read_dir(&usb_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let path = entry.path();

                // Check if it's a controller (contains colon, e.g., "usb1")
                if name.starts_with("usb")
                    && name
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == 'u' || c == 's' || c == 'b')
                    && !name.contains(':')
                {
                    let _vendor_id = self
                        .read_file_opt(&path.join("idVendor"))
                        .map(HardwareField::detected)
                        .unwrap_or_else(|| HardwareField::unavailable("idVendor not readable"));
                    let _product_id = self
                        .read_file_opt(&path.join("idProduct"))
                        .map(HardwareField::detected)
                        .unwrap_or_else(|| HardwareField::unavailable("idProduct not readable"));
                    let _manufacturer = self.read_file_opt(&path.join("manufacturer"));
                    let _product = self.read_file_opt(&path.join("product"));
                    let speed = self
                        .read_file_opt(&path.join("speed"))
                        .map(|s| {
                            let s = s.trim_end_matches("M");
                            match s {
                                "1.5" => "1.0".to_string(),
                                "12" => "1.1".to_string(),
                                "480" => "2.0".to_string(),
                                "5000" => "3.0".to_string(),
                                "10000" => "3.1".to_string(),
                                "20000" => "3.2".to_string(),
                                _ => "Unknown".to_string(),
                            }
                        })
                        .map(HardwareField::detected)
                        .unwrap_or_else(|| HardwareField::unavailable("speed not readable"));

                    controllers.push(UsbController {
                        name: HardwareField::detected(name.clone()),
                        pci_ids: HardwareField::unavailable("PCI IDs not implemented"),
                        driver: HardwareField::unavailable("driver not implemented"),
                        usb_version: speed,
                        port_count: HardwareField::unavailable("port count not implemented"),
                        pci_location: HardwareField::unavailable("PCI location not implemented"),
                    });
                }
            }
        }

        (controllers, Vec::new())
    }

    /// Get ACPI table summaries.
    fn get_acpi_tables(&self) -> Vec<AcpiTableSummary> {
        let mut tables = Vec::new();
        let acpi_dir = self.sys_root.join("firmware/acpi/tables");
        if let Ok(entries) = fs::read_dir(&acpi_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Read first 36 bytes for header
                    if let Ok(data) = fs::read(&path) {
                        if data.len() >= 36 {
                            let signature = String::from_utf8_lossy(&data[0..4]).to_string();
                            let length = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
                            let oem_id = String::from_utf8_lossy(&data[10..16]).to_string();
                            let oem_table_id = String::from_utf8_lossy(&data[16..24]).to_string();
                            let oem_revision =
                                u32::from_le_bytes([data[24], data[25], data[26], data[27]]);
                            let creator_id = String::from_utf8_lossy(&data[28..32]).to_string();
                            let creator_revision =
                                u32::from_le_bytes([data[32], data[33], data[34], data[35]]);

                            tables.push(AcpiTableSummary {
                                signature,
                                length,
                                oem_id,
                                oem_table_id,
                                oem_revision,
                                creator_id,
                                creator_revision,
                            });
                        }
                    }
                }
            }
        }
        tables
    }

    /// Dump ACPI tables to directory.
    fn dump_acpi_tables_impl(&self, dir: &Path) -> Result<(), HardwareError> {
        fs::create_dir_all(dir)?;
        let acpi_dir = self.sys_root.join("firmware/acpi/tables");
        if let Ok(entries) = fs::read_dir(&acpi_dir) {
            for entry in entries.flatten() {
                let src = entry.path();
                let dst = dir.join(entry.file_name());
                fs::copy(&src, &dst)
                    .map_err(|e| HardwareError::permission_denied(dst.display().to_string(), e))?;
            }
        }
        Ok(())
    }

    /// Get OS information.
    fn get_os_info(&self) -> OsInfo {
        let kernel_version = self
            .read_file_opt(&self.proc_root.join("sys/kernel/osrelease"))
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable("kernel version not readable"));

        let distro_id = self
            .read_file_opt(&PathBuf::from("/etc/os-release"))
            .and_then(|s| {
                for line in s.lines() {
                    if line.starts_with("ID=") {
                        return Some(line.split('=').nth(1)?.trim_matches('"').to_string());
                    }
                }
                None
            })
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable("distro ID not readable"));

        let distro_version = self
            .read_file_opt(&PathBuf::from("/etc/os-release"))
            .and_then(|s| {
                for line in s.lines() {
                    if line.starts_with("VERSION_ID=") {
                        return Some(line.split('=').nth(1)?.trim_matches('"').to_string());
                    }
                }
                None
            })
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable("distro version not readable"));

        let hostname = self
            .read_file_opt(&self.proc_root.join("sys/kernel/hostname"))
            .map(HardwareField::detected)
            .unwrap_or_else(|| HardwareField::unavailable("hostname not readable"));

        OsInfo {
            name: HardwareField::detected("Linux".to_string()),
            version: distro_version.clone(),
            kernel_version,
            architecture: HardwareField::detected(std::env::consts::ARCH.to_string()),
            distro_id,
            distro_version,
            hostname,
        }
    }
}

impl Default for LinuxDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::HardwareDetector for LinuxDetector {
    fn detect(&self) -> Result<HardwareReport, HardwareError> {
        let mut report = HardwareReport::new();

        // OS info
        report.os = self.get_os_info();

        // CPU info
        let cpuinfo = self.parse_cpuinfo()?;
        report.cpu.vendor = self.cpu_vendor(&cpuinfo);
        report.cpu.brand = self.cpu_brand(&cpuinfo);
        let (family, model, stepping) = self.cpu_family_model_stepping(&cpuinfo);
        report.cpu.family = family;
        report.cpu.model = model;
        report.cpu.stepping = stepping;
        report.cpu.microcode = self.cpu_microcode(&cpuinfo);
        let (physical, logical) = self.cpu_cores_threads(&cpuinfo);
        report.cpu.physical_cores = physical;
        report.cpu.logical_threads = logical;
        let (base_freq, max_freq) = self.cpu_frequencies();
        report.cpu.base_frequency_mhz = base_freq;
        report.cpu.max_turbo_frequency_mhz = max_freq;
        report.cpu.codename =
            self.cpu_codename(&report.cpu.vendor, &report.cpu.family, &report.cpu.model);
        report.cpu.cache = self.cpu_cache();
        report.cpu.features = self.cpu_features(&cpuinfo);
        report.cpu.is_hybrid = self.cpu_is_hybrid(&cpuinfo);

        // Memory info
        let meminfo = self.parse_meminfo()?;
        report.memory.total_mb = meminfo
            .get("MemTotal")
            .map(|v| HardwareField::detected(*v / 1024 / 1024))
            .unwrap_or_else(|| HardwareField::unavailable("MemTotal not in /proc/meminfo"));
        report.memory.available_mb = meminfo
            .get("MemAvailable")
            .map(|v| HardwareField::detected(*v / 1024 / 1024))
            .unwrap_or_else(|| HardwareField::unavailable("MemAvailable not in /proc/meminfo"));

        // Motherboard/DMI info
        report.motherboard.vendor = self.read_dmi("board_vendor");
        report.motherboard.model = self.read_dmi("board_name");
        report.motherboard.version = self.read_dmi("board_version");
        report.motherboard.serial = self.read_dmi("board_serial");
        report.motherboard.bios_vendor = self.read_dmi("bios_vendor");
        report.motherboard.bios_version = self.read_dmi("bios_version");
        report.motherboard.bios_date = self.read_dmi("bios_date");
        report.motherboard.chipset =
            HardwareField::unavailable("chipset detection not implemented");
        report.motherboard.form_factor = self.read_dmi("chassis_type");
        report.motherboard.uuid = self.read_dmi("product_uuid");

        // PCI devices (using lspci as primary source)
        let pci_devices = self.parse_lspci();
        report.pci_devices = pci_devices.clone();

        // GPUs from PCI
        report.gpus = self.get_gpus(&pci_devices);

        // Audio from PCI
        report.audio = self.get_audio(&pci_devices);

        // Network interfaces
        report.network = self.get_network();

        // Storage
        report.storage = self.get_storage();

        // USB
        let (controllers, ports) = self.get_usb();
        report.usb_controllers = controllers;
        report.usb_ports = ports;

        // ACPI tables
        report.acpi_tables = self.get_acpi_tables();

        Ok(report)
    }

    fn detect_into(&self, report: &mut HardwareReport) -> Result<(), HardwareError> {
        let new_report = self.detect()?;
        *report = new_report;
        Ok(())
    }

    fn get_acpi_tables(&self) -> Result<Vec<AcpiTableSummary>, HardwareError> {
        Ok(self.get_acpi_tables())
    }

    fn dump_acpi_tables(&self, dir: &Path) -> Result<(), HardwareError> {
        self.dump_acpi_tables_impl(dir)
    }

    fn platform_name(&self) -> &'static str {
        "linux"
    }
}
