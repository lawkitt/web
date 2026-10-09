# mdoc performance numbers (2026-10-10)

Machine: MacBook Air 15" (Mac16,13), Apple M4, 24 GB, macOS 15.7.7. Fanless; the
machine was also compiling during the runs, so figures are conservative.
Build: mdoc `main` @ 03ab81a, release profile, built from public sources only.
Harness: temporary `#[ignore]` tests (not committed to mdoc) calling
`import::prepare_cancellable` with the user's installed default models
(OCR V6Small, 150 DPI). Documents are synthetic (no real parties): monospaced
contract text, ~50 lines/page, generated with `cupsfilter`; the scan is the same
text rasterized to 200 DPI grayscale image-only pages with PDFKit.

| Measurement | Runs | Median |
| --- | ---: | ---: |
| Native-text PDF → Markdown, 100 pages (477 KB Markdown) | 7 | 0.124 s |
| Native-text PDF → Markdown, 20 pages | 7 | 0.025 s |
| OCR, 10 image-only pages (47 KB Markdown) | 3 | 14.9 s → ~40 pages/min |

Published: the 100-page conversion and ~40 pages/min.

Not published: pseudonymization scan time. The 20-page scan (95 KB) was killed
by the OS (SIGKILL after ~21 s, 1.8 GB peak RSS) in the test harness; cause not
established. Measurement was stopped when the user reported the CPU overheating.
