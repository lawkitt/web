#!/usr/bin/env bash
# One scripted take of the mdoc story in an isolated profile, step by step.
# Usage: capture/take.sh <profile-dir> <Dark|Light> <step...>
# Steps run in order; each records a clip into out/clips/<theme>/ and/or saves a
# still into stills/<name>-<theme>.png. Coordinates are global points for
# a 1440 × 900 window at (120, 60) (capture/session.sh).
# Steps: start detect tooltip popup alias keep address apply copy restore
#        compare scan
set -euo pipefail
cd "$(dirname "$0")/.."
home=$1 theme=$2
shift 2
lower=$(echo "$theme" | tr '[:upper:]' '[:lower:]')
clips=out/clips/$lower
mkdir -p "$clips" stills
in=out/bin/input
state=$home/take.state  # "<pid> <window-id>" of the running take

ids() { read -r pid win <"$state"; }
# rec <name> <seconds> <input commands...>: record while driving the pointer.
rec() {
  local name=$1 secs=$2
  shift 2
  out/bin/rec "$win" "$secs" "$clips/$name.mov" >/dev/null &
  local recorder=$!
  sleep 1
  "$in" front "$pid" "$@"
  wait "$recorder"
}
# still <name>: window capture with alpha, pointer parked outside the window.
still() {
  "$in" move 1650 1050 300
  sleep 0.6
  screencapture -x -o -l"$win" "stills/$1-$lower.png"
}
park() { "$in" front "$pid" move "$1" "$2" 300; sleep 0.4; }

for step in "$@"; do
  case $step in
  start)
    capture/session.sh "$home" "$theme" letter-plain scan >"$state"
    ids
    still letter
    ;;
  detect)
    ids; park 1300 520
    rec detect 10 wait 600 move 1486 91 1000 wait 450 click wait 300 move 1300 560 1000
    still review
    ;;
  tooltip)
    ids; park 1000 650
    rec tooltip 5 wait 400 move 938 536 900 wait 2200
    screencapture -x -o -l"$win" "stills/tooltip-$lower.png"
    ;;
  popup)
    ids
    rec popup 4 wait 300 click wait 1800 move 1100 760 700
    still popup
    ;;
  alias)
    ids
    rec alias 4 wait 300 move 697 571 700 wait 200 click wait 1500
    screencapture -x -o -l"$win" "stills/alias-$lower.png"
    "$in" front "$pid" key 53 wait 300 key 53 wait 300
    ;;
  keep)
    ids; park 1300 650
    rec keep 4 wait 300 move 1199 573 700 wait 300 click wait 1200 move 1330 700 600
    still keep
    ;;
  address)
    ids; "$in" front "$pid" click 900 640 wait 300; park 900 400
    rec address 9 wait 300 move 787 306 600 wait 200 drag 787 306 471 330 wait 1200 \
      move 433 302 700 wait 300 click wait 1200 move 838 344 700 wait 200 click wait 1400
    screencapture -x -o -l"$win" "stills/category-$lower.png"
    "$in" front "$pid" move 861 497 400 click wait 600 key 53 wait 300
    ;;
  apply)
    ids; park 1000 640
    rec apply 5 wait 300 move 1484 91 800 wait 300 click wait 400 move 1100 600 900 wait 1500
    "$in" front "$pid" move 1144 344 300 click wait 400
    still share
    ;;
  copy)
    ids; park 1300 700
    rec copy 4 wait 300 move 1360 266 800 wait 300 click wait 900
    screencapture -x -o -l"$win" "stills/copied-$lower.png"
    ;;
  restore)
    ids; LANG=en_US.UTF-8 pbcopy <fixture/ai-reply.md; park 1300 700
    rec restore 6 wait 300 move 1360 480 900 wait 300 click wait 2500
    still restore
    ;;
  compare)
    ids; "$in" front "$pid" move 220 236 400 click wait 800 move 900 600 300 wait 1500
    rec compare 5 wait 300 move 828 915 1000 wait 500 click wait 1200 move 1300 700 800
    still compare-panel
    "$in" front "$pid" move 1428 90 400 click wait 600
    still compare
    ;;
  scan)
    ids; "$in" front "$pid" move 220 304 400 click wait 1500
    still scan-prompt
    ;;
  *)
    echo "unknown step $step" >&2
    exit 2
    ;;
  esac
done
