#!/usr/bin/env bash
# Copies a take's stills into the website gallery (src/assets) and the film's
# scan still (film/). Gallery view <- still: scan <- scan-prompt, review <- popup,
# compare <- compare, copy <- copied, restore <- restore.
set -euo pipefail
cd "$(dirname "$0")/.."
assets=../../src/assets
for theme in dark light; do
  cp "stills/scan-prompt-$theme.png" "$assets/mdoc-scan-$theme.png"
  cp "stills/popup-$theme.png" "$assets/mdoc-review-$theme.png"
  cp "stills/compare-$theme.png" "$assets/mdoc-compare-$theme.png"
  cp "stills/copied-$theme.png" "$assets/mdoc-copy-$theme.png"
  cp "stills/restore-$theme.png" "$assets/mdoc-restore-$theme.png"
  cp "stills/scan-prompt-$theme.png" "film/still-scan-$theme.png"
done
echo "gallery and film stills updated"
