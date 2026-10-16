# mdoc product media

Screenshots and films for the mdoc product page, made from **scripted screen
recordings of mdoc 0.1.0-beta.11** (ADR 0007). Everything shown is a fictional
letter before claim; nothing is mocked except the neutral AI chat scene.

## Pipeline

1. `fixture/build.sh` builds the letter as a scanned PDF (`claim-letter-scan.pdf`,
   Chrome headless + sharp + PDFKit) and as Word (`Letter before claim.docx` and the
   matching `.md`, `fixture/docx.py`). `fixture/ai-reply.md` is the AI reply that
   only uses aliases.
2. `capture/take.sh <profile-dir> Dark|Light <steps...>` runs one take in an
   isolated mdoc profile (`capture/session.sh`: own `HOME`, links the installed
   OCR and pseudonymization models, spelling and update checks off, tabs and theme
   seeded through `session.json`, so the user's own session is never touched).
   It drives the window with `out/bin/input` (CGEvent input that refuses to type
   unless the capture process is frontmost), saves window stills with transparent
   corners to `stills/` (ignored) and records ProRes clips with `out/bin/rec`
   (ScreenCaptureKit, whole display cropped to the window, so macOS does not swap
   the traffic lights for its capture indicator) into `out/clips/<theme>/`.
   Build the helpers once:
   `swiftc -O capture/{winid,input,rec}.swift -o out/bin/{winid,input,rec}` (one each).
3. `capture/clips.sh` trims the recordings into `film/clip-<scene>-<theme>.mp4`
   (1440 × 900, 30 fps). `film/` is the films' runtime media folder (the
   compile-time media macro cannot hold MP4s).
4. `src/scenes.rs` is the scene library; `export-web.sh` renders the page's films
   from ordered scene lists into `public/videos/mdoc/`.

The site's gallery PNGs in `src/assets/mdoc-{scan,review,compare,copy,restore}-{dark,light}.png`
are copies of stills from a take.

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

## On the website

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

## Work on it

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

## Earlier README animation

mdoc's previous README used an animated WebP of the `readme` cut, 1200 px at 15 fps:

```sh
cargo run --release -- --cut readme render -o out/mdoc-readme.mp4
mkdir -p out/webp
ffmpeg -i out/mdoc-readme.mp4 -vf "fps=15,scale=1200:-1:flags=lanczos" out/webp/%04d.png
img2webp -loop 0 -d 67 -lossy -q 60 -m 6 out/webp/*.png -o out/demo.webp
```

Copy `out/demo.webp` to `docs/images/demo.webp` in the mdoc repository.
