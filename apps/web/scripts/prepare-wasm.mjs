import { copyFile, mkdir, readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// Preserve the published wasm-bindgen distribution. No Rust build is required
// for the website and deployment cannot silently compile a different engine.
const require = createRequire(import.meta.url);
const packagePath = require.resolve("@spensercai/qimen-wasm/package.json");
const packageDirectory = dirname(packagePath);
const metadata = JSON.parse(await readFile(packagePath, "utf8"));
if (!/^\d+\.\d+\.\d+(?:-[\w.-]+)?$/.test(metadata.version)) {
  throw new Error("Invalid qimen-wasm package version");
}
const appDirectory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const output = join(
  appDirectory,
  "public",
  "wasm",
  `qimen-${metadata.version}`,
);
await mkdir(output, { recursive: true });
for (const name of ["qimen_wasm.js", "qimen_wasm_bg.wasm"]) {
  await copyFile(join(packageDirectory, name), join(output, name));
}
console.log(`Prepared published qimen-wasm ${metadata.version}`);
