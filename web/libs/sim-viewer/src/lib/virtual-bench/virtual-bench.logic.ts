// Module: web/libs/sim-viewer/src/lib/virtual-bench/virtual-bench.logic.ts — Pure DSO, 32-Ch Logic, FFT ENOB & Smith Chart Math
// Invariants: Computes exact ENOB=(SINAD-1.76)/6.02 and conformal Γ->Z Smith impedance; flattens all 4 modes into uniform grid VMs.
// Spec: architecture/01-product-vision-and-features.md, architecture/17-extended-feature-specifications-ui-and-rust.md
import type {
  BenchInstrumentMode,
  BenchModeOption,
  SmithImpedancePoint,
  VirtualBenchVm,
} from "./virtual-bench.model";

export const BENCH_MODE_OPTIONS: readonly BenchModeOption[] = [
  { id: "DSO_4CH", label: "4-Ch DSO" },
  { id: "LOGIC_32CH", label: "32-Ch Logic Analyzer" },
  { id: "FFT_SPECTRUM", label: "FFT Spectrum (THD/ENOB)" },
  { id: "VNA_2PORT_SMITH", label: "2-Port VNA Smith Chart" },
];

export function computeEnobFromSinadDb(sinadDb: number): number {
  return Number(((sinadDb - 1.76) / 6.02).toFixed(2));
}

export function gammaToSmithImpedance(
  freqHz: bigint,
  gammaReal: number,
  gammaImag: number,
  z0Ohm = 50,
): SmithImpedancePoint {
  const denom = (1 - gammaReal) * (1 - gammaReal) + gammaImag * gammaImag;
  const safeDenom = denom > 1e-12 ? denom : 1e-12;
  const normR = (1 - gammaReal * gammaReal - gammaImag * gammaImag) / safeDenom;
  const normX = (2 * gammaImag) / safeDenom;
  const mag = Math.min(0.9999, Math.hypot(gammaReal, gammaImag));
  const vswr = (1 + mag) / (1 - mag);
  const returnLossDb = mag > 1e-9 ? -20 * Math.log10(mag) : 180;
  return {
    freqHz,
    zRealOhm: Number((z0Ohm * normR).toFixed(2)),
    zImagOhm: Number((z0Ohm * normX).toFixed(2)),
    vswr: Number(vswr.toFixed(2)),
    returnLossDb: Number(returnLossDb.toFixed(1)),
  };
}

export function buildVirtualBenchVm(
  mode: BenchInstrumentMode,
  rbwHz: bigint,
  triggerUv: bigint,
): VirtualBenchVm {
  if (mode === "DSO_4CH") {
    return {
      activeMode: mode,
      modes: BENCH_MODE_OPTIONS,
      instrumentTitle: "4-Channel Mixed-Signal DSO (20 GSa/s Equivalent)",
      metrics: [
        { id: "m1", label: `Trig CH1: ${(triggerUv / 1_000_000n).toString(10)} V Rising`, tone: "ok" },
        { id: "m2", label: "Timebase: 500_000 ps/div", tone: "neutral" },
        { id: "m3", label: "Dead-Time Skew: 42_000 ps", tone: "ok" },
      ],
      columns: [
        { key: "ch", header: "Channel", mono: true },
        { key: "probe", header: "Probed Signal", mono: true },
        { key: "vpp", header: "Vpp (uV)", mono: true },
        { key: "rms", header: "RMS Metric", mono: true },
      ],
      rows: [
        { id: "c1", ch: "CH1", probe: "V(HV_SW_PHASE_U)", vpp: "804_200_000 uV", rms: "565_800_000 uV" },
        { id: "c2", ch: "CH2", probe: "V(GATE_HS_U)", vpp: "18_100_000 uV", rms: "12_700_000 uV" },
        { id: "c3", ch: "CH3", probe: "V(GATE_LS_U)", vpp: "18_050_000 uV", rms: "12_650_000 uV" },
        { id: "c4", ch: "CH4", probe: "I(L_PHASE_U)", vpp: "35_000 mA", rms: "425_000 mA" },
      ],
    };
  }

  if (mode === "LOGIC_32CH") {
    return {
      activeMode: mode,
      modes: BENCH_MODE_OPTIONS,
      instrumentTitle: "32-Channel Digital Bus & Protocol Decoder",
      metrics: [
        { id: "l1", label: "Active Lanes: LA[31:0]", tone: "ok" },
        { id: "l2", label: "Sample Clock: 1_000_000_000 Hz", tone: "neutral" },
        { id: "l3", label: "Setup/Hold Violations: 0", tone: "ok" },
      ],
      columns: [
        { key: "bus", header: "Bus Slice", mono: true },
        { key: "timestamp", header: "Timestamp (ps)", mono: true },
        { key: "hex", header: "Decoded Hex Word", mono: true },
        { key: "meaning", header: "Protocol Frame" },
      ],
      rows: [
        { id: "b1", bus: "LA[7:0] SPI_DRV", timestamp: "120_000 ps", hex: "0xA5_0F", meaning: "SET_DEADTIME_42NS" },
        { id: "b2", bus: "LA[13:8] PWM_6PH", timestamp: "250_000 ps", hex: "0x2A", meaning: "SVPWM_SECTOR_3" },
        { id: "b3", bus: "LA[31:16] ADC_SYNC", timestamp: "500_000 ps", hex: "0x7F1C", meaning: "ISENSE_SAMPLE_OK" },
      ],
    };
  }

  if (mode === "FFT_SPECTRUM") {
    const sinadDb = 74.0;
    const thdDb = -82.4;
    const enob = computeEnobFromSinadDb(sinadDb);
    return {
      activeMode: mode,
      modes: BENCH_MODE_OPTIONS,
      instrumentTitle: "Blackman-Harris FFT Spectrum Analyzer",
      metrics: [
        { id: "f1", label: `RBW: ${rbwHz.toString(10)} Hz`, tone: "neutral" },
        { id: "f2", label: `THD: ${thdDb} dB`, tone: "ok" },
        { id: "f3", label: `SINAD: ${sinadDb} dB`, tone: "ok" },
        { id: "f4", label: `ENOB: ${enob} bits`, tone: "ok" },
      ],
      columns: [
        { key: "harmonic", header: "Harmonic", mono: true },
        { key: "freq", header: "Frequency (Hz)", mono: true },
        { key: "magDb", header: "Magnitude (dBc)", mono: true },
      ],
      rows: [
        { id: "h1", harmonic: "f0 (Fund)", freq: "2_000_000 Hz", magDb: "0.0 dBc" },
        { id: "h2", harmonic: "2*f0 (HD2)", freq: "4_000_000 Hz", magDb: "-88.2 dBc" },
        { id: "h3", harmonic: "3*f0 (HD3)", freq: "6_000_000 Hz", magDb: "-83.9 dBc" },
      ],
    };
  }

  const pts = [
    gammaToSmithImpedance(100_000_000n, 0.02, 0.01, 50),
    gammaToSmithImpedance(1_000_000_000n, 0.08, -0.06, 50),
    gammaToSmithImpedance(2_400_000_000n, 0.22, 0.15, 50),
  ];
  return {
    activeMode: mode,
    modes: BENCH_MODE_OPTIONS,
    instrumentTitle: "2-Port Vector Network Analyzer Smith Impedance Table (Z0 = 50 Ω)",
    metrics: [
      { id: "v1", label: "Port 1-2 S11/S21 Calibrated", tone: "ok" },
      { id: "v2", label: `Min Return Loss: ${pts[2].returnLossDb} dB`, tone: "warn" },
      { id: "v3", label: `Peak VSWR: ${pts[2].vswr}:1`, tone: "ok" },
    ],
    columns: [
      { key: "freq", header: "Frequency (Hz)", mono: true },
      { key: "zComplex", header: "Z = R + jX (Ω)", mono: true },
      { key: "vswr", header: "VSWR", mono: true },
      { key: "rl", header: "Return Loss", mono: true },
    ],
    rows: pts.map((p, i) => ({
      id: `s-${i}`,
      freq: `${p.freqHz.toString(10)} Hz`,
      zComplex: `${p.zRealOhm} ${p.zImagOhm >= 0 ? "+" : "-"} j${Math.abs(p.zImagOhm)} Ω`,
      vswr: `${p.vswr}:1`,
      rl: `${p.returnLossDb} dB`,
    })),
  };
}
