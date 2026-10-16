# Current mdoc product films

The mdoc product page now uses **actual captures of installed mdoc 0.1.0-beta.10**,
made on October 16, 2026. DM Sans, charcoal and sage, restrained title motion,
and crossfades between light and dark captures. No reconstructed application UI.

| cut | subject | length |
| --- | --- | --- |
| `review` | names and consistent aliases in the Review panel | 7 s |
| `compare` | original and pseudonymized text side by side | 7 s |
| `share` | the Share panel and Copy for AI | 7 s |
| `current` | all three, for standalone playback | 21 s |

All four exports are 1920 × 1080, 30 fps, H.264/yuv420p, with fast-start MP4
metadata and **no audio track**. Silence is intentional for the product page.
Theme transitions and title motion are editorial; these are films of still captures,
not screen recordings or processing-speed demonstrations. All agreement content is
fictional. Detection can miss identifiers; the film retains a review reminder.

The mdoc repository holds the original inputs and all eight screenshots under
`docs/product-media/` and `docs/images/`. This project's `media/` holds the six
unaltered review/comparison/Share images used by the films. Web screenshot assets
in `src/assets/` additionally include Word conversion beside the source document.

## Build, inspect and watch

```sh
# From videos/mdoc; uses Skia on Metal, needs FFmpeg libraries and cwebp.
cargo build --release --locked --bin mdoc-captures
cargo run --release --bin mdoc-captures -- timeline
cargo run --release --bin mdoc-captures -- inspect --all-frames --fail-on warning
cargo run --release --bin mdoc-captures -- strip all -n 12 -o review/capture-storyboard.png
cargo run --release --bin mdoc-captures -- frame Review@2s,Compare@5s,Share@2s -o review/frames
cargo run --release --bin mdoc-captures -- audio analyze
cargo run --release --bin mdoc-captures -- preview
./export-web.sh
```

`src/current.rs` defines the film; `src/captures.rs` exposes the CLI and GPU player.
`--cut review|compare|share|current` selects a cut (default: `current`).
`export-web.sh` writes the four MP4s and three WebP posters into `public/videos/mdoc/`.
`review/capture-storyboard.png` and the JSON inspection/timeline/audio/export reports
record validation. Website playback retains native controls, pauses off screen,
and respects reduced motion. The complete film is also linked below the player.

## Earlier illustrated films (retained as editable sources)

The following describes the earlier `mdoc-videos` binary. These cuts are no longer
used on the current product page. `cargo run` still defaults to this legacy binary;
use `--bin mdoc-captures` for the current films.

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
