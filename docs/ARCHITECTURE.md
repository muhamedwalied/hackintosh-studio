# Architecture

## Overview
```text
React + TypeScript UI
        │ Tauri commands/events
        ▼
Desktop command layer
        ▼
Workflow orchestration (core)
  ├── Hardware discovery
  ├── Compatibility rules
  ├── OpenCore / EFI workspace
  ├── USB mapping
  ├── macOS resource manager
  ├── Validation
  └── Platform adapters
        ├── Windows
        ├── Linux
        └── Upstream tools / permitted resource sources
```

The UI must not own compatibility logic or perform privileged operations directly. Tauri commands validate inputs, invoke services, and return typed results.

## Crate responsibilities

### `core`
Shared identifiers, project manifest, workflow state, typed errors, operation progress, and serialization contracts. Avoid OS-specific APIs.

### `hardware`
Collect host hardware information from supported APIs/tools. Track source and confidence for each field. Distinguish detected, unavailable, and unknown values.

### `compatibility`
Evaluate versioned rules against a hardware report and target macOS release. Every rule should include source, review date, applicability, and confidence. Results: `supported`, `limited`, `unsupported`, or `unknown`.

### `opencore`
Manage isolated EFI workspaces; resolve pinned OpenCore resources; generate version-aware configuration; validate plist syntax, required keys, referenced files, and version consistency. Back up user edits before regeneration. Do not implement a replacement bootloader.

### `usb`
Define platform-neutral controller/port/mapping models. Report host limitations. Integrate upstream logic only where technically feasible and license-compatible. Do not claim physical behavior was tested if it was inferred.

### `macos-media`
Model Recovery resources, full installers, and user-provided images separately. Track source, version/build, size if known, checksum if authoritative, and download state. Keep downloading separate from creating bootable media.

### `integrations`
Track upstream tool versions, licenses, notices, provenance, and adapters. Isolate subprocesses; never concatenate untrusted shell strings.

### `platform`
Windows/Linux implementations for hardware discovery, removable-device enumeration, filesystem behavior, and least-privilege operations.

### `validation`
Cross-module checks. Report errors, warnings, and information separately. Validation should be deterministic for identical inputs and pinned resources.

## Data flow
1. Create a project with a unique ID.
2. Collect a hardware report.
3. Select target macOS and OpenCore versions.
4. Evaluate compatibility rules and show sources.
5. Resolve version-pinned resources.
6. Generate EFI in a project-owned directory.
7. Validate and display findings.
8. Export to folder/ZIP.
9. Add physical-media writing only after safety controls and tests exist.

## Suggested domain types
`ProjectManifest`, `HardwareReport`, `HardwareComponent`, `TargetConfiguration`, `CompatibilityFinding`, `ResourceDescriptor`, `ResourceProvenance`, `EfiWorkspace`, `ValidationReport`, `OperationId`, `OperationProgress`, `PlatformCapability`, `StudioError`.

Use enums for known states and represent missing data explicitly.

## Tauri command groups (proposals)
- `project_create`, `project_open`, `project_export`
- `hardware_scan`, `hardware_report_get`
- `compatibility_evaluate`
- `resource_catalog_list`, `resource_download`, `resource_import`
- `efi_build`, `efi_validate`
- `usb_devices_list`, `usb_mapping_preview`

Avoid exposing arbitrary shell execution to the frontend.

## Persistence and errors
Use atomic writes for manifests/configuration where practical. Keep generated output in project-owned directories. Back up user edits before regeneration. Errors should identify the failed stage, retryability, required user action, and whether files/devices may have changed.
