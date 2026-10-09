import type { ImageMetadata } from "astro";
import mdocIcon from "../assets/mdoc-icon.png";
import mdocScreenshot from "../assets/mdoc-screenshot.png";
import { latestMdocRelease, mdocAvailability } from "./releases";

export interface Tool {
  slug: string;
  name: string;
  category: string;
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

export const tools = [
  {
    slug: "mdoc",
    name: "mdoc",
    category: "Document preparation",
    pitch:
      "Prepare PDF and Word documents as editable Markdown. Review beside the original, then copy to your AI tool of choice.",
    headline: "Turn legal documents into editable Markdown for AI tools.",
    maturity: "In development",
    availability: mdocAvailability,
    availabilityNote: latestMdocRelease
      ? "Features vary by platform. Check downloads for available installers."
      : "Features vary by platform. Public installers are being prepared.",
    platforms: ["macOS", "Windows", "Linux"],
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
export const toolPath = (tool: Tool) => `/tools/${tool.slug}/`;
