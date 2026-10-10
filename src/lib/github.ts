// Build-time only: visitors never call the GitHub API (ADR 0002).
const SHOW_COUNT_FROM = 100;

export async function starCount(repo: string): Promise<number | undefined> {
  try {
    const response = await fetch(`https://api.github.com/repos/${repo}`, {
      headers: { accept: "application/vnd.github+json" },
      signal: AbortSignal.timeout(5000),
    });
    if (!response.ok) return undefined;
    const { stargazers_count } = (await response.json()) as {
      stargazers_count?: unknown;
    };
    return typeof stargazers_count === "number" ? stargazers_count : undefined;
  } catch {
    return undefined;
  }
}

/** "1.2k" style label, or undefined while the count is too small to show. */
export function starLabel(count: number | undefined): string | undefined {
  if (count === undefined || count < SHOW_COUNT_FROM) return undefined;
  if (count < 1000) return String(count);
  const thousands = count / 1000;
  return `${thousands < 10 ? thousands.toFixed(1).replace(/\.0$/, "") : Math.floor(thousands)}k+`;
}
