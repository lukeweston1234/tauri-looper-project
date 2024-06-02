import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import Header from "./shared/ui/Header";

function App() {
  const [isRecording, setIsRecording] = createSignal(false);

  async function record() {
    await invoke("record_clip");
  }

  async function playClips() {
    setIsRecording(true);
    await invoke("play_clips");
    setIsRecording(false);
  }

  return (
    <div class="flex h-full w-full flex-col bg-black">
      <Header />
      <button class="text-appPrimary">Test</button>
    </div>
  );
}

export default App;
