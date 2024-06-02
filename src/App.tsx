import { invoke } from "@tauri-apps/api/tauri";
import Header from "./shared/ui/Header";
import AudioVisualization from "./features/audio-clips/AudioVisualization";
import { createSignal, For, Show } from "solid-js";

function App() {
  const [isRecording, setIsRecording] = createSignal<boolean>(false);
  const [clips, setClips] = createSignal<number[][]>([]);

  async function record() {
    if (isRecording()) return;
    setIsRecording(true);
    const res = (await invoke("record_clip")) as number[];
    console.log(res);
    setClips((prevState) => [...prevState, res]);
    setIsRecording(false);
  }

  async function playClips() {
    await invoke("play_clips");
  }

  return (
    <div class="flex h-full w-full flex-col bg-black">
      <Header />
      <div class="flex flex-col items-center justify-center">
        <div class="flex gap-3">
          <button
            onClick={record}
            class="border-2 border-appPrimary p-3 text-appPrimary"
          >
            {isRecording() ? "Recording" : "Record"}
          </button>
          <button
            onClick={playClips}
            class="border-2 border-appPrimary p-3 text-appPrimary"
          >
            Play
          </button>
        </div>
      </div>
      <Show when={clips().length}>
        <div class="flex flex-col gap-3">
          <For each={clips()}>
            {(clip) => (
              <div class="border-b-2 border-t-2 border-appPrimary p-3">
                <AudioVisualization downsampledData={clip} />
              </div>
            )}
          </For>
        </div>
      </Show>
    </div>
  );
}

export default App;
