# ADR 0001: Lawkitt marketing catalog

Date: 2026-10-09
Status: Accepted direction (Round 1); implementation details pending interview

## Context

The user wants to publish and market free open-source software useful to lawyers,
beginning with mdoc. The supplied visual/source reference markets a single coding
product, while Lawkitt's stated purpose is a software catalog.

## Decision

Build toward a Lawkitt marketing catalog, with mdoc as its first project.
First establish a clone of the supplied T3 Code marketing reference, then tailor
it to the different Lawkitt product and audience. Use existing workspace assets
as design inputs.

The catalog includes only software developed by Lawkitt. Launch language is
English only, written for practicing lawyers with little technical knowledge.
Position Lawkitt around practical everyday legal work, with local processing,
accounts and AI capabilities described per project where verified. The main
visitor outcome is understanding usefulness and reaching a download; source and
contribution links are secondary.

The first clone checkpoint covers the reference homepage and download layout,
retaining reference branding. Clone and Lawkitt adaptation are separately
reviewable. Delivery is deployment-ready after local review; publication is
outside the accepted scope.

## Consequences and invariants

- The reference is a starting point; final product claims describe Lawkitt and
  its actual catalog entries.
- Own-software inclusion, English language, visitor priority and delivery boundary
  are settled. Round 2 settled routes, visual direction, implementation foundation,
  release availability handling and clone review sequence in ADR 0002.
- There are no third-party submissions, localized pages or catalog-wide claims
  that all tools process locally or have no account requirements.
- This ADR authorizes the recorded direction, not implementation before the
  user confirms the completed grilling design.
