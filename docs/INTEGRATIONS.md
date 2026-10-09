# Integrations and Upstream Dependencies

## Policy
Reuse upstream projects where technically and legally appropriate. Do not copy code, bundle binaries, or redistribute resources before reviewing the exact license and notices.

**Core Architectural Requirement**: All core functionality from integrated external tools must eventually be reimplemented natively in Rust. Temporary adapters are permitted during migration but must be feature-gated, documented, and have removal timelines. See [Rust Reimplementation Strategy](RUST_REIMPLEMENTATION.md).

Record for each integration:
- canonical repository and exact version/tag/commit;
- license and required notices;
- redistribution status;
- supported platforms and dependencies;
- invocation method and provenance;
- local patches and tests.
- **migration status**: `adapter` (temporary), `native` (reimplemented), `deprecated` (adapter removed)

## OpenCorePkg
https://github.com/acidanthera/OpenCorePkg

Planned: retrieve pinned releases, follow the documented package layout, validate required files and version consistency. Keep bootloader binaries upstream; do not rewrite the bootloader in Rust. Verify release licensing and authoritative checksums/signatures where available.

**Migration Status**: `native` (Hackintosh Studio manages OpenCore resources; does not reimplement bootloader)

## OpCore-Simplify
https://github.com/lzhoang2801/OpCore-Simplify

**License**: BSD-3-Clause

Planned study: hardware discovery, compatibility logic, and EFI-generation workflow. Identify reusable components and obligations. Prefer a clean Rust implementation of required domain behavior where practical. If invoking an external component, isolate it behind an adapter and document its runtime dependencies. Do not assume all features are cross-platform.

**Migration Status**: `adapter` (temporary) → `native` (phased)
- Hardware discovery → Phase 1 (native in `hardware` crate)
- Compatibility checking → Phase 2 (native in `compatibility` crate)
- ACPI patch selection → Phase 2 (native in `compatibility` crate)
- Kext/SMBIOS selection → Phase 2 (native in `compatibility` crate)
- EFI generation → Phase 4 (native in `opencore` crate)
- Resource download → Phase 5 (native in `macos-media` crate)

**Adapter**: `integrations::opcore_simplify` (feature `adapter-opcore-simplify`)
- Invokes `OpCore-Simplify.py` as subprocess
- Parses JSON output
- **Removal target**: After Phase 4 completion

## Hardware Sniffer
https://github.com/lzhoang2801/Hardware-Sniffer

**License**: BSD-3-Clause

Planned: native Rust hardware discovery for Windows (WMI) and Linux (sysfs/proc). ACPI table dumping. JSON report export compatible with OpCore-Simplify format.

**Migration Status**: `adapter` (temporary) → `native` (Phase 1)
- Hardware collection → Phase 1 (native in `hardware` crate)
- ACPI dumping → Phase 1 (native in `hardware` crate)
- Report export → Phase 1 (native in `hardware` crate)

**Adapter**: `integrations::hardware_sniffer` (feature `adapter-hardware-sniffer`)
- Invokes `Hardware-Sniffer-CLI.py` as subprocess
- Parses JSON output
- **Removal target**: After Phase 1 completion

## SSDTTime
https://github.com/corpnewt/SSDTTime

**License**: MIT

Planned: native Rust SSDT/DSDT hotpatch generation. All common patches (HPET, EC, USBX, PLUG, PMC, AWAC, USB-Reset, Bridge, PNLF, XOSI, DMAR, SBUS-MCHC, IMEI Bridge). DSDT dumping on Windows/Linux.

**Migration Status**: `adapter` (temporary) → `native` (Phase 3)
- SSDT generation → Phase 3 (native in `opencore::acpi` module)
- DSDT dumping → Phase 3 (native in `platform` crate)

**Adapter**: `integrations::ssdttime` (feature `adapter-ssdttime`)
- Invokes `SSDTTime.py` as subprocess
- Parses generated AML files
- **Removal target**: After Phase 3 completion

## USBToolBox
https://github.com/USBToolBox/tool

**License**: MIT

Planned study: USB discovery/mapping workflow and integration feasibility. Verify license and platform limitations. Expose a platform-neutral mapping model. Do not claim identical behavior on Windows and Linux until tested.

**Migration Status**: `adapter` (temporary) → `native` (Phase 6)
- USB discovery → Phase 6 (native in `usb` + `platform` crates)
- Port mapping → Phase 6 (native in `usb` crate)
- Kext generation → Phase 6 (native in `usb` crate)

**Adapter**: `integrations::usbtoolbox` (feature `adapter-usbtoolbox`)
- Invokes `Windows.exe` / `macOS` binary as subprocess
- Parses output
- **Removal target**: After Phase 6 completion

## Adapter Design
Conceptual adapter capabilities:
- ID/name and version;
- supported platforms;
- dependency availability check;
- typed input validation;
- execution with explicit argument arrays;
- structured output parsing;
- timeout/cancellation;
- diagnostics and exit status;
- provenance and license metadata.
- **migration status field** (enum: `Active`, `Deprecated`, `Removed`)

Define exact Rust traits after the first concrete integration use cases are tested. All adapters live in `integrations` crate behind feature flags.

## Subprocess Safety
- Use explicit executable paths and argument arrays.
- Never pass untrusted values through a shell.
- Validate paths and cap captured output.
- Apply timeouts and support cancellation where feasible.
- Do not automatically execute downloaded scripts.

## Resource Integrity
Pin versions rather than using mutable `latest` URLs. Prefer upstream metadata and authoritative hashes. If integrity cannot be verified, disclose that. Cache by version and verified digest.

## License Checklist
- [ ] Read the license at the exact revision.
- [ ] Confirm modification and redistribution rights.
- [ ] Preserve copyright/attribution notices.
- [ ] Include required license texts.
- [ ] Review source-disclosure obligations.
- [ ] Review transitive dependencies and bundled binaries.
- [ ] Re-check when updating versions.
- [ ] Document license compliance for each native reimplementation (see [Rust Reimplementation Strategy](RUST_REIMPLEMENTATION.md#licensing-compliance)).
