import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://lawkitt.com",
  output: "static",
  outDir: process.env.PUBLIC_PREVIEW === "true" ? "./.preview-dist" : "./dist",
  trailingSlash: "always",
  devToolbar: { enabled: false },
  server: { port: Number(process.env.PORT ?? 4173) },
});
