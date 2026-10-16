# Product media verification · October 16, 2026

Prepared on `codex/refresh-product-media` in both mdoc and web.
Captured installed `/Applications/mdoc.app` version `0.1.0-beta.10`. The mdoc
application code was not changed. Fictional agreement inputs, capture instructions
and SHA-256 inventory are in mdoc's `docs/product-media/`.

## Website

`npm run check` passed after the final asset/markup changes: Prettier, Astro
(26 files, zero errors/warnings/hints), all nine tests, production build, and
verification of four pages, internal links/assets/anchors, metadata and CSP.
`git diff --check` passed in both repositories. README image and local document
links resolve. Website and film source captures match the mdoc originals byte for
byte. Native capture bytes were preserved as JPEGs; Astro delivers WebP variants.

Local browser checks used `http://127.0.0.1:4173/mdoc/`:

- All six Convert / Review / Compare × Light / Dark states loaded successfully.
- Desktop at 1440 × 1050: inspected both appearances and the active film player.
- Mobile at 390 × 844: no document-level horizontal overflow; screenshots retain
  horizontal scrolling to keep document text readable. The hint remains visible.
- At 320 × 844 all screenshot controls stay within the card and wrap onto two rows.
- Video selection loads the selected MP4; the active video plays and inactive
  videos pause. Native playback controls remain available.
- Under `prefers-reduced-motion: reduce`, all videos stayed paused after selection;
  posters and controls remained available. ArrowRight moved the focused Review tab
  to Compare with the corresponding panel selected.

Review images: `desktop-dark.png`, `desktop-light.png`, `desktop-film.png`,
`mobile-gallery.png`, `mobile-320.png`, `mobile-film.png`.

## Films

Skia/Metal fframes release build and `export-web.sh` passed. The current film is
21 seconds, with three seven-second cuts. Inspection of all 630 frames reported
no findings; storyboard and full-size keyframes were visually inspected. These
are editorial films of still captures, not live screen recordings.

Final files were checked with ffprobe and fully decoded with FFmpeg: H.264,
yuv420p, 1920 × 1080, 30 fps, 210 frames per short cut / 630 combined, correct
7.0 / 21.0 second durations, fast-start metadata and no audio track. Silence is
intentional. Reports and the storyboard live in `videos/mdoc/review/`.

Browser playback and technical inspection establish the checks above. A human
watch-through of the full film has not been recorded. These checks preceded
publication; production deployment results are recorded separately.
