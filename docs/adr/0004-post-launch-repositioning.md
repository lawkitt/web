# ADR 0004: Post-launch repositioning of mdoc messaging

Date: 2026-10-10
Status: Accepted (iteration 2, Rounds 1–3)

## Context

v1 went live on 2026-10-09. Its copy is written in engineering terms
("Markdown", "native-text import") and caveats dominate the mdoc page. There is
still no mdoc release. Other `lawkitt` org projects use the Lawkitt name.

## Decision

- lawkitt.com stays a catalog of free, open-source Lawkitt utilities only. The
  VS Code extension, CLI, marketplace and hosted services are out of scope for this site.
- mdoc is the only catalog entry for the foreseeable future.
- Keep the T3-derived visual language (ADR 0002). Polish it; no rebrand.
- Fix priorities, in order: copy that lawyers understand; the path to download;
  visual polish; single-product emptiness.
- The site is designed for an mdoc release existing. Until then, offer GitHub
  "watch releases". Still no forms or tracking.
- mdoc messaging combines AI-ready document preparation, outcome-led wording and
  local privacy. The pillars are local parsing of documents including scanned PDFs;
  local pseudonymization of sensitive documents, presented as a **core** feature;
  high performance; macOS and Windows.

## Claim rules (Round 2)

- Pseudonymization: a core feature with no experimental badge. Copy must state
  that the lawyer reviews and approves every replacement. Never claim guaranteed
  anonymization. Accuracy limits live in the FAQ. (Measured EN recall is about 84%.)
- Formats: PDFs and scans, Word, Excel, PowerPoint, OpenDocument, RTF and more
  (all anydoc formats mdoc imports). The original preview covers PDF and Word only.
- Performance: qualitative wording backed by measured numbers that name the machine.
- Platforms: Apple Silicon macOS and Windows x64. Others only in the FAQ.
- Homepage: Lawkitt umbrella hero, then an mdoc landing block. mdoc page:
  pillars, workflow, privacy, FAQ, download. No warning callouts in the body.

## Copy, evidence and delivery (Round 3)

- Homepage H1 stays "Practical tools for everyday legal work.", followed by an mdoc-led sub
  and "Get mdoc" / "See how it works". mdoc H1: "Prepare legal documents for AI.
  Privately." AI assistants are named nominatively ("such as ChatGPT or Claude").
- Navigation: mdoc · Download · GitHub, with a persistent "Get mdoc".
- Performance numbers come from a purpose-built harness on synthetic documents,
  with the hardware stated. Screenshots are new captures of synthetic documents.
- First public release is signed and notarized (macOS arm64, Windows x64).
- The mdoc README intro is aligned with the site's pseudonymization framing.
- Work ships via branch → local review → PR. Production deploys need explicit approval.

## Consequences

- This supersedes ADR 0002's "pseudonymization is secondary" hierarchy and the
  "Lead mdoc with document-to-Markdown" headline direction.
- The claim that source builds need a private anydoc is outdated (anydoc is
  public) and must be corrected.
- Exact claim wording for formats, pseudonymization, performance and platforms is
  constrained by verified mdoc facts (see `docs/design/iteration-2.md`).

## Amendment (2026-10-10, after launch review)

The user asked for a less promotional hero that stresses free, open source and
local. This supersedes the Round 3 hero copy:

- H1 "Free, open-source tools for legal work."
- A factual mdoc sentence, then three facts: free with no account; open source;
  mdoc runs locally on Mac and Windows.
- Actions: "Get mdoc" and "View the source".
  The catalog section heading and intro were removed so the mdoc screenshot shows
  on first load. Local processing stays attributed to mdoc, not to all Lawkitt
  tools (ADR 0001).
