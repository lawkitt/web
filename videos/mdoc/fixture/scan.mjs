// Makes a page render look like an office scan: grey paper, slight skew, soft focus, noise.
import sharp from "sharp";
const [input, output, angle] = process.argv.slice(2);
const { width, height } = await sharp(input).metadata();
const noise = Buffer.alloc(width * height);
for (let i = 0; i < noise.length; i++) noise[i] = 128 + Math.round((Math.random() - 0.5) * 34);
const grain = await sharp(noise, { raw: { width, height, channels: 1 } }).png().toBuffer();
const skewed = await sharp(input)
  .grayscale()
  .rotate(Number(angle), { background: "#ffffff" })
  .resize(width, height, { fit: "cover" })
  .blur(0.6)
  .linear(0.93, 6)
  .toBuffer();
await sharp(skewed)
  .composite([{ input: grain, blend: "overlay" }])
  .grayscale()
  .jpeg({ quality: 72 })
  .toFile(output);
