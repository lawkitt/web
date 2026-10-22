# ADR 0008: Listing mdoc's first stable release

Date: 2026-10-22
Status: Accepted

## Context

mdoc v0.1.0 was published on 2026-10-19 as GitHub's Latest release, after
rc.1, beta.12 and beta.13. The site still listed v0.1.0-beta.11. v0.1.0 ships
an Apple Silicon `.dmg` (signed and notarized), a Windows x64 NSIS `.exe`, a
Windows `.msi` (stable tags only) and `SHA256SUMS`. The Windows builds are
unsigned. Per mdoc ADR 0037, the `.exe` installs the Visual C++ Redistributable
when it is missing; the `.msi` refuses to install without it.

## Decision

- **Windows installer**: offer only the NSIS `.exe`, because it installs the
  runtime that OCR needs. The `.msi` is not listed. Add it later as a secondary
  "for IT deployment" link only if someone asks for it. That would mean letting
  the manifest validator accept two assets for one platform.
- **Manifest**: list only v0.1.0. Listing a pre-release next to the first
  stable release would confuse what "Stable" means. Earlier releases are not
  shown anywhere on the page.
- **Beta wording**: remove it completely. The FAQ answer "Can I download mdoc
  today?" no longer mentions beta, rough edges or trying made-up documents
  first. The reliability and fidelity FAQs already tell readers to check
  results. It also says that on a Mac, mdoc can download and install updates
  itself. The download page's beta note and the "Stable" maturity tag follow
  the manifest automatically.
- **Visual C++ Redistributable**: the download page and FAQ say that the
  installer adds the runtime if it is missing. Users are no longer told to
  install it themselves.
- **Product media**: keep the beta.11 captures and their "beta.11" captions for
  now, because the captions are accurate. Re-capture from v0.1.0 in a follow-up
  PR using `videos/mdoc/capture` (ADR 0007). The UI has visibly changed: view
  switcher, Replacements panel grouped by category, zoom pill that fades,
  macOS menus.
- **Messaging**: add no banner. The download page gets one quiet line saying
  this is the first stable release. Nothing else on the site announces it.
- **Delivery**: a branch and PR. Before opening the PR, check `/mdoc/` and
  `/mdoc/download/` in a local preview and have Codex review the diff.

## Consequences

- The README's release paragraph and its "two newest releases" rule change to
  describe a single stable release.
- Follow-up: re-capture the product media from v0.1.0.
- Windows signing is still outstanding (required for mdoc 1.0), so the
  SmartScreen note stays.
