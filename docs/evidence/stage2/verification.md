# Stage 2 verification

Date: 2026-10-09. English-only Lawkitt adaptation, authorized explicitly by the
user after the Stage 1 checkpoint. No deployment, publication or DNS changes.

## Delivered

- `/`: introduction → actual mdoc catalog card → principles → GitHub community.
- `/tools/mdoc/`: status → authentic screenshot → Open/Review/Copy → capabilities,
  limitations, experimental pseudonymization, FAQ and project actions.
- `/tools/mdoc/download/`: honest availability, public source/build constraint,
  platform capabilities and support.
- Custom missing page, Lawkitt social cards, canonical metadata, sitemap/robots,
  static Workers configuration and caching/security headers.

T3 styling retains the dark palette, hue 250 accent, DM Sans/JetBrains Mono,
white/outlined buttons and restrained entrances. Artcraft informs information
hierarchy. The source clone and its evidence are preserved at
`reference-clone-stage1`; its T3 product assets are absent from the built site.

## Build and routing

- `npm run typecheck`: 0 errors, warnings or hints (17 files).
- `npm run build`: static output successful, 4 pages plus sitemap and robots.
- `npm run verify`: all internal links/images/anchors, metadata, brand cleanup,
  sitemap, headers file and production robots policy passed.
- `npm audit`: 0 vulnerabilities, including development dependencies.
- `npx wrangler deploy --dry-run`: 37 static assets read; package accepted locally.
- Local Wrangler Workers runtime at `127.0.0.1:8787`: homepage and both deep links
  return 200; an unknown URL returns custom HTML with **404**, not a homepage fallback.
- `/tools/mdoc` redirects to `/tools/mdoc/` with 307. `_headers` policies are present.
- A `PUBLIC_PREVIEW=true` build was verified separately: pages `noindex, nofollow`,
  robots `Disallow: /`. Production output was then rebuilt and reverified.

No Cloudflare account/zone configuration was tested. The custom domain becomes
active only during a separately authorized deployment to the appropriate account.

## Responsive and interaction review

`browser-report.json` records all 12 route/viewport combinations through Workers.

| Page | 1440px desktop | 1024px tablet | 390px phone |
| --- | --- | --- | --- |
| Homepage | Pass | Pass | Pass |
| mdoc | Pass | Pass | Pass |
| Availability | Pass | Pass | Pass |
| Missing page | Pass | Pass | Pass |

Every page has one H1, no broken images or unresolved internal anchors, and its
page scroll width equals the viewport. Captures wait for fonts, images and
entrance animations to finish. Main captures use ordinary motion, not a forced
static override. Phone screenshots keep the real app visible at a readable
scale in a horizontally scrollable region; the second pane is reachable by
horizontal scrolling. This is intentional image scrolling, not page overflow.

Keyboard checks: skip link is visible with solid focus outline, Enter focuses
`main`; screenshot region has visible focus and ArrowRight scrolls it; Space on
an FAQ summary expands the native details element. Product availability action
navigates to the expected page. The navigation and section anchors resolve.
Reduced motion reports zero animations and `scroll-behavior: auto`.
No browser errors were reported during these checks.

Automated axe checks: **0 violations** at all 12 combinations. Color contrast
checks involving gradients/pseudo elements require manual assessment and remain
listed as incomplete in the raw report. A conservative brightest-background
calculation (full accent overlay over the brighter card surface: RGB 21,35,49)
checks the shared colors: dim 4.61:1, muted 6.22:1, accent 5.53:1, foreground
15.28:1. All exceed 4.5:1. `interaction-report.json` records that calculation.
This is focused QA, not accessibility certification. Native screen-reader,
Safari/iOS/Android and physical-device behavior were not tested.

## Product evidence and claims

`mdoc-native.png` is an authentic native screenshot using
`fixtures/Services Agreement.docx`. The fixture names and contract are synthetic.
`capture-provenance.json` records the existing local binary and source revision;
this is a development capture, not evidence of a published release.

Current mdoc README was checked for conversion/image loss, DOCX preview caveats,
OCR platform/runtime limits, experimental scanning and known misses, explicit
review/application before copy, application license and private build dependency.
No new detector, OCR or cross-platform qualification was performed for this site.

The public GitHub releases API returned `[]`, rechecked at implementation
handoff on 2026-10-09. The checked-in manifest is empty, so no installer links or
release/channel claims are emitted. Installer availability remains separate
from maturity and build targets. There is no runtime GitHub API fetch or tracking.

## Review assets

Full-page captures are adjacent to this file:
`home-1440.png`, `home-1024.png`, `home-390.png`, and matching `mdoc-*`,
`download-*`, `404-*` captures. Source and synthetic fixture remain editable.
The main Astro preview is retained at http://127.0.0.1:4173/ for user review.
Temporary Workers/browser QA sessions are stopped after verification.
