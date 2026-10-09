# Feature Specification

## Project dashboard
- Create/open/rename/export projects.
- Show target versions, last operation, validation status, and activity.
- Keep outputs isolated per project.

## Hardware inspection
Candidate fields include CPU, motherboard/system model, GPU, audio/network devices, storage/USB controllers, and host OS/architecture when available.
- Track the source of each field.
- Mark unavailable fields explicitly.
- Do not upload reports automatically.
- Let users review and export reports.

## Compatibility advisor
- Select target macOS and OpenCore versions.
- Evaluate known hardware using versioned rules.
- Separate blockers, warnings, and unknowns.
- Explain findings and link sources.
- Never guarantee boot success based only on model-name matching.

## OpenCore / EFI builder
- Create an isolated EFI workspace.
- Select pinned OpenCore resources.
- Manage ACPI, Drivers, Kexts, Tools, and configuration references.
- Generate version-aware configuration files.
- Validate syntax, required fields, file paths, and resource consistency.
- Back up user configuration before regeneration.
- Preview diffs before applying changes.
- Export to folder/ZIP.
- Never overwrite an existing EFI without preview and confirmation.

## USB mapping
- Detect controllers/ports where host support allows.
- Explain steps requiring a supported environment or physical testing.
- Preview and validate maps.
- Preserve prior versions.
- Do not present inferred mapping as physically verified.

## macOS resource manager
- Separate Recovery resources, full installers, and user-provided images.
- Show version/build, file type, size if known, source, and verification state.
- Support progress, cancellation, and resume when possible.
- Validate imported/downloaded content.
- Keep resource download separate from bootable-media creation.
- Do not advertise an ISO for every macOS release or call unverified files official.

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
