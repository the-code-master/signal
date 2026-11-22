// Module: web/libs/code-ide/src/index.ts — Public Barrel for @signal/code-ide
// Invariants: Sole public entrypoint for lazy route loading and CU-P6-03 Firmware/BIOS/RTL Co-Sim IDE.
// Spec: docs/engineering/ANGULAR_ENGINEERING_GUIDE.md
export { WORKBENCH_DESCRIPTOR, WORKBENCH_ROUTES } from "./code-ide.routes";
export { CodeIdeWorkbenchComponent } from "./lib/code-ide-workbench/code-ide-workbench.component";
export {
  CODE_IDE_WORKBENCH_DESCRIPTOR,
  buildCodeIdeWorkbenchVm,
} from "./lib/code-ide-workbench/code-ide-workbench.logic";
export type {
  ActiveIdeLanguage,
  CodeIdeWorkbenchVm,
} from "./lib/code-ide-workbench/code-ide-workbench.model";
export { MemmapPinmuxEnergyComponent } from "./lib/memmap-pinmux-energy/memmap-pinmux-energy.component";
export {
  DEFAULT_ENERGY_SYMBOLS,
  DEFAULT_MEMMAP_REGIONS,
  buildMemmapPinmuxEnergyVm,
  countAddressRangeOverlaps,
  estimateCoinCellYears,
} from "./lib/memmap-pinmux-energy/memmap-pinmux-energy.logic";
export type {
  FirmwareEnergySymbol,
  MemmapPinmuxEnergyVm,
  MemoryMapRegion,
} from "./lib/memmap-pinmux-energy/memmap-pinmux-energy.model";
