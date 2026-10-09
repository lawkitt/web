# Lawkitt marketing site

Interview started: 2026-10-09. Status: design settled, all three rounds confirmed;
user confirmed shared understanding; Stage 1 complete and awaiting visual review.

Implementation starts only after the design tree is settled and the user
confirms shared understanding. Research and decision documentation are authorized now.

## Confirmed brief

- Lawkitt will be a marketing website and catalog of free open-source software
  useful to lawyers, chiefly practical utilities.
- The first listed project is mdoc, with source checkout at
  `/Users/tebriz/Developer/lawkitt/mdoc`.
- First clone the supplied T3 Code marketing reference, then tailor it to Lawkitt.
- Use the existing assets in the website workspace as design inputs.
- Catalog only software we develop. No third-party listings or public submissions.
- English only; write for practicing lawyers with little technical knowledge.
- First checkpoint: faithful local reference homepage clone plus its download
  layout, retaining T3 branding. Clone and adaptation remain separately reviewable.
- Lead with practical tools for everyday legal work. Local processing, account
  requirements and AI capabilities are verified per project, not catalog-wide guarantees.
- Primary outcome: understand a tool's usefulness and reach its download quickly.
  Source and contribution actions are secondary.
- Deliver a deployment-ready website after local review. Public publication is
  outside this implementation scope.
- User accepted all Q7–Q14 recommendations on 2026-10-09; see the accepted Round
  2 decisions below. Review the reference clone before tailoring begins.
- User accepted Q15–Q21, specifying Cloudflare Workers and `lawkitt.com` for Q18.
  The production origin is `https://lawkitt.com`; preparation does not authorize
  publishing or establish that DNS/account configuration is complete.

## Verified inputs

- This workspace contains six PNG/SVG brand assets and skill files; it currently
  has no app scaffold and is not a Git repository.
- `lawkitt-black-vector.svg` and `lawkitt-white-vector.svg` are monochrome
  wordmarks. Their PNG exports are 3816 × 1024. `logo.png` and
  `logo-black-bg.png` are 1254 × 1254 icon assets. The logo image was inspected.
- Reference: https://github.com/pingdotgg/t3code/tree/main/apps/marketing
- Inspected upstream commit: `ec80933ac8cd02fec5c97b342462ccc9567cdb1e`.
  The live https://t3.codes site was also read; live deployment parity with this
  source commit has not been established.
- The reference uses Astro, a dark palette, DM Sans and JetBrains Mono,
  large hero typography, product screenshot, testimonials, feature sections,
  open-source section and download calls to action. Its package includes a
  workspace shared dependency and install-script staging, so copying the
  directory alone is not a verified standalone build.
- Upstream has an MIT license. Preserve its notice with any reused source.
- Current mdoc README, CONTEXT, ROADMAP and Cargo metadata were inspected:
  local document preparation into editable Markdown for external AI tools;
  PDF/DOCX conversion, local OCR, source preview, and experimental reviewable
  pseudonymization. Cargo declares GPL-3.0-or-later. Building currently requires
  access to a private pinned AnyDoc dependency. Public release/download and
  platform availability must be checked before writing launch claims.
- Round 2 research: the public `lawkitt/mdoc` repository exists, but its release
  API returned an empty list and its public Releases page says there are no
  releases. Checked 2026-10-09: https://github.com/lawkitt/mdoc/releases
  A local release workflow is present; this does not establish published binaries.
- The reference download layout has Stable/Nightly channels, desktop architecture
  cards, mobile-store links and CLI commands, with browser-side release lookup.
  These are reference behavior, not evidence that mdoc offers the same channels,
  platforms or installation commands.
- The live reference homepage was inspected in an isolated browser session.
  Viewport capture: `/Users/tebriz/.codex/visualizations/2026/10/09/01a1211b-9702-7c21-b1e8-9b0ffd921bf5/t3-reference-home.png`.
  Its hero shows a subtle grid, floating integration marks, large centered
  typography and a white primary download button. Browser session was closed.
- Additional reference requested by the user: https://getartcraft.com/apps
  Inspected its rendered desktop catalog and https://getartcraft.com/apps/pdfcraft.
  Artcraft organizes its collection as introduction → app lineup → principles
  → community action. Cards contain task category, screenshot/icon, name, pitch,
  maturity, installer availability, platforms and Explore link. Product pages
  expand into screenshots, features, downloads and source. Maturity and installer
  availability are separate labels. These are observed structure, not verified
  claims about the listed software. Analysis: `docs/design/artcraft-reference.md`.
- Cloudflare's official Astro guide was checked on 2026-10-09. A fully static
  Astro build can be hosted on Workers using a Wrangler assets directory (`dist`),
  without SSR, an Astro Cloudflare adapter or a Worker entry script. The guide
  documents custom 404 handling. Custom Domains documentation was also consulted.
  Sources: https://developers.cloudflare.com/workers/framework-guides/web-apps/astro/
  and https://developers.cloudflare.com/workers/configuration/routing/custom-domains/

## Design tree

All stakeholder decisions in the three-round tree are settled. The frontier is
empty. Routine implementation details follow these constraints; changes to scope
or unresolved verification limits must be reported rather than silently assumed.

1. Catalog boundary [SETTLED, Q1: own software only]
   - Launch information architecture [SETTLED, Q7].
   - Artcraft-informed catalog presentation/metadata [SETTLED, Q15].
2. Audience and launch languages [SETTLED, Q2: English only]
   - Localization and language-navigation branches closed.
   - Product pitch [SETTLED, Q9]; homepage wording/composition [SETTLED, Q16].
3. Clone milestone [SETTLED, Q3: homepage + download layout, T3 branding]
   - Visual adaptation and asset roles [SETTLED, Q8].
   - Review sequence [SETTLED, Q14].
   - Clone/adaptation acceptance and responsive checks [SETTLED, Q20].
4. Brand promise [SETTLED, Q4: practical legal utilities, verified per-tool claims]
   - mdoc feature hierarchy [SETTLED, Q9].
   - Product evidence [SETTLED, Q12].
   - Product page composition [SETTLED, Q19].
5. Main visitor outcome [SETTLED, Q5: usefulness → download]
   - Current no-release experience [SETTLED, Q10].
   - Release metadata and update behavior [SETTLED, Q17].
6. Delivery constraints [SETTLED, Q6: deployment-ready, reviewed locally]
   - Framework and content editing [SETTLED, Q11].
   - Privacy, analytics and contact behavior [SETTLED, Q13].
   - Host/domain preparation [SETTLED, Q18: Cloudflare Workers, lawkitt.com].
   - Operational copy/FAQ folded into composition [SETTLED, Q16/Q19].
   - Website source licensing [SETTLED, Q21: MIT].

Cross-cutting branches are covered by Q8/Q12 (branding, motion and screenshots),
Q10/Q15/Q17/Q19 (claims and availability), Q18 (production SEO metadata),
Q20 (responsive/accessibility acceptance) and Q21 (source attribution).

## Round 2 decisions (accepted)

- Q7: `/` is the Lawkitt introduction and actual catalog, `/tools/mdoc/` is the
  product page, and `/tools/mdoc/download/` is its download/status page. With one
  product, omit separate catalog page, search, filters and empty future cards.
- Q8: Keep the reference dark palette, purple accent, typography, spacing and
  restrained motion. Use the white Lawkitt SVG wordmark in navigation and the
  supplied square mark for favicon/social branding. No theme switch at launch.
- Q9: Lead mdoc with PDF/DOCX → editable Markdown → external AI workflow.
  Explain OCR and original-document preview as support. Put experimental
  pseudonymization in a secondary section with its limits nearby.
- Q10: Until verified published installers exist, use a preview/coming-soon
  state and View source; never substitute an unrelated or nonexistent download.
  Keep the future download route ready for real release assets. Creating an
  mdoc release is a separate project scope.
- Q11: Standalone static Astro, typed project data and Markdown content in this
  repository. No CMS, app backend or runtime dependence on the T3 monorepo.
- Q12: Use current real mdoc screenshots from synthetic legal examples and a
  short visual workflow. No video in initial scope, invented testimonials,
  adoption statistics or customer logos.
- Q13: No analytics/tracking, cookies for tracking, accounts, forms or mailing
  list at launch. Contact/support and contributions go to appropriate GitHub pages.
- Q14: Stop after the separately reviewable reference clone for user visual
  review, then tailor following that review. This checkpoint approval is separate
  from the interview's shared-understanding confirmation.

## Round 3 decisions (accepted)

- Q15: Use Artcraft as an information-structure reference, retaining the settled
  T3 visual language and routes. One prominent mdoc card now; reusable card layout
  for later actual projects. Fields: icon, screenshot, category, name, pitch,
  development status, installer availability and Explore. Show platform download
  badges only when actual assets exist; target/feature support belongs on mdoc's page.
- Q16: Homepage hero “Practical tools for everyday legal work.” with subtitle
  “Free, open-source utilities from Lawkitt.”, Explore mdoc and Browse on GitHub.
  Order: hero → actual catalog → short principles/about → GitHub contribution
  action. No mdoc feature list duplicated across the homepage and product page.
- Q17: Checked-in typed release manifest, manually updated against verified
  published assets. No visitor-side release API fetch. Empty state remains honest;
  future entries carry version, platform/architecture, verified artifact URL,
  release notes and applicable feature limits. Recheck public releases before
  implementation handoff; installer publication remains separate work.
- Q18: Cloudflare Workers with Static Assets serving Astro's `dist/`, documented
  build/deploy commands and production URL `https://lawkitt.com` for canonical,
  social and sitemap metadata. Prepare Wrangler custom-domain and custom-404
  configuration plus local Workers validation. No actual deployment, domain
  purchase, DNS mutation or account changes in current scope.
- Q19: mdoc page order: hero/status → real screenshot → Open/Review/Copy workflow
  → conversion/preview/OCR capabilities → experimental pseudonymization → short
  practical FAQ → availability/source action. Explain platform-specific feature
  support; avoid unconditional cross-platform OCR/PII claims or a build-from-source
  quickstart that omits the current private dependency requirement.
- Q20: Validate both checkpoints with typecheck/build, working navigation and
  current link/status checks, keyboard/focus and contrast checks, reduced motion,
  and screenshots at desktop (1440), tablet (1024) and phone (390) widths. Clone
  compared to pinned source baseline, not assumed identical to changing live site.
  Screenshots remain visible/readable on the adapted mobile product page.
- Q21: MIT for website code, preserving upstream MIT notices and font licenses;
  project software licenses are separate (mdoc: GPL-3.0-or-later). Artcraft is a
  structural inspiration only; its source, artwork, copy and app assets are not
  part of the approved clone.

## Decision log

- 2026-10-09: Recorded explicit brief and research.
- 2026-10-09: User confirmed Q1 own software only, Q2 English only, and the
  recommendations for Q3–Q6. The proposed bilingual/external-curation branches
  were rejected/closed. Recorded new release research and Round 2 frontier.
- 2026-10-09: User accepted Q7–Q14 and requested inspection of Artcraft's apps
  catalog. Recorded its structure and the Round 3 frontier. No implementation yet.
- 2026-10-09: User accepted Q15–Q21, replacing hosting-neutral Q18 with Cloudflare
  Workers and lawkitt.com. Verified official static Astro/Workers guidance,
  completed ADR 0003 and implementation brief. Frontier empty; final shared-
  understanding confirmation pending. No application code or hosting changes made.
- 2026-10-09: User said “implement”, confirming the settled brief. Stage 1 clone
  implementation is authorized. Stage 2 still awaits the agreed visual review.
- 2026-10-09: Stage 1 imported from the pinned source as a standalone Astro
  package. Typecheck/build, live release controls, failure fallback, motion/focus
  and responsive captures verified. Six viewport comparisons matched the pinned
  baseline exactly. Inherited accessibility findings recorded, not marked passing.
  Main local preview retained for review; Stage 2 and deployment have not begun.
