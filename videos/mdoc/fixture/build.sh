#!/usr/bin/env bash
# Builds the fictional claim letter as a scanned PDF (claim-letter-scan.pdf).
set -euo pipefail
cd "$(dirname "$0")"
chrome="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
mkdir -p out
for n in 1 2; do
  "$chrome" --headless=new --disable-gpu --hide-scrollbars --force-device-scale-factor=1 \
    --window-size=1240,1754 --screenshot="$PWD/out/page$n.png" "file://$PWD/page$n.html" 2>/dev/null
done
node scan.mjs out/page1.png out/page1.jpg 0.45
node scan.mjs out/page2.png out/page2.jpg -0.3
swift topdf.swift claim-letter-scan.pdf out/page1.jpg out/page2.jpg
ls -l claim-letter-scan.pdf
# The same letter as Word, for the review scenes (clean structure, Source pane).
python3 docx.py "Letter before claim.docx"
