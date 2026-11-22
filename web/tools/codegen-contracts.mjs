// Module: web/tools/codegen-contracts.mjs — Build-Time TypeScript bigint Contract Generator
// Invariants: Writes ONLY to gitignored ../contracts/gen/ts/signal_v1.ts; maps 100% of 64-bit proto fields to ES2020 bigint (F-144).
// Spec: architecture/10-api-and-data-contracts.md
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const toolDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(toolDir, "../..");
const protoDir = join(repoRoot, "contracts/proto/signal/v1");
const outFile = join(repoRoot, "contracts/gen/ts/signal_v1.ts");

function mapProtoType(ty) {
  if (["int64", "uint64", "sint64", "fixed64", "sfixed64"].includes(ty)) return "bigint";
  if (["int32", "uint32", "sint32", "fixed32", "sfixed32", "double", "float"].includes(ty)) return "number";
  if (ty === "bool") return "boolean";
  if (ty === "string") return "string";
  if (ty === "bytes") return "Uint8Array";
  return ty;
}

const files = readdirSync(protoDir).filter((f) => f.endsWith(".proto")).sort();
const lines = [
  "// AUTO-GENERATED AT BUILD TIME FROM contracts/proto/signal/v1/*.proto — DO NOT COMMIT",
  "export function parseInt64Str(v: string | number | bigint): bigint { return BigInt(v); }",
  "export function formatInt64Str(v: bigint): string { return v.toString(10); }",
  "",
];

for (const file of files) {
  const src = readFileSync(join(protoDir, file), "utf8").split(/\r?\n/);
  let mode = null;
  for (const raw of src) {
    const line = raw.trim();
    if (!line || line.startsWith("//")) continue;
    if (line.startsWith("enum ")) {
      const name = line.split(/\s+/)[1];
      lines.push(`export type ${name} = string | number;`);
      mode = "enum";
      continue;
    }
    if (line.startsWith("message ")) {
      const name = line.split(/\s+/)[1].replace(/\{.*$/, "");
      if (line.endsWith("{}")) {
        lines.push(`export type ${name} = Record<string, never>;\n`);
        mode = null;
      } else {
        lines.push(`export interface ${name} {`);
        mode = "msg";
      }
      continue;
    }
    if (line === "}") {
      if (mode === "msg") lines.push("}\n");
      mode = null;
      continue;
    }
    if (mode === "msg") {
      const clean = line.replace(/\[.*?\];$/, ";").replace(/;$/, "");
      const parts = clean.split("=")[0].trim().split(/\s+/);
      if (parts.length >= 2) {
        const isRep = parts[0] === "repeated";
        const ty = mapProtoType(isRep ? parts[1] : parts[0]);
        const fname = isRep ? parts[2] : parts[1];
        lines.push(`  readonly ${fname}?: ${isRep ? `readonly ${ty}[]` : ty};`);
      }
    }
  }
}

mkdirSync(dirname(outFile), { recursive: true });
writeFileSync(outFile, lines.join("\n") + "\n", "utf8");
