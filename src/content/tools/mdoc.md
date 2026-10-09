## Documents, ready for your next step

A contract in PDF or Word format is useful to read. Editable Markdown makes its
text easier to review, revise and pass to an external AI tool. mdoc brings those
steps into one local workspace.

### PDF and Word to Markdown

Open a PDF or DOCX to convert its text and structure into editable Markdown.
Compare the result with the original preview and make corrections before handoff.
Your original file stays unchanged. Conversion does not preserve embedded images.

### Edit with the source beside you

Headings, lists and tables render as you edit. Keep the original PDF or DOCX in
view to check wording and structure. DOCX previews can render equations, charts,
SmartArt, embedded fonts and alternate content incompletely.

### Local recognition for scanned PDFs

On Apple Silicon macOS and Windows x64, explicitly choose OCR for pages that need
recognition. A one-time model and runtime download is required; recognition then
runs locally and offline. Windows x64 also needs the Microsoft Visual C++
Redistributable. Intel macOS, Windows ARM64 and Linux currently support native-text
import only. Image-only DOCX content has no app OCR path. Handwriting and complex
table reconstruction are not qualified.

<div class="review-note">
<strong>Always check the result.</strong> Conversion and OCR can omit or misread
content. Review the prepared Markdown against the source before using it.
</div>

## Prepare thoughtfully. Share deliberately.

mdoc processes documents locally. Opening a document does not send it to an AI
service. **Copy Markdown** copies your current Markdown source; you choose where
to paste it. Any external service you use has its own data-handling terms.

### Experimental pseudonymization

An optional local scan proposes replacements for identifying text. Review the
proposals, correct or add aliases, explicitly apply them, then check the remaining
text before copying. Copying does not apply pending proposals.

Automatic scanning is experimental on Apple Silicon macOS and Windows x64.
Manual review is available on every platform. Detection has known misses,
including identifying text in English, Russian and hidden Markdown source.
Dates, amounts and contextual clues may still identify the parties.

<div class="review-note experimental">
<strong>Review assistance, not guaranteed anonymization.</strong> Replacing names
does not establish that a document is safe to share. Originals, filenames,
attachments and undo history remain locally. Only prepared Markdown is copied;
alias mappings are not included.
</div>

## A few practical questions

<details>
<summary>Can I download mdoc today?</summary>
<p>Check the <a href="/tools/mdoc/download/">availability page</a> for verified
downloads and current status. The source and all of its dependencies are public,
so you can also build mdoc yourself.</p>
</details>

<details>
<summary>Does mdoc connect to an AI provider?</summary>
<p>The workflow prepares Markdown for another tool. You review it and copy it
there yourself. mdoc does not automatically submit your documents to an AI service.</p>
</details>

<details>
<summary>Which platforms and features are supported?</summary>
<p>mdoc targets macOS, Windows and Linux. Native-text import works across build
targets. Local OCR and automatic experimental pseudonymization are currently
limited to Apple Silicon macOS and Windows x64. Platform features and installer
availability are separate; see the availability page for the current status.</p>
</details>

<details>
<summary>Will the conversion match my original document?</summary>
<p>Conversion preserves text and structure rather than the original page layout.
Embedded images are not retained. Scans, complicated tables and DOCX preview
features need particular care. Keep the source in view and review the result.</p>
</details>

<details>
<summary>Is it free and open source?</summary>
<p>mdoc is free software under GPL-3.0-or-later. Its source and development are
public on GitHub, including its document-conversion library
<code>lawkitt/anydoc</code>.</p>
</details>

<details>
<summary>Where do I report an issue or contribute?</summary>
<p>Use the <a href="https://github.com/lawkitt/mdoc/issues">mdoc issue tracker</a>.
Include the platform, steps to reproduce and a synthetic example where possible.</p>
</details>
