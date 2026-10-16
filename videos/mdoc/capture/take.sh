#!/usr/bin/env bash
# One scripted take of the mdoc story in an isolated profile, step by step.
# Usage: capture/take.sh <profile-dir> <Dark|Light> <step...>
# Steps: start detect tooltip popup alias keep address apply copy restore compare scan
# (run them in this order; each builds on the app state the previous one left).
# Each step records a clip into out/clips/<theme>/<step>.mov and/or saves window
# stills into stills/<name>-<theme>.png. Control positions live in capture/ui.env;
# when mdoc's UI moves, re-measure there rather than editing this file.
set -euo pipefail
cd "$(dirname "$0")/.."
source capture/ui.env
home=$1 theme=$2
shift 2
lower=$(echo "$theme" | tr '[:upper:]' '[:lower:]')
clips=out/clips/$lower
mkdir -p "$clips" stills
in=out/bin/input
state=$home/take.state # "<pid> <window-id>" of the running take

# at NAME: a ui.env position as global screen points.
at() {
  local x y
  read -r x y <<<"${!1}"
  echo "$((x + WIN_X)) $((y + WIN_Y))"
}
ids() { read -r pid win <"$state"; }
# rec <name> <seconds> <input commands...>: record while driving the pointer.
# Recording starts about a second before the first command (see capture/clips.sh trims).
rec() {
  local name=$1 secs=$2
  shift 2
  out/bin/rec "$win" "$secs" "$clips/$name.mov" >/dev/null &
  local recorder=$!
  sleep 1
  "$in" front "$pid" "$@"
  wait "$recorder"
}
shot() { screencapture -x -o -l"$win" "stills/$1-$lower.png"; }
# still <name>: window capture with alpha, pointer parked outside the window.
still() {
  "$in" move $((WIN_X + WIN_W + 60)) $((WIN_Y + WIN_H + 40)) 300
  sleep 0.6
  shot "$1"
}
park() { "$in" front "$pid" move $(at "$1") 300; sleep 0.4; }

for step in "$@"; do
  case $step in
  start)
    capture/session.sh "$home" "$theme" letter-plain scan >"$state"
    ids
    still letter
    ;;
  detect) # Pseudonymize, proposals land in the inspector
    ids; park REST_INSPECTOR
    rec detect 10 wait 600 move $(at PSEUDONYMIZE) 1000 wait 450 click wait 300 \
      move $(at REST_INSPECTOR) 1000
    still review
    ;;
  tooltip) # hover a mention
    ids; park REST_EDITOR
    rec tooltip 5 wait 400 move $(at MENTION_PRIYA) 900 wait 2200
    shot tooltip
    ;;
  popup) # click it: mention bar and the identity's mentions
    ids
    rec popup 4 wait 300 click wait 1800 move $(at REST_EDITOR) 700
    still popup
    ;;
  alias) # "Same as…" dropdown
    ids
    rec alias 4 wait 300 move $(at ALIAS_FIELD) 700 wait 200 click wait 1500
    shot alias
    "$in" front "$pid" key 53 wait 300 key 53 wait 300
    ;;
  keep) # untick a false positive
    ids; park REST_INSPECTOR
    rec keep 4 wait 300 move $(at KEEP_COURTS) 700 wait 300 click wait 1200 \
      move $(at REST_INSPECTOR) 600
    still keep
    ;;
  address) # select a missed address, Replace, file it as Address
    ids; "$in" front "$pid" click $(at DESELECT) wait 300; park REST_EDITOR
    rec address 9 wait 300 move $(at MISSED_FROM) 600 wait 200 drag $(at MISSED_FROM) $(at MISSED_TO) \
      wait 1200 move $(at REPLACE_CHIP) 700 wait 300 click wait 1200 move $(at CATEGORY) 700 \
      wait 200 click wait 1400
    shot category
    "$in" front "$pid" move $(at CATEGORY_ADDRESS) 400 click wait 600 key 53 wait 300
    ;;
  apply) # Apply: replacement motion, Share opens
    ids; park REST_EDITOR
    rec apply 5 wait 300 move $(at APPLY) 800 wait 300 click wait 400 move $(at REST_EDITOR) 900 wait 1500
    "$in" front "$pid" move $(at BAR_CLOSE) 300 click wait 400
    still share
    ;;
  copy) # Copy for AI
    ids; park REST_INSPECTOR
    rec copy 4 wait 300 move $(at COPY_FOR_AI) 800 wait 300 click wait 900
    shot copied
    ;;
  restore) # paste the alias-only AI reply back
    ids; LANG=en_US.UTF-8 pbcopy <fixture/ai-reply.md; park REST_INSPECTOR
    rec restore 6 wait 300 move $(at PASTE_REPLY) 900 wait 300 click wait 2500
    still restore
    ;;
  compare) # back to the letter, Side by side from the view pill
    ids; "$in" front "$pid" move $(at TAB_LETTER) 400 click wait 800 move $(at PILL_HOVER) 300 wait 1500
    rec compare 5 wait 300 move $(at SIDE_BY_SIDE) 1000 wait 500 click wait 1200 move $(at REST_INSPECTOR) 800
    still compare-panel
    "$in" front "$pid" move $(at PANEL_TOGGLE) 400 click wait 600
    still compare
    ;;
  scan) # the scanned PDF asks for text recognition
    ids; "$in" front "$pid" move $(at TAB_SCAN) 400 click wait 1500
    still scan-prompt
    ;;
  *)
    echo "unknown step $step" >&2
    exit 2
    ;;
  esac
done
