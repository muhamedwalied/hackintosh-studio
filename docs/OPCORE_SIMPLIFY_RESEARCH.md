# OpCore-Simplify Research

Research snapshot: 2026-10-10. This note describes the upstream workflow as a reference for Hackintosh Studio's first release. It does not propose copying upstream source code.

## How OpCore-Simplify works

1. **Get a hardware report.** It accepts an existing JSON report, validates the report schema, and on Windows can invoke Hardware Sniffer to export a report and ACPI tables. The report includes processor, graphics, motherboard/BIOS, network, storage, audio, USB and other device information. Exact device IDs are important to compatibility matching. [Report selection and validation](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/OpCore-Simplify.py#L1194-L1307), [Hardware Sniffer features and report workflow](https://github.com/lzhoang2801/Hardware-Sniffer#features).

2. **Check compatibility by component.** The checker evaluates CPU, GPU, audio, network, storage, Bluetooth and SD controllers against curated rules. It calculates native compatibility bounds and a separate OpenCore Legacy Patcher (OCLP) range. [Compatibility checker](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/compatibility_checker.py#L61-L389).

3. **Recommend a macOS version.** The suggested native target begins at the highest supported macOS version and is lowered to satisfy each relevant component's maximum version. The project handles OCLP-patched support as a separate route and presents a warning about its limitations before continuing. The user may choose a different target. [Version suggestion and selection](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/OpCore-Simplify.py#L1351-L1435), [OCLP warning](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/OpCore-Simplify.py#L1309-L1349).

4. **Resolve resources for the selected target.** Kext metadata includes compatibility ranges, dependencies, conflicts, and upstream locations. Resource resolution uses Dortania Builds metadata where available and GitHub releases as a fallback; downloads are tracked with version/source information and integrity data. This is a changing catalog rather than a fixed list of files. [Kext metadata](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/datasets/kext_data.py), [resource resolution and download handling](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/gathering_files.py).

5. **Network behavior.** The current updater retrieves the latest commit information from GitHub. If that request fails, it asks whether to skip the update check; EFI creation later downloads the selected OpenCore and kext resources. So OpCore-Simplify depends on internet for updates and EFI resources, but its updater does not itself hard-block every launch offline. Hackintosh Studio has a stricter product requirement: require internet at every launch because hardware compatibility checks use online API data, and block all use while offline, even if there is no update. [Updater behavior](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/updater.py#L642-L656), [updater offline fallback](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/updater.py#L760-L793), [resource downloads](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/OpCore-Simplify.py#L287-L296).

## Linux-first implications

Hardware Sniffer's README says Linux support is still a work in progress. OpCore-Simplify can import a JSON report, but its integrated Hardware Sniffer collection path is Windows-only. Therefore Hackintosh Studio should implement a native Linux collector behind a platform abstraction, while keeping the report model portable for a future Windows backend. [Hardware Sniffer Linux status](https://github.com/lzhoang2801/Hardware-Sniffer#q-a), [integrated collection path](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/gathering_files.py#L276-L305).

The user has since expanded Hackintosh Studio's first-release scope to include EFI generation like OpCore-Simplify. The upstream generation flow is substantially broader than recommendations: choose/customize hardware, select ACPI patches, resolve and download compatible kexts, choose SMBIOS, generate version-aware `config.plist`, assemble a complete `EFI` folder, and export it. ACPI table input is used by parts of the patch workflow. [EFI build flow](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/OpCore-Simplify.py#L180-L280), [ACPI patch selection](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/acpi_guru.py#L3095-L3201), [SMBIOS selection](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/smbios.py#L83-L218), [configuration generation](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/config_prodigy.py#L541-L688).

The first release will not download macOS itself. It will resolve and download the OpenCore/EFI resources after presenting the complete file manifest and receiving one user confirmation.

USB port mapping is not completed by OpCore-Simplify itself; its generated EFI contains a placeholder and the user follows a separate USBToolBox step. This is an explicit scope choice still to settle for Hackintosh Studio. [Upstream post-build USB instructions](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/OpCore-Simplify.py#L302-L326).

## Compatibility behavior to decide

Upstream marks devices Unsupported when it has no matching rule. Some critical conditions, such as no compatible GPU or storage path, stop the compatibility flow; other peripheral incompatibilities can be reported while evaluation continues. Hackintosh Studio should express this distinction in its own rules rather than treating every unknown device as either a hard failure or as harmless. Unknown devices and raw IDs should remain visible so users can identify missing compatibility data.

## Licensing note

OpCore-Simplify and Hardware Sniffer identify their code as BSD-3-Clause. Any adaptation or distribution still needs license and notice review; third-party resources, datasets, kexts, and bootloader components can have separate terms. Treat upstream behavior as a reference and review each dependency before shipping it. [OpCore-Simplify license](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/LICENSE), [Hardware Sniffer license](https://github.com/lzhoang2801/Hardware-Sniffer/blob/main/LICENSE).

## Primary sources

- [OpCore-Simplify repository](https://github.com/lzhoang2801/OpCore-Simplify)
- [Hardware Sniffer repository](https://github.com/lzhoang2801/Hardware-Sniffer)
- [Compatibility checker source](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/compatibility_checker.py)
- [Kext catalog source](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/datasets/kext_data.py)
- [Resource collection and download source](https://github.com/lzhoang2801/OpCore-Simplify/blob/main/Scripts/gathering_files.py)
