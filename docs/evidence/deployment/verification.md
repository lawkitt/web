# Production publication verification

Published on 2026-10-09 following the user's explicit GitHub and Cloudflare request.

- Public repository: https://github.com/lawkitt/web
- Branch: `main`; deployed application commit: `d5a6d00a0119c6c554dbced00097b79d63cda627`.
- Preserved tags: `reference-clone-stage1`, `lawkitt-catalog-stage2`.
- Cloudflare Workers Static Assets service: `lawkitt-web`.
- Production custom domain: https://lawkitt.com
- Cloudflare version: `ef8e17f7-8d32-4011-8ebe-a005da1ee75f`, deployed at 2026-10-09 19:44:56 UTC, 100% traffic.
- GitHub Actions initial main run passed: https://github.com/lawkitt/web/actions/runs/37982377523
- Local `npm run check` passed: formatting, Astro diagnostics, six tests, production build and generated link/metadata verification. Deployment rebuilt and validated production output.

## Live checks

`http-report.json` records HTTPS responses with certificate verification enabled:

- Homepage, product and download pages: 200, correct titles, `index, follow`.
- Unknown route: custom 404, `noindex, nofollow`.
- `/tools/mdoc`: 307 to `/tools/mdoc/`.
- robots.txt allows indexing and references the production sitemap.
- Sitemap contains all three public pages at the canonical production origin.
- CSP and configured security headers are present.

`asset-report.json` confirms all 30 emitted public files match the production build byte for byte, including the custom 404 via an unknown route. `_headers` is configuration and is not a served asset.

Browser checks: all three public pages loaded; no broken images or horizontal page overflow at 1440px. Homepage also checked at 390px with no broken images or page overflow. Desktop and phone screenshots are saved alongside this report, with entry animations completed before capture.

Public DNS resolved through Cloudflare's authoritative nameserver, 1.1.1.1 and 8.8.8.8. This computer's resolver retained a negative result immediately after the new DNS record was created. HTTPS and browser verification temporarily pinned the publicly resolved Cloudflare IP for `lawkitt.com`, preserving the hostname, TLS verification and actual production content. No operating-system DNS or hosts settings were changed.

Deployments are manual (`npm run deploy`). GitHub Actions validates pushes and pull requests; it does not deploy. No credentials are committed.
