# Security and Safety

## Objectives
Protect user data, prevent accidental disk damage, avoid executing untrusted code, and explain high-impact operations before they occur.

## Native Implementation Safety
**Core Architectural Requirement**: All core functionality is implemented natively in Rust. This eliminates entire classes of vulnerabilities associated with external process execution:
- No shell injection (no subprocess shell invocation)
- No untrusted script execution (no Python scripts downloaded or bundled)
- No PATH manipulation attacks (explicit executable paths only for temporary adapters)
- Memory safety by default (Rust ownership model)
- No arbitrary code execution from malformed upstream output (structured parsing with validation)

Temporary adapters in `integrations` crate are feature-gated, use explicit argument arrays (never shell), validate all inputs/outputs, and have documented removal timelines.

## Physical disks and removable media
Physical-media writing is out of scope for the initial MVP. Before enabling it:
- Enumerate devices using platform APIs (native Rust, no external tools).
- Display stable identity, model, capacity, and mount points where available.
- Require explicit target selection.
- Show a final confirmation naming the device and warning of data loss.
- Re-check device identity immediately before writing.
- Never choose a target automatically based only on removable status.
- Never target the system disk by default.
- Handle cancellation and partial failures.
- Test with disposable media and mocks.

## Privileges
- Run normally without administrator/root privileges.
- Request elevation only for the specific operation that needs it.
- Explain why it is required.
- Do not expose arbitrary shell execution to the frontend.
- Validate all paths and arguments crossing the UI/backend boundary.
- **Native Rust implementations run in-process** — no privilege escalation for hardware discovery, compatibility checking, EFI generation, or USB mapping.

## Downloads and archives
- Use HTTPS where supported and validate redirects.
- Pin expected versions.
- Verify trusted checksums/signatures when available; do not claim verification otherwise.
- Never execute downloaded content automatically.
- Prevent archive path traversal and unexpected writes.
- Use temporary directories and atomic replacement where practical.
- Clean up incomplete downloads safely.

## File safety
- Keep outputs inside a project-owned workspace.
- Preview modifications to user-edited files.
- Back up configuration before regeneration.
- Prevent path traversal and accidental external writes.
- Avoid following untrusted symlinks during cleanup/extraction.

## Privacy and logs
- Do not upload hardware reports by default.
- Avoid collecting unrelated files.
- Do not log secrets or tokens.
- Let users inspect diagnostics before sharing.
- Bound and rotate logs.

## Supply Chain
- Pin dependencies and review licenses.
- Record source/version for generated artifacts.
- Avoid automatic execution of fetched scripts.
- Build releases in CI and publish checksums.
- **Native Rust implementations eliminate supply chain risk from upstream Python tool updates** — behavior is versioned in our Cargo.lock.

## Threat Scenarios
Test wrong USB target selection, device disappearance, modified/truncated downloads, malicious archive paths, malformed upstream output, corrupted manifests, path traversal, invalid UI input, and cancellation during an operation.

**Additional threat scenarios for native implementation**:
- Malformed hardware report JSON (validated on parse)
- Invalid ACPI table data (bounds-checked parsing)
- Resource version mismatch (deterministic validation)
- Adapter subprocess compromise (feature-gated, sandboxed, removal timeline)
