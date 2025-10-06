import { execSync } from "child_process";
import fs from "fs";
import readline from "readline";

const packageJsonPath = "package.json";
const tauriConfigPath = "src-tauri/tauri.conf.json";
const cargoTomlPath = "src-tauri/Cargo.toml";

const getLatestTag = () => {
  try {
    const tag = execSync("git describe --tags --abbrev=0", { encoding: "utf8" }).trim();
    return tag;
  } catch (error) {
    console.error("No tags found", error);
    throw error;
  }
};

const getNewVersion = (version, type = "patch") => {
  let [major, minor, patch] = version.split(".").map(Number);

  if (type === "major") {
    major += 1;
    minor = 0;
    patch = 0;
  } else if (type === "minor") {
    minor += 1;
    patch = 0;
  } else if (type === "patch") {
    patch += 1;
  } else {
    throw new Error("Invalid version bump type. Use 'major', 'minor', or 'patch'.");
  }

  return `${major}.${minor}.${patch}`;
};

const updateAppVersion = (type) => {
  // Get the latest tag to determine current version
  const latestTag = getLatestTag();
  const currentVersion = latestTag.replace("v", "");
  const newVersion = getNewVersion(currentVersion, type);
  
  console.log(`Current version: ${currentVersion}`);
  console.log(`New version: ${newVersion}`);

  // Update package.json
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  packageJson.version = newVersion;
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));

  // Update tauri.conf.json
  const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, "utf8"));
  tauriConfig.version = newVersion;
  fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2));

  // Update Cargo.toml
  let cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
  cargoToml = cargoToml.replace(/version\s*=\s*"\d+\.\d+\.\d+"/, `version = "${newVersion}"`);
  fs.writeFileSync(cargoTomlPath, cargoToml);

  console.log(`✅ Version updated from ${currentVersion} to ${newVersion}`);

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question("Do you want to commit, tag, and push the changes? (y/n) ", (answer) => {
    if (answer.toLowerCase() === "y" || answer.toLowerCase() === "yes") {
      try {
        console.log("Committing version changes...");
        execSync(`git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml`);
        execSync(`git commit -m "Bump version to ${newVersion}"`);
        
        console.log("Creating git tag...");
        execSync(`git tag -a v${newVersion} -m "Version ${newVersion}"`);
        
        console.log("Pushing changes and tag...");
        execSync("git push origin main");
        execSync("git push origin --tags");
        
        console.log(`✅ Version ${newVersion} released successfully!`);
        console.log(`🚀 GitHub Actions will now build and publish the release automatically.`);
      } catch (error) {
        console.error("❌ Error pushing changes:", error.message);
      }
    } else {
      console.log("Changes saved locally but not committed.");
      console.log("To release later, run:");
      console.log(`  git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml`);
      console.log(`  git commit -m "Bump version to ${newVersion}"`);
      console.log(`  git tag -a v${newVersion} -m "Version ${newVersion}"`);
      console.log(`  git push origin main && git push origin --tags`);
    }

    rl.close();
  });
};

const bumpType = process.argv[2] || "patch";

if (!["major", "minor", "patch"].includes(bumpType)) {
  console.error("❌ Invalid version bump type. Use 'major', 'minor', or 'patch'.");
  process.exit(1);
}

updateAppVersion(bumpType);
