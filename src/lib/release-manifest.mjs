/** @typedef {{ platform: 'macOS' | 'Windows' | 'Linux', architecture: 'Apple Silicon' | 'Intel' | 'x64' | 'ARM64', label: string, url: string, sha256: string }} DownloadAsset */
/** @typedef {{ version: string, publishedAt: string, notesUrl: string, assets: DownloadAsset[] }} Release */
/** @typedef {{ checkedAt: string, releases: Release[] }} ReleaseManifest */
const architectures = {
  macOS: ["Apple Silicon", "Intel"],
  Windows: ["x64", "ARM64"],
  Linux: ["x64", "ARM64"],
};
/** @param {unknown} value @param {string} label */
function assertRecord(value, label) {
  if (!value || typeof value !== "object" || Array.isArray(value))
    throw new Error(`${label} must be an object`);
}
/** @param {unknown} value @param {string} label */
function date(value, label) {
  if (
    typeof value !== "string" ||
    !/^\d{4}-\d{2}-\d{2}$/.test(value) ||
    !Number.isFinite(Date.parse(value)) ||
    new Date(value).toISOString().slice(0, 10) !== value
  )
    throw new Error(`${label} must be a valid YYYY-MM-DD date`);
  return value;
}
/** @param {unknown} value @param {string} label */
function text(value, label) {
  if (typeof value !== "string" || !value.trim())
    throw new Error(`${label} must be nonempty text`);
  return value;
}
/** @param {unknown} value @param {string} label @param {RegExp} pattern */
function githubUrl(value, label, pattern) {
  const url = new URL(text(value, label));
  if (
    url.protocol !== "https:" ||
    url.hostname !== "github.com" ||
    url.port ||
    url.username ||
    url.password ||
    url.search ||
    url.hash ||
    !pattern.test(url.pathname)
  )
    throw new Error(
      `${label} must be a public lawkitt/mdoc GitHub release URL`,
    );
  return url;
}
/** Validate the maintained manifest before any installer links enter the build.
 * @param {unknown} input
 * @returns {ReleaseManifest}
 */
export function validateReleaseManifest(input) {
  assertRecord(input, "Release manifest");
  const manifest = /** @type {ReleaseManifest} */ (input);
  const checkedAt = date(manifest.checkedAt, "checkedAt");
  if (!Array.isArray(manifest.releases))
    throw new Error("releases must be an array");
  const versions = new Set();
  let previousDate = checkedAt;
  for (const release of manifest.releases) {
    assertRecord(release, "Release");
    text(release.version, "version");
    if (versions.has(release.version))
      throw new Error("Duplicate release version");
    versions.add(release.version);
    const publishedAt = date(release.publishedAt, "publishedAt");
    if (publishedAt > previousDate)
      throw new Error(
        "Releases must be newest first and published by checkedAt",
      );
    previousDate = publishedAt;
    const notes = githubUrl(
      release.notesUrl,
      "notesUrl",
      /^\/lawkitt\/mdoc\/releases\/tag\/[^/]+$/,
    );
    const tag = notes.pathname.split("/").at(-1);
    if (!Array.isArray(release.assets) || !release.assets.length)
      throw new Error(
        "A published release must have verified installer assets",
      );
    const targets = new Set();
    const urls = new Set();
    for (const asset of release.assets) {
      assertRecord(asset, "Asset");
      if (
        !Object.hasOwn(architectures, asset.platform) ||
        !architectures[asset.platform].includes(asset.architecture)
      )
        throw new Error("Unsupported platform/architecture pairing");
      text(asset.label, "Asset label");
      if (
        typeof asset.sha256 !== "string" ||
        !/^[a-fA-F0-9]{64}$/.test(asset.sha256)
      )
        throw new Error("Each installer needs a SHA-256 checksum");
      const url = githubUrl(
        asset.url,
        "Asset URL",
        /^\/lawkitt\/mdoc\/releases\/download\/[^/]+\/[^/]+$/,
      );
      if (url.pathname.split("/")[5] !== tag)
        throw new Error("Installer tag must match release notes");
      const target = `${asset.platform}/${asset.architecture}`;
      if (targets.has(target) || urls.has(asset.url))
        throw new Error("Duplicate installer target or URL");
      targets.add(target);
      urls.add(asset.url);
    }
  }
  return manifest;
}
/** @param {Release | undefined} release */
export function releaseAvailability(release) {
  return release ? "Downloads available" : "Downloads coming soon";
}
