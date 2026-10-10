use serde::{Deserialize, Serialize};

/// Source of a hardware field value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldSource {
    /// Value was directly detected from system interfaces.
    Detected,
    /// Value was inferred from other detected data.
    Inferred,
    /// Value was provided by the user manually.
    UserProvided,
    /// Value is unavailable on this platform.
    Unavailable,
    /// Value could not be determined.
    Unknown,
}

/// Confidence level for a detected value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    /// High confidence - directly read from authoritative source.
    High,
    /// Medium confidence - derived or from secondary source.
    Medium,
    /// Low confidence - heuristic or best guess.
    Low,
}

/// A hardware field with metadata about its source and confidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareField<T> {
    /// The actual value, if available.
    pub value: Option<T>,
    /// Where this value came from.
    pub source: FieldSource,
    /// Confidence in the value.
    pub confidence: Confidence,
    /// Additional notes about how the value was obtained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl<T> HardwareField<T> {
    /// Create a detected field with high confidence.
    pub fn detected(value: T) -> Self {
        Self {
            value: Some(value),
            source: FieldSource::Detected,
            confidence: Confidence::High,
            notes: None,
        }
    }

    /// Create an inferred field with medium confidence.
    pub fn inferred(value: T, notes: impl Into<String>) -> Self {
        Self {
            value: Some(value),
            source: FieldSource::Inferred,
            confidence: Confidence::Medium,
            notes: Some(notes.into()),
        }
    }

    /// Create a user-provided field.
    pub fn user_provided(value: T) -> Self {
        Self {
            value: Some(value),
            source: FieldSource::UserProvided,
            confidence: Confidence::High,
            notes: None,
        }
    }

    /// Create an unavailable field.
    pub fn unavailable(notes: impl Into<String>) -> Self {
        Self {
            value: None,
            source: FieldSource::Unavailable,
            confidence: Confidence::Low,
            notes: Some(notes.into()),
        }
    }

    /// Create an unknown field.
    pub fn unknown(notes: impl Into<String>) -> Self {
        Self {
            value: None,
            source: FieldSource::Unknown,
            confidence: Confidence::Low,
            notes: Some(notes.into()),
        }
    }

    /// Check if the field has a value.
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    /// Get the value if present.
    pub fn as_ref(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

/// CPU information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// Vendor string (e.g., "GenuineIntel", "AuthenticAMD").
    pub vendor: HardwareField<String>,
    /// Brand/model string (e.g., "Intel(R) Core(TM) i7-12700K").
    pub brand: HardwareField<String>,
    /// Family number.
    pub family: HardwareField<u8>,
    /// Model number.
    pub model: HardwareField<u8>,
    /// Stepping number.
    pub stepping: HardwareField<u8>,
    /// Microcode version.
    pub microcode: HardwareField<String>,
    /// Number of physical cores.
    pub physical_cores: HardwareField<u32>,
    /// Number of logical threads.
    pub logical_threads: HardwareField<u32>,
    /// Base frequency in MHz.
    pub base_frequency_mhz: HardwareField<u32>,
    /// Max turbo frequency in MHz.
    pub max_turbo_frequency_mhz: HardwareField<u32>,
    /// CPU codename (e.g., "Alder Lake", "Zen 3").
    pub codename: HardwareField<String>,
    /// Whether it's a hybrid architecture (P-cores + E-cores).
    pub is_hybrid: HardwareField<bool>,
    /// Cache sizes in KB (L1, L2, L3).
    pub cache: HardwareField<CpuCache>,
    /// Feature flags (e.g., "vmx", "avx2", "sse4.2").
    pub features: HardwareField<Vec<String>>,
}

/// CPU cache information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuCache {
    pub l1_data_kb: u32,
    pub l1_instruction_kb: u32,
    pub l2_kb: u32,
    pub l3_kb: u32,
}

/// GPU information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    /// Vendor ID (PCI).
    pub vendor_id: HardwareField<String>,
    /// Device ID (PCI).
    pub device_id: HardwareField<String>,
    /// Vendor name (e.g., "Intel", "AMD", "NVIDIA").
    pub vendor: HardwareField<String>,
    /// Device name/model.
    pub name: HardwareField<String>,
    /// GPU codename (e.g., "Alder Lake GT1", "Navi 21").
    pub codename: HardwareField<String>,
    /// Revision.
    pub revision: HardwareField<String>,
    /// Video memory in MB (if dedicated).
    pub vram_mb: HardwareField<u32>,
    /// Whether it's an integrated GPU.
    pub is_integrated: HardwareField<bool>,
    /// PCI bus location (domain:bus:device.function).
    pub pci_location: HardwareField<String>,
    /// Driver in use (if detectable).
    pub driver: HardwareField<String>,
}

/// Motherboard / system board information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotherboardInfo {
    /// Vendor/manufacturer.
    pub vendor: HardwareField<String>,
    /// Model/product name.
    pub model: HardwareField<String>,
    /// Version/revision.
    pub version: HardwareField<String>,
    /// Serial number.
    pub serial: HardwareField<String>,
    /// BIOS/UEFI vendor.
    pub bios_vendor: HardwareField<String>,
    /// BIOS/UEFI version.
    pub bios_version: HardwareField<String>,
    /// BIOS/UEFI date.
    pub bios_date: HardwareField<String>,
    /// Chipset (e.g., "Z690", "B550").
    pub chipset: HardwareField<String>,
    /// Form factor.
    pub form_factor: HardwareField<String>,
    /// SMBIOS UUID.
    pub uuid: HardwareField<String>,
}

/// Memory module information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryModule {
    /// Slot/locator.
    pub locator: HardwareField<String>,
    /// Size in MB.
    pub size_mb: HardwareField<u32>,
    /// Type (e.g., "DDR4", "DDR5").
    pub type_: HardwareField<String>,
    /// Speed in MT/s.
    pub speed_mt_s: HardwareField<u32>,
    /// Manufacturer.
    pub manufacturer: HardwareField<String>,
    /// Part number.
    pub part_number: HardwareField<String>,
    /// Serial number.
    pub serial: HardwareField<String>,
    /// Voltage in millivolts.
    pub voltage_mv: HardwareField<u32>,
}

/// System memory information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total system memory in MB.
    pub total_mb: HardwareField<u64>,
    /// Available memory in MB.
    pub available_mb: HardwareField<u64>,
    /// Individual memory modules.
    pub modules: Vec<MemoryModule>,
}

/// Storage device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    /// Device name (e.g., "nvme0n1", "sda").
    pub name: HardwareField<String>,
    /// Model string.
    pub model: HardwareField<String>,
    /// Serial number.
    pub serial: HardwareField<String>,
    /// Size in bytes.
    pub size_bytes: HardwareField<u64>,
    /// Type (e.g., "NVMe", "SATA SSD", "HDD").
    pub type_: HardwareField<String>,
    /// Interface (e.g., "PCIe", "SATA", "USB").
    pub interface: HardwareField<String>,
    /// PCI vendor/device ID if applicable.
    pub pci_ids: HardwareField<PciIds>,
    /// Partitions.
    pub partitions: Vec<StoragePartition>,
    /// SMART health status (if available).
    pub smart_health: HardwareField<String>,
}

/// PCI IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciIds {
    pub vendor_id: String,
    pub device_id: String,
    pub class_code: String,
    pub subclass_code: String,
}

/// Storage partition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePartition {
    pub name: String,
    pub size_bytes: u64,
    pub filesystem: Option<String>,
    pub mount_point: Option<String>,
    pub part_type: Option<String>,
}

/// Network interface information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Interface name (e.g., "enp3s0", "wlan0").
    pub name: HardwareField<String>,
    /// MAC address.
    pub mac: HardwareField<String>,
    /// PCI vendor/device IDs.
    pub pci_ids: HardwareField<PciIds>,
    /// Driver in use.
    pub driver: HardwareField<String>,
    /// Type (e.g., "Ethernet", "WiFi", "Bluetooth").
    pub type_: HardwareField<String>,
    /// Whether it's up/active.
    pub is_up: HardwareField<bool>,
    /// Speed in Mbps (if known).
    pub speed_mbps: HardwareField<u32>,
}

/// Audio device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    /// Device name.
    pub name: HardwareField<String>,
    /// PCI vendor/device IDs.
    pub pci_ids: HardwareField<PciIds>,
    /// Driver in use.
    pub driver: HardwareField<String>,
    /// Codec (e.g., "ALC897", "ALC1220").
    pub codec: HardwareField<String>,
    /// Layout ID (for Hackintosh).
    pub layout_id: HardwareField<u32>,
}

/// USB controller information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbController {
    /// Controller name.
    pub name: HardwareField<String>,
    /// PCI vendor/device IDs.
    pub pci_ids: HardwareField<PciIds>,
    /// Driver in use.
    pub driver: HardwareField<String>,
    /// USB standard version (e.g., "2.0", "3.0", "3.1", "3.2", "4.0").
    pub usb_version: HardwareField<String>,
    /// Number of ports.
    pub port_count: HardwareField<u32>,
    /// PCI location.
    pub pci_location: HardwareField<String>,
}

/// USB port information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbPort {
    /// Port number.
    pub port_number: HardwareField<u32>,
    /// USB version.
    pub usb_version: HardwareField<String>,
    /// Connected device info (if any).
    pub device: HardwareField<UsbDevice>,
    /// Physical location/connector type.
    pub location: HardwareField<String>,
}

/// USB device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    /// Vendor ID.
    pub vendor_id: String,
    /// Product ID.
    pub product_id: String,
    /// Manufacturer string.
    pub manufacturer: Option<String>,
    /// Product string.
    pub product: Option<String>,
    /// Serial number.
    pub serial: Option<String>,
    /// Device class.
    pub device_class: Option<u8>,
    /// Device subclass.
    pub device_subclass: Option<u8>,
    /// Device protocol.
    pub device_protocol: Option<u8>,
}

/// PCI device information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PciDevice {
    /// PCI address (domain:bus:device.function).
    pub address: HardwareField<String>,
    /// Vendor ID.
    pub vendor_id: HardwareField<String>,
    /// Device ID.
    pub device_id: HardwareField<String>,
    /// Class code.
    pub class_code: HardwareField<String>,
    /// Subclass code.
    pub subclass_code: HardwareField<String>,
    /// Programming interface.
    pub prog_if: HardwareField<String>,
    /// Vendor name (from PCI ID database).
    pub vendor_name: HardwareField<String>,
    /// Device name (from PCI ID database).
    pub device_name: HardwareField<String>,
    /// Driver in use.
    pub driver: HardwareField<String>,
    /// Subsystem vendor ID.
    pub subsystem_vendor_id: HardwareField<String>,
    /// Subsystem device ID.
    pub subsystem_device_id: HardwareField<String>,
}

/// ACPI table summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpiTableSummary {
    /// Table signature (e.g., "DSDT", "SSDT", "FADT").
    pub signature: String,
    /// Table length in bytes.
    pub length: u32,
    /// OEM ID.
    pub oem_id: String,
    /// OEM table ID.
    pub oem_table_id: String,
    /// OEM revision.
    pub oem_revision: u32,
    /// Creator ID.
    pub creator_id: String,
    /// Creator revision.
    pub creator_revision: u32,
}

/// Operating system / host information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsInfo {
    /// OS name (e.g., "Linux", "Windows").
    pub name: HardwareField<String>,
    /// OS version.
    pub version: HardwareField<String>,
    /// Kernel version.
    pub kernel_version: HardwareField<String>,
    /// Architecture (e.g., "x86_64", "aarch64").
    pub architecture: HardwareField<String>,
    /// Distribution ID (Linux).
    pub distro_id: HardwareField<String>,
    /// Distribution version (Linux).
    pub distro_version: HardwareField<String>,
    /// Hostname.
    pub hostname: HardwareField<String>,
}

/// Complete hardware report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareReport {
    /// Report schema version.
    pub schema_version: u32,
    /// Timestamp when report was generated (ISO 8601).
    pub timestamp: String,
    /// Host OS information.
    pub os: OsInfo,
    /// CPU information.
    pub cpu: CpuInfo,
    /// GPU(s) information.
    pub gpus: Vec<GpuInfo>,
    /// Motherboard information.
    pub motherboard: MotherboardInfo,
    /// Memory information.
    pub memory: MemoryInfo,
    /// Storage devices.
    pub storage: Vec<StorageDevice>,
    /// Network interfaces.
    pub network: Vec<NetworkInterface>,
    /// Audio devices.
    pub audio: Vec<AudioDevice>,
    /// USB controllers.
    pub usb_controllers: Vec<UsbController>,
    /// USB ports.
    pub usb_ports: Vec<UsbPort>,
    /// PCI devices.
    pub pci_devices: Vec<PciDevice>,
    /// ACPI table summaries.
    pub acpi_tables: Vec<AcpiTableSummary>,
}

impl HardwareReport {
    /// Create a new empty hardware report with current timestamp.
    pub fn new() -> Self {
        use chrono::Utc;
        Self {
            schema_version: 1,
            timestamp: Utc::now().to_rfc3339(),
            os: OsInfo::default(),
            cpu: CpuInfo::default(),
            gpus: Vec::new(),
            motherboard: MotherboardInfo::default(),
            memory: MemoryInfo::default(),
            storage: Vec::new(),
            network: Vec::new(),
            audio: Vec::new(),
            usb_controllers: Vec::new(),
            usb_ports: Vec::new(),
            pci_devices: Vec::new(),
            acpi_tables: Vec::new(),
        }
    }
}

impl Default for HardwareReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for OsInfo {
    fn default() -> Self {
        Self {
            name: HardwareField::unavailable("not detected"),
            version: HardwareField::unavailable("not detected"),
            kernel_version: HardwareField::unavailable("not detected"),
            architecture: HardwareField::unavailable("not detected"),
            distro_id: HardwareField::unavailable("not detected"),
            distro_version: HardwareField::unavailable("not detected"),
            hostname: HardwareField::unavailable("not detected"),
        }
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            vendor: HardwareField::unavailable("not detected"),
            brand: HardwareField::unavailable("not detected"),
            family: HardwareField::unavailable("not detected"),
            model: HardwareField::unavailable("not detected"),
            stepping: HardwareField::unavailable("not detected"),
            microcode: HardwareField::unavailable("not detected"),
            physical_cores: HardwareField::unavailable("not detected"),
            logical_threads: HardwareField::unavailable("not detected"),
            base_frequency_mhz: HardwareField::unavailable("not detected"),
            max_turbo_frequency_mhz: HardwareField::unavailable("not detected"),
            codename: HardwareField::unavailable("not detected"),
            is_hybrid: HardwareField::unavailable("not detected"),
            cache: HardwareField::unavailable("not detected"),
            features: HardwareField::unavailable("not detected"),
        }
    }
}

impl Default for MotherboardInfo {
    fn default() -> Self {
        Self {
            vendor: HardwareField::unavailable("not detected"),
            model: HardwareField::unavailable("not detected"),
            version: HardwareField::unavailable("not detected"),
            serial: HardwareField::unavailable("not detected"),
            bios_vendor: HardwareField::unavailable("not detected"),
            bios_version: HardwareField::unavailable("not detected"),
            bios_date: HardwareField::unavailable("not detected"),
            chipset: HardwareField::unavailable("not detected"),
            form_factor: HardwareField::unavailable("not detected"),
            uuid: HardwareField::unavailable("not detected"),
        }
    }
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self {
            total_mb: HardwareField::unavailable("not detected"),
            available_mb: HardwareField::unavailable("not detected"),
            modules: Vec::new(),
        }
    }
}

impl Default for GpuInfo {
    fn default() -> Self {
        Self {
            vendor_id: HardwareField::unavailable("not detected"),
            device_id: HardwareField::unavailable("not detected"),
            vendor: HardwareField::unavailable("not detected"),
            name: HardwareField::unavailable("not detected"),
            codename: HardwareField::unavailable("not detected"),
            revision: HardwareField::unavailable("not detected"),
            vram_mb: HardwareField::unavailable("not detected"),
            is_integrated: HardwareField::unavailable("not detected"),
            pci_location: HardwareField::unavailable("not detected"),
            driver: HardwareField::unavailable("not detected"),
        }
    }
}
