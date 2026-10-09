# Capture procedure

The screenshot was taken through the native-app `cua_repl` API from an existing
local mdoc development binary, opened on `fixtures/Services Agreement.docx`.
The entire captured app window was saved without editing its contents.
`src/assets/mdoc-screenshot.png` is the same source image; Astro creates its
responsive WebP exports during build. No generated UI or personal documents.

Browser QA used an isolated `agent-browser` session and local Wrangler runtime.
Representative commands, after reading `agent-browser skills get core`:

```sh
agent-browser --session lawkitt-stage2-qa open http://127.0.0.1:8787/
agent-browser --session lawkitt-stage2-qa set viewport 1440 1000
agent-browser --session lawkitt-stage2-qa wait --fn 'document.fonts.status === "loaded" && Array.from(document.images).every(i => i.complete)'
agent-browser --session lawkitt-stage2-qa wait --fn 'document.getAnimations().every(a => a.playState === "finished")'
agent-browser --session lawkitt-stage2-qa screenshot --full /absolute/output/home-1440.png
agent-browser --session lawkitt-stage2-qa a11y --json
agent-browser --session lawkitt-stage2-qa close
```

Repeat for the four routes and widths listed in verification.md. Use ordinary
motion for captures; verify `set media dark reduced-motion` separately.
