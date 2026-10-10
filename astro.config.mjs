import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://lawkitt.com",
  output: "static",
  outDir: process.env.PUBLIC_PREVIEW === "true" ? "./.preview-dist" : "./dist",
  trailingSlash: "always",
  vite: {
    build: {
      // Executable scripts must remain external for script-src 'self'.
      assetsInlineLimit: (filePath) =>
        filePath.endsWith(".js") ? false : undefined,
    },
  },
  devToolbar: { enabled: false },
  server: { port: Number(process.env.PORT ?? 4173) },
});
