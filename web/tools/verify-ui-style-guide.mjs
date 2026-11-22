// Module: web/tools/verify-ui-style-guide.mjs — Automated Angular v22+ UI Style Guide & Zero-Leak DLP Verifier
// Invariants: Enforces Angular >=22 ONLY, zero zone.js, .component.ts <=120 LOC, .component.html <=150 LOC & nesting <=2, no local CSS, zero DLP leaks.
// Spec: docs/engineering/ANGULAR_UI_STYLE_GUIDE.md, architecture/07-security-encryption-dlp-and-sftp.md
import { readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const webRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const errors = [];

const pkg = JSON.parse(readFileSync(join(webRoot, "package.json"), "utf8"));
const ngVer = String(pkg.dependencies?.["@angular/core"] ?? "");
if (!ngVer.startsWith("22.")) {
  errors.push(`package.json: @angular/core must be >=22.0.0 ONLY (found '${ngVer}')`);
}
if (pkg.dependencies?.["zone.js"] || pkg.devDependencies?.["zone.js"]) {
  errors.push("package.json: zone.js is strictly forbidden in Zoneless Angular v22+");
}

function walk(dir) {
  const out = [];
  for (const entry of readdirSync(dir)) {
    if (["node_modules", "dist", ".angular", "tools"].includes(entry)) continue;
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) out.push(...walk(full));
    else out.push(full);
  }
  return out;
}

function checkTemplateNesting(rel, lines) {
  let depth = 0;
  let maxDepth = 0;
  for (const raw of lines) {
    const l = raw.trim();
    if (/^@(if|for|switch)\b/.test(l) && l.endsWith("{")) {
      depth += 1;
      if (depth > maxDepth) maxDepth = depth;
    } else if (l === "}" && depth > 0) {
      depth -= 1;
    }
  }
  if (maxDepth > 2) errors.push(`${rel}: template @if/@for/@switch nesting depth ${maxDepth} > 2`);
}

for (const file of walk(webRoot)) {
  const rel = relative(webRoot, file);
  const content = readFileSync(file, "utf8");
  const lines = content.split(/\r?\n/);

  if (file.endsWith(".component.ts")) {
    if (lines.length > 120) errors.push(`${rel}: ${lines.length} LOC exceeds 120 LOC .component.ts limit`);
    if (/\btemplate\s*:/.test(content)) errors.push(`${rel}: inline template: is forbidden; use templateUrl`);
    if (/\bstyles\s*:/.test(content)) errors.push(`${rel}: inline styles: is forbidden; use @signal/design-system`);
    if (/\bstyleUrl(s)?\s*:/.test(content)) errors.push(`${rel}: component-local styleUrl is forbidden`);
  }

  if (file.endsWith(".component.html")) {
    if (lines.length > 150) errors.push(`${rel}: ${lines.length} LOC exceeds 150 LOC .component.html limit`);
    if (/\*ng(If|For|Switch)/.test(content)) errors.push(`${rel}: legacy *ngIf/*ngFor forbidden`);
    if (/\sstyle\s*=/.test(content)) errors.push(`${rel}: inline style="..." attribute forbidden`);
    checkTemplateNesting(rel, lines);
  }

  if ((file.endsWith(".css") || file.endsWith(".scss")) && !rel.startsWith("libs/design-system/") && !rel.startsWith("libs/ui-kit/")) {
    if (!lines[0]?.startsWith("/* @sig-style-exception:")) {
      errors.push(`${rel}: per-component stylesheet forbidden outside design-system/ui-kit without @sig-style-exception`);
    } else if (lines.length > 60) {
      errors.push(`${rel}: exempted stylesheet exceeds 60 LOC`);
    }
  }

  if ((file.endsWith(".ts") || file.endsWith(".html")) && !file.endsWith(".spec.ts")) {
    for (const bad of ["localStorage", "sessionStorage", "indexedDB", "window.print", "bypassSecurityTrust"]) {
      if (content.includes(bad)) errors.push(`${rel}: Zero-Leak DLP / security violation '${bad}'`);
    }
  }
}

if (errors.length > 0) {
  console.error("UI_STYLE_AND_DLP_GATE_FAILED:\n" + errors.map((e) => `  - ${e}`).join("\n"));
  process.exit(1);
}
console.log("[OK] Angular v22+ UI Style Guide, centralized styles, and Zero-Leak DLP gates passed.");
