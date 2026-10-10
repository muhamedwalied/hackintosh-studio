# Feature Specification

## Product delivery and lifetime
- Support a portable executable and an installed app package; the user can run the helper on demand.
- The helper is on-demand and exits when its preparation task is complete.
- Keep user-created outputs in the selected project folder after the helper exits, including EFI releases, SMBIOS JSON metadata, and optional backups.
- Require an internet connection every time the helper launches, including when no update is available, because compatibility checks depend on online API data. Block all use while offline.
- Check for application and data updates at launch and label each as `mandatory` or `optional`. A mandatory update blocks use until complete; an optional update can be skipped, but remind the user again at each launch until it is installed.
- For an installed app package, update from within the app. For a portable executable, direct the user to download the new package from the official product site.
- Data updates include compatibility rules and EFI resource metadata; identify each by version and review date before applying it.

## Project dashboard
- Create/open/rename/export projects.
- Show target versions, last operation, validation status, and activity.
- Keep outputs isolated per project.

## Hardware inspection
Candidate fields include CPU, motherboard/system model, GPU, audio/network devices, storage/USB controllers, and host OS/architecture when available.
- Track the source of each field.
- Mark unavailable fields explicitly.
- Let users review and export reports.

## Compatibility advisor
- Select target macOS and OpenCore versions.
- Recommend the highest stable macOS version compatible with all included devices; offer newer choices with device-specific warnings.
- Resolve the latest stable OpenCore version compatible with the selected macOS; allow older compatible versions.
- Evaluate known hardware using versioned rules. Fetch rule updates separately from app releases, and show their version/date before applying them.
- Separate blockers, warnings, and unknowns.
- Explain findings and link sources.
- Treat missing compatibility rules as unsupported until known, and never guarantee boot success based only on a model-name match.
- Let users retry detection, skip a device, or enter details manually when the scanner cannot identify it; if compatibility remains unknown, show it as unsupported.
- Permit skipping an unsupported component only when a compatible alternative can provide its function; otherwise block EFI generation.
- Require a clear risk acknowledgment before building for an OCLP-only macOS path.

## OpenCore / EFI builder
- Create an isolated EFI workspace.
- Show a complete versioned manifest for compatible OpenCore, kext, driver, and other EFI resources; download after one user confirmation.
- Check current resource metadata when a build begins. If sources are unavailable while the internet connection is active, offer a dated cached catalog only after user confirmation. Offline use remains blocked.
- Manage ACPI, Drivers, Kexts, Tools, and configuration references.
- Generate version-aware configuration files.
- Validate syntax, required fields, file paths, and resource consistency.
- Require ACPI tables for builds that need ACPI-dependent patches.
- Generate and display SMBIOS values; store values and a compatibility-relevant hardware fingerprint in a project-level JSON file.
- Reuse existing SMBIOS values automatically when the fingerprint matches. If it differs, let the user reuse existing values or generate new ones; require explicit confirmation to reuse values against a changed fingerprint. Offer timestamped backup when generating a new set.
- Generate one EFI at `Releases/{macOS}-macos/EFI`; after generation, let the user review the result or finish.
- If an EFI already exists for that macOS release, warn the user. Replace only after explicit selection, preserve an optional timestamped backup, and leave the old EFI intact if replacement is not selected.
- Clean up partial downloads/build outputs on failure without changing an existing EFI.
- Do not download macOS installers/Recovery images, map USB ports, or write bootable media in the first release.

## USB mapping (later release)
- Detect controllers/ports where host support allows.
- Explain steps requiring a supported environment or physical testing.
- Preview and validate maps.
- Preserve prior versions.
- Do not present inferred mapping as physically verified.

## macOS installer resources (later release)
- The first release recommends a macOS version but does not download Recovery resources, full installers, or user-provided images.
- Keep future installer downloads separate from EFI resource downloads and bootable-media creation.

## Validation center
Severity: `error`, `warning`, `info`.
Each finding should include a stable rule ID, affected file/resource, explanation, suggested action, and source when applicable.

## Operation manager
- Progress, current step, safe cancellation, and operation logs.
- Distinguish complete, failed, cancelled, and partially completed states.
- Report cleanup/recovery actions after failure.

## User experience
- Modern dark/light desktop UI.
- Keyboard navigation and accessible labels.
- English-first UI structured for future localization and Arabic/RTL.
- No terminal required for supported everyday workflows.

## Future candidates
Diagnostic bundles, EFI diff viewer, update notifications, community-reviewed rule packs, and more platform adapters.
