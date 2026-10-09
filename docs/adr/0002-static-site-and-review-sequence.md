# ADR 0002: Static catalog and staged review

Date: 2026-10-09
Status: Accepted (Round 2)

## Context

The catalog initially has one Lawkitt project. The reference uses Astro within
a larger monorepo. mdoc currently has no published public releases.

## Decision

Use standalone static Astro, typed project metadata and Markdown content.
Routes are `/`, `/tools/mdoc/` and `/tools/mdoc/download/`. Do not add a CMS,
backend, catalog filters/search or empty future-project cards.

Retain the reference dark palette, purple accent, typography and restrained
motion, with reduced-motion support. Use Lawkitt's white wordmark and square
brand mark. No theme switch at launch.

Lead mdoc with document-to-Markdown preparation for external AI tools. Explain
OCR and original preview as supporting capabilities. Experimental pseudonymization
is secondary, with its limits nearby. Use real screenshots with synthetic legal
documents and a short visual workflow, without initial video or interactive demos.

Until published installers are verified, show downloads coming soon and View
source. Preparing an mdoc release is outside this website scope.

No tracking, forms, accounts or mailing list. Support and contribution links go
to GitHub. Do not reuse T3 testimonials, statistics or customer logos as Lawkitt claims.

The user reviews the faithful reference clone before Lawkitt tailoring begins.
Both checkpoints remain separately reviewable. Implementation begins only after
the completed grilling design receives shared-understanding confirmation.

## Consequences

Content and deployment remain repository-driven. Adding the next actual project
should reuse catalog metadata and product-page structure. Website publication is
not included. Artcraft was subsequently accepted as a catalog-structure reference;
ADR 0003 records its influence and Cloudflare deployment preparation.
