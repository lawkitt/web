import test from "node:test";
import assert from "node:assert/strict";
import {
  validateReleaseManifest,
  releaseAvailability,
} from "../src/lib/release-manifest.mjs";
const asset = {
  platform: "macOS",
  architecture: "Apple Silicon",
  label: "Download .dmg",
  url: "https://github.com/lawkitt/mdoc/releases/download/v1.0.0/mdoc.dmg",
  sha256: "a".repeat(64),
  signed: true,
};
const release = {
  version: "1.0.0",
  prerelease: false,
  publishedAt: "2026-10-01",
  notesUrl: "https://github.com/lawkitt/mdoc/releases/tag/v1.0.0",
  assets: [asset],
};
const manifest = () =>
  structuredClone({ checkedAt: "2026-10-09", releases: [release] });
test("an empty verified manifest keeps downloads coming soon", () => {
  const result = validateReleaseManifest({
    checkedAt: "2026-10-09",
    releases: [],
  });
  assert.equal(
    releaseAvailability(result.releases[0]),
    "Downloads coming soon",
  );
});
test("verified installers produce the available state", () => {
  const result = validateReleaseManifest(manifest());
  assert.equal(releaseAvailability(result.releases[0]), "Downloads available");
});
test("rejects unsafe, unrelated, or mismatched installer URLs", () => {
  for (const url of [
    "http://github.com/lawkitt/mdoc/releases/download/v1.0.0/mdoc.dmg",
    "https://github.com.evil.example/lawkitt/mdoc/releases/download/v1.0.0/mdoc.dmg",
    "https://github.com/other/repo/releases/download/v1.0.0/mdoc.dmg",
    "https://user:pass@github.com/lawkitt/mdoc/releases/download/v1.0.0/mdoc.dmg",
    "https://github.com/lawkitt/mdoc/releases/download/v2.0.0/mdoc.dmg",
  ]) {
    const m = manifest();
    m.releases[0].assets[0].url = url;
    assert.throws(() => validateReleaseManifest(m));
  }
});
test("rejects missing checksums, installers and unsupported targets", () => {
  for (const patch of [
    { sha256: "bad" },
    { platform: "Windows", architecture: "Intel" },
    { platform: "Android" },
  ]) {
    const m = manifest();
    Object.assign(m.releases[0].assets[0], patch);
    assert.throws(() => validateReleaseManifest(m));
  }
  const m = manifest();
  m.releases[0].assets = [];
  assert.throws(() => validateReleaseManifest(m));
});
test("rejects ambiguous duplicates and contradictory dates", () => {
  const duplicate = manifest();
  duplicate.releases[0].assets.push({ ...asset });
  assert.throws(() => validateReleaseManifest(duplicate));
  const versions = manifest();
  versions.releases.push(structuredClone(release));
  assert.throws(() => validateReleaseManifest(versions));
  for (const checkedAt of ["2026-02-30", "2026-09-01", "not a date"]) {
    const m = manifest();
    m.checkedAt = checkedAt;
    assert.throws(() => validateReleaseManifest(m));
  }
  const order = manifest();
  order.releases.push({
    ...structuredClone(release),
    version: "1.1.0",
    publishedAt: "2026-10-08",
  });
  assert.throws(() => validateReleaseManifest(order));
});
test("a pre-release is offered as a public beta", () => {
  const m = manifest();
  m.releases[0].version = "1.0.0-beta.1";
  m.releases[0].prerelease = true;
  assert.equal(
    releaseAvailability(validateReleaseManifest(m).releases[0]),
    "Public beta available",
  );
});
test("rejects a pre-release flag that contradicts the version", () => {
  const m = manifest();
  m.releases[0].prerelease = true;
  assert.throws(() => validateReleaseManifest(m));
  const beta = manifest();
  beta.releases[0].version = "1.0.0-beta.1";
  assert.throws(() => validateReleaseManifest(beta));
});
test("every installer states whether it is signed", () => {
  const m = manifest();
  delete m.releases[0].assets[0].signed;
  assert.throws(() => validateReleaseManifest(m));
});
