# Hackintosh Studio

A planned on-demand assistant for preparing Hackintosh EFI on Windows and Linux, built with Rust, Tauri 2, React, TypeScript, and Tailwind CSS. It runs directly from a portable executable or an installed app package; it is not a long-running management suite or background service.

The executable may be temporary, but user-created project data is persistent: EFI releases, SMBIOS metadata, and optional backups remain in the chosen project folder until the user removes them.

> Status: planning/scaffolding. The features below are goals, not implemented functionality.

## Core Architectural Requirement: Native Rust Implementation

**All core functionality integrated from external tools (OpCore-Simplify, Hardware Sniffer, SSDTTime, USBToolBox) must eventually be reimplemented natively in Rust within Hackintosh Studio.** This is not an optional optimization—it is a fundamental design constraint.

- **No permanent dependency on external executables**: The application must not be designed around launching Python scripts, shell scripts, or third-party executables for core operations. Temporary adapters may be used during development but must be documented as migration steps with removal timelines.
- **Research before implementation**: Study each upstream project's functionality, algorithms, edge cases, compatibility rules, and license before implementing its replacement.
- **Incremental migration**: Break work into small, testable modules. Compare Rust implementations against upstream behavior on representative hardware before replacing.
- **OpenCore clarification**: OpenCore itself is an upstream bootloader project. Do not rewrite the OpenCore bootloader in Rust. Implement Hackintosh Studio's management, configuration, validation, and integration logic in Rust, while distributing or obtaining OpenCore components per their licenses.
- **Feature parity tracking**: Maintain a feature-by-feature inventory for every integrated tool (see [Rust Reimplementation Strategy](docs/RUST_REIMPLEMENTATION.md)).

See [Rust Reimplementation Strategy](docs/RUST_REIMPLEMENTATION.md) for the complete migration plan, feature-parity matrix, and phase breakdown.

## Goals
- Inspect host hardware and report unknowns honestly (native Rust implementation).
- Recommend a macOS version compatible with the detected hardware; offer an OCLP path with explicit risk notice where applicable.
- Resolve and download the OpenCore resources needed for EFI generation after showing a versioned manifest and getting user confirmation.
- Generate, validate, review, and export a complete EFI in native Rust.
- Require an internet connection at every launch for update checks and compatibility data access; block all use while offline.
- Label application and data updates as mandatory or optional. Mandatory updates block use; skipped optional updates are shown again at the next launch until installed.

## Non-goals
- Guarantee macOS compatibility with every PC.
- Replace OpenCore's bootloader.
- Silently modify or format disks.
- Promise that every macOS version is available as a ready-made ISO.
- Download macOS installers or Recovery images in the first release.
- Map USB ports or write bootable media in the first release.
- Install a background service or keep the helper running after the task ends.
- Depend permanently on external Python scripts or executables for core functionality.

## Planned stack
- Rust workspace for application logic (native implementations).
- Tauri 2 desktop shell.
- React + TypeScript + Tailwind CSS UI.
- Serde for structured data; Tokio where asynchronous tasks are useful; tracing for diagnostics.

## Planned structure
```text
hackintosh-studio/
├── apps/desktop/             # Tauri + React UI
├── crates/
│   ├── core/                 # Domain types, workflow state
│   ├── hardware/             # Host hardware discovery (native)
│   ├── compatibility/        # Versioned compatibility rules (native)
│   ├── opencore/             # EFI workspace, config, ACPI (native)
│   ├── usb/                  # USB discovery/mapping (native)
│   ├── macos-media/          # Resource discovery/download/import
│   ├── integrations/         # Upstream adapters (temporary, feature-gated)
│   ├── platform/             # Windows/Linux-specific operations
│   └── validation/           # Cross-module checks
├── resources/compatibility/  # Reviewed rule data and provenance
├── tests/                    # Fixtures and integration tests
└── docs/                     # Project specifications
```

## MVP
1. Run on demand as a portable executable or installed app package, with Linux hardware discovery first and platform boundaries ready for Windows support.
2. Scan hardware or import a compatible JSON report; retain unknown and unsupported devices explicitly.
3. Recommend the highest stable macOS version compatible with the included hardware, and a separate OCLP alternative with a risk acknowledgment.
4. Show and confirm a manifest for compatible OpenCore, kext, driver, and other EFI resources before downloading.
5. Generate and validate a complete EFI natively in Rust; ACPI-dependent builds require ACPI tables.
6. Save output under `Releases/{macOS}-macos/EFI`, with confirmation and optional backup before replacing an existing EFI.
7. Keep a project-level SMBIOS JSON profile; compare its hardware fingerprint on later builds and let the user reuse existing values or generate new ones, with optional timestamped backup.
8. Do not download macOS installers/Recovery images, map USB ports, or write bootable media in the first release.
9. Require internet access on every launch, even when no update is available, because hardware compatibility checks use online API data. Check application and data updates at launch; mandatory updates block use, while skipped optional updates are shown again next time. Update an installed app package in place; for a portable build, provide the new package from the official product site.

## Documentation
- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Features](docs/FEATURES.md)
- [Integrations](docs/INTEGRATIONS.md)
- [Security](docs/SECURITY.md)
- [Testing](docs/TESTING.md)
- [Rust Reimplementation Strategy](docs/RUST_REIMPLEMENTATION.md)

## Licensing and platform note
Apple software is subject to Apple's terms and licensing. Before implementing distribution or download features, document applicable terms and use official or otherwise permitted sources. Upstream tools have their own licenses; review and preserve required notices before reusing code or bundling binaries. All native reimplementations must comply with upstream licenses (BSD-3-Clause, MIT) — see [Rust Reimplementation Strategy](docs/RUST_REIMPLEMENTATION.md#licensing-compliance).
