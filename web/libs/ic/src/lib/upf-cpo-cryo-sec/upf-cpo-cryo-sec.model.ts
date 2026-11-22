// Module: web/libs/ic/src/lib/upf-cpo-cryo-sec/upf-cpo-cryo-sec.model.ts — I-07 UPF, F-148 CPO/Cryo & FEAT-050 Security Models
// Invariants: Optical carrier frequencies (_hz), cryo stage temperatures (_mkelvin), and UPF rails (_uv) stay in ES2020 bigint.
// Spec: architecture/03-rust-eda-engine.md, contracts/proto/signal/v1/ic_fpga_cosim_twin.proto
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";

export interface UpfCpoCryoSecVm {
  readonly domMaskingEnabled: boolean;
  readonly cryo4kActive: boolean;
  readonly opticalCarrierHz: bigint;
  readonly sveaBasebandStepPs: bigint;
  readonly stageTempMkelvin: bigint;
  readonly maxWelchTStatistic: number;
  readonly tvlaLeakageDetected: boolean;
  readonly securityBadge: string;
  readonly securityTone: SigStatusTone;
  readonly cpoCryoBadge: string;
  readonly columns: readonly SigGridColumn[];
  readonly rows: readonly SigGridRow[];
}
