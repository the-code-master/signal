// Module: web/libs/code-ide/src/lib/code-ide-workbench/code-ide-workbench.logic.spec.ts — Unit Tests
// Invariants: Verifies F-069 windowed LSP slice badge and FEAT-050 RTL GLIFT key-to-IO taint detection.
// Spec: architecture/07-security-encryption-dlp-and-sftp.md
import { describe, expect, it } from "vitest";
import {
  CODE_IDE_WORKBENCH_DESCRIPTOR,
  buildCodeIdeWorkbenchVm,
} from "./code-ide-workbench.logic";

describe("code-ide-workbench.logic (CU-P6-03)", () => {
  it("flags unmasked RTL key-to-UART taint path and clears when GLIFT zeroizer gate is enabled", () => {
    const tainted = buildCodeIdeWorkbenchVm(
      CODE_IDE_WORKBENCH_DESCRIPTOR,
      "SYSTEMVERILOG_RTL",
      false,
    );
    expect(tainted.gliftTone).toBe("danger");

    const clean = buildCodeIdeWorkbenchVm(
      CODE_IDE_WORKBENCH_DESCRIPTOR,
      "SYSTEMVERILOG_RTL",
      true,
    );
    expect(clean.gliftTone).toBe("ok");
    expect(clean.activeBufferRangeBadge).toContain("verible-verilog-ls");
  });
});
