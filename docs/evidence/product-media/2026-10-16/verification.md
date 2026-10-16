# Product media verification · October 16, 2026

Captured with installed `/Applications/mdoc.app` 0.1.0-beta.11 in an isolated
profile (`videos/mdoc/capture/session.sh`); the user's own mdoc session was not
used. Fixture: the fictional letter before claim in `videos/mdoc/fixture/`.

- Gallery PNGs keep transparent window corners (alpha 0 at the corners); no
  white corners in either theme. Films clip the window to its 13 px radius.
- Every capture shows the traffic lights, no notification badge, no update card,
  no spelling marks.
- `mdoc-captures inspect` found no problems in either theme (199 frames each).
- `npm run check` passed: Prettier, Astro check, tests, build, `verify-build`.
- Browser (`/mdoc/`, dev server): gallery opens on Review; Light/Dark and all
  five views load; the four films autoplay in sequence with no console errors;
  at 375 px there is no horizontal page scroll and the gallery pans.

Screenshots of these checks are attached to the pull request, not stored here.
