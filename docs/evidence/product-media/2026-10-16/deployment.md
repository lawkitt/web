# Product media publication · October 16, 2026

Published the existing `codex/refresh-product-media` branches at the user's request.

- mdoc: `c52a33b` — refreshed README, eight actual screenshots and fictional inputs.
- web: `66ed811` — refreshed screenshot gallery and three product films.
- Both commits pushed to `origin/codex/refresh-product-media` in their repositories.
- Website deployed with `npm run deploy` to Cloudflare Worker `lawkitt-web` and
  custom domain <https://lawkitt.com>.
- Cloudflare version: `07284403-2f21-4045-a86f-df104ae59d8e`.

The deployed website build passed Astro checks (zero errors/warnings), generation
and static-output verification. The GitHub [Check website run](https://github.com/lawkitt/web/actions/runs/38847154629)
for the media commit passed.

## Production verification

Both `/` and `/mdoc/`, and all 35 referenced image/font/script/style/video assets
checked, returned HTTP 200 and matched the local production build byte for byte.
`production-http.json` records URLs, byte counts and SHA-256 hashes. The edge
returned the complete 3.9 MB MP4 with HTTP 200 for a range request; native browser
video playback was verified successfully.

On <https://lawkitt.com/mdoc/>, all six Convert / Review / Compare × Light / Dark
states loaded. The Compare film loaded and played. At 390 px the document width
remained 390 px, with horizontal scrolling confined to the screenshot region.
Production desktop, mobile and player captures are saved alongside this record.

The follow-up publication commit contains only this verification record, the HTTP
report and production screenshots. The deployed application assets correspond to
web media commit `66ed811`; the follow-up documentation does not change them.
