// Overlays a labelled 50-point grid on a Retina window still, so ui.env positions
// (window points) can be read straight off the image.
// Usage: node capture/measure.mjs stills/review-dark.png [out.png]
import sharp from "sharp";
const [input, output = "out/measure.png"] = process.argv.slice(2);
const { width, height } = await sharp(input).metadata();
const scale = width / 1440; // pixels per window point
let lines = "";
for (let p = 0; p <= 1440; p += 50) {
  const x = p * scale, major = p % 100 === 0;
  lines += `<line x1="${x}" y1="0" x2="${x}" y2="${height}" stroke="#ff2d55" stroke-opacity="${major ? 0.55 : 0.2}" stroke-width="${major ? 2 : 1}"/>`;
  if (major) lines += `<text x="${x + 4}" y="22" font-size="20" fill="#ff2d55" font-family="Menlo">${p}</text>`;
}
for (let p = 0; p <= 900; p += 50) {
  const y = p * scale, major = p % 100 === 0;
  lines += `<line x1="0" y1="${y}" x2="${width}" y2="${y}" stroke="#ff2d55" stroke-opacity="${major ? 0.55 : 0.2}" stroke-width="${major ? 2 : 1}"/>`;
  if (major) lines += `<text x="4" y="${y - 4}" font-size="20" fill="#ff2d55" font-family="Menlo">${p}</text>`;
}
const svg = Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}">${lines}</svg>`);
await sharp(input).composite([{ input: svg }]).png().toFile(output);
console.log(output);
