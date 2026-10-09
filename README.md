# Lawkitt website

Current checkpoint: **Stage 1 — faithful T3 Code reference clone**.

The approved design is in [docs/design/implementation-brief.md](docs/design/implementation-brief.md).
Lawkitt tailoring starts after the user reviews this baseline. Nothing is deployed.

## Local preview

Requires Node.js 22.12 or newer.

```sh
npm ci
npm run dev
```

Open the Local URL printed by Astro. The default configured port is 4173; Astro
selects another port if it is occupied. Homepage: `/`; download page: `/download`.
The reference's Stable/Nightly switch and platform-aware download actions remain.
They query T3's public GitHub releases with upstream release-page fallbacks.
Reference policy links lead to the original T3 website. CLI scripts are served
as files, matching the pinned upstream; no installation script is run during setup.

The review preview is running at `http://127.0.0.1:4173/`. To stop the Astro
daemon, run `./node_modules/.bin/astro dev stop` from this directory.

## Verification

```sh
npm run typecheck
npm run build
npm run preview
```

This standalone package preserves the pinned Astro/TypeScript versions. Sharp
uses the 0.35.5 security patch instead of upstream's 0.35.4 (GHSA-wq5f-xc86-pv6w).
It excludes the T3 application JSON-schema endpoint, retro page, Vercel deployment
configuration and monorepo test runner. The marketing pages need none of them.
Pinned CLI scripts are copied into public instead of staged from a monorepo.
The homepage, download page, fonts, imagery, styles and motion stay upstream-derived.
Local previews have noindex metadata.
The Astro development toolbar is disabled so it does not cover the reference design.

## Source baseline

Upstream commit: `ec80933ac8cd02fec5c97b342462ccc9567cdb1e`.
See [NOTICE.md](NOTICE.md), [LICENSE](LICENSE), and
[docs/reference/import-manifest.json](docs/reference/import-manifest.json).
Standalone changes and visual verification are recorded in docs/reference.
Read [docs/reference/verification.md](docs/reference/verification.md) for capture
comparisons, behavior checks and inherited accessibility findings.

## Next checkpoint

After clone review: adapt to the approved English-only Lawkitt catalog and mdoc
product/download pages. Prepare Cloudflare Workers Static Assets for
`https://lawkitt.com`; deployment remains outside the current authorized scope.
