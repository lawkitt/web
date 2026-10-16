# mdoc product media

Screenshots and films for the mdoc product page, made from **scripted screen
recordings of mdoc 0.1.0-beta.11** (ADR 0007). Everything shown is a fictional
letter before claim; nothing is mocked except the neutral AI chat scene.

## Rebuild everything

```sh
videos/mdoc/capture/all.sh        # from the web repo root, about 6 minutes
```

It builds the helpers, the fixture, records a dark and a light take (plus the
`court` step: *Brown v. Board* recognized by OCR, for mdoc's README), trims the
clips, copies the stills into the site gallery and renders the films. mdoc's
README media are made from the same stills and scenes (see lawkitt/mdoc
`docs/product-media/README.md`). Before you
run it:

- mdoc is installed in `/Applications` and has downloaded its OCR and
  pseudonymization models once (the capture profile links them read-only).
- The terminal running it has **Screen Recording** and **Accessibility**
  permission (System Settings → Privacy & Security).
- Google Chrome, ffmpeg, cwebp and the Xcode command line tools are installed,
  and `npm install` has run in this repo.
- Nothing else uses the mouse or keyboard while the takes run. The input helper
  refuses to type unless the capture instance is frontmost, so a stolen focus
  stops the take instead of typing into another app. Close the main mdoc window
  if you like; it is never touched (the capture runs as its own process with its
  own `HOME`).

Then look at `review/library-dark.webp` and the stills, run `npm run check`, and
open `/mdoc/` in the dev server.

## When mdoc's UI changes

1. **Moved controls:** run `capture/take.sh <profile> Dark start detect`, then
   `node capture/measure.mjs stills/review-dark.png` and read the new positions off
   `out/measure.png` (labels are window points). Update only the changed names in
   `capture/ui.env`. Steps run in order and build on each other, so run a take up
   to the step you changed and check its still before running the rest.
2. **New or changed flow:** add or edit a step in `capture/take.sh`. Each step is a
   `rec <scene> <seconds> <input commands>` (recording) and/or `still`/`shot`
   (window PNG). Input commands: `move X Y [ms]`, `click`, `dclick`, `drag X1 Y1 X2 Y2`,
   `key CODE [cmd,shift,alt,ctrl]`, `type TEXT`, `wait MS` (see `capture/input.swift`).
3. **Timing:** `capture/sheet.sh <scene>` makes a 0.5 s contact sheet of a raw
   recording; set its trim (`<scene> <start> <seconds>`) in `capture/clips.sh` and
   the same length in `LIBRARY` in `src/scenes.rs`.
4. **Different document:** edit `fixture/docx.py` (Word and Markdown) and
   `fixture/page*.html` (the scan), run `fixture/build.sh`. Positions in `ui.env`
   that point into the text (`MENTION_*`, `MISSED_*`) move with the wording.
5. **Gallery picks:** `capture/publish.sh` maps stills to gallery views; captions
   and alt text live in `src/lib/catalog.ts`.
6. **Films:** reorder or swap scenes in `export-web.sh`; captions per scene are in
   `LIBRARY` in `src/scenes.rs`.

## Pipeline

| piece | does |
| --- | --- |
| `fixture/build.sh` | the letter as a scanned PDF (Chrome headless, `scan.mjs`, `topdf.swift`) and as Word + Markdown (`docx.py`), and *Brown v. Board* as `brown-v-board.pdf` for the OCR proof; `ai-reply.md` is the alias-only AI reply |
| `capture/session.sh` | an isolated mdoc process: own `HOME`, models linked, spelling and update checks off, tabs and theme written to `session.json`, window placed per `ui.env` |
| `capture/take.sh` | drives the steps, records ProRes clips to `out/clips/<theme>/`, saves stills to `stills/` (ignored) |
| `capture/input.swift` | CGEvent pointer and keys with eased moves and the frontmost guard |
| `capture/rec.swift` | ScreenCaptureKit recording of the whole display cropped to the window, so macOS keeps the traffic lights (a window filter swaps them for its capture indicator) |
| `capture/winid.swift` | the capture window's id for `screencapture -l` |
| `capture/clips.sh` | trims recordings into `film/clip-<scene>-<theme>.mp4` (1440 × 900, 30 fps); `film/` is the films' runtime media |
| `capture/publish.sh` | stills into `src/assets/mdoc-*-{dark,light}.png` and `film/still-scan-*.png` |
| `src/scenes.rs`, `export-web.sh` | the scene library and the page films in `public/videos/mdoc/` |

## Scene library

| scene | shows | length |
| --- | --- | --- |
| `intro` | logo card | 3 s |
| `scan` | a scan asks for text recognition (still) | 3 s |
| `detect` | Pseudonymize: proposals by category | 6 s |
| `tooltip` | hover tooltip with the alias | 3.4 s |
| `popup` | a mention opens every mention of the identity | 3 s |
| `alias` | "Same as…" alias dropdown | 2.5 s |
| `keep` | untick a false positive to keep it | 3 s |
| `address` | add a missed address, file it as Address | 8 s |
| `apply` | apply all replacements | 3.3 s |
| `compare` | side by side | 3.6 s |
| `copy` | Copy for AI | 2.9 s |
| `chat` | neutral chat that only sees aliases (composited) | 4.5 s |
| `restore` | Paste AI reply restores the real names | 4.3 s |

Page films (`export-web.sh`, dark): `find` = detect, tooltip, popup; `fix` = keep,
address; `share` = apply, compare, copy; `restore` = chat, restore. Any ordered
subset renders: `--cut keep,address`, `--theme light`.

```sh
# From videos/mdoc; Skia on Metal, FFmpeg libraries and cwebp.
cargo build --release --locked --bin mdoc-captures
B=target/release/mdoc-captures
$B timeline                                  # the whole library
$B --theme light --cut all inspect
$B --cut keep,address strip all -n 12 -o review/fix.png
$B --cut detect,tooltip preview
./export-web.sh
```

`review/library-dark.webp` is a contact sheet of the whole library.

## Earlier illustrated films (retained as editable sources)

The following describes the earlier `mdoc-videos` binary. These cuts are no longer
used on the product page. `cargo run` still defaults to this legacy binary; use
`--bin mdoc-captures` for the scene library.

### Earlier mdoc explainer videos

Videos for the mdoc product page and the mdoc README, made with
[fframes](https://github.com/dmtrKovalenko/fframes) (Rust, Skia on Metal) in the
site's dark style: DM Sans, JetBrains Mono, near-black background, blue accent,
amber for detected personal data. 1920×1080 at 30 fps, no audio.

| cut | scenes | length |
| --- | --- | --- |
| `ocr` | a scanned page recognized into Markdown | 9.5 s |
| `pseudo` | names, an email and a phone number proposed, applied, listed as aliases | 10.5 s |
| `ai` | Copy, then paste into an AI chat that only sees aliases | 9.5 s |
| `hero` | the three above plus an mdoc outro | 33.5 s |
| `readme` | `ocr` then `pseudo`; the source of mdoc's `docs/images/demo.webp` | 20 s |

All content is fictitious or public domain: the services agreement and its
parties are made up (the phone number is from Ofcom's drama range), and the scan
is *Brown v. Board of Education*, 347 U.S. 483 (1954). The fonts are under the
SIL Open Font License; their licenses are in `public/fonts/`. The ChatGPT and
Claude icons in the `ai` cut are rasterized from `src/assets/ai/` and are
trademarks of OpenAI and Anthropic (see ADR 0005 and the site footer).

### Legacy files

The previous exporter rendered the `ocr`, `pseudo` and `ai` cuts into
`public/videos/mdoc/` as 1280 px H.264 files with WebP posters, which the
"See it work" section of the mdoc page (`src/components/DemoVideos.astro`)
previously played in sequence.

| file | what |
| --- | --- |
| `src/lib.rs` | the scenes and cuts |
| `src/main.rs` | the command line (`fframes::cli`) with `--cut` |
| `media/` | fonts, the scan page and the mdoc icon, embedded into the binary |
| `tests/frames.rs` | frame snapshots of the hero and a check of every frame of every cut |

### Work on the legacy cuts

Needs Rust and the ffmpeg libraries (`brew install pkg-config ffmpeg x264 x265 opus nasm ninja`).
`--cut` defaults to `hero`.

```sh
cargo run --release -- --cut pseudo timeline        # scenes and durations
cargo run --release -- --cut pseudo inspect         # missing media/fonts, clipped text, panics
cargo run --release -- --cut pseudo strip all -n 12 # contact sheet, writes strip.png
cargo run --release -- --cut pseudo frame PseudoScene@4s  # full-size PNG into frames/
cargo run --release -- --cut pseudo preview         # real-time window (space, h/l, j/k, q)
cargo run --release -- --cut pseudo render -o out/mdoc-pseudo.mp4
cargo test --release                                # snapshots (FFRAMES_UPDATE_SNAPSHOTS=1 to accept)
```

### Earlier README animation

mdoc's previous README used an animated WebP of the `readme` cut, 1200 px at 15 fps:

```sh
cargo run --release -- --cut readme render -o out/mdoc-readme.mp4
mkdir -p out/webp
ffmpeg -i out/mdoc-readme.mp4 -vf "fps=15,scale=1200:-1:flags=lanczos" out/webp/%04d.png
img2webp -loop 0 -d 67 -lossy -q 60 -m 6 out/webp/*.png -o out/demo.webp
```

Copy `out/demo.webp` to `docs/images/demo.webp` in the mdoc repository.
