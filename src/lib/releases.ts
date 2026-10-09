import rawManifest from "../data/mdoc-releases.json";
import {
  validateReleaseManifest,
  releaseAvailability,
} from "./release-manifest.mjs";
export type {
  DownloadAsset,
  Release,
  ReleaseManifest,
} from "./release-manifest.mjs";

// Build-time only. No visitor or browser requests to GitHub's API.
export const mdocManifest = validateReleaseManifest(rawManifest);
export const mdocReleases = mdocManifest.releases;
export const releaseStatusCheckedAt = mdocManifest.checkedAt;
export const latestMdocRelease = mdocReleases[0];
export const mdocAvailability = releaseAvailability(latestMdocRelease);
