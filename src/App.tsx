import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

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
    <div class="padding-6 flex items-center justify-center gap-3">
      <button onClick={playClips}>Play</button>
      <button onClick={record}>Record</button>
      {isRecording() && <span>Recording!!</span>}
    </div>
  );
}

export default App;
