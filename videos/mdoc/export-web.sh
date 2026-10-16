#!/usr/bin/env bash
# Rebuild the current product films from installed-app captures with Skia/Metal.
set -euo pipefail
cd "$(dirname "$0")"
out=../../public/videos/mdoc
mkdir -p "$out" out
cargo build --release --locked --bin mdoc-captures
for cut in review compare share current; do
  target/release/mdoc-captures --cut "$cut" render -o "out/$cut.mp4"
  # Browser delivery: video only, H.264, front-loaded MP4 metadata.
  ffmpeg -v error -y -i "out/$cut.mp4" -an -c:v copy -movflags +faststart "$out/$cut.mp4"
  if [ "$cut" != current ]; then
    ffmpeg -v error -y -ss 2 -i "out/$cut.mp4" -frames:v 1 "out/$cut-poster.png"
    cwebp -quiet -q 86 "out/$cut-poster.png" -o "$out/$cut-poster.webp"
  fi
done
