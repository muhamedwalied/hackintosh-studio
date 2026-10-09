# Hackintosh Studio

A planned cross-platform desktop workbench for Hackintosh preparation on Windows and Linux, built with Rust, Tauri 2, React, TypeScript, and Tailwind CSS.

> Status: planning/scaffolding. The features below are goals, not implemented functionality.

## Goals
- Inspect host hardware and report unknowns honestly.
- Manage versioned OpenCore resources and generate an EFI workspace.
- Validate configuration files and resource consistency.
- Integrate supported USB-mapping workflows.
- Download/import macOS resources from documented, permitted sources.
- Prepare media only after explicit target-device confirmation.

## Non-goals
- Guarantee macOS compatibility with every PC.
- Replace OpenCore's bootloader.
- Silently modify or format disks.
- Promise that every macOS version is available as a ready-made ISO.

## Planned stack
- Rust workspace for application logic.
- Tauri 2 desktop shell.
- React + TypeScript + Tailwind CSS UI.
- Serde for structured data; Tokio where asynchronous tasks are useful; tracing for diagnostics.

## Planned structure
```text
hackintosh-studio/
├── apps/desktop/             # Tauri + React UI
├── crates/
│   ├── core/                 # Domain types, workflow state
│   ├── hardware/             # Host hardware discovery
│   ├── compatibility/        # Versioned compatibility rules
│   ├── opencore/             # EFI workspace and config validation
│   ├── usb/                  # USB discovery/mapping abstractions
│   ├── macos-media/          # Resource discovery/download/import
│   ├── integrations/         # Upstream adapters and metadata
│   ├── platform/             # Windows/Linux-specific operations
│   └── validation/           # Cross-module checks
├── resources/compatibility/  # Reviewed rule data and provenance
├── tests/                    # Fixtures and integration tests
└── docs/                     # Project specifications
```

## MVP
1. App shell builds on Windows and Linux.
2. Project creation and hardware report export.
3. Versioned OpenCore resource selection.
4. EFI workspace generation and configuration validation.
5. Compatibility findings with sources.
6. A documented Recovery-resource workflow, subject to source/platform constraints.
7. EFI export to a folder or ZIP without physical disk writes.

## Documentation
- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Features](docs/FEATURES.md)
- [Integrations](docs/INTEGRATIONS.md)
- [Security](docs/SECURITY.md)
- [Testing](docs/TESTING.md)

## Licensing and platform note
Apple software is subject to Apple's terms and licensing. Before implementing distribution or download features, document applicable terms and use official or otherwise permitted sources. Upstream tools have their own licenses; review and preserve required notices before reusing code or bundling binaries.
