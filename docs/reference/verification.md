# Stage 1 reference-clone verification

Date: 2026-10-09
Status: Ready for the agreed visual-review checkpoint

Preview: http://127.0.0.1:4173/ and http://127.0.0.1:4173/download

## Source and standalone changes

Pinned source: https://github.com/pingdotgg/t3code/tree/ec80933ac8cd02fec5c97b342462ccc9567cdb1e/apps/marketing

58 reference files are inventoried in import-manifest.json. 56 retain identical
SHA-256 hashes, including both pages, motion/release scripts, all imported imagery
and fonts. Two imported files differ:

- `astro.config.mjs`: disable the development toolbar so it does not cover the page.
- `src/layouts/Layout.astro`: noindex/nofollow for this local checkpoint; policy
  links point to original T3 pages because those routes are outside clone scope.

The standalone package excludes the application schema endpoint/shared package,
retro route/assets, Vercel configuration and monorepo test runner. Pinned CLI
scripts are copied into public instead of staged from the monorepo. Their commands
remain T3 commands and were not executed. Astro and TypeScript match the pinned
reference; Sharp is updated from 0.35.4 to 0.35.5 to address the reported advisory.
Source: https://github.com/advisories/GHSA-wq5f-xc86-pv6w

MIT and font notices are retained. The original upstream package is preserved in
package.upstream.json. source-fidelity.json records file comparisons.

## Build and runtime evidence

- `npm run typecheck`: 0 errors, warnings or hints.
- `npm run build`: both static pages and optimized images generated successfully.
- npm installation audit after the Sharp patch: 0 reported vulnerabilities.
- Homepage and download routes return HTTP 200 in dev and built preview servers.
- 132 local HTML route/asset references checked against dist; none missing.
- Browser inspection found no horizontal overflow or broken visible images at
  1440 × 1000, 1024 × 768 and 390 × 844. No unhandled page errors observed.
- Stable/Nightly switching updates URL query, selected state, release metadata,
  all seven desktop asset links, CLI text and mobile visibility. Keyboard
  focus + Enter activates the Nightly tab with a visible focus indicator.
- Live reference download links resolved against T3's public release metadata.
  No binaries were downloaded or installed; installed-app/platform QA is not claimed.
- With the release API intentionally blocked, the download page shows Releases
  and all desktop cards use the upstream release-page fallback.
- Normal hero card animations run while visible. Offscreen hero motion pauses.
  Reduced motion disables card/rise animations and pauses the motion gates.

Behavior and viewport JSON reports accompany this file. The built preview and
temporary baseline servers and isolated QA browser were stopped. The main Astro
dev preview remains running for user review.

## Visual comparison

A separate local baseline used the original pinned Layout and imported pages,
with the same standalone dependencies and development-toolbar suppression.
Both were captured in the same browser with reduced motion and loaded fonts.

The six initial viewport comparisons (two pages × three sizes) contain **zero
different pixels**. Full desktop captures have equal dimensions and nearly
identical output: homepage mean absolute RGB channel difference 0.001451/255;
download page 0.000000891/255. Full captures were taken after scrolling settled;
small capture/render differences remain, so full-page pixel identity is not claimed.
See pixel-comparison.json for the measured values.

Homepage, download and full desktop captures were visually inspected, including
phone layouts. The reference hides its product screenshot on phones; this is
preserved here. The approved Lawkitt adaptation will keep product evidence visible.

## Accessibility findings carried from the reference

Keyboard focus and reduced-motion behavior were exercised. The axe audit reports
low-contrast text in the source design (for example #71717a on #09090b is about
4.11:1 for small text), including footer links. Homepage: 10 failing nodes;
Nightly download page: 12 failing nodes. Full audit JSON is saved.

The homepage also has an incomplete ARIA finding for a carousel div with aria-label
but no applicable role. The fixed grain overlay prevents automated contrast
classification of additional nodes; those remain incomplete, not passing checks.
These are reported for the adaptation, while this faithful checkpoint retains
the source design. This review does not establish full accessibility conformance.

## Review captures

- [Homepage desktop](home-desktop.png)
- [Homepage tablet](home-tablet.png)
- [Homepage phone](home-phone.png)
- [Full homepage](home-desktop-full.png)
- [Download desktop](download-desktop.png)
- [Download tablet](download-tablet.png)
- [Download phone](download-phone.png)
- [Full download page](download-desktop-full.png)
- [Nightly download](download-nightly-desktop.png)

## Next action

Review this reference clone. Lawkitt tailoring begins after that review as agreed
in Q14. Cloudflare Workers/domain configuration belongs to the adaptation; no
deployment, DNS change or Cloudflare account modification was performed.
