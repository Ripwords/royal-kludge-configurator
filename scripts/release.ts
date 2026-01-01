#!/usr/bin/env bun
/**
 * Script to sync versions across package.json, Cargo.toml, and tauri.conf.json
 * and optionally create a GitHub release
 *
 * Usage:
 *   bun run scripts/release.ts [version] [--create-release]
 *   bun run scripts/release.ts 0.1.2 --create-release
 *   bun run scripts/release.ts --sync-only (syncs versions without bumping)
 *   bun run scripts/release.ts --bump patch (manual bump, then syncs)
 *   bun run scripts/release.ts --bump minor
 *
 * Note: For automated releases, use changelogen --bump first, then this script
 * with --sync-only to sync versions across all files.
 */

import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const PROJECT_ROOT = import.meta.dir + "/..";

interface VersionFiles {
  packageJson: string;
  cargoToml: string;
  tauriConf: string;
}

function readVersionFiles(): VersionFiles {
  return {
    packageJson: readFileSync(join(PROJECT_ROOT, "package.json"), "utf-8"),
    cargoToml: readFileSync(
      join(PROJECT_ROOT, "src-tauri/Cargo.toml"),
      "utf-8"
    ),
    tauriConf: readFileSync(
      join(PROJECT_ROOT, "src-tauri/tauri.conf.json"),
      "utf-8"
    ),
  };
}

function getCurrentVersion(): string {
  const pkg = JSON.parse(
    readFileSync(join(PROJECT_ROOT, "package.json"), "utf-8")
  );
  return pkg.version;
}

function bumpVersion(
  version: string,
  type: "patch" | "minor" | "major"
): string {
  const [major, minor, patch] = version.split(".").map(Number);

  switch (type) {
    case "major":
      return `${major + 1}.0.0`;
    case "minor":
      return `${major}.${minor + 1}.0`;
    case "patch":
      return `${major}.${minor}.${patch + 1}`;
    default:
      throw new Error(`Invalid bump type: ${type}`);
  }
}

function updatePackageJson(version: string): void {
  const filePath = join(PROJECT_ROOT, "package.json");
  const content = JSON.parse(readFileSync(filePath, "utf-8"));
  content.version = version;
  writeFileSync(filePath, JSON.stringify(content, null, 2) + "\n");
  console.log(`✓ Updated package.json to version ${version}`);
}

function updateCargoToml(version: string): void {
  const filePath = join(PROJECT_ROOT, "src-tauri/Cargo.toml");
  let content = readFileSync(filePath, "utf-8");

  // Update version line: version = "x.y.z"
  content = content.replace(/^version = ".*"$/m, `version = "${version}"`);

  writeFileSync(filePath, content);
  console.log(`✓ Updated Cargo.toml to version ${version}`);
}

function updateTauriConf(version: string): void {
  const filePath = join(PROJECT_ROOT, "src-tauri/tauri.conf.json");
  const content = JSON.parse(readFileSync(filePath, "utf-8"));
  content.version = version;
  writeFileSync(filePath, JSON.stringify(content, null, 2) + "\n");
  console.log(`✓ Updated tauri.conf.json to version ${version}`);
}

async function createGitHubRelease(version: string): Promise<void> {
  const token = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;

  if (!token) {
    console.error(
      "❌ GITHUB_TOKEN or GH_TOKEN environment variable is required to create a release"
    );
    process.exit(1);
  }

  const tagName = `app-v${version}`;
  const releaseName = `rk-configurator v${version}`;

  console.log(`\n📦 Creating GitHub release...`);
  console.log(`   Tag: ${tagName}`);
  console.log(`   Name: ${releaseName}`);

  try {
    // Use changelogen to create the release with proper changelog
    const proc = Bun.spawn(["bunx", "changelogen@latest", "gh", "release"], {
      env: {
        ...process.env,
        GITHUB_TOKEN: token,
      },
      stdout: "inherit",
      stderr: "inherit",
    });

    await proc.exited;

    if (proc.exitCode !== 0) {
      throw new Error(`changelogen exited with code ${proc.exitCode}`);
    }

    console.log(`✓ GitHub release created successfully`);
  } catch (error) {
    console.error("❌ Failed to create GitHub release:", error);
    process.exit(1);
  }
}

async function main() {
  const args = process.argv.slice(2);

  let targetVersion: string | null = null;
  let shouldCreateRelease = false;
  let bumpType: "patch" | "minor" | "major" | null = null;
  let syncOnly = false;

  // Parse arguments
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg === "--create-release" || arg === "--release") {
      shouldCreateRelease = true;
    } else if (arg === "--sync-only" || arg === "--sync") {
      syncOnly = true;
    } else if (arg === "--bump") {
      const type = args[i + 1];
      if (type === "patch" || type === "minor" || type === "major") {
        bumpType = type;
        i++; // Skip next arg
      } else {
        console.error(
          `❌ Invalid bump type: ${type}. Must be patch, minor, or major`
        );
        process.exit(1);
      }
    } else if (!arg.startsWith("--") && !targetVersion) {
      // Assume it's a version string if it doesn't start with --
      targetVersion = arg;
    }
  }

  // Determine target version
  if (syncOnly || (!bumpType && !targetVersion)) {
    // Sync-only mode: use current package.json version
    const files = readVersionFiles();
    const pkgVersion = JSON.parse(files.packageJson).version;
    const cargoVersion = files.cargoToml.match(/^version = "(.*)"$/m)?.[1];
    const tauriVersion = JSON.parse(files.tauriConf).version;

    console.log("📋 Current versions:");
    console.log(`   package.json: ${pkgVersion}`);
    console.log(`   Cargo.toml: ${cargoVersion || "not found"}`);
    console.log(`   tauri.conf.json: ${tauriVersion}`);

    if (pkgVersion === cargoVersion && pkgVersion === tauriVersion) {
      console.log("\n✓ All versions are already in sync!");
      if (!shouldCreateRelease) {
        return;
      }
      // Continue to create release even if versions are in sync
      targetVersion = pkgVersion;
    } else {
      console.log(
        "\n⚠️  Versions are out of sync. Syncing to package.json version..."
      );
      targetVersion = pkgVersion;
    }
  } else if (bumpType) {
    // Manual bump mode
    const currentVersion = getCurrentVersion();
    targetVersion = bumpVersion(currentVersion, bumpType);
    console.log(
      `🔄 Bumping version from ${currentVersion} to ${targetVersion} (${bumpType})`
    );
  }

  // Validate version format
  if (!/^\d+\.\d+\.\d+$/.test(targetVersion)) {
    console.error(
      `❌ Invalid version format: ${targetVersion}. Must be semver (e.g., 0.1.0)`
    );
    process.exit(1);
  }

  console.log(`\n🔄 Syncing all files to version ${targetVersion}...\n`);

  // Update all version files
  updatePackageJson(targetVersion);
  updateCargoToml(targetVersion);
  updateTauriConf(targetVersion);

  console.log(`\n✓ All version files synced to ${targetVersion}`);

  // Create GitHub release if requested
  if (shouldCreateRelease) {
    await createGitHubRelease(targetVersion);
  } else {
    console.log(
      `\n💡 Tip: Add --create-release flag to create a GitHub release`
    );
  }
}

main().catch((error) => {
  console.error("❌ Error:", error);
  process.exit(1);
});
