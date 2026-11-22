// Module: web/libs/power-studio/src/lib/bode-bms-motor/bode-bms-motor.logic.spec.ts — Unit Tests
// Invariants: Verifies Type-III >=50 deg phase margin, 216S4P 800V BMS pack math, and PMSM dq0 torque.
// Spec: architecture/03-rust-eda-engine.md
import { describe, expect, it } from "vitest";
import {
  buildBodeBmsMotorVm,
  computeBmsPackVoltageUv,
  computePmsmDqTorqueNm,
} from "./bode-bms-motor.logic";

describe("bode-bms-motor.logic (FEAT-026/027/028)", () => {
  it("computes 799.2V nominal 216S pack voltage, 330.6 N·m FOC torque, and Type-III loop stability", () => {
    expect(computeBmsPackVoltageUv(216)).toBe(799_200_000n);
    expect(computePmsmDqTorqueNm(4, 0.145, 380)).toBe(330.6);

    const vm = buildBodeBmsMotorVm("TYPE_III_VOLTAGE_MODE", 216, 4);
    expect(vm.summaryTone).toBe("ok");
  });
});
