# ADR 0003: Catalog content and Cloudflare Workers

Date: 2026-10-09
Status: Accepted (Round 3)

## Context

The user requested Artcraft's apps catalog as an additional structural reference,
then accepted the final recommendations and selected Cloudflare Workers with
domain `lawkitt.com`.

## Decision

Combine the approved T3 visual language with Artcraft's catalog/product separation.
Begin with one prominent mdoc entry, using reusable card metadata: icon, screenshot,
category, name, pitch, maturity, installer availability and Explore action. Download
platform badges require real artifacts; intended targets do not establish downloads.

Homepage: “Practical tools for everyday legal work.” and “Free, open-source
utilities from Lawkitt.” Follow with catalog, short principles/about and GitHub
contribution action. Primary action: Explore mdoc.

mdoc page: hero/status, screenshot, Open/Review/Copy workflow, conversion/preview/OCR,
experimental pseudonymization, practical FAQ, availability/source action. Explain
feature limits by platform and the current private source-build dependency where relevant.

Maintain verified download metadata in a checked-in typed release manifest.
No visitor-side release API lookup. Recheck releases before handoff.

Prepare static Astro hosting with Cloudflare Workers Static Assets and canonical
origin `https://lawkitt.com`. Include Wrangler configuration, real 404 behavior,
local Workers validation and deployment instructions. Publishing, DNS and account
changes remain outside scope.

Validate build/typecheck, navigation/status links, keyboard/focus/contrast and
reduced motion. Capture desktop, tablet and phone layouts. Compare the clone
against the pinned reference source baseline. Adapted mobile pages retain product
screenshots. Report verification limits candidly.

Website code uses MIT with inherited notices and font licenses preserved.
Listed applications retain their own licenses. Artcraft is structural inspiration;
its code, copy, artwork and app assets are not part of the clone.

## Rationale

This gives lawyers a clear catalog while preserving the approved reference style.
Static repository-driven content and reviewed release data match the small owned
catalog. Workers satisfies the selected hosting target without introducing an
application backend.

## Consequences

The design frontier is empty. The user confirmed shared understanding with
“implement” on 2026-10-09. The agreed clone review checkpoint must occur before
adaptation. Publication is a later authorized action.

## Technical references checked 2026-10-09

- https://developers.cloudflare.com/workers/framework-guides/web-apps/astro/
- https://developers.cloudflare.com/workers/static-assets/routing/static-site-generation/
- https://developers.cloudflare.com/workers/configuration/routing/custom-domains/
