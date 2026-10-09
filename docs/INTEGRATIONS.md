# Integrations and Upstream Dependencies

## Policy
Reuse upstream projects where technically and legally appropriate. Do not copy code, bundle binaries, or redistribute resources before reviewing the exact license and notices.

Record for each integration:
- canonical repository and exact version/tag/commit;
- license and required notices;
- redistribution status;
- supported platforms and dependencies;
- invocation method and provenance;
- local patches and tests.

## OpenCorePkg
https://github.com/acidanthera/OpenCorePkg

Planned: retrieve pinned releases, follow the documented package layout, validate required files and version consistency. Keep bootloader binaries upstream; do not rewrite the bootloader in Rust. Verify release licensing and authoritative checksums/signatures where available.

## OpCore-Simplify
https://github.com/lzhoang2801/OpCore-Simplify

Planned study: hardware discovery, compatibility logic, and EFI-generation workflow. Identify reusable components and obligations. Prefer a clean Rust implementation of required domain behavior where practical. If invoking an external component, isolate it behind an adapter and document its runtime dependencies. Do not assume all features are cross-platform.

## USBToolBox
https://github.com/USBToolBox/tool

Planned study: USB discovery/mapping workflow and integration feasibility. Verify license and platform limitations. Expose a platform-neutral mapping model. Do not claim identical behavior on Windows and Linux until tested.

## Adapter design
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

Define exact Rust traits after the first concrete integration use cases are tested.

## Subprocess safety
- Use explicit executable paths and argument arrays.
- Never pass untrusted values through a shell.
- Validate paths and cap captured output.
- Apply timeouts and support cancellation where feasible.
- Do not automatically execute downloaded scripts.

## Resource integrity
Pin versions rather than using mutable `latest` URLs. Prefer upstream metadata and authoritative hashes. If integrity cannot be verified, disclose that. Cache by version and verified digest.

## License checklist
- [ ] Read the license at the exact revision.
- [ ] Confirm modification and redistribution rights.
- [ ] Preserve copyright/attribution notices.
- [ ] Include required license texts.
- [ ] Review source-disclosure obligations.
- [ ] Review transitive dependencies and bundled binaries.
- [ ] Re-check when updating versions.
