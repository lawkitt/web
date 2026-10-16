# ADR 0007: mdoc product media from scripted, clean captures

Date: 2026-10-16
Status: Accepted

## Context

PR #6 replaced the explainer films and single screenshot with a Convert /
Review / Compare gallery and three films. Review found: the dark captures were
flattened to JPG with white rounded-window corners (also visible in every film);
captures disagree with each other (screen-capture indicator instead of traffic
lights, notification badge, zoom pill over text, unsaved dot, resize grip); the
films only crossfade light and dark; the scan/OCR and reply-restore stories are
gone; the fixture agreement is too thin to fill the review panel; old cuts and
duplicate assets remain.

## Decision

- Fix PR #6 in place before merging, not in a follow-up.
- Captures are scripted: mdoc opens a fixture document and each state is taken
  with `screencapture -o -l<window>` as PNG with alpha. Astro makes the WebP;
  fframes uses the PNGs directly. No JPG for UI captures.
- Clean state for every capture: traffic lights visible, no notification badge,
  no floating view pill over text, saved file, no resize grip.
- The fixture becomes a richer fictional legal document (claim letter or NDA)
  with at least eight entity types: people, organizations, addresses, phones,
  emails, IBANs, case numbers, dates.
- Films are short real screen recordings (about 6–8 s per step) edited in
  fframes, which adds only captions and framing. No theme crossfade; the
  gallery's Light/Dark toggle covers themes.
- Story: scan/PDF → review → Copy for AI → AI reply with real names restored.
  Compare is a gallery view, not a film.
- The AI step is a neutral mocked chat panel inside the fframes composition,
  not a recording of the ChatGPT or Claude UI.
- Capture the polished details, not only whole screens: the hover tooltip on a
  mention, the replacement popup, replacement motion, OCR and pseudonymization
  marks on sidebar tabs, notices as toasts, the Review | Share inspector, the
  categories popup and the before/after view.
- The website uses mdoc's vector logo (`mdoc/build/appicon.svg`, optimized)
  instead of `src/assets/mdoc-icon.png`.
- Remove unused media: `public/videos/mdoc/{ocr,pseudo,ai}*`,
  `src/assets/mdoc-screenshot.png`, duplicated capture copies, and move
  `docs/evidence` screenshots out of git.
- Capture from mdoc 0.1.0-beta.11 installed in `/Applications`. The beta.11
  release must be published and `mdoc-releases.json` updated in the same PR, so
  the site never shows a build visitors cannot download.
- Fixture: a fictional pre-action claim letter as a scanned 1–2 page PDF, so one
  document carries scan → review → Copy for AI → restore, with 8+ categories.
- Gallery tabs: Scan, Review (replacement popup open), Compare, Copy for AI,
  Restore. Each shows a polished detail in use.
- Films: a library of many short scenes (about 2–4 s each) rendered as separate
  cuts, to be curated and reordered later. The page keeps the tabbed player with
  autoplay-when-visible and the reduced-motion fallback; `current.mp4` and its
  link go.
- Logo: `mdoc/build/appicon.svg` (optimized) replaces `mdoc-icon.png` wherever it
  is used, and appears on the film title card. The PNG stays only where a raster
  is required (OG image, Apple touch icon).
- Deployment proof goes in the PR description; only a short `verification.md`
  stays in the repo.

## Amendments during capture (2026-10-16)

- **OCR is shown only up to the recognition prompt.** On the scanned letter,
  beta.11's OCR merges paragraphs and headings and splits table rows. The `scan`
  gallery view and scene show the "Some pages need text recognition" card; the
  review story uses the same letter as a Word file. Show OCR output again once
  mdoc's layout recovery improves.
- **Wording, not detection, was adjusted.** Surname-only mentions ("Ms Raman") are
  not linked to the full name, and adding one manually splits the existing entity
  (an mdoc bug), so the fictional letter avoids surname-only mentions. The missed
  Southampton address is kept and fixed on camera (`address` scene).
- Page films: `find`, `fix`, `share`, `restore`, each an ordered list of library
  scenes; dark only (the site is dark). The light set renders on demand.

## Scene library (first cut)

Each scene is its own 2–4 s cut; the page picks and orders a subset later.

| id | shows |
| --- | --- |
| `intro` | logo card |
| `scan` | a scan asks for text recognition |
| `detect` | Pseudonymize: mentions highlight by category |
| `tooltip` | hover tooltip on a mention |
| `popup` | a mention opens every mention of the identity |
| `alias` | "Same as…" alias dropdown |
| `keep` | untick a false positive |
| `address` | add a missed address, file it as Address |
| `apply` | Apply with replacement motion |
| `compare` | original and pseudonymized side by side |
| `copy` | Share → Copy for AI |
| `chat` | neutral mocked chat answers using aliases (composited) |
| `restore` | Paste AI reply: a new tab with real names restored |
