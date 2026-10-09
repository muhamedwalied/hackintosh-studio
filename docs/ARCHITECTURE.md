# Architecture

## Overview
```text
React + TypeScript UI
        │ Tauri commands/events
        ▼
Desktop command layer
        ▼
Workflow orchestration (core)
  ├── Hardware discovery (native Rust)
  ├── Compatibility rules (native Rust)
  ├── OpenCore / EFI workspace (native Rust)
  ├── USB mapping (native Rust)
  ├── macOS resource manager
  ├── Validation (native Rust)
  └── Platform adapters
        ├── Windows
        ├── Linux
        └── Upstream tools / permitted resource sources (adapters: temporary, feature-gated)
```

The UI must not own compatibility logic or perform privileged operations directly. Tauri commands validate inputs, invoke services, and return typed results.

**Core Architectural Requirement**: All core functionality from integrated external tools (OpCore-Simplify, Hardware Sniffer, SSDTTime, USBToolBox) must be reimplemented natively in Rust. Temporary adapters in `integrations` crate are feature-gated and documented as migration steps with removal timelines. See [Rust Reimplementation Strategy](RUST_REIMPLEMENTATION.md).

## Crate responsibilities

### `core`
Shared identifiers, project manifest, workflow state, typed errors, operation progress, and serialization contracts. Avoid OS-specific APIs.

### `hardware`
**Native Rust implementation** of host hardware discovery. Collect hardware information from platform APIs (Windows WMI, Linux sysfs/proc). Track source and confidence for each field. Distinguish detected, unavailable, and unknown values. Export JSON reports compatible with OpCore-Simplify format. ACPI table enumeration and dumping.

### `compatibility`
**Native Rust implementation** of versioned compatibility rules. Evaluate rules against a hardware report and target macOS release. Every rule includes source, review date, applicability, and confidence. Results: `supported`, `limited`, `unsupported`, or `unknown`. Kext selection, SMBIOS selection, ACPI patch selection logic.

### `opencore`
**Native Rust implementation** of EFI workspace management. Manage isolated EFI workspaces; resolve pinned OpenCore resources; generate version-aware configuration; validate plist syntax, required keys, referenced files, and version consistency. Back up user edits before regeneration. ACPI patch generation (SSDTTime replacement). Do not implement a replacement bootloader.

### `usb`
**Native Rust implementation** of USB discovery and mapping. Define platform-neutral controller/port/mapping models. Report host limitations. Companion port detection and port type guessing on Windows. Custom injector kext generation (USBToolBox kext and AppleUSBHostMergeProperties formats). Do not claim physical behavior was tested if it was inferred.

### `macos-media`
Model Recovery resources, full installers, and user-provided images separately. Track source, version/build, size if known, checksum if authoritative, and download state. Keep downloading separate from creating bootable media. Cancellable downloads with resume. Authoritative checksum verification.

### `integrations`
**Temporary adapters for upstream tools during migration**. Track upstream tool versions, licenses, notices, provenance, and adapters. Isolate subprocesses; never concatenate untrusted shell strings. Each adapter is feature-gated (`adapter-opcore-simplify`, `adapter-ssdttime`, `adapter-usbtoolbox`) and has a documented removal timeline. See [Rust Reimplementation Strategy](RUST_REIMPLEMENTATION.md#migration-tracking-process).

### `platform`
Windows/Linux implementations for hardware discovery, removable-device enumeration, filesystem behavior, USB discovery, and least-privilege operations. Platform-specific backends for `hardware` and `usb` crates.

### `validation`
Cross-module checks. Report errors, warnings, and information separately. Validation should be deterministic for identical inputs and pinned resources. EFI validation, hardware report validation, resource integrity validation.

## Data flow
1. Create a project with a unique ID.
2. Collect a hardware report (native Rust `hardware` crate).
3. Select target macOS and OpenCore versions.
4. Evaluate compatibility rules and show sources (native Rust `compatibility` crate).
5. Resolve version-pinned resources.
6. Generate EFI in a project-owned directory (native Rust `opencore` crate).
7. Validate and display findings (native Rust `validation` crate).
8. Export to folder/ZIP.
9. Add physical-media writing only after safety controls and tests exist.

## Suggested domain types
`ProjectManifest`, `HardwareReport`, `HardwareComponent`, `TargetConfiguration`, `CompatibilityFinding`, `ResourceDescriptor`, `ResourceProvenance`, `EfiWorkspace`, `ValidationReport`, `OperationId`, `OperationProgress`, `PlatformCapability`, `StudioError`, `AcpiPatch`, `UsbPortMap`, `KextDescriptor`, `SmbiosProfile`.

Use enums for known states and represent missing data explicitly.

## Tauri command groups (proposals)
- `project_create`, `project_open`, `project_export`
- `hardware_scan`, `hardware_report_get`
- `compatibility_evaluate`
- `resource_catalog_list`, `resource_download`, `resource_import`
- `efi_build`, `efi_validate`
- `usb_devices_list`, `usb_mapping_preview`, `usb_mapping_build`

Avoid exposing arbitrary shell execution to the frontend.

## Persistence and errors
Use atomic writes for manifests/configuration where practical. Keep generated output in project-owned directories. Back up user edits before regeneration. Errors should identify the failed stage, retryability, required user action, and whether files/devices may have changed.
