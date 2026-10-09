import { defineConfig } from "astro/config";

export default defineConfig({
  site: "https://t3.codes",
  devToolbar: { enabled: false },
  server: {
    port: Number(process.env.PORT ?? 4173),
  },
});
