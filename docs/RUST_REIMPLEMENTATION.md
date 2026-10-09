# Rust Reimplementation Strategy

## Overview

This document describes the migration strategy for replacing external tool dependencies (OpCore-Simplify, Hardware Sniffer, SSDTTime, USBToolBox) with native Rust implementations within Hackintosh Studio. This is a **core architectural requirement**, not an optional optimization.

## Guiding Principles

1. **Rust-first implementation**: All core functionality from integrated tools must eventually have native Rust implementations.
2. **No permanent dependency on external executables**: Do not design around launching Python scripts, shell scripts, or third-party executables for core operations. Temporary adapters may be used during development but must be documented as migration steps.
3. **Research before implementation**: Study each upstream project's functionality, algorithms, edge cases, compatibility rules, and license before implementing its replacement. Preserve behavior where appropriate and improve when justified.
4. **Incremental migration**: Break work into small, testable modules. Compare Rust implementations against upstream behavior and test on representative hardware before replacing.
5. **OpenCore clarification**: OpenCore itself is an upstream bootloader project. Do not rewrite the entire OpenCore bootloader in Rust. Instead, implement Hackintosh Studio's management, configuration, validation, and integration logic in Rust, while distributing or obtaining OpenCore components according to their licenses.
6. **Feature parity tracking**: Maintain a feature-by-feature inventory for every integrated tool, identifying implemented features, missing features, tests, upstream references, licensing requirements, and migration status.
7. **Platform support**: Keep architecture compatible with Windows and Linux, isolating platform-specific behavior behind Rust interfaces.
8. **Safety and correctness**: Validate generated configurations, protect user data, and require explicit confirmation before destructive operations.

## Upstream Tools Analysis

### 1. OpCore-Simplify (BSD-3-Clause)
**Repository**: https://github.com/lzhoang2801/OpCore-Simplify  
**Language**: Python  
**Key Features**:
- Hardware detection and compatibility checking (via Hardware Sniffer)
- Automatic ACPI patch selection and generation (integrates SSDTTime)
- Kext selection and configuration based on hardware
- SMBIOS selection and customization
- OpenCore EFI building with version-pinned resources
- Automatic updates from Dortania Builds and GitHub releases
- GPU ID spoofing, CPU topology rebuild, SIP configuration
- WiFi profiles for itlwm
- OpenCore Legacy Patcher integration

**Dependencies**: Hardware Sniffer, SSDTTime, OpenCorePkg, Dortania kext builds

### 2. Hardware Sniffer (BSD-3-Clause)
**Repository**: https://github.com/lzhoang2801/Hardware-Sniffer  
**Language**: Python (Windows via WMIC, Linux WIP)  
**Key Features**:
- Comprehensive hardware collection via WMI (Windows)
- Motherboard chipset identification via PCI device details
- CPU codename recognition via Family/Model/Stepping
- GPU codename recognition via device ID
- Input device connection type detection (I2C, PS2, SMBus, USB)
- ACPI table dumping
- JSON report export

**Dependencies**: WMIC (Windows), pci.ids, usb.ids, cpuid.py

### 3. SSDTTime (MIT)
**Repository**: https://github.com/corpnewt/SSDTTime  
**Language**: Python  
**Key Features**:
- SSDT/DSDT hotpatch generation
- Supported patches: HPET, EC, USBX, PLUG, PMC, AWAC, USB-Reset, Bridge, PNLF, XOSI, DMAR, SBUS-MCHC, IMEI Bridge
- DSDT dumping on Linux/Windows
- Cross-platform (macOS, Linux, Windows)

**Dependencies**: iasl (Intel ASL compiler)

### 4. USBToolBox (MIT)
**Repository**: https://github.com/USBToolBox/tool  
**Language**: Python  
**Key Features**:
- USB port discovery and mapping on Windows and macOS
- Companion port binding (Windows)
- Port type guessing (Windows)
- Custom injector kext generation (USBToolBox kext or AppleUSBHostMergeProperties)
- Multiple port matching methods

**Dependencies**: Windows USB APIs, macOS IOKit, USBToolBox kext

## Migration Phases

### Phase A: Foundation & Hardware Discovery (Highest Priority)
**Target**: Replace Hardware Sniffer with native Rust hardware discovery
**Crates**: `hardware`, `platform`
**Rationale**: Hardware discovery is the foundation for all downstream features (compatibility, ACPI, kexts, SMBIOS). It runs on the host OS (Windows/Linux) where we have full control.

**Features to Reimplement**:
- [ ] CPU detection (vendor, model, codename, family/model/stepping, core/thread counts, features)
- [ ] Motherboard detection (vendor, model, chipset via PCI)
- [ ] GPU detection (vendor, model, device ID, codename)
- [ ] Audio device detection
- [ ] Network device detection
- [ ] USB controller detection
- [ ] Storage controller detection
- [ ] Input device detection with connection type
- [ ] Monitor detection
- [ ] ACPI table enumeration and dumping
- [ ] JSON report export (compatible with OpCore-Simplify format)
- [ ] Platform abstraction (Windows WMI, Linux sysfs/proc, future macOS IOKit)

**License Considerations**: BSD-3-Clause (permissive). Can reimplement algorithms. Must preserve copyright notices if adapting code.

**Testing Strategy**: Compare JSON output against Hardware Sniffer on same hardware. Use fixture tests with known hardware reports.

### Phase B: ACPI Patch Generation (SSDTTime Replacement)
**Target**: Replace SSDTTime with native Rust SSDT/DSDT hotpatch generation
**Crates**: `opencore` (new `acpi` submodule or separate crate)
**Rationale**: ACPI patches are deterministic transformations based on hardware data. No external process execution needed.

**Features to Reimplement**:
- [ ] SSDT-HPET (IRQ conflict resolution)
- [ ] SSDT-EC (fake EC for laptop/desktop)
- [ ] SSDT-USBX (USB power properties)
- [ ] SSDT-PLUG (plugin-type on CPU0)
- [ ] SSDT-PMC (PMCR for 300-series NVRAM)
- [ ] SSDT-AWAC (disable AWAC, enable/fake RTC)
- [ ] SSDT-USB-Reset (root hub status)
- [ ] SSDT-Bridge (PCI bridge creation)
- [ ] SSDT-PNLF (backlight control)
- [ ] SSDT-XOSI (_OSI patch)
- [ ] DMAR (reserved memory removal)
- [ ] SSDT-SBUS-MCHC (SMBus compatibility)
- [ ] IMEI Bridge
- [ ] DSDT dumping (platform-specific)
- [ ] AML compilation (iasl replacement or binding)

**License Considerations**: MIT (permissive). Can reimplement freely with attribution.

**Testing Strategy**: Compare generated AML against SSDTTime output for same hardware. Unit test each patch generator.

### Phase C: Compatibility Engine & Kext Selection (OpCore-Simplify Core)
**Target**: Replace OpCore-Simplify's compatibility checking, kext selection, and SMBIOS logic
**Crates**: `compatibility`, `opencore`
**Rationale**: This is the decision engine that maps hardware → configuration. Pure logic, no external deps.

**Features to Reimplement**:
- [ ] Hardware → macOS version compatibility matrix
- [ ] CPU compatibility (Intel: Nehalem→Arrow Lake; AMD: Ryzen/Threadripper via AMD Vanilla)
- [ ] GPU compatibility (Intel iGPU, AMD APU/dGPU, NVIDIA Kepler→Fermi)
- [ ] Automatic kext selection (Lilu, WhateverGreen, AppleALC, VirtualSMC, etc.)
- [ ] ACPI patch selection based on hardware
- [ ] SMBIOS selection and customization (CPU name, board ID, etc.)
- [ ] GPU ID spoofing logic
- [ ] CpuTopologyRebuild for hybrid CPUs
- [ ] SIP configuration presets
- [ ] CPU ID spoofing (Pentium, Celeron, Core, Xeon)
- [ ] Resizable BAR configuration
- [ ] iGPU configuration (headless vs display)
- [ ] VESA mode forcing for Intel GPU
- [ ] OpenCore Legacy Patcher integration config
- [ ] Built-in device properties (network, storage)
- [ ] WiFi profiles for itlwm
- [ ] Version-pinned resource resolution (OpenCore, kexts from Dortania)

**License Considerations**: BSD-3-Clause. Compatibility data is factual; algorithms can be reimplemented. Kext metadata from Dortania Builds needs separate licensing review.

**Testing Strategy**: Fixture-based compatibility evaluation tests. Compare findings against OpCore-Simplify for known hardware reports.

### Phase D: EFI Workspace Generation & Validation
**Target**: Native Rust EFI workspace management, config.plist generation, validation
**Crates**: `opencore`, `validation`
**Rationale**: Core product value - deterministic, validated EFI generation.

**Features to Reimplement**:
- [ ] Isolated EFI workspace creation
- [ ] Version-aware config.plist generation (OpenCore version specific)
- [ ] ACPI, Kexts, Drivers, Tools section management
- [ ] Plist syntax validation
- [ ] Required key validation
- [ ] File reference validation
- [ ] Version consistency checks
- [ ] User edit backup before regeneration
- [ ] Diff preview before apply
- [ ] Export to folder/ZIP
- [ ] Resource integrity verification (checksums)

**Testing Strategy**: Golden file tests for config.plist generation. Validation tests with valid/invalid fixtures.

### Phase E: USB Mapping (USBToolBox Replacement)
**Target**: Replace USBToolBox with native Rust USB discovery and mapping
**Crates**: `usb`, `platform`
**Rationale**: Platform-specific USB APIs. High value for laptop users.

**Features to Reimplement**:
- [ ] USB controller/port enumeration (Windows: SetupAPI/WinUSB; Linux: sysfs/udev)
- [ ] Port discovery and monitoring
- [ ] Companion port detection (Windows)
- [ ] Port type guessing (Windows)
- [ ] Port mapping UI model
- [ ] Custom injector kext generation (USBToolBox kext format)
- [ ] AppleUSBHostMergeProperties format support
- [ ] Port limit handling

**License Considerations**: MIT (permissive). USBToolBox kext is separate - must distribute per its license.

**Testing Strategy**: Mock platform USB APIs. Compare generated kext against USBToolBox output.

### Phase F: macOS Resource Management
**Target**: Native resource catalog, download, verification, import
**Crates**: `macos-media`, `integrations`
**Rationale**: Security-critical (downloads, checksums, media writing).

**Features to Reimplement**:
- [ ] Resource catalog with version pinning
- [ ] Recovery/full installer/user image modeling
- [ ] Cancellable downloads with resume
- [ ] Authoritative checksum verification
- [ ] Format detection
- [ ] Platform-specific media writing (after safety review)

## Feature Parity Matrix

| Feature Area | Upstream Tool | Status | Rust Crate | Priority | Notes |
|--------------|---------------|--------|------------|----------|-------|
| Hardware Detection | Hardware Sniffer | 🔴 Not Started | `hardware` | P0 | Foundation for all else |
| ACPI Dump | Hardware Sniffer | 🔴 Not Started | `hardware` | P0 | Needed for custom patches |
| Hardware Report Export | Hardware Sniffer | 🔴 Not Started | `hardware` | P0 | JSON compatibility |
| SSDT Generation | SSDTTime | 🔴 Not Started | `opencore/acpi` | P1 | Deterministic, testable |
| DSDT Dump | SSDTTime | 🔴 Not Started | `platform` | P1 | Platform-specific |
| Compatibility Check | OpCore-Simplify | 🔴 Not Started | `compatibility` | P1 | Core decision engine |
| Kext Selection | OpCore-Simplify | 🔴 Not Started | `compatibility` | P1 | Data-driven |
| SMBIOS Selection | OpCore-Simplify | 🔴 Not Started | `compatibility` | P1 | Data-driven |
| ACPI Patch Selection | OpCore-Simplify | 🔴 Not Started | `compatibility` | P1 | Rule-based |
| EFI Workspace | OpCore-Simplify | 🔴 Not Started | `opencore` | P1 | Core product |
| Config.plist Generation | OpCore-Simplify | 🔴 Not Started | `opencore` | P1 | Version-aware |
| Validation | OpCore-Simplify | 🔴 Not Started | `validation` | P1 | Cross-module |
| Export Folder/ZIP | OpCore-Simplify | 🔴 Not Started | `opencore` | P1 | MVP requirement |
| Resource Download | OpCore-Simplify | 🔴 Not Started | `macos-media` | P2 | Security-critical |
| Resource Verification | OpCore-Simplify | 🔴 Not Started | `macos-media` | P2 | Checksums/signatures |
| USB Discovery | USBToolBox | 🔴 Not Started | `usb` | P2 | Platform-specific |
| USB Mapping | USBToolBox | 🔴 Not Started | `usb` | P2 | Interactive |
| Kext Generation | USBToolBox | 🔴 Not Started | `usb` | P2 | Injector format |
| Media Writing | OpCore-Simplify/UnPlugged | 🔴 Not Started | `macos-media` | P3 | Safety review required |

**Legend**: 🔴 Not Started | 🟡 In Progress | 🟢 Implemented | 🔵 Testing | ✅ Done

## Crate Boundary Updates

### New/Modified Crates

| Crate | Responsibility | New Modules |
|-------|---------------|-------------|
| `hardware` | Host hardware discovery | `detection`, `report`, `acpi`, `platform` |
| `platform` | OS-specific implementations | `windows`, `linux`, `usb` |
| `compatibility` | Hardware→config rules | `rules`, `kexts`, `smbios`, `acpi_selection` |
| `opencore` | EFI workspace & config | `workspace`, `config`, `resources`, `acpi` |
| `usb` | USB mapping | `discovery`, `mapping`, `kextgen` |
| `macos-media` | Resource management | `catalog`, `download`, `verify`, `media` |
| `integrations` | Upstream adapters (temporary) | `opcore_simplify`, `ssdttime`, `usbtoolbox` |
| `validation` | Cross-module checks | `efi`, `hardware`, `resources` |

### Adapter Crate (Temporary)
`integrations` will contain adapter implementations for each upstream tool during migration. Each adapter:
- Implements a common trait (e.g., `HardwareDetector`, `AcpiGenerator`, `UsbMapper`)
- Is feature-gated behind `adapter-*` features
- Has clear migration status documentation
- Will be removed when native implementation reaches parity

## Licensing Compliance

| Upstream | License | Reimplementation Approach |
|----------|---------|---------------------------|
| OpCore-Simplify | BSD-3-Clause | Clean-room reimplementation of algorithms. Preserve copyright in NOTICE. |
| Hardware Sniffer | BSD-3-Clause | Clean-room reimplementation. Preserve copyright in NOTICE. |
| SSDTTime | MIT | Clean-room reimplementation. Preserve copyright in NOTICE. |
| USBToolBox | MIT | Clean-room reimplementation. Preserve copyright in NOTICE. |
| OpenCorePkg | BSD-3-Clause | Distribute binaries per license. Do not reimplement bootloader. |
| Kexts (Dortania) | Various | Download at runtime. Verify checksums. Do not bundle. |
| pci.ids / usb.ids | Public Domain / GPL | Download at runtime or embed with license compliance. |
| cpuid.py | MIT | Reimplement CPUID decoding in Rust. |

## Migration Tracking Process

For each feature being reimplemented:

1. **Research**: Document upstream behavior, edge cases, test cases
2. **Design**: Define Rust types, traits, and module structure
3. **Implement**: Write native Rust code with unit tests
4. **Validate**: Compare against upstream on fixture hardware reports
5. **Integrate**: Wire into Hackintosh Studio workflow
6. **Test**: End-to-end integration tests
7. **Deprecate**: Mark adapter as deprecated, remove when confident
8. **Document**: Update feature parity matrix and migration status

## Quality Gates

Before marking a feature "native implemented":
- [ ] Unit tests cover all public functions
- [ ] Fixture tests match upstream output for ≥3 hardware configurations
- [ ] Integration test passes in CI (Windows + Linux)
- [ ] Documentation updated
- [ ] License compliance verified
- [ ] Adapter marked deprecated with removal timeline

## First Implementation Target: Hardware Discovery

**Why**: 
- Foundation for all other features
- Pure data collection (no external deps)
- Well-defined input/output (JSON report)
- Platform-specific but isolatable behind traits
- High testability with fixtures
- Immediate value: users get hardware report without Python

**Scope**:
1. Define `HardwareReport` types in `core`
2. Implement `HardwareDetector` trait in `hardware`
3. Implement Windows backend using `wmi` crate in `platform`
4. Implement Linux backend using `sysfs`/`proc` in `platform`
5. Add ACPI table enumeration
6. JSON serialization compatible with OpCore-Simplify format
7. Tauri command for hardware scan
8. Fixture tests with sample reports

**Estimated Effort**: 2-3 weeks for MVP Windows + Linux hardware detection

## Appendix: Upstream Source References

### OpCore-Simplify Key Files
- `OpCore-Simplify.py` - Main entry point
- `Scripts/datasets/kext_data.py` - Kext metadata and versions
- `Scripts/datasets/smbios_data.py` - SMBIOS data
- `Scripts/datasets/acpi_data.py` - ACPI patch definitions
- `Scripts/utils/*.py` - Utility functions

### Hardware Sniffer Key Files
- `HardwareSniffer.py` - Main collection logic
- `Scripts/wmi.py` - WMI queries
- `Scripts/cpuid.py` - CPUID decoding
- `Scripts/pci_ids.py` - PCI ID database
- `Scripts/usb_ids.py` - USB ID database

### SSDTTime Key Files
- `SSDTTime.py` - Main logic
- `Scripts/*.py` - Individual SSDT generators

### USBToolBox Key Files
- `Windows.py` - Windows USB discovery
- `macOS.py` - macOS USB discovery
- `base.py` - Shared logic
- `TYPES.md` - Data structure definitions

---

*This document is a living specification. Update as migration progresses.*