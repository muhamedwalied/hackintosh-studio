# Testing Strategy

## Principles
- Unit-test business logic without physical hardware.
- Use fixtures for hardware reports, configuration files, and resource metadata.
- Mock platform and disk operations.
- Never write to physical disks in normal CI.
- Test Windows and Linux.
- Represent unknown/unsupported behavior explicitly.
- **Compare native Rust implementations against upstream tool output** for regression detection.

## Unit Tests
Test parsing/serialization, state transitions, compatibility evaluation, resource metadata, plist validation, path/archive safety, cancellation, error mapping, ACPI patch generation, USB mapping logic, hardware report construction.

## Fixture Tests
Include sanitized fixtures for:
- complete/partial hardware reports (from real hardware, anonymized);
- unknown device IDs;
- supported/limited/unsupported findings;
- valid/invalid plist files;
- missing Kexts/Drivers;
- mismatched resource versions;
- incomplete downloads and checksum failures;
- corrupted or unsupported project manifests.
- **Upstream tool output fixtures**: Hardware Sniffer JSON, OpCore-Simplify compatibility results, SSDTTime AML output, USBToolBox port maps.

Fixtures must not include personal serial numbers, UUIDs, MAC addresses, or other sensitive identifiers.

## Reimplementation Validation Tests
For each native Rust reimplementation:
1. **Golden file tests**: Compare native output against upstream tool output for identical inputs (hardware reports, ACPI tables, etc.)
2. **Cross-platform parity tests**: Verify Windows and Linux native implementations produce equivalent logical results
3. **Regression fixtures**: Lock upstream tool versions; run both native and adapter paths in CI; fail on divergence
4. **Property-based tests**: For deterministic algorithms (compatibility rules, ACPI patch generation), test with generated inputs

Example validation flow for hardware discovery:
```rust
#[test]
fn hardware_report_matches_hardware_sniffer() {
    let fixture = load_fixture("hardware_report_i7_12700k.json");
    let upstream = run_hardware_sniffer_adapter(&fixture.input);
    let native = hardware::detect(&fixture.input);
    assert_eq!(normalize(upstream), normalize(native));
}
```

## Integration Tests
- Create and reopen a project.
- Generate an EFI workspace from fixtures.
- Validate and export it.
- Import a local resource and report detected format.
- Simulate interrupted downloads and retry.
- Verify cancellation and error reporting.
- Assert that default test runs never write to physical devices.
- **Run both native and adapter paths; compare results**

## Platform CI
Run on Windows and Linux for:
- build;
- formatting and linting;
- unit/fixture tests;
- frontend typecheck/lint/build;
- packaging smoke tests where supported.
- **reimplementation validation tests** (both native and adapter paths)

Hardware-specific tests must be clearly labeled and run only on known test machines.

## Suggested Quality Gates
```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace --features adapter-opcore-simplify,adapter-hardware-sniffer,adapter-ssdttime,adapter-usbtoolbox
```
Configure frontend checks after selecting the package manager. Commands may need adjustment as the workspace evolves.

## MVP Acceptance Criteria
- Identical fixture inputs yield identical compatibility findings.
- Every finding has a source or is explicitly unknown.
- Generated config passes syntax and reference checks.
- Export does not mutate physical disks.
- Cancelled downloads are never marked verified.
- Errors identify the failed stage and safe next step.
- CI results are available for both target platforms.
- **Native hardware detection matches Hardware Sniffer output for ≥3 fixture configurations**
- **Native ACPI patch generation matches SSDTTime output for ≥3 fixture configurations**
- **Native compatibility engine matches OpCore-Simplify findings for ≥3 fixture configurations**
