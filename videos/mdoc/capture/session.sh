#!/usr/bin/env bash
# Starts a clean, isolated mdoc for captures and prints "<pid> <window-id>".
# Usage: capture/session.sh <profile-dir> <Dark|Light> <tabs...>
#   tabs: letter (the DOCX letter as Markdown with its Source pane), letter-plain
#         (the same without the Source pane), court (Brown v. Board, a real
#         scan for the OCR proof) and/or
#         scan (the scanned PDF, which asks for text recognition)
# The profile links the real OCR and pseudonymization models (read-only use) so
# nothing downloads, and turns off spelling and update checks so no squiggles or
# badges appear. Tabs and theme come from a written session.json, so no Open
# panel is driven. It never touches the user's own mdoc session or cases.
set -euo pipefail
dir="$(cd "$(dirname "$0")" && pwd)"
source "$dir/ui.env"
home=$1 theme=$2
shift 2
support="$home/Library/Application Support/mdoc"
real="$HOME/Library/Application Support/mdoc"
docs="$home/Documents/Brightwater v Northgate"
mkdir -p "$support" "$docs"
[ -f "$home/pid" ] && kill "$(cat "$home/pid")" 2>/dev/null && sleep 1
rm -rf "$support/cases" "$support/recents.json" "$docs"/*
cp "$dir/../fixture/Letter before claim.docx" "$dir/../fixture/Letter before claim.md" "$docs/"
cp "$dir/../fixture/claim-letter-scan.pdf" "$docs/Letter before claim.pdf"
cp "$dir/../fixture/brown-v-board.pdf" "$docs/Brown v. Board of Education.pdf"
ln -sfn "$real/ocr" "$support/ocr"
ln -sfn "$real/pseudonymization" "$support/pseudonymization"
python3 - "$real/settings.json" "$support" "$docs" "$theme" "$@" <<'PY'
import json, sys
real, support, docs, theme, *tabs = sys.argv[1:]
s = json.load(open(real))
s["spelling"]["enabled"] = False
s["updates"]["check_automatically"] = False
json.dump(s, open(f"{support}/settings.json", "w"), indent=2)
records = {
    "letter": {"markdown": f"{docs}/Letter before claim.md",
               "attachment": f"{docs}/Letter before claim.docx", "preview_visible": True,
               "caret": 20},
    "letter-plain": {"markdown": f"{docs}/Letter before claim.md",
                     "attachment": f"{docs}/Letter before claim.docx", "caret": 20},
    "scan": {"markdown": "", "source_only": True,
             "attachment": f"{docs}/Letter before claim.pdf", "preview_visible": True},
    "court": {"markdown": "", "source_only": True,
              "attachment": f"{docs}/Brown v. Board of Education.pdf", "preview_visible": True},
}
session = {"version": 1, "tabs": [records[t] for t in tabs], "active": 0,
           "sidebar_visible": True, "theme_choice": theme}
json.dump(session, open(f"{support}/session.json", "w"))
PY
cd "$home"
HOME="$home" nohup /Applications/mdoc.app/Contents/MacOS/mdoc >"$home/mdoc.log" 2>&1 &
echo $! >"$home/pid"
cd - >/dev/null
sleep 4
pid=$(cat "$home/pid")
osascript -e "tell application \"System Events\" to tell (first process whose unix id is $pid)
  set position of window 1 to {$WIN_X, $WIN_Y}
  set size of window 1 to {$WIN_W, $WIN_H}
  set frontmost to true
end tell" >/dev/null
sleep 0.5
echo "$pid $("$dir/../out/bin/winid" "$pid" | cut -d' ' -f1)"
