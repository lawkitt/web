// Check the actual static output, including internal links and metadata.
import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
const root = path.resolve(
  process.argv.includes("--preview") ? ".preview-dist" : "dist",
);
const pages = [
  "index.html",
  "mdoc/index.html",
  "mdoc/download/index.html",
  "404.html",
];
const resolveLocal = (url, page) => {
  const parsed = new URL(url, `https://lawkitt.com/${page}`);
  if (parsed.origin !== "https://lawkitt.com") return undefined;
  const pathname = decodeURIComponent(parsed.pathname);
  let file = path.join(root, pathname);
  if (pathname.endsWith("/")) file = path.join(file, "index.html");
  else if (!path.extname(pathname)) file = path.join(file, "index.html");
  return { file, hash: parsed.hash };
};
for (const page of pages) {
  const html = await readFile(path.join(root, page), "utf8");
  assert.equal((html.match(/<h1\b/g) ?? []).length, 1, `${page}: one H1`);
  assert.match(html, /<html lang="en"/, `${page}: English document`);
  for (const [, attributes, body] of html.matchAll(
    /<script\b([^>]*)>([\s\S]*?)<\/script>/g,
  )) {
    if (/\btype="application\/ld\+json"/.test(attributes)) continue;
    assert.ok(
      /\bsrc="/.test(attributes) && body.trim() === "",
      `${page}: executable scripts must be external for the content security policy`,
    );
  }
  assert.doesNotMatch(
    html,
    /T3 Code|t3\.codes|pingdotgg|nightly|testimonials/i,
    `${page}: legacy product content`,
  );
  if (page !== "404.html")
    assert.match(
      html,
      /rel="canonical" href="https:\/\/lawkitt\.com\//,
      `${page}: production canonical`,
    );
  assert.match(
    html,
    /property="og:image" content="https:\/\/lawkitt\.com\/social\//,
    `${page}: branded social card`,
  );
  const expectedRobots =
    page === "404.html" || process.argv.includes("--preview")
      ? "noindex, nofollow"
      : "index, follow";
  assert.ok(
    html.includes(`name="robots" content="${expectedRobots}"`),
    `${page}: robots policy`,
  );
  const urls = [...html.matchAll(/\b(?:href|src)="([^"]+)"/g)].map(
    (match) => match[1],
  );
  for (const set of html.matchAll(/\bsrcset="([^"]+)"/g))
    urls.push(
      ...set[1].split(",").map((entry) => entry.trim().split(/\s+/)[0]),
    );
  for (const url of urls) {
    const local = resolveLocal(url, page);
    if (!local) continue;
    assert.ok(existsSync(local.file), `${page}: missing ${url}`);
    if (local.hash) {
      const target = await readFile(local.file, "utf8");
      assert.ok(
        target.includes(`id="${decodeURIComponent(local.hash.slice(1))}"`),
        `${page}: missing anchor ${url}`,
      );
    }
  }
}
const sitemap = await readFile(path.join(root, "sitemap.xml"), "utf8");
for (const route of ["/", "/mdoc/", "/mdoc/download/"])
  assert.ok(sitemap.includes(`<loc>https://lawkitt.com${route}</loc>`));
assert.ok(!sitemap.includes("404"));
const robots = await readFile(path.join(root, "robots.txt"), "utf8");
assert.ok(
  robots.includes(
    process.argv.includes("--preview")
      ? "Disallow: /"
      : "Sitemap: https://lawkitt.com/sitemap.xml",
  ),
);
assert.ok(existsSync(path.join(root, "_headers")));
assert.ok(
  !(await readdir(root)).some((name) =>
    /install\.(sh|ps1)|nightly|harnesses|pfps/.test(name),
  ),
);
console.log(
  "Verified 4 pages, internal links/images/anchors, branding, canonical metadata, sitemap and robots policy.",
);
