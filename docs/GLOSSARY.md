# Glossary

- **Lawkitt**: The proposed marketing website and catalog for free open-source
  software developed by Lawkitt and useful to lawyers. The site is English only.
- **Catalog entry**: A software project presented to visitors; its exact fields
  are icon, screenshot, category, name, pitch, development status, installer
  availability and Explore action. Catalog entries are on the homepage; details
  use `/tools/<slug>/` and downloads use `/tools/<slug>/download/`.
- **mdoc**: The first catalog project. A local document-preparation utility
  that turns documents into editable Markdown for use with external AI tools.
- **Reference clone**: The initial reproduction of the supplied T3 Code
  marketing homepage and download layout, retaining its reference branding as
  a separately reviewable local checkpoint.
- **Lawkitt adaptation**: The subsequent change to Lawkitt branding, catalog
  structure and actual product content.
- **Free OSS**: The user's requirement for the catalog's software to be free
  and open source. The catalog lists only Lawkitt-developed projects.
- **Deployment-ready**: A website prepared for hosting after local review;
  public publication is outside the currently approved delivery scope.
- **Development status**: The maturity of a project; separate from whether an
  installer is currently published.
- **Installer availability**: Whether verified public downloadable artifacts
  exist. It does not establish feature parity or platform qualification.
- **Artcraft reference**: An additional catalog-structure reference requested
  by the user. Its collection/product separation and card metadata were accepted
  in Round 3 within the T3 visual language.
- **Release manifest**: Checked-in typed metadata for verified published download
  assets, maintained manually. It keeps installer availability separate from maturity.
- **Production origin**: `https://lawkitt.com`, the selected canonical origin.
- **Workers Static Assets**: The Cloudflare Workers hosting mode selected for the
  generated Astro site. Current scope prepares configuration; publication comes later.
- **Pseudonymization**: In mdoc, reviewable replacement of selected identifying
  text with consistent aliases. Marketed as a core mdoc feature (ADR 0004):
  mdoc proposes replacements, the lawyer reviews and approves each one. It does
  not establish that a document cannot be identified.
- **Pillars**: The four mdoc marketing claims (ADR 0004): local parsing of PDFs,
  scans and Word files; local pseudonymization; fast native performance; macOS
  (Apple Silicon) and Windows x64.
- **Umbrella hero**: The homepage hero carrying the Lawkitt brand line, which
  leads directly into mdoc while it is the only catalog entry.
