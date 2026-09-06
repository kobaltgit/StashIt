import { readFileSync, mkdirSync, copyFileSync, existsSync } from 'node:fs';
import { resolve, join } from 'node:path';

const rootDir = process.cwd();
const pkgPath = join(rootDir, 'package.json');
const pkg = JSON.parse(readFileSync(pkgPath, 'utf-8'));
const version = pkg.version || '1.0.0';

const outputDir = join(rootDir, '.output');
if (!existsSync(outputDir)) {
  mkdirSync(outputDir, { recursive: true });
}

// 1. Portable EXE
const exeSource = join(rootDir, 'src-tauri', 'target', 'release', 'stashit.exe');
const exeTarget = join(outputDir, `StashIt_${version}_x64_portable.exe`);

if (existsSync(exeSource)) {
  copyFileSync(exeSource, exeTarget);
  console.log(`[build:release] Copied: ${exeTarget}`);
} else {
  console.warn(`[build:release] Warning: ${exeSource} not found.`);
}

// 2. Bundle installers (NSIS & MSI)
const bundleMsi = join(rootDir, 'src-tauri', 'target', 'release', 'bundle', 'msi', `StashIt_${version}_x64_en-US.msi`);
const bundleNsis = join(rootDir, 'src-tauri', 'target', 'release', 'bundle', 'nsis', `StashIt_${version}_x64-setup.exe`);

if (existsSync(bundleMsi)) {
  copyFileSync(bundleMsi, join(outputDir, `StashIt_${version}_x64_en-US.msi`));
  console.log(`[build:release] Copied MSI installer to .output/`);
}

if (existsSync(bundleNsis)) {
  copyFileSync(bundleNsis, join(outputDir, `StashIt_${version}_x64-setup.exe`));
  console.log(`[build:release] Copied NSIS setup to .output/`);
}

console.log('[build:release] All release artifacts placed into .output/');
