import test from "node:test";
import assert from "node:assert/strict";
import {
  cp,
  mkdtemp,
  readFile,
  rm,
  symlink,
  writeFile,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
const exec = promisify(execFile);
const root = fileURLToPath(new URL("../", import.meta.url));

test(
  "published installers render consistent catalog, product and download pages",
  { timeout: 60000 },
  async () => {
    // A disposable build fixture never changes the checked-in manifest or preview.
    const fixture = await mkdtemp(path.join(tmpdir(), "lawkitt-release-view-"));
    try {
      for (const file of [
        "src",
        "public",
        "scripts",
        "astro.config.mjs",
        "tsconfig.json",
        "package.json",
      ])
        await cp(path.join(root, file), path.join(fixture, file), {
          recursive: true,
        });
      await symlink(
        path.join(root, "node_modules"),
        path.join(fixture, "node_modules"),
        process.platform === "win32" ? "junction" : "dir",
      );
      const assets = [
        { platform: "macOS", architecture: "Apple Silicon", file: "mdoc.dmg" },
        { platform: "Windows", architecture: "x64", file: "mdoc.exe" },
        { platform: "Linux", architecture: "x64", file: "mdoc.AppImage" },
      ].map(({ file, ...target }) => ({
        ...target,
        label: "Download installer",
        url: `https://github.com/lawkitt/mdoc/releases/download/v1.0.0/${file}`,
        sha256: "a".repeat(64),
      }));
      await writeFile(
        path.join(fixture, "src/data/mdoc-releases.json"),
        JSON.stringify({
          checkedAt: "2026-10-09",
          releases: [
            {
              version: "1.0.0",
              publishedAt: "2026-10-01",
              notesUrl: "https://github.com/lawkitt/mdoc/releases/tag/v1.0.0",
              assets,
            },
          ],
        }),
      );
      await exec(
        process.execPath,
        [path.join(root, "node_modules/astro/bin/astro.mjs"), "build"],
        {
          cwd: fixture,
          env: {
            ...process.env,
            PUBLIC_PREVIEW: "false",
            ASTRO_TELEMETRY_DISABLED: "1",
          },
          timeout: 45000,
        },
      );
      const home = await readFile(
        path.join(fixture, "dist/index.html"),
        "utf8",
      );
      const product = await readFile(
        path.join(fixture, "dist/tools/mdoc/index.html"),
        "utf8",
      );
      const download = await readFile(
        path.join(fixture, "dist/tools/mdoc/download/index.html"),
        "utf8",
      );
      for (const page of [home, product, download]) {
        assert.match(page, /Downloads available/);
        assert.doesNotMatch(page, /Downloads coming soon/);
      }
      assert.doesNotMatch(home, /Public installers are being prepared/);
      assert.match(download, /Version 1\.0\.0/);
      assert.match(download, /datetime="2026-10-01"/);
      assert.match(download, /Release notes/);
      assert.match(download, /A note about source builds/);
      assert.match(download, /lawkitt\/anydoc/);
      assert.doesNotMatch(download, /Public downloads are coming soon/);
      for (const asset of assets) {
        assert.ok(download.includes(asset.url));
        assert.ok(download.includes(asset.sha256));
        assert.ok(
          download.includes(`for ${asset.platform} (${asset.architecture})`),
        );
      }
      await exec(
        process.execPath,
        [path.join(fixture, "scripts/verify-build.mjs")],
        { cwd: fixture },
      );
    } finally {
      await rm(fixture, { recursive: true, force: true });
    }
  },
);
