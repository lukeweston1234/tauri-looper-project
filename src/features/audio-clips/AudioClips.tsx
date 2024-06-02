import { Accessor, For } from "solid-js";
import { AudioClip } from "./models/AudioClip";
import AudioVisualization from "./AudioVisualization";

export default function AudioClips(props: { clips: Accessor<AudioClip[]> }) {
  return (
    <div class="flex h-full w-full">
      <div class="flex flex-1 flex-col">
        <For each={props.clips()}>
          {(clip, i) => (
            <div class="flex">
              <div
                class={`flex-1 border-t-2 border-appPrimary p-3 ${i() + 1 === props.clips().length ? "border-b-2" : ""}`}
              >
                <AudioVisualization downsampledData={clip} />
              </div>
              <div class="h-full w-[96px]" />
            </div>
          )}
        </For>
      </div>
    </div>
  );
}
