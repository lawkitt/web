# ADR 0005: Hero workflow animation

Date: 2026-10-10
Status: Accepted (design); implementation after prototype approval

## Decision

- Placement (revised by the user after the first prototype): the hero stays
  centered. The animation sits under the mdoc screenshot in the left column of
  the homepage catalog card, and stacks under it on phones.
- Storyboard (~9 s loop): a PDF or Word card arrives → enters an mdoc window and
  is parsed into clean Markdown → names glow amber and flip to blue alias chips
  (`PERSON_1`, `ORG_1`) with a "✓ approved" tick → clean text flies to a ChatGPT
  or Claude chat window. Loops alternate PDF/Word and ChatGPT/Claude.
- Labels say "Pseudonymize", never "Anonymize". The approval tick depicts that
  the lawyer approves every replacement.
- Abstract schematic style in the site palette, not a replica of the mdoc UI.
- Pure CSS keyframes on inline markup, no animation library. It pauses offscreen,
  shows a static final frame under reduced motion, and has a text alternative.
- **Supersedes ADR 0004's "no logos":** the official ChatGPT (OpenAI) and Claude
  icons appear, small and labelled, sourced from the T3 reference
  (`apps/marketing/public/harnesses/openai_dark.svg`, `claude-ai-icon.svg`).
  The footer states they are trademarks of their owners and Lawkitt is not
  affiliated with OpenAI or Anthropic. No other harness icons are copied.
- A standalone prototype is reviewed before any site change.
