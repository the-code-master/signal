// Module: web/libs/sim-viewer/src/lib/stimulus-audition/stimulus-audition.component.ts — FEAT-090 Stimulus Audition Component
// Invariants: <=120 LOC; Zoneless OnPush; templateUrl ONLY; plays volatile AudioBuffer in RAM with zero download routes.
// Spec: architecture/01-product-vision-and-features.md, architecture/07-security-encryption-dlp-and-sftp.md
import { ChangeDetectionStrategy, Component, computed, signal } from "@angular/core";
import { SigPanelComponent, SigStatusPillComponent } from "@signal/ui-kit";
import { buildStimulusAuditionVm, synthesizeVolatilePcmPair } from "./stimulus-audition.logic";
import type { AuditionChannelAb, StimulusPresetKind } from "./stimulus-audition.model";

@Component({
  selector: "sig-stimulus-audition",
  standalone: true,
  imports: [SigPanelComponent, SigStatusPillComponent],
  templateUrl: "./stimulus-audition.component.html",
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class StimulusAuditionComponent {
  readonly presets: readonly StimulusPresetKind[] = [
    "SINE_1KHZ_PURE",
    "PRBS7_BITSTREAM",
    "ISO_7637_LOAD_DUMP",
    "CHIRP_SWEEP",
  ];
  readonly channels: readonly AuditionChannelAb[] = ["CHANNEL_A_STIMULUS", "CHANNEL_B_FILTERED"];

  readonly activePreset = signal<StimulusPresetKind>("SINE_1KHZ_PURE");
  readonly activeChannel = signal<AuditionChannelAb>("CHANNEL_A_STIMULUS");
  readonly playbackNote = signal<string>("Ready in Volatile RAM");

  readonly pcmPair = computed(() => synthesizeVolatilePcmPair(this.activePreset()));
  readonly vm = computed(() =>
    buildStimulusAuditionVm(this.activePreset(), this.activeChannel(), this.pcmPair()),
  );

  selectPreset(preset: StimulusPresetKind): void {
    this.activePreset.set(preset);
  }

  selectChannel(ch: AuditionChannelAb): void {
    this.activeChannel.set(ch);
  }

  auditionInVolatileWebAudio(): void {
    const pair = this.pcmPair();
    const samples =
      this.activeChannel() === "CHANNEL_A_STIMULUS" ? pair.channelAFloat32 : pair.channelBFloat32;
    const AudioCtx = globalThis.AudioContext;
    if (typeof AudioCtx === "function") {
      const ctx = new AudioCtx();
      const buffer = ctx.createBuffer(1, samples.length, Number(pair.sampleRateHz));
      buffer.getChannelData(0).set(samples);
      const src = ctx.createBufferSource();
      src.buffer = buffer;
      src.connect(ctx.destination);
      src.start();
      src.onended = () => {
        void ctx.close();
      };
      this.playbackNote.set(`Auditioned ${samples.length} volatile PCM frames`);
    } else {
      this.playbackNote.set(`Synthesized ${samples.length} volatile frames (Headless)`);
    }
  }
}
