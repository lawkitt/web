# mdoc explainer videos

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

`./export-web.sh` renders the `ocr`, `pseudo` and `ai` cuts into
`public/videos/mdoc/` as 1280 px H.264 files with WebP posters, which the
"See it work" section of the mdoc page (`src/components/DemoVideos.astro`)
plays in sequence.

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

## README animation

mdoc's README uses an animated WebP of the `readme` cut, 1200 px at 15 fps:

```sh
cargo run --release -- --cut readme render -o out/mdoc-readme.mp4
mkdir -p out/webp
ffmpeg -i out/mdoc-readme.mp4 -vf "fps=15,scale=1200:-1:flags=lanczos" out/webp/%04d.png
img2webp -loop 0 -d 67 -lossy -q 60 -m 6 out/webp/*.png -o out/demo.webp
```

Copy `out/demo.webp` to `docs/images/demo.webp` in the mdoc repository.
