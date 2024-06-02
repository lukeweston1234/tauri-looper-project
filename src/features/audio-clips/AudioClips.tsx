import { Accessor, For } from "solid-js";
import { AudioClip } from "./models/AudioClip";
import AudioVisualization from "./AudioVisualization";

export default function AudioClips(props: { clips: Accessor<AudioClip[]> }) {
  return (
    <div class="flex h-full w-full">
      <div class="flex w-full flex-col overflow-y-auto">
        <For each={props.clips()}>
          {(clip, i) => (
            <div class="flex">
              <div
                class={`relative flex-1 border-t-2 border-appPrimary p-3 ${i() + 1 === props.clips().length ? "border-b-2" : ""}`}
              >
                <AudioVisualization downsampledData={clip} />
                <span class="absolute left-1 top-1 text-sm">
                  {i() <= 9 ? `0${i()}` : i()}
                </span>
              </div>
              <div class="h-full w-[96px]" />
            </div>
          )}
        </For>
      </div>
    </div>
  );
}
