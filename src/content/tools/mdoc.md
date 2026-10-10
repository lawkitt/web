## Questions lawyers ask

<details>
<summary>Can I download mdoc today?</summary>
<p>The first public release is being prepared. The <a href="/mdoc/download/">download page</a>
always shows what is available. Until installers are published, you can watch the
repository on GitHub for the release, or build mdoc yourself from its public source.</p>
</details>

<details>
<summary>Does mdoc send my documents anywhere?</summary>
<p>No. Opening, reading, recognizing scans and pseudonymizing all happen on your
computer. mdoc connects to the internet only when you approve the one-time download of
its recognition and pseudonymization models, from GitHub and Hugging Face.
After that it works offline. Text reaches an AI tool only when you copy it and paste
it there yourself, and that tool’s own data terms then apply.</p>
</details>

<details>
<summary>How reliable is pseudonymization?</summary>
<p>It catches most names and identifiers, but not all of them. In our tests on
synthetic English legal documents it found about 84% of identifying details;
Russian text is detected less reliably. That is why mdoc only <em>proposes</em>
replacements: you review each one, add anything it missed, and approve before
copying. Dates, amounts and other context are not replaced automatically and
may still identify the parties. Pseudonymization helps you prepare a document;
it does not guarantee that the result is anonymous.</p>
</details>

<details>
<summary>Which documents can I open?</summary>
<p>PDF (including scanned PDFs), Word (DOC and DOCX), Excel (XLS and XLSX),
PowerPoint (PPT and PPTX), OpenDocument (ODT, ODS and ODP), RTF, EPUB and CSV,
as well as Markdown and plain text. The side-by-side view of the original is available
for PDF and Word files. Your original file is never changed.</p>
</details>

<details>
<summary>Will the result match my original document?</summary>
<p>mdoc keeps the text and its structure, such as headings, lists and tables,
rather than the page layout. Embedded images are not carried over, and complex
tables or unusual Word features may need a quick correction. Keep the original in
view and check the result before you rely on it.</p>
</details>

<details>
<summary>How well does it read scanned documents?</summary>
<p>Scanned PDF pages are recognized locally in English, or in English and
Russian. Clean, printed scans work best. Handwriting is not supported, and
recognition can misread characters, so check figures and names against the
original. Recognition applies to PDFs; images inside Word files are not read.</p>
</details>

<details>
<summary>Which computers does it run on?</summary>
<p>mdoc is made for Macs with Apple silicon (M1 or later) and Windows PCs (x64).
On Windows, scanned-page recognition also needs the free Microsoft Visual C++
Redistributable. Developers can build mdoc for Intel Macs, Windows on ARM and Linux;
there it reads documents with built-in text, but scan recognition and automatic
pseudonymization are not available.</p>
</details>

<details>
<summary>Is it really free?</summary>
<p>Yes. mdoc is free and open-source software under the GPL-3.0-or-later license.
Its code, including the <code>lawkitt/anydoc</code> conversion library it uses,
is public on GitHub. There is no account and no subscription.</p>
</details>

<details>
<summary>Where do I report a problem or suggest an improvement?</summary>
<p>Use the <a href="https://github.com/lawkitt/mdoc/issues">mdoc issue tracker</a>.
Please describe your computer, the steps that led to the problem, and, if possible,
a made-up example document rather than a real client file.</p>
</details>
