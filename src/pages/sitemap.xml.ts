import type { APIRoute } from "astro";
import { SITE } from "../lib/site";
export const GET: APIRoute = () =>
  new Response(
    `<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">${["/", "/tools/mdoc/", "/tools/mdoc/download/"].map((path) => `<url><loc>${SITE.origin}${path}</loc></url>`).join("")}</urlset>`,
    { headers: { "Content-Type": "application/xml; charset=utf-8" } },
  );
