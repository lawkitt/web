import type { APIRoute } from "astro";
import { SITE } from "../lib/site";
export const GET: APIRoute = () =>
  new Response(
    import.meta.env.PUBLIC_PREVIEW === "true" || import.meta.env.DEV
      ? "User-agent: *\nDisallow: /\n"
      : `User-agent: *\nAllow: /\nSitemap: ${SITE.origin}/sitemap.xml\n`,
    { headers: { "Content-Type": "text/plain; charset=utf-8" } },
  );
