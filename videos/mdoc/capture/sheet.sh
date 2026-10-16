#!/usr/bin/env bash
# Contact sheet of a raw recording, one frame every 0.5 s (left to right, top to
# bottom: frame n is at n × 0.5 s), to choose its trim window in capture/clips.sh.
# Usage: capture/sheet.sh <scene> [dark|light]  ->  out/sheet-<scene>.png
set -euo pipefail
cd "$(dirname "$0")/.."
ffmpeg -nostdin -v error -y -i "out/clips/${2:-dark}/$1.mov" \
  -vf "fps=2,scale=480:-1,tile=6x4:padding=4:color=gray" -frames:v 1 "out/sheet-$1.png"
echo "out/sheet-$1.png"
