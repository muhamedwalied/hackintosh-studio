# Hackintosh Studio

A planned cross-platform desktop workbench for Hackintosh preparation on Windows and Linux, built with Rust, Tauri 2, React, TypeScript, and Tailwind CSS.

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
- Manage versioned OpenCore resources and generate an EFI workspace (native Rust implementation).
- Validate configuration files and resource consistency (native Rust implementation).
- Integrate supported USB-mapping workflows (native Rust implementation).
- Download/import macOS resources from documented, permitted sources.
- Prepare media only after explicit target-device confirmation.

## Non-goals
- Guarantee macOS compatibility with every PC.
- Replace OpenCore's bootloader.
- Silently modify or format disks.
- Promise that every macOS version is available as a ready-made ISO.
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
1. App shell builds on Windows and Linux.
2. Project creation and hardware report export (native Rust hardware detection).
3. Versioned OpenCore resource selection.
4. EFI workspace generation and configuration validation (native Rust).
5. Compatibility findings with sources (native Rust compatibility engine).
6. A documented Recovery-resource workflow, subject to source/platform constraints.
7. EFI export to a folder or ZIP without physical disk writes.

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
