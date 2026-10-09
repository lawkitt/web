# Completed implementation

Verified: 2026-10-09. The full agreed Lawkitt website implementation is complete
locally. Cloudflare deployment and domain/account changes remain outside scope.

## Completed behavior

All agreed pages are implemented: English-only catalog homepage, mdoc product
walkthrough and FAQ, availability/download page, and custom 404. T3 typography,
dark palette and accents remain; Artcraft informs catalog metadata and page
organization. The real mdoc capture uses the synthetic fixture documented in
`../stage2/capture-provenance.json`. The original clone remains preserved at
`reference-clone-stage1`.

The remaining implementation gaps were closed:

- Catalog cards are reusable and driven by typed project metadata.
- `src/data/mdoc-releases.json` is the manually maintained release manifest.
  Build-time validation rejects malformed dates, out-of-order/duplicate releases,
  missing checksums/installers, unsupported targets, unrelated GitHub URLs and
  installer tags that disagree with release notes.
- Installer availability, catalog note, product status, download description,
  publication date and installer actions use the same manifest. Source-build
  constraints remain visible with either availability state.
- The availability page shows the date when its public status was checked.
- Production builds always select production metadata/output. Preview builds
  write separately to `.preview-dist/` and cannot overwrite `dist/`.
- Source formatting, functional release tests, output verification and a local
  GitHub Actions check workflow are included. The workflow does not deploy.

## Checks

`npm run check` passes source formatting, Astro/type checks (21 files, zero
errors/warnings/hints), six tests, production build and output verification.
`npm audit` reports zero vulnerabilities.

Five tests cover the release manifest and availability state; the sixth builds
an isolated temporary Astro fixture with macOS, Windows and Linux installers.
It verifies real generated catalog/product/download HTML, release dates,
checksums, accessible installer labels, source-build requirements and absence
of obsolete “coming soon” copy. No synthetic release enters the real manifest
or user preview.

`npm run workers:preview` builds and verifies its separate output and serves
through local Wrangler on port 8787. Production HTML remains byte-identical
before/after that preview build. Production indexing and preview `noindex` /
robots `Disallow: /` are both verified.

`npx wrangler deploy --dry-run` accepts the production configuration and 37
static assets. Local Workers deep links return 200, slash normalization returns
307 and unknown routes return the custom page with HTTP 404. Security headers
are present. No Cloudflare deployment, account checks or DNS changes occurred.

## Browser review

`browser-report.json` and adjacent screenshots record homepage, mdoc, downloads
and 404 at 1440, 1024 and 390 pixels. All 12 combinations have one H1, no broken
images, no page overflow and zero automated axe violations. The screenshots
were captured after fonts, images and entrance animations settled.

Keyboard skip navigation focuses the main content. Space toggles native FAQ
summaries. Reduced motion reports zero animations and automatic scrolling.
No browser errors were reported. Prior manual gradient contrast assessment and
screenshot keyboard-scrolling checks remain documented in
`../stage2/verification.md`; the color tokens and behavior are preserved.

Gradient contrast still requires manual assessment in the automated report.
This is focused local QA, not accessibility certification. Safari, mobile device
browsers and native screen readers were not tested. GitHub Actions is configured
locally; it has not executed remotely.

The public `lawkitt/mdoc` release API was rechecked during this completion pass
and returned an empty list. Real public installers are still unavailable, so
“Downloads coming soon” is the correct implemented state. The private pinned
conversion dependency remains documented as a source-build requirement.

## Handoff

Editable source and instructions: repository root and `README.md`.
User preview: http://127.0.0.1:4173/ (retained).
The temporary owned browser/Workers QA processes are closed after review.
