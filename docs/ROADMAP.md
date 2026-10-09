# Roadmap

All items are unstarted at project initialization.

## Phase 0 — Foundation
- [ ] Create Cargo workspace and Tauri desktop shell.
- [ ] Configure formatting, linting, and CI for Windows/Linux.
- [ ] Establish dependency and license review policy.
- [ ] Add typed errors, tracing, and project persistence.
- [ ] Build initial navigation and empty states.

Exit: application builds on both target OSes and opens an empty project.

## Phase 1 — Project and hardware report
- [ ] Define versioned `ProjectManifest`.
- [ ] Implement Windows hardware discovery.
- [ ] Implement Linux hardware discovery.
- [ ] Represent unknown/unavailable values explicitly.
- [ ] Display and export hardware reports.
- [ ] Add fixture tests for partial discovery.

Exit: report is useful without inventing missing values.

## Phase 2 — Compatibility engine
- [ ] Define rule schema and source metadata.
- [ ] Add target macOS/OpenCore selection.
- [ ] Implement deterministic rule evaluation.
- [ ] Display supported/limited/unsupported/unknown findings.
- [ ] Include source links, review dates, and confidence.
- [ ] Add regression fixtures.

Exit: every result is traceable to a reviewed rule or marked unknown.

## Phase 3 — OpenCore workspace
- [ ] Pin and resolve resource versions.
- [ ] Define reproducible EFI layout.
- [ ] Add version-aware configuration templates.
- [ ] Parse and validate plist files.
- [ ] Check references and resource consistency.
- [ ] Preserve/back up user edits.
- [ ] Export EFI to folder/ZIP.
- [ ] Compare outputs against reviewed fixtures.

Exit: deterministic workspace generation without physical disk writes.

## Phase 4 — macOS resources
- [ ] Document permitted, supported sources.
- [ ] Model Recovery, full installers, and imported images separately.
- [ ] Add resource catalog and provenance.
- [ ] Add cancellable downloads and resume where supported.
- [ ] Verify authoritative checksums when available.
- [ ] Detect/report actual file format.
- [ ] Document platform requirements and limitations.

Exit: users can tell what they downloaded and whether it is bootable media.

## Phase 5 — USB mapping and media
- [ ] Define platform capability detection.
- [ ] Review USBToolBox license and integration feasibility.
- [ ] Implement supported device/controller enumeration.
- [ ] Add mapping preview and validation.
- [ ] Review destructive-operation safety model.
- [ ] Add media writing only after tests and safety review.
- [ ] Test on disposable media and supported hardware.

Exit: no write occurs without explicit target confirmation.

## Phase 6 — Release readiness
- [ ] Package for Windows and Linux.
- [ ] Test clean install and upgrade.
- [ ] Add diagnostic export and crash-safe handling.
- [ ] Audit dependencies and upstream notices.
- [ ] Verify accessibility and keyboard navigation.
- [ ] Publish known limitations, release notes, and checksums.

## Prioritization rule
Do not promise automatic compatibility or implement advanced USB writing before hardware reporting, resource provenance, validation, and safety layers are tested.
