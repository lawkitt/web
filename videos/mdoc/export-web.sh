#!/usr/bin/env bash
# Renders the product page's films from the scene library (src/scenes.rs) into
# public/videos/mdoc/: <film>.mp4 (H.264, no audio, fast start) and <film>-poster.webp.
# A film is an ordered list of library scenes; reorder or swap scenes here.
set -euo pipefail
cd "$(dirname "$0")"
out=../../public/videos/mdoc
mkdir -p "$out" out
cargo build --release --locked --bin mdoc-captures
bin=target/release/mdoc-captures
# film, scenes, poster time (a settled moment)
while read -r film scenes poster; do
  "$bin" --cut "$scenes" --theme dark render -o "out/$film.mp4"
  ffmpeg -nostdin -v error -y -i "out/$film.mp4" -an -c:v copy -movflags +faststart "$out/$film.mp4"
  ffmpeg -nostdin -v error -y -ss "$poster" -i "out/$film.mp4" -frames:v 1 "out/$film-poster.png"
  cwebp -quiet -q 86 "out/$film-poster.png" -o "$out/$film-poster.webp"
done <<'FILMS'
find detect,tooltip,popup 11
fix keep,address 9
share apply,compare,copy 5.5
restore chat,restore 7.5
FILMS
ls -l "$out"
