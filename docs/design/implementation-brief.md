# Lawkitt website implementation brief

Date: 2026-10-09
Status: Confirmed by user; Stage 1 complete and awaiting visual review

Decision record: `marketing-site.md` and `../adr/0001-*` through `0003-*`.

## Stage 1: Reference clone

Reproduce T3 Code's marketing homepage and download layout as a standalone
Astro project in this workspace. Pin the source to
`ec80933ac8cd02fec5c97b342462ccc9567cdb1e`. Keep T3 branding, visual assets,
fonts, layout and relevant interactions for this local reference checkpoint.
Remove dependence on the full T3 monorepo while preserving required attribution.

Preserve this baseline separately for review. Verify build/typecheck, navigation,
reference imagery/fonts and responsive layouts against the pinned version.
Supply a working local preview and review screenshots. Keep it local; it is not
the Lawkitt production content.

Stop for the user's visual review before Stage 2, as explicitly agreed in Q14.

Completed: 2026-10-09. Preview: http://127.0.0.1:4173/
Evidence and inherited accessibility findings: `../reference/verification.md`.

## Stage 2: Lawkitt adaptation

English-only marketing catalog for Lawkitt-developed free OSS legal utilities.
Keep the accepted dark T3 visual language, purple accent, typography and restrained
motion. Use the supplied Lawkitt wordmark/icon. Borrow Artcraft's information
hierarchy rather than its source, copy or assets.

Routes:

- `/`: hero, actual catalog, principles/about, contribution action.
- `/tools/mdoc/`: product pitch/status, real screenshot, three-step workflow,
  capabilities, experimental pseudonymization, FAQ, availability/source action.
- `/tools/mdoc/download/`: honest status and, when verified artifacts exist,
  platform/architecture download choices and release information.
- A useful custom 404 page.

Homepage headline: “Practical tools for everyday legal work.”
Subtitle: “Free, open-source utilities from Lawkitt.”
Main action: Explore mdoc; secondary: Browse on GitHub.

mdoc headline: “Turn legal documents into editable Markdown for AI tools.”
Workflow: Open a document, review/edit the Markdown alongside the original,
copy the prepared Markdown to an external tool. Explain local OCR and preview
limitations accurately. Pseudonymization is experimental and requires review.

Start with one prominent mdoc card. Catalog metadata is typed; longer content is
Markdown. Release metadata is checked in and updated manually against verified
published artifacts. Recheck mdoc's public release status at implementation handoff.
While no installers exist, display Downloads coming soon and View source.

Use real mdoc screenshots with synthetic legal documents and a short visual
walkthrough. Preserve readable product evidence on mobile. No invented social proof,
unqualified platform claims, future-project filler, search/filter controls, CMS,
backend, accounts, tracking, forms, mailing list, video or interactive demo at launch.
Support and contributions link to GitHub. Do not duplicate the full mdoc feature
list on the Lawkitt homepage.

## Deployment preparation

Target: Cloudflare Workers Static Assets, domain `lawkitt.com`.
Canonical production origin: `https://lawkitt.com`.

Prepare Wrangler assets/custom-domain configuration for the static `dist/`,
custom 404 handling, build/deploy instructions and local Workers preview.
Static output needs no SSR/backend. Prepare page titles/descriptions, canonical
links, sitemap and Lawkitt social images with the selected production origin.
Use appropriate asset caching and preserve attribution. Validate generated routes
through local Workers handling, including deep links and a genuine missing-page 404.

Deliver deployment-ready code; do not deploy or change DNS/account configuration.

## Acceptance and handoff

- Build and Astro/type checks pass.
- Navigation, project/source/support links and release-status behavior are verified.
- Desktop (1440), tablet (1024) and phone (390) screenshots are reviewed for
  overflow, readability, imagery, content hierarchy and responsive navigation.
- Keyboard focus, contrast and reduced-motion behavior are checked; report any
  unverified accessibility behavior rather than claiming certification.
- Reference clone matches the pinned baseline, allowing necessary standalone
  packaging changes that do not change the reviewed design.
- Adaptation has only actual Lawkitt/mdoc content, honest product availability
  and locally verified Cloudflare routing/404 behavior.
- MIT website license, upstream/font notices and separate application licenses
  are preserved and accurately described.
- Provide local preview, source, captures, checks performed and remaining limits.

## Confirmation boundary

All design questions are answered. The user confirmed shared understanding with
“implement” on 2026-10-09. Stage 1 is authorized and begins now. Stage 2 awaits
the already-agreed clone visual review.
