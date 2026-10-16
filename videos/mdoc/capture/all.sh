#!/usr/bin/env bash
# Rebuilds every mdoc product asset from the installed app, end to end.
# Usage: videos/mdoc/capture/all.sh [profile-dir]
# Needs: /Applications/mdoc.app with its OCR and pseudonymization models already
# downloaded, Screen Recording and Accessibility permission for the terminal that
# runs this, Google Chrome, ffmpeg, cwebp, Xcode command line tools, and
# `npm install` in the web repo (sharp). The mouse and keyboard are driven for
# about four minutes: leave the computer alone until it finishes. Typing stops
# by itself if another app takes focus.
set -euo pipefail
cd "$(dirname "$0")/.."
profile=${1:-${TMPDIR:-/tmp}/mdoc-capture-profile}
export NODE_PATH=$PWD/../../node_modules

echo "1/6 helpers"
mkdir -p out/bin
for tool in winid input rec; do swiftc -O "capture/$tool.swift" -o "out/bin/$tool"; done

echo "2/6 fixture"
fixture/build.sh

echo "3/6 takes (driving mdoc)"
steps=(start detect tooltip popup alias keep address apply copy restore compare scan)
capture/take.sh "$profile" Dark "${steps[@]}"
capture/take.sh "$profile" Light "${steps[@]}"
kill "$(cat "$profile/pid")" 2>/dev/null || true

echo "4/6 clips"
capture/clips.sh

echo "5/6 stills -> gallery"
capture/publish.sh

echo "6/6 films"
./export-web.sh
target/release/mdoc-captures --cut all strip all -n 26 --columns 5 --width 480 -o out/library-dark.png
cwebp -quiet -q 80 out/library-dark.png -o review/library-dark.webp

echo "Done. Check review/library-dark.webp and the stills, then run npm run check."
