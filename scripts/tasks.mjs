// Explicit build modes and separate output keep local previews out of production.
import { spawn } from "node:child_process";
import { fileURLToPath } from "node:url";
const root = fileURLToPath(new URL("../", import.meta.url));
const task = process.argv[2];
async function run(script, args = [], preview = false) {
  const child = spawn(process.execPath, [script, ...args], {
    cwd: root,
    stdio: "inherit",
    env: { ...process.env, PUBLIC_PREVIEW: String(preview) },
  });
  const stop = () => child.kill("SIGINT");
  process.once("SIGINT", stop);
  process.once("SIGTERM", stop);
  const code = await new Promise((resolve, reject) => {
    child.once("error", reject);
    child.once("exit", (code, signal) => resolve(code ?? (signal ? 130 : 1)));
  });
  process.removeListener("SIGINT", stop);
  process.removeListener("SIGTERM", stop);
  if (code !== 0) process.exit(Number(code));
}
const astro = "node_modules/astro/bin/astro.mjs";
const wrangler = "node_modules/wrangler/bin/wrangler.js";
if (task === "build") await run(astro, ["build"]);
else if (task === "build-preview") await run(astro, ["build"], true);
else if (task === "workers-preview") {
  await run(astro, ["build"], true);
  await run("scripts/verify-build.mjs", ["--preview"]);
  await run(
    wrangler,
    [
      "dev",
      "--local",
      "--assets",
      "./.preview-dist",
      "--ip",
      "127.0.0.1",
      "--port",
      "8787",
    ],
    true,
  );
} else if (task === "deploy") {
  await run(astro, ["check"]);
  await run(astro, ["build"]);
  await run("scripts/verify-build.mjs");
  await run(wrangler, ["deploy"]);
} else throw new Error(`Unknown task: ${task}`);
