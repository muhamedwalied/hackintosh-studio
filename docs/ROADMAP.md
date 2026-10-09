# Roadmap

All items are unstarted at project initialization.

## Phase 0 — Foundation
- [ ] Create Cargo workspace and Tauri desktop shell.
- [ ] Configure formatting, linting, and CI for Windows/Linux.
- [ ] Establish dependency and license review policy.
- [ ] Add typed errors, tracing, and project persistence.
- [ ] Build initial navigation and empty states.

Exit: application builds on both target OSes and opens an empty project.

## Phase 1 — Project and Hardware Report (Native Rust)
- [ ] Define versioned `ProjectManifest`.
- [ ] Implement **native Rust** Windows hardware discovery (WMI-based).
- [ ] Implement **native Rust** Linux hardware discovery (sysfs/proc-based).
- [ ] Implement ACPI table enumeration and dumping.
- [ ] Represent unknown/unavailable values explicitly.
- [ ] Display and export hardware reports (JSON compatible with OpCore-Simplify).
- [ ] Add fixture tests for partial discovery.

Exit: report is useful without inventing missing values; no Python dependency for hardware detection.

## Phase 2 — Compatibility Engine (Native Rust)
- [ ] Define rule schema and source metadata.
- [ ] Add target macOS/OpenCore selection.
- [ ] Implement deterministic rule evaluation (native Rust).
- [ ] Implement kext selection logic (native Rust).
- [ ] Implement SMBIOS selection logic (native Rust).
- [ ] Implement ACPI patch selection logic (native Rust).
- [ ] Display supported/limited/unsupported/unknown findings.
- [ ] Include source links, review dates, and confidence.
- [ ] Add regression fixtures.

Exit: every result is traceable to a reviewed rule or marked unknown; no Python dependency for compatibility checking.

## Phase 3 — ACPI Patch Generation (Native Rust, SSDTTime Replacement)
- [ ] Implement SSDT-HPET, SSDT-EC, SSDT-USBX, SSDT-PLUG generators.
- [ ] Implement SSDT-PMC, SSDT-AWAC, SSDT-USB-Reset generators.
- [ ] Implement SSDT-Bridge, SSDT-PNLF, SSDT-XOSI generators.
- [ ] Implement DMAR, SSDT-SBUS-MCHC, IMEI Bridge generators.
- [ ] DSDT dumping via platform crate.
- [ ] AML compilation (iasl binding or native).
- [ ] Unit tests comparing output against SSDTTime fixtures.

Exit: all common ACPI patches generated natively; no Python dependency for ACPI patches.

## Phase 4 — OpenCore Workspace (Native Rust)
- [ ] Pin and resolve resource versions.
- [ ] Define reproducible EFI layout.
- [ ] Add version-aware configuration templates.
- [ ] Parse and validate plist files.
- [ ] Check references and resource consistency.
- [ ] Preserve/back up user edits.
- [ ] Export EFI to folder/ZIP.
- [ ] Compare outputs against reviewed fixtures.

Exit: deterministic workspace generation without physical disk writes; no Python dependency for EFI generation.

## Phase 5 — macOS Resources
- [ ] Document permitted, supported sources.
- [ ] Model Recovery, full installers, and imported images separately.
- [ ] Add resource catalog and provenance.
- [ ] Add cancellable downloads and resume where supported.
- [ ] Verify authoritative checksums when available.
- [ ] Detect/report actual file format.
- [ ] Document platform requirements and limitations.

Exit: users can tell what they downloaded and whether it is bootable media.

## Phase 6 — USB Mapping (Native Rust, USBToolBox Replacement)
- [ ] Define platform capability detection.
- [ ] Implement Windows USB discovery (SetupAPI/WinUSB).
- [ ] Implement Linux USB discovery (sysfs/udev).
- [ ] Implement companion port detection (Windows).
- [ ] Implement port type guessing (Windows).
- [ ] Add mapping preview and validation.
- [ ] Implement custom injector kext generation (USBToolBox kext format).
- [ ] Implement AppleUSBHostMergeProperties format support.
- [ ] Review destructive-operation safety model.
- [ ] Add media writing only after tests and safety review.
- [ ] Test on disposable media and supported hardware.

Exit: no write occurs without explicit target confirmation; no Python dependency for USB mapping.

## Phase 7 — Release Readiness
- [ ] Package for Windows and Linux.
- [ ] Test clean install and upgrade.
- [ ] Add diagnostic export and crash-safe handling.
- [ ] Audit dependencies and upstream notices.
- [ ] Verify accessibility and keyboard navigation.
- [ ] Publish known limitations, release notes, and checksums.
- [ ] Remove all temporary adapter code (integrations crate adapters).

## Prioritization Rule
Do not promise automatic compatibility or implement advanced USB writing before hardware reporting, resource provenance, validation, and safety layers are tested.

**Migration Rule**: Each phase must deliver native Rust implementations. Temporary adapters in `integrations` crate are feature-gated and have documented removal timelines. See [Rust Reimplementation Strategy](RUST_REIMPLEMENTATION.md).
