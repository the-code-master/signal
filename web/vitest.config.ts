// Module: web/vitest.config.ts — Vitest Workspace Alias Resolver for Pure .logic.ts & Contract Unit Tests
// Invariants: Resolves @signal/* path aliases without Zone.js or browser storage side effects.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  resolve: {
    alias: {
      "@signal/contracts": resolve(root, "../contracts/gen/ts/signal_v1.ts"),
      "@signal/design-system": resolve(root, "libs/design-system/src/index.ts"),
      "@signal/ui-kit": resolve(root, "libs/ui-kit/src/index.ts"),
      "@signal/canvas-core": resolve(root, "libs/canvas-core/src/index.ts"),
      "@signal/schematic": resolve(root, "libs/schematic/src/index.ts"),
      "@signal/sim-viewer": resolve(root, "libs/sim-viewer/src/index.ts"),
      "@signal/academy": resolve(root, "libs/academy/src/index.ts"),
      "@signal/specs": resolve(root, "libs/specs/src/index.ts"),
      "@signal/library-studio": resolve(root, "libs/library-studio/src/index.ts"),
      "@signal/visual-diff": resolve(root, "libs/visual-diff/src/index.ts"),
      "@signal/review": resolve(root, "libs/review/src/index.ts"),
      "@signal/admin": resolve(root, "libs/admin/src/index.ts"),
      "@signal/ai-copilot": resolve(root, "libs/ai-copilot/src/index.ts"),
      "@signal/pcb": resolve(root, "libs/pcb/src/index.ts"),
      "@signal/magnetics": resolve(root, "libs/magnetics/src/index.ts"),
      "@signal/power-studio": resolve(root, "libs/power-studio/src/index.ts"),
      "@signal/emc-lab": resolve(root, "libs/emc-lab/src/index.ts"),
      "@signal/safety-wcca": resolve(root, "libs/safety-wcca/src/index.ts"),
      "@signal/dfm-panel": resolve(root, "libs/dfm-panel/src/index.ts"),
      "@signal/sftp-release": resolve(root, "libs/sftp-release/src/index.ts"),
      "@signal/mcad-studio": resolve(root, "libs/mcad-studio/src/index.ts"),
      "@signal/harness-3d": resolve(root, "libs/harness-3d/src/index.ts"),
      "@signal/cam-print": resolve(root, "libs/cam-print/src/index.ts"),
      "@signal/plm-ops": resolve(root, "libs/plm-ops/src/index.ts"),
    },
  },
  test: {
    include: ["libs/**/*.spec.ts", "apps/**/*.spec.ts"],
  },
});
