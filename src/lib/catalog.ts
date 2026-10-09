import type { ImageMetadata } from "astro";
import mdocIcon from "../assets/mdoc-icon.png";
import mdocScreenshot from "../assets/mdoc-screenshot.png";
import { latestMdocRelease, mdocAvailability } from "./releases";

export interface Tool {
  slug: string;
  name: string;
  category: string;
  tagline: string;
  pitch: string;
  headline: string;
  maturity: "In development" | "Beta" | "Stable";
  availability: string;
  availabilityNote: string;
  platforms: readonly string[];
  source: string;
  issues: string;
  license: string;
  icon: ImageMetadata;
  screenshot: ImageMetadata;
  screenshotAlt: string;
  screenshotCaption: string;
}

export interface Pillar {
  title: string;
  body: string;
  detail?: string;
}

export const tools = [
  {
    slug: "mdoc",
    name: "mdoc",
    category: "Document preparation",
    tagline: "Prepare legal documents for AI. Privately.",
    pitch:
      "Open PDFs, scans and Word files, pseudonymize sensitive details, and copy clean text into any AI tool. Everything runs locally on your Mac or PC.",
    headline: "Prepare legal documents for AI. Privately.",
    maturity: "In development",
    availability: mdocAvailability,
    availabilityNote: latestMdocRelease
      ? "For Apple Silicon Macs and Windows PCs."
      : "Public installers are being prepared. Watch the repository for the first release.",
    platforms: ["macOS (Apple Silicon)", "Windows"],
    source: "https://github.com/lawkitt/mdoc",
    issues: "https://github.com/lawkitt/mdoc/issues",
    license: "GPL-3.0-or-later",
    icon: mdocIcon,
    screenshot: mdocScreenshot,
    screenshotAlt:
      "Real mdoc window: a synthetic services agreement converted to editable Markdown on the left, with its original DOCX preview on the right.",
    screenshotCaption: "Synthetic contract. Real workflow.",
  },
] satisfies Tool[];

export const mdoc = tools[0]!;

export const mdocFormats =
  "PDFs and scans, Word, Excel, PowerPoint, OpenDocument, RTF and more";

// The four ADR 0004 pillars. Claims must stay within verified mdoc behavior.
export const mdocPillars: readonly Pillar[] = [
  {
    title: "Every common document, even scans",
    body: "Open PDFs, Word, Excel, PowerPoint, OpenDocument and RTF files. mdoc reads their text and structure, and recognizes scanned PDF pages on your computer.",
  },
  {
    title: "Pseudonymize sensitive details",
    body: "mdoc finds names, companies, emails, phone numbers, addresses and ID numbers, and proposes consistent replacements. You review and approve every one before you copy.",
  },
  {
    title: "Fast and native",
    body: "A desktop app built in Rust, not a web page. Nothing uploads, nothing waits on a server, and large documents stay responsive.",
  },
  {
    title: "On your Mac or PC",
    body: "Made for Apple Silicon Macs and Windows. Reading, recognition and pseudonymization all run locally and work offline after a one-time setup.",
  },
];
export const toolPath = (tool: Tool) => `/tools/${tool.slug}/`;

export interface Metric {
  value: string;
  label: string;
}

// Measured with synthetic documents; see docs/evidence/performance/. Leave
// empty rather than publishing an estimate.
export const mdocMetrics: readonly Metric[] = [];
export const mdocMetricsNote = "";
