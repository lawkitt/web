# Lawkitt marketing catalog

English-only, static Astro website for Lawkitt-developed free OSS legal utilities.
T3's reviewed styling and Artcraft's catalog organization, starting with mdoc.
Live: **Cloudflare Workers Static Assets** at [lawkitt.com](https://lawkitt.com).
Public source: [lawkitt/web](https://github.com/lawkitt/web).

## Local development

Use Node.js 22.12+ (verified with 26.11) and npm.

```sh
npm ci
npm run dev
```

Open http://127.0.0.1:4173/. Astro 7's dev server can continue as a daemon after
the command exits. Stop that server with `npx astro dev stop`.

```sh
npm run check
npm run preview
```

The static build is `dist/`. Dev pages use `noindex`; production builds use
`https://lawkitt.com` canonical URLs. `npm run build:preview` generates an explicitly non-indexable build in
`.preview-dist/`, including a disallow-all robots file. Production `npm run build`
always writes to `dist/` with indexing enabled, even if the shell has
`PUBLIC_PREVIEW=true`.
`node scripts/verify-build.mjs --preview` verifies that preview policy.

## Cloudflare Workers

```sh
npm run workers:preview
```

This builds a non-indexable `.preview-dist/` and runs Wrangler's **local** Workers runtime
at http://127.0.0.1:8787/. Ctrl+C stops it. Use this preview for deep-link, trailing
slash, custom 404 and `_headers` behavior; Astro's dev server does not reproduce
Workers routing and headers. Preview builds leave existing production output intact.

`wrangler.jsonc` configures `dist/`, trailing slash normalization, a genuine 404
fallback, and the custom domain `lawkitt.com`. No SSR script, adapter, database or
API is needed. Fingerprinted `/_astro/` assets receive immutable caching; HTML
keeps Workers' revalidation behavior. No analytics, forms or client release fetch.

**Published on 2026-10-09.** The production site is deployed as `lawkitt-web`
with the custom domain `lawkitt.com`. Deployment verification is recorded in
`docs/evidence/deployment/verification.md`. For future deployments, use a
Cloudflare account with access to the active `lawkitt.com` zone, authenticate
Wrangler, then run:

```sh
npx wrangler login
npm run deploy
```

This checks types, explicitly rebuilds production assets, validates the output and deploys
the configured custom domain. Wrangler creates/manages its custom-domain DNS
record; review existing domain records before that authorized deployment. No
account IDs or credentials are checked in. `npx wrangler deploy --dry-run` checks
packaging locally without deployment. See the official [Astro SSG guide](https://developers.cloudflare.com/workers/framework-guides/web-apps/astro/),
[SSG/404 routing](https://developers.cloudflare.com/workers/static-assets/routing/static-site-generation/)
and [custom domains](https://developers.cloudflare.com/workers/configuration/routing/custom-domains/).

## Content and release updates

- `src/lib/catalog.ts`: typed catalog metadata, icon, screenshot, maturity,
  installer availability, platform targets and source links.
- `src/content/tools/mdoc.md`: longer product content, limitations and FAQ.
- `src/data/mdoc-releases.json`: manually maintained release manifest.
- `src/lib/release-manifest.mjs`: validates dates, release ordering, installer
  targets, GitHub release URLs, duplicate entries and SHA-256 checksums at build time.
- `src/lib/releases.ts`: typed access to the validated manifest.
- `src/components/ToolCard.astro`: reusable app card.
- `src/layouts/Layout.astro`: shared navigation, metadata and footer.
- `src/styles/global.css`: T3-inspired tokens and shared styles.

mdoc has **no public releases** as checked on 2026-10-09. The empty manifest
produces “Downloads coming soon.” Source builds currently require access to the
private pinned `lawkitt/anydoc` dependency. Neither build targets nor maturity
indicate installer availability.

Before adding a release: verify it on the public mdoc repository, confirm every
installer URL/platform/architecture, calculate its SHA-256, record the version,
publication date and release-notes URL, and populate `src/data/mdoc-releases.json` in newest-first order. Update its
`checkedAt` field and review product feature limitations. Availability labels,
catalog notes, download metadata, release date and installer links update together
from the manifest. Never add inferred asset names or unfinished
packaging links. Re-run the checks and inspect the download page before publishing.

The social-card sources and technical exports are checked in. Regenerate the
favicon, touch icon and social PNG/SVG cards from supplied brand assets with
`npm run assets:brand` when branding changes. Product screenshots are optimized
by Astro during the build. Capture provenance is in `docs/evidence/stage2/`.

## Checks and maintainability

`npm run check` checks source formatting, Astro/types, six release tests, the
production build and generated links/metadata. `npm run format` formats editable
source. The installer render test builds a synthetic release in an isolated
temporary directory; it never changes the real manifest or user preview.

`.github/workflows/check.yml` runs these checks and validates the separate preview
build on pushes and pull requests. It uses read-only repository access and does
not deploy. The workflow passed on GitHub for the initial publication of `main`.
Deployments remain manual through `npm run deploy`; GitHub pushes only run checks.

## Review checkpoints and licensing

The reviewed Stage 1 clone is preserved at Git tag `reference-clone-stage1`;
its upstream pin, source hashes and QA captures remain in `docs/reference/`.
Stage 2 decisions and implementation acceptance are in `docs/design/` and
`docs/adr/`. Current verification: `docs/evidence/completion/verification.md`.

Website code: MIT (retained T3 copyright). mdoc: GPL-3.0-or-later.
See LICENSE and NOTICE.md for source, asset and font attribution.
