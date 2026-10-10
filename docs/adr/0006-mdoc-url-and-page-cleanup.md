# ADR 0006: Simpler mdoc URL and leaner pages

Date: 2026-10-10
Status: Accepted

## Decision

- mdoc lives at `/mdoc/` and `/mdoc/download/`. The old `/tools/mdoc/*` and
  `/tools/` URLs 301-redirect via `public/_redirects` (Workers Static Assets).
  Future tools use `/<slug>/`.
- "Get mdoc" leads to the mdoc page, not the download page.
- Homepage: hero (with the workflow animation), the mdoc card, and "What we build"
  (free, open tools for AI-native lawyers). The How it works, Why mdoc and Get
  started sections are removed.
- mdoc page: less promotional and feature-led. Hero, screenshot, a six-item
  feature grid (formats, scans, side-by-side original, pseudonymize, copy for
  AI, local and fast) and the FAQ, which keeps all limits. No workflow, pillars,
  privacy section or CTA panel. A screenshot carousel and demo videos come later.
- The navigation's right-hand button is a GitHub star badge for `lawkitt/mdoc`
  on every page, while mdoc is the only tool. The count is read at build time
  (never by visitors). It shows a count only from 100 stars upward; below that
  it reads "Star on GitHub" (mdoc had 0 stars on 2026-10-10).
