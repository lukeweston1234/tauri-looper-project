import { invoke } from "@tauri-apps/api/tauri";
import Header from "./shared/ui/Header";
import { createSignal, Show } from "solid-js";
import AudioClips from "./features/audio-clips/AudioClips";

function App() {
  const [isRecording, setIsRecording] = createSignal<boolean>(false);
  const [isPlaying, setIsPlaying] = createSignal<boolean>(false);
  const [isMetronomeOn, setIsMetronomeOn] = createSignal<boolean>(false);

  const [clips, setClips] = createSignal<number[][]>([]);

  async function record() {
    if (isRecording()) return;
    setIsRecording(true);
    const res = (await invoke("record_clip")) as number[];
    setClips((prevState) => [...prevState, res]);
    setIsRecording(false);
  }

  async function playClips() {
    if (isPlaying()) return;
    setIsPlaying(true);
    await invoke("play_clips");
    setIsPlaying(false);
  }

  async function onMetronomeToggle() {
    setIsMetronomeOn((prev) => !prev);
    if (!isMetronomeOn()) {
      await invoke("stop_metronome");
      return;
    }
    await invoke("start_metronome");
  }

  return (
    <div class="flex h-full w-full flex-col bg-black">
      <Header
        isPlaying={isPlaying}
        onRecord={record}
        onPlay={playClips}
        isRecording={isRecording}
        isMetronomeOn={isMetronomeOn}
        onMetronomeToggle={onMetronomeToggle}
      />
      <div class="flex flex-col items-center justify-center">
        <div class="flex gap-3" />
      </div>
      <div class="flex-1 pl-12 pt-12">
        <Show when={clips().length}>
          <AudioClips clips={clips} />
        </Show>
      </div>
    </div>
  );
}

export default App;
