// Module: web/libs/sim-viewer/src/lib/protocol-track/protocol-track.component.ts — FEAT-053 Protocol Track Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; zero local CSS.
// Spec: architecture/03-rust-eda-engine.md, docs/engineering/ANGULAR_UI_STYLE_GUIDE.md
import { ChangeDetectionStrategy, Component, computed, output, signal } from "@angular/core";
import { SigDataGridComponent, SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildProtocolTrackVm, findNextErrorPacket } from "./protocol-track.logic";
import type { DecodedBusProtocol } from "./protocol-track.model";

@Component({
  selector: "sig-protocol-track",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent, SigDataGridComponent],
  templateUrl: "./protocol-track.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class ProtocolTrackComponent {
  readonly activeProtocol = signal<DecodedBusProtocol>("ALL");
  readonly searchQuery = signal<string>("");
  readonly focusedPacketId = signal<string>("PKT-I2C-01");
  readonly packetJumped = output<string>();

  readonly vm = computed(() =>
    buildProtocolTrackVm(
      this.activeProtocol(),
      this.searchQuery(),
      this.focusedPacketId(),
    ),
  );

  selectProtocol(proto: DecodedBusProtocol): void {
    this.activeProtocol.set(proto);
  }

  onSearchInput(event: Event): void {
    this.searchQuery.set((event.target as HTMLInputElement).value);
  }

  jumpToNextNackOrCrcError(): void {
    const nextErr = findNextErrorPacket(this.focusedPacketId());
    this.focusedPacketId.set(nextErr.packetId);
    this.packetJumped.emit(`${nextErr.busProtocol}:${nextErr.packetId}`);
  }
}
