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
    },
  },
  test: {
    include: ["libs/**/*.spec.ts", "apps/**/*.spec.ts"],
  },
});
