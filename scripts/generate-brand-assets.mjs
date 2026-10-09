// Deterministic technical exports from the user-supplied brand assets.
import sharp from "sharp";
import { readFile, mkdir, writeFile } from "node:fs/promises";
await mkdir("public/social", { recursive: true });
await sharp("logo-black-bg.png")
  .resize(32, 32)
  .png()
  .toFile("public/favicon-32x32.png");
await sharp("logo-black-bg.png")
  .resize(180, 180)
  .png()
  .toFile("public/apple-touch-icon.png");
const wordmark = (await readFile("lawkitt-white-vector.svg", "utf8"))
  .replace(/<\?xml[^>]*>/g, "")
  .replace(
    /<svg[^>]*>/,
    '<svg x="72" y="66" width="200" height="54" viewBox="0 0 954 256">',
  );
const escapeXml = (value) =>
  value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
for (const [name, label, lines, sub] of [
  [
    "lawkitt",
    "FREE & OPEN-SOURCE UTILITIES",
    ["Practical tools for", "everyday legal work."],
    "lawkitt.com",
  ],
  [
    "mdoc",
    "MDOC / DOCUMENT PREPARATION",
    ["Legal documents.", "Ready for your next step."],
    "PDF & DOCX → Editable Markdown",
  ],
]) {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="630"><defs><radialGradient id="glow"><stop stop-color="#263552"/><stop offset="1" stop-color="#09090b"/></radialGradient></defs><rect width="1200" height="630" fill="#09090b"/><ellipse cx="1040" cy="60" rx="800" ry="590" fill="url(#glow)"/>${wordmark}<text x="72" y="202" fill="#89b4ff" font-family="monospace" font-size="14" letter-spacing="2">${escapeXml(label)}</text><text x="72" y="308" fill="#fafafa" font-family="Arial, sans-serif" font-weight="600" font-size="66" letter-spacing="-2">${escapeXml(lines[0])}</text><text x="72" y="388" fill="#fafafa" font-family="Arial, sans-serif" font-weight="600" font-size="66" letter-spacing="-2">${escapeXml(lines[1])}</text><path d="M72 475h1056" stroke="#ffffff" stroke-opacity=".15"/><text x="72" y="536" fill="#a1a1aa" font-family="Arial, sans-serif" font-size="22">${escapeXml(sub)}</text></svg>`;
  await writeFile(`public/social/${name}.svg`, svg);
  await sharp(Buffer.from(svg)).png().toFile(`public/social/${name}.png`);
}
console.log("Exported Lawkitt icons and social cards.");
