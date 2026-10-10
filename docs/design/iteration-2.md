# Lawkitt marketing site — iteration 2 grilling

Started: 2026-10-09, after the first version went live at https://lawkitt.com.
Prior decisions: `marketing-site.md`, ADR 0001–0003. This file records the
post-launch review. Nothing here is confirmed until marked SETTLED.

## Verified inputs (2026-10-09)

- Live site: homepage (hero, single mdoc card, three principles, GitHub panel),
  `/tools/mdoc/`, `/tools/mdoc/download/` ("Downloads coming soon").
- mdoc still has **no GitHub releases**; manifest is empty.
- **Stale claim:** site and README say source builds need the *private*
  `lawkitt/anydoc`. `lawkitt/anydoc` is now **public** (mdoc pins it by git rev)
  and its Cargo.toml has no further git dependencies. Public source builds look
  possible; not yet confirmed by a clean build.
- Other `lawkitt` org repos that could be catalog candidates or brand conflicts:
  `lawkitt-marketplace` (public; skills/MCP/modes "for the Lawkitt ecosystem —
  Lawkitt (VS Code extension), Lawkitt CLI"), private `extension` (VS Code
  extension named "Lawkitt", uses a hosted OCR endpoint), `editor` (VS Code
  build), `word-add-in`, `markdown2docx`, `docx2markdown`, `office`, plus
  private `backend`, `api-gateway`, `zitadel` (auth).
- Private `lawkitt/brand` repo has `animation/`, `icon/`, `logo/`;
  `lawkitt/theme` exists (color/font customization). The site currently uses the
  T3 reference palette, not these.

## Facts checked for Round 2 (mdoc @ 2026-10-09 HEAD)

- ~~mdoc opens only PDF, DOCX and Markdown~~ **Corrected 2026-10-10:** mdoc imports
  every anydoc format (`src/import.rs:10`): PDF (scans via local OCR), DOC/DOCX,
  PPT/PPTX, XLS/XLSX, ODT/ODS/ODP, RTF, EPUB, CSV, plus Markdown/text. Only the
  side-by-side original preview is limited to PDF and DOCX (`src/workspace.rs:603`).
  OCR applies to PDFs only.
- Local OCR and automatic pseudonymization: **Apple Silicon macOS and Windows x64
  only**. Intel macOS, Windows ARM64 and Linux import native text only.
- Pseudonymization quality on the synthetic qualification set (GLiNER2 FP16,
  threshold 0.5): EN recall 83.9%, RU recall 68.8%. Roughly one English
  identifier in six is missed. The mdoc README still calls the feature experimental.
- Performance: no published user-facing benchmark. `perf_tests.rs` exists in
  mdoc and gpui-pdf.

## Design tree

1. Site identity [SETTLED Q1: free OSS utilities catalog only; no ecosystem/paid products]
2. Catalog scope [SETTLED Q2: mdoc only for the foreseeable future]
3. Problem priority [SETTLED Q3: copy for lawyers > no download > T3 look > single-product emptiness]
4. Visual identity [SETTLED Q4: keep T3-derived look; polish only]
5. Conversion without installers [SETTLED Q5: mdoc release is the real blocker; site designed for it existing; GitHub "watch releases" meanwhile; no forms]
6. mdoc message [SETTLED Q6: combine document prep for AI, outcome-led, privacy.
   Pillars: parse any document incl. scanned PDFs locally; local pseudonymization
   for sensitive documents as a **core** feature; high performance; macOS + Windows]
   - Pseudonymization wording [SETTLED Q7(b): core feature, no "experimental" badge;
     one plain sentence nearby that the lawyer reviews and approves every replacement;
     detailed accuracy and limits only in the FAQ. No claim of guaranteed anonymization.]
   - Formats [SETTLED Q8, revised after the fact correction: name the real list —
     PDFs and scans, Word, Excel, PowerPoint, OpenDocument, RTF and more. Preview
     beside the original is PDF/Word only. Never "any document" unqualified.]
   - Performance [SETTLED Q9(c): qualitative headline plus 1–3 measured numbers,
     naming the machine. Numbers published only after measurement.]
   - Platforms [SETTLED Q10(a): market macOS (Apple Silicon) + Windows x64 only;
     other platforms only in the FAQ. First release excludes Intel macOS and Windows ARM.]
7. Homepage [SETTLED Q11(b): Lawkitt umbrella hero, then an mdoc landing block
   (pillars, workflow, download); small collection framing; becomes a grid when a
   second tool exists.]
8. mdoc page [SETTLED Q12: hero → screenshot → four pillars → Open/Review/Copy →
   privacy statement → FAQ (all limits) → download. No callout boxes in the body.]
9. Release [SETTLED Q13(c): build the release-ready state now, keep the GitHub
   "watch releases" fallback until a release lands. Signing: SETTLED Q19(a) —
   signed + notarized macOS arm64 and Windows x64 before any marketing push.]
10. Homepage hero [SETTLED Q14]: eyebrow "Free & open source · Runs on your computer";
    H1 "Practical tools for everyday legal work."; sub naming mdoc: PDFs, scans and
    Word files ready for ChatGPT or Claude, client names replaced before anything
    leaves your computer; CTAs "Get mdoc" (or Watch releases) + "See how it works".
    Drop "Made for the work between the big decisions".
11. mdoc headline [SETTLED Q15(a)]: "Prepare legal documents for AI. Privately."
    Sub: "Open PDFs, scans and Word files, pseudonymize sensitive details, and copy
    clean text into any AI tool. Everything runs locally on your Mac or PC."
    "Markdown" appears once, explained in the workflow.
12. Brand mentions [SETTLED Q16(a)]: "such as ChatGPT or Claude" in body text,
    no logos.
13. Performance evidence [SETTLED Q17]: new harness, synthetic docs, on the user's
    Mac (chip stated): 100-page PDF open+convert; OCR pages/min; pseudonymization
    scan of ~20 pages. Publish only flattering numbers; otherwise words only.
14. mdoc repo alignment [SETTLED Q18(a)]: update only the mdoc README intro to
    the "core, reviewed by you" framing, in a separate mdoc commit.
15. Screenshots [SETTLED Q20(a)]: Claude captures from a local mdoc build with
    synthetic documents: pseudonymization review, OCR beside source, clean workspace.
16. Navigation [SETTLED Q21(a)]: mdoc · Download · GitHub + persistent "Get mdoc".
17. Delivery [SETTLED Q22]: branch → local desktop+phone preview → user review → PR.
    Deploy to lawkitt.com only on explicit go.

Frontier: empty pending confirmation of shared understanding.



## Delivery

2026-10-10: user confirmed shared understanding ("yes, implement"), reviewed
PRs #1–#2 and said "deploy". #2 had merged into its stacked base, so #3 landed
the identical content on `main`. Deployed from `main` (348974a) with
`npm run deploy`: Worker version `cae2688f-a97a-4d1e-a244-aabfb4b38fb8`.
Live check: `/`, `/tools/mdoc/` and `/tools/mdoc/download/` return 200 with the
new headings; unknown paths return 404.
Open follow-ups: new app screenshots (pseudonymization, OCR), and the
pseudonymization timing (the harness run was killed by the OS).

2026-10-10: hero animation and plainer hero (ADR 0005, ADR 0004 amendment)
merged as #4 and deployed from `main` (961ac47), Worker version
`ea80b978-a2ac-4e34-999d-12deba13139a`.
