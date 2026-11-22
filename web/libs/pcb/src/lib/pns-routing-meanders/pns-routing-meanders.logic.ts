// Module: web/libs/pcb/src/lib/pns-routing-meanders/pns-routing-meanders.logic.ts — FEAT-012/015/017 PNS & Flight-Time Logic
// Invariants: Sums Die-to-Ball + PCB flight times in bigint femtoseconds and computes trombone meander compensation.
// Spec: architecture/03-rust-eda-engine.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type { SigGridColumn, SigGridRow, SigStatusTone } from "@signal/ui-kit";
import type {
  DiffPairFlightChannel,
  PnsRouterMode,
  PnsRoutingMeandersVm,
} from "./pns-routing-meanders.model";

export const DEFAULT_PCIE6_CHANNELS: readonly DiffPairFlightChannel[] = [
  {
    pairId: "PCIE6_TX0_DP/DN",
    dieToBallDelayFs: 42_100n,
    pcbRoutedDelayFs: 315_400n,
    targetTotalDelayFs: 358_000n,
    traceWidthNm: 85_000n,
    zDiffMohm: 85_000n,
  },
  {
    pairId: "PCIE6_TX1_DP/DN",
    dieToBallDelayFs: 48_900n,
    pcbRoutedDelayFs: 308_950n,
    targetTotalDelayFs: 358_000n,
    traceWidthNm: 85_000n,
    zDiffMohm: 85_000n,
  },
  {
    pairId: "DDR5_DQS0_T/C",
    dieToBallDelayFs: 31_200n,
    pcbRoutedDelayFs: 188_100n,
    targetTotalDelayFs: 220_000n,
    traceWidthNm: 90_000n,
    zDiffMohm: 90_000n,
  },
];

const FLIGHT_COLUMNS: readonly SigGridColumn[] = [
  { key: "pairId", header: "Diff Pair / Bus Trunk", mono: true },
  { key: "pkgFs", header: "Die-to-Ball (FEAT-015)", mono: true },
  { key: "pcbFs", header: "PCB Flight", mono: true },
  { key: "residualFs", header: "Residual Skew", mono: true },
  { key: "meanderLoops", header: "Trombone Loops (FEAT-012)", mono: true },
];

export function computeTotalFlightDelayFs(ch: DiffPairFlightChannel): bigint {
  return ch.dieToBallDelayFs + ch.pcbRoutedDelayFs;
}

export function computeRequiredTromboneLoops(
  deficitFs: bigint,
  tromboneAmplitudeNm: bigint,
): number {
  if (deficitFs <= 0n || tromboneAmplitudeNm <= 0n) return 0;
  const fsPerLoop = (tromboneAmplitudeNm * 12n) / 1_000n;
  if (fsPerLoop <= 0n) return 1;
  return Number((deficitFs + fsPerLoop - 1n) / fsPerLoop);
}

export function buildPnsRoutingMeandersVm(
  mode: PnsRouterMode,
  tromboneAmplitudeNm: bigint,
  channels: readonly DiffPairFlightChannel[] = DEFAULT_PCIE6_CHANNELS,
): PnsRoutingMeandersVm {
  let maxSkewFs = 0n;
  const rows: SigGridRow[] = channels.map((ch) => {
    const totalFs = computeTotalFlightDelayFs(ch);
    const deficitFs =
      ch.targetTotalDelayFs > totalFs
        ? ch.targetTotalDelayFs - totalFs
        : totalFs - ch.targetTotalDelayFs;
    if (deficitFs > maxSkewFs) maxSkewFs = deficitFs;
    const loops = computeRequiredTromboneLoops(deficitFs, tromboneAmplitudeNm);
    return {
      id: ch.pairId,
      pairId: ch.pairId,
      pkgFs: `${ch.dieToBallDelayFs.toString(10)} fs`,
      pcbFs: `${ch.pcbRoutedDelayFs.toString(10)} fs`,
      residualFs: `${deficitFs.toString(10)} fs`,
      meanderLoops: `${loops} Euler-arc loops`,
    };
  });

  const skewGateTone: SigStatusTone = maxSkewFs <= 1_000n ? "ok" : "warn";
  const skewGateBadge = `PNS (${mode}) • Max Residual Flight Skew: ${maxSkewFs.toString(10)} fs (224G Gate <= 1000 fs)`;
  const tromboneSummary = `Trombone Amplitude: ${(tromboneAmplitudeNm / 1_000n).toString(10)} um • Coupled-Arc Miter Active`;

  return {
    mode,
    modes: ["SHOVE", "WALKAROUND", "BUS_TRUNK_MULTI_LANE"],
    tromboneAmplitudeNm,
    maxIntraPairSkewFs: maxSkewFs,
    skewGateBadge,
    skewGateTone,
    tromboneSummary,
    columns: FLIGHT_COLUMNS,
    rows,
  };
}
