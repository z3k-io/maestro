import fs from "fs";

// Paths to your configuration files with versions
const packageJsonPath = "package.json";
const tauriConfigPath = "src-tauri/tauri.conf.json";

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
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  const oldVersion = packageJson.version;

  const newVersion = getNewVersion(packageJson.version, type);
  packageJson.version = newVersion;

  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));

  const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, "utf8"));
  tauriConfig.package.version = newVersion;
  fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2));

  console.log(`Version bumped from ${oldVersion} to ${newVersion}`);
};

const bumpType = process.argv[2] || "patch";

updateAppVersion(bumpType);
