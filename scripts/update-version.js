import { execSync } from "child_process";
import fs from "fs";
import readline from "readline";

const packageJsonPath = "package.json";
const tauriConfigPath = "src-tauri/tauri.conf.json";
const cargoTomlPath = "src-tauri/Cargo.toml";

const pushChanges = (newVersion) => {
  execSync("git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml");
  execSync(`git commit -m "Bump version to ${newVersion}"`);
  execSync(`git tag -a v${newVersion} -m "Version ${newVersion}"`);

  execSync("git push origin && git push origin --tags");
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
  // Update package.json
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  const oldVersion = packageJson.version;
  const newVersion = getNewVersion(packageJson.version, type);
  packageJson.version = newVersion;
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));

  // Update tauri.conf.json
  const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, "utf8"));
  tauriConfig.package.version = newVersion;
  fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2));

  // Update Cargo.toml
  let cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
  cargoToml = cargoToml.replace(/version\s*=\s*"\d+\.\d+\.\d+"/, `version = "${newVersion}"`);
  fs.writeFileSync(cargoTomlPath, cargoToml);

  console.log(`Version bumped from ${oldVersion} to ${newVersion}`);

  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question("Do you want to push the changes to the repository? (y/n) ", (answer) => {
    if (answer.toLowerCase() === "y" || answer.toLowerCase() === "yes") {
      console.log("Pushing new version...");
      pushChanges(newVersion);
    } else {
      console.log("Changes not pushed.");
    }

    rl.close();
  });
};

const bumpType = process.argv[2] || "patch";

updateAppVersion(bumpType);
