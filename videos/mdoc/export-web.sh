#!/usr/bin/env bash
# Usage: videos/mdoc/export-web.sh
# Renders the ocr, pseudo and ai cuts and writes the site's copies into
# public/videos/mdoc/: 1280 px H.264 (no audio, fast start) and a WebP poster
# of each video's settled final state, shown before playback and under
# reduced motion.
set -euo pipefail
cd "$(dirname "$0")"
out=../../public/videos/mdoc
mkdir -p "$out" out
# cut, poster time (a settled frame before the fade-out)
for spec in ocr:8s pseudo:9s ai:8s; do
  cut=${spec%%:*}
  at=${spec#*:}
  cargo run --release -q -- --cut "$cut" render -o "out/mdoc-$cut.mp4"
  ffmpeg -v error -y -i "out/mdoc-$cut.mp4" -an -vf "scale=1280:-2:flags=lanczos" \
    -c:v libx264 -preset slow -crf 24 -tune animation -pix_fmt yuv420p \
    -movflags +faststart "$out/$cut.mp4"
  ffmpeg -v error -y -ss "${at%s}" -i "out/mdoc-$cut.mp4" -frames:v 1 \
    -vf "scale=1280:-2:flags=lanczos" "out/$cut-poster.png"
  cwebp -quiet -q 82 "out/$cut-poster.png" -o "$out/$cut-poster.webp"
done
ls -l "$out"
