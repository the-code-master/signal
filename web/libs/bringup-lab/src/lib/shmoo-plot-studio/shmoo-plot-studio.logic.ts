// Module: web/libs/bringup-lab/src/lib/shmoo-plot-studio/shmoo-plot-studio.logic.ts — FEAT-088 2D/3D PVT Shmoo Grid Logic
// Invariants: Evaluates alpha-power f_max(V_dd, T) pass/fail boundary in bigint _uv/_hz/_mkelvin vs ATE silicon.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { ShmooCellPoint, ShmooPlotStudioVm } from "./shmoo-plot-studio.model";

const SHMOO_VOLTAGES_UV: readonly bigint[] = [750_000n, 800_000n, 850_000n, 900_000n];
const SHMOO_FREQS_HZ: readonly bigint[] = [350_000_000n, 400_000_000n, 450_000_000n, 500_000_000n];

const SHMOO_COLUMNS: readonly SigGridColumn[] = [
  { key: "vdd", header: "V_DD (FEAT-088)", mono: true },
  { key: "f350", header: "350 MHz", mono: true },
  { key: "f400", header: "400 MHz", mono: true },
  { key: "f450", header: "450 MHz", mono: true },
  { key: "f500", header: "500 MHz", mono: true },
];

export function evaluateMaxClockAtCornerHz(
  supplyVoltageUv: bigint,
  tempMkelvin: bigint,
): bigint {
  const vOverdriveUv = supplyVoltageUv > 350_000n ? supplyVoltageUv - 350_000n : 0n;
  const tempDeratePct = tempMkelvin > 298_150n ? (tempMkelvin - 298_150n) / 8_000n : 0n;
  const baseMaxHz = vOverdriveUv * 980n;
  return (baseMaxHz * (100n - tempDeratePct)) / 100n;
}

export function generateShmooGrid(tempMkelvin: bigint): readonly ShmooCellPoint[] {
  const out: ShmooCellPoint[] = [];
  for (const vUv of SHMOO_VOLTAGES_UV) {
    const fMaxSim = evaluateMaxClockAtCornerHz(vUv, tempMkelvin);
    const fMaxAte = fMaxSim - 4_000_000n;
    for (const fHz of SHMOO_FREQS_HZ) {
      out.push({
        cellId: `V${(vUv / 1_000n).toString(10)}_F${(fHz / 1_000_000n).toString(10)}`,
        supplyVoltageUv: vUv,
        clockFreqHz: fHz,
        tempMkelvin,
        simPassed: fHz <= fMaxSim,
        ateSiliconPassed: fHz <= fMaxAte,
      });
    }
  }
  return out;
}

export function buildShmooPlotStudioVm(
  tempMkelvin: bigint,
  selectedCellId: string,
): ShmooPlotStudioVm {
  const cells = generateShmooGrid(tempMkelvin);
  const selectedCell = cells.find((c) => c.cellId === selectedCellId) ?? cells[10];
  const passCount = cells.filter((c) => c.simPassed).length;
  const agreeCount = cells.filter((c) => c.simPassed === c.ateSiliconPassed).length;
  const passRegionPct = Number(((passCount / cells.length) * 100).toFixed(1));
  const ateAgreementPct = Number(((agreeCount / cells.length) * 100).toFixed(1));

  const shmooTone: SigStatusTone = passRegionPct >= 60 ? "ok" : "warn";
  const tempC = Number((tempMkelvin - 273_150n) / 1_000n);
  const shmooBadge = `Shmoo @ ${tempC} °C: Pass Region=${passRegionPct}% • Sim-vs-ATE Boundary Match=${ateAgreementPct}%`;
  const selectedPointBadge = `Selected ${selectedCell.cellId}: V=${(Number(selectedCell.supplyVoltageUv) / 1_000).toFixed(0)} mV, f=${(selectedCell.clockFreqHz / 1_000_000n).toString(10)} MHz -> Sim=${selectedCell.simPassed ? "PASS" : "FAIL"} / ATE=${selectedCell.ateSiliconPassed ? "PASS" : "FAIL"}`;

  const rows: SigGridRow[] = SHMOO_VOLTAGES_UV.map((vUv) => {
    const rowCells = cells.filter((c) => c.supplyVoltageUv === vUv);
    const fmt = (idx: number): string => (rowCells[idx]?.simPassed ? "PASS [●]" : "FAIL [×]");
    return {
      id: `row-${vUv.toString(10)}`,
      vdd: `${(vUv / 1_000n).toString(10)} mV`,
      f350: fmt(0),
      f400: fmt(1),
      f450: fmt(2),
      f500: fmt(3),
    };
  });

  return {
    tempMkelvin,
    tempPresetsMkelvin: [233_150n, 298_150n, 398_150n],
    cells,
    selectedCell,
    passRegionPct,
    ateAgreementPct,
    shmooBadge,
    shmooTone,
    selectedPointBadge,
    columns: SHMOO_COLUMNS,
    rows,
  };
}
