#!/usr/bin/env bash
# Trims the raw take recordings (out/clips/<theme>/*.mov, ProRes, variable frame
# rate) into the films' runtime media: film/clip-<id>-<theme>.mp4, 1440 × 900, 30 fps CFR.
# Trim windows are "<id> <start s> <seconds>", measured from contact strips.
set -euo pipefail
cd "$(dirname "$0")/.."
while read -r id start secs; do
  for theme in dark light; do
    ffmpeg -nostdin -v error -y -ss "$start" -t "$secs" -i "out/clips/$theme/$id.mov" -an \
      -vf "fps=30,scale=1440:900:flags=lanczos,format=yuv420p" \
      -c:v libx264 -preset slow -crf 16 -tune animation "film/clip-$id-$theme.mp4"
  done
done <<'TRIMS'
detect 2.4 6.0
tooltip 1.0 3.4
popup 0.8 3.0
alias 1.0 2.5
keep 1.0 3.0
address 1.2 8.0
apply 1.2 3.3
copy 1.0 2.9
restore 1.1 4.3
compare 1.4 3.6
TRIMS
ls -l film/clip-*.mp4 | awk '{s+=$5} END {print NR " clips, " s/1e6 " MB"}'
