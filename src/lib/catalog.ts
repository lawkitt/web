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

export interface Feature {
  title: string;
  body: string;
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

// Concise feature list (ADR 0006). Claims must stay within verified mdoc behavior.
export const mdocFeatures: readonly Feature[] = [
  {
    title: "Opens what you have",
    body: "PDF, Word, Excel, PowerPoint, OpenDocument, RTF, EPUB and CSV files become clean, editable text.",
  },
  {
    title: "Reads scanned PDFs",
    body: "Text recognition runs on your computer, in English or English and Russian. About 40 pages a minute.",
  },
  {
    title: "Original side by side",
    body: "Check the converted text against the PDF or Word original while you edit. The original file is never changed.",
  },
  {
    title: "Pseudonymize",
    body: "Names, companies, emails, phone numbers, addresses and IDs are replaced with consistent aliases like PERSON_1. You approve each one.",
  },
  {
    title: "Copy for AI",
    body: "One click copies the prepared text as Markdown, ready to paste into ChatGPT or Claude.",
  },
  {
    title: "Local and fast",
    body: "No upload, no account. Works offline after a one-time model download. A 100-page PDF converts in 0.12 s.",
  },
];
export const toolPath = (tool: Tool) => `/${tool.slug}/`;

// Measured with synthetic documents; see docs/evidence/performance/.
export const mdocMetricsNote =
  "Speeds measured on a fanless MacBook Air (M4, 24 GB) with synthetic text-heavy documents. Your results depend on your computer and documents.";
