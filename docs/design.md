# Product Design and Visual Identity

## Status

This document records the product and interface decisions agreed so far, plus the visual design system created in Google Stitch. It is a design reference for future UI work; it does not describe an implemented interface.

- Product name for now: **Hackintosh Studio** (temporary name).
- Product form: a small, one-time-use desktop utility for people preparing Hackintosh systems, including repair shops. It is not intended to stay installed as a long-running background service.
- First target platform: Linux, because that is the development environment in use. Keep the product architecture ready to expand to other desktop platforms.
- First version: a hardware compatibility assessment and EFI preparation assistant. It does not download macOS installers.
- Website: not required for the first release; the desktop application can be developed and released independently.
- Language: English first. Keep layouts and typography suitable for future Arabic and right-to-left support.
- Privacy: no privacy-specific feature is required for this product as currently scoped. This is a product-scope decision, not a claim that the app collects or transmits no data; network and API behavior must still be described accurately in the UI and technical documentation.

## Product experience decisions

### Guided flow

- Open directly into a welcome step; there is no separate home page in the first version.
- Use a step-by-step wizard with a clear Back action so users can return to the previous step.
- Make the current step and next action obvious. Keep explanations close to the relevant decision.
- After EFI generation, offer **Review** and **Continue** choices; the user decides whether to inspect the result.
- The future final step may become downloading the selected macOS installer. Installer downloading is explicitly out of scope for v1.

### Compatibility and macOS recommendations

- Detect hardware and recommend compatible macOS releases and matching resources. Show the recommendation and file links before any download action; downloads require user approval.
- Select the newest release compatible with all recognized components by default, while allowing the user to change it.
- When useful, present an older release supported natively and a newer release that depends on OCLP, with a clear warning about the risks of the OCLP path.
- Distinguish essential components, such as GPU and storage, from peripheral components. An essential incompatibility can block a recommendation; a peripheral incompatibility should be explained as a warning with an explicit choice to continue.
- If a component cannot be detected, explain what is unknown and offer retry, skip, or manual entry. Manual entry is for unknown detection only. A detected component with unknown or unsupported compatibility receives a warning and an informed continue/stop choice rather than being treated as undetected.
- If the compatibility data has no match for a component, report it as unsupported/unknown according to the result; do not imply that it works.

### EFI generation and files

- Create the generated EFI under `Releases/{version}-macos`.
- If that release already has an EFI, tell the user and require an explicit replace choice before deleting/replacing it.
- Backups are optional. When made, use a name containing the creation timestamp and ask before making a backup copy.
- Store the SMBIOS choice and relevant hardware fingerprint in a JSON file created during the first SMBIOS setup. Reuse existing SMBIOS resources only when the recorded hardware still matches the compatibility check.
- Let the user choose whether to reuse a valid existing SMBIOS resource or download it again. If they request replacement, offer an optional backup and ask for consent before copying it.
- Warn clearly before any action that replaces an existing EFI or other user data.

### Network and updates

- On launch, check whether an application update is available and identify it as **Required** or **Optional**.
- A required update prevents use until the user updates, either by downloading a newer app release from the app website or by using an in-app updater when the application was installed in a supported package format.
- Optional updates may be skipped; these include small bug fixes and UI improvements that do not block the main workflow.
- The app requires an internet connection for its online compatibility data/API checks. If offline, explain that the workflow cannot continue, including when there is no app update pending.
- Show update and connection status in plain language and provide a clear next action.

## Brand direction

- Personality: precise, helpful, transparent, calm, and approachable to first-time users while retaining useful technical detail for advanced users.
- Visual direction: restrained, modern technical utility with a diagnostic-lab feel. Favor clear hierarchy, tonal surfaces, and functional desktop ergonomics over decorative effects or web trends.
- Use one brand accent: Signal Blue. Amber, green, and red belong to semantic statuses, not decoration.
- Follow the operating system's light/dark preference where available.
- Logo direction, if revisited: an original geometric mark using two or three layered paths that converge into a boot/start route; legible at app-icon size; no Apple mark, hardware-company imitation, or implied Apple affiliation. **No logo has been generated or approved yet.**

## Stitch design system

The design system was created in the private Google Stitch project **Hackintosh Studio** on 2026-10-10.

- Stitch project: `projects/6712922669637128072`
- Design system asset: `assets/5f9a9f40c4274196b73dac18e0d77f06`
- Welcome-flow screen instance: `projects/6712922669637128072/screens/11139377571668710575` (390 × 884)
- The design-system instance is 960 × 540.
- The shared palette sheet shown in Stitch is the source for the tokens below. Token names are documented here so implementation can use stable semantic names rather than hard-coded colors scattered through the UI.

### Dark appearance

| Token | Hex | Use |
|---|---|---|
| Graphite Canvas | `#10151B` | Main application canvas |
| Deep Panel | `#171E26` | Primary content surface |
| Raised Panel | `#202A35` | Elevated section, used sparingly |
| Inset Surface | `#0C1117` | Recessed fields and technical snippets |
| Primary Text | `#E8EEF4` | Headings and primary copy |
| Secondary Text | `#A8B4C0` | Supporting copy and metadata |
| Subtle Border | `#33414E` | Control boundaries and grouping |
| Signal Blue | `#236FB5` | Primary buttons and active step |
| Blue Focus | `#7DB8F0` | Keyboard focus and links on dark surfaces |
| Warning Amber | `#F2B84B` | Compatibility cautions |
| Success Green | `#56B887` | Confirmed compatible/completed state |
| Critical Red | `#E56B6F` | Blockers and destructive-action warnings |

### Light appearance

| Token | Hex | Use |
|---|---|---|
| Mist Canvas | `#F3F6F8` | Main application canvas |
| White Surface | `#FFFFFF` | Primary content surface |
| Soft Raised Surface | `#E9EEF3` | Secondary panels and grouped content |
| Primary Text | `#17232E` | Headings and primary copy |
| Secondary Text | `#536474` | Supporting copy and metadata |
| Subtle Border | `#D2DCE5` | Control boundaries and grouping |
| Signal Blue | `#1D68AA` | Primary buttons and active step |
| Blue Focus | `#155A98` | Keyboard focus and links |
| Warning Amber | `#8A5A00` | Warning text/icons on pale amber surfaces |
| Success Green | `#176B45` | Success text/icons on pale green surfaces |
| Critical Red | `#B4232B` | Blockers and destructive-action warnings |

White text should be used on filled primary controls only after checking contrast for the selected mode. Pair all status colors with an icon and a plain-language label; never rely on color alone.

### Typography

- **Geist Sans** for headings, body copy, buttons, and labels.
- **JetBrains Mono** only for PCI identifiers, SMBIOS values, paths, and configuration snippets.
- Stitch type scale: headline large 24/32 px, medium 18/24 px, small 14/20 px; body 14/22 px and 13/18 px; label 12/16 px; code 12/16 px.
- Keep copy concise and readable, with room for future Arabic/RTL behavior.

### Layout, components, and interaction

- Desktop-first, focused single-column wizard with a stable progress/navigation region.
- Constrain the content width so it remains usable on wide desktop windows; reflow for narrower windows without horizontal scrolling.
- Use a predictable 8 px/16 px spacing rhythm and restrained 8 px corner radius.
- Establish depth mainly through tonal surface layers, not heavy shadows.
- Primary buttons use the active mode's Signal Blue. Secondary actions use neutral outlines or low-emphasis styling.
- Provide visible hover, pressed, disabled, and keyboard-focus states. All controls must be keyboard accessible.
- Use short, restrained transitions. Respect reduced-motion preferences; animate only meaningful work such as hardware scanning or downloads.
- Show task-specific progress and concise status text. Use a linear step indicator with explicit step names.
- Warnings state what is unsupported and what continuing means. Critical blockers use a distinct stop state and explain the consequence.

### Avoid

- Apple logos, Apple product-mark imitation, or implied Apple affiliation.
- Pure-black canvas, purple accents, RGB/gaming aesthetics, neon gradients, glow, and decorative gradients.
- Color-only status, low-contrast text, emoji as interface icons, or fake compatibility guarantees.
- Dense dashboard-first layout or tiny technical text without a user-facing explanation.
- Success styling for unsupported hardware.

## References

- [Google Stitch project](https://stitch.withgoogle.com/projects/6712922669637128072)
- [Stitch design system asset](https://stitch.withgoogle.com/projects/6712922669637128072?asset=5f9a9f40c4274196b73dac18e0d77f06)
