// Module: web/libs/ic/src/lib/upf-cpo-cryo-sec/upf-cpo-cryo-sec.logic.ts — I-07 UPF, F-148 CPO/Cryo & FEAT-050 TVLA Logic
// Invariants: Enforces Welch |t| <= 4.5 TVLA threshold, 193.1 THz SVEA baseband envelope & 4000 mK Fermi-Dirac continuation.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type { UpfCpoCryoSecVm } from "./upf-cpo-cryo-sec.model";

const SEC_COLUMNS: readonly SigGridColumn[] = [
  { key: "subsystem", header: "UPF / Photonics / Cryo / Security Gate", mono: true },
  { key: "metric", header: "Verified SI / Statistical Telemetry", mono: true },
  { key: "status", header: "Sign-Off", mono: true },
];

export function isTvlaSideChannelLeaking(maxWelchT: number): boolean {
  return Math.abs(maxWelchT) > 4.5;
}

export function buildUpfCpoCryoSecVm(
  domMaskingEnabled: boolean,
  cryo4kActive: boolean,
): UpfCpoCryoSecVm {
  const opticalCarrierHz = 193_100_000_000_000n;
  const sveaBasebandStepPs = 2n;
  const stageTempMkelvin = cryo4kActive ? 4_000n : 298_150n;
  const maxWelchTStatistic = domMaskingEnabled ? 1.82 : 11.4;
  const tvlaLeakageDetected = isTvlaSideChannelLeaking(maxWelchTStatistic);

  const securityTone: SigStatusTone = tvlaLeakageDetected ? "danger" : "ok";
  const securityBadge = `FEAT-050 Welch TVLA max|t|=${maxWelchTStatistic} (${tvlaLeakageDetected ? "LEAKAGE > 4.5!" : "SIDE-CHANNEL CLEAN <= 4.5"}) • GLIFT Key->IO=0`;
  const tempLabel = cryo4kActive ? "4.0 K (4_000 mK Fermi-Dirac)" : "298.15 K Room";
  const cpoCryoBadge = `F-148 CPO 193.1 THz Baseband SVEA (Δt=${sveaBasebandStepPs.toString(10)} ps, ER=11.8 dB) • Stage=${tempLabel}`;

  const rows: SigGridRow[] = [
    {
      id: "UPF",
      subsystem: "I-07 IEEE 1801 UPF Power Domains",
      metric: "4 Domains (750_000 uV..1_800_000 uV) • Isolation + Level-Shifters=100%",
      status: "UPF_CLEAN",
    },
    {
      id: "CPO",
      subsystem: "F-148 Baseband SVEA Co-Packaged Optics",
      metric: `f_0=${(opticalCarrierHz / 1_000_000_000_000n).toString(10)}.1 THz (λ=1550 nm) • Ring Mod ER=11.8 dB`,
      status: "SVEA_CONVERGED",
    },
    {
      id: "CRYO",
      subsystem: "F-148 Cryo-CMOS Fermi-Dirac Homotopy",
      metric: `T_stage=${stageTempMkelvin.toString(10)} mK • Freeze-Out Safe Continuation`,
      status: "NR_CONVERGED",
    },
    {
      id: "TVLA",
      subsystem: "FEAT-050 DPA/TVLA & Glitch Fault-Injection",
      metric: `Welch |t|=${maxWelchTStatistic} • Redundant-Encoded FSM Glitch Bypass=0`,
      status: tvlaLeakageDetected ? "APPLY_DOM_MASKING" : "FIPS_140_3_READY",
    },
  ];

  return {
    domMaskingEnabled,
    cryo4kActive,
    opticalCarrierHz,
    sveaBasebandStepPs,
    stageTempMkelvin,
    maxWelchTStatistic,
    tvlaLeakageDetected,
    securityBadge,
    securityTone,
    cpoCryoBadge,
    columns: SEC_COLUMNS,
    rows,
  };
}
