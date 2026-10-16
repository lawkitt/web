import type { ImageMetadata } from "astro";
import mdocIcon from "../assets/mdoc-icon.png";
import sourceDark from "../assets/mdoc-source-dark.jpg";
import sourceLight from "../assets/mdoc-source-light.jpg";
import reviewDark from "../assets/mdoc-review-dark.jpg";
import reviewLight from "../assets/mdoc-review-light.jpg";
import compareDark from "../assets/mdoc-compare-dark.jpg";
import compareLight from "../assets/mdoc-compare-light.jpg";
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
  captures: readonly {
    id: string;
    label: string;
    dark: ImageMetadata;
    light: ImageMetadata;
    alt: string;
  }[];
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
    maturity: !latestMdocRelease
      ? "In development"
      : latestMdocRelease.prerelease
        ? "Beta"
        : "Stable",
    availability: mdocAvailability,
    availabilityNote: latestMdocRelease
      ? "For Apple Silicon Macs and Windows PCs."
      : "Public installers are being prepared. Watch the repository for the first release.",
    platforms: ["macOS (Apple Silicon)", "Windows"],
    source: "https://github.com/lawkitt/mdoc",
    issues: "https://github.com/lawkitt/mdoc/issues",
    license: "GPL-3.0-or-later",
    icon: mdocIcon,
    captures: [
      {
        id: "source",
        label: "Convert",
        dark: sourceDark,
        light: sourceLight,
        alt: "mdoc showing a fictional Word services agreement as editable Markdown beside its original document.",
      },
      {
        id: "review",
        label: "Review",
        dark: reviewDark,
        light: reviewLight,
        alt: "mdoc reviewing highlighted names and proposed aliases in a fictional services agreement.",
      },
      {
        id: "compare",
        label: "Compare",
        dark: compareDark,
        light: compareLight,
        alt: "mdoc comparing original names with applied aliases, side by side.",
      },
    ],
    screenshotCaption: "Fictional agreement. Captured in mdoc 0.1.0-beta.10.",
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
