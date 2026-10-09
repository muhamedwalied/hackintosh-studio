# Testing Strategy

## Principles
- Unit-test business logic without physical hardware.
- Use fixtures for hardware reports, configuration files, and resource metadata.
- Mock platform and disk operations.
- Never write to physical disks in normal CI.
- Test Windows and Linux.
- Represent unknown/unsupported behavior explicitly.

## Unit tests
Test parsing/serialization, state transitions, compatibility evaluation, resource metadata, plist validation, path/archive safety, cancellation, and error mapping.

## Fixture tests
Include sanitized fixtures for:
- complete/partial hardware reports;
- unknown device IDs;
- supported/limited/unsupported findings;
- valid/invalid plist files;
- missing Kexts/Drivers;
- mismatched resource versions;
- incomplete downloads and checksum failures;
- corrupted or unsupported project manifests.

Fixtures must not include personal serial numbers, UUIDs, MAC addresses, or other sensitive identifiers.

## Integration tests
- Create and reopen a project.
- Generate an EFI workspace from fixtures.
- Validate and export it.
- Import a local resource and report detected format.
- Simulate interrupted downloads and retry.
- Verify cancellation and error reporting.
- Assert that default test runs never write to physical devices.

## Platform CI
Run on Windows and Linux for:
- build;
- formatting and linting;
- unit/fixture tests;
- frontend typecheck/lint/build;
- packaging smoke tests where supported.

Hardware-specific tests must be clearly labeled and run only on known test machines.

## Suggested quality gates
```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
Configure frontend checks after selecting the package manager. Commands may need adjustment as the workspace evolves.

## MVP acceptance criteria
- Identical fixture inputs yield identical compatibility findings.
- Every finding has a source or is explicitly unknown.
- Generated config passes syntax and reference checks.
- Export does not mutate physical disks.
- Cancelled downloads are never marked verified.
- Errors identify the failed stage and safe next step.
- CI results are available for both target platforms.
