import { Slider } from "@/components/ui/slider";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useRef, useState } from "react";

appWindow.show();

function App() {
  const [process, setProcess] = useState("chrome");
  const [volume, setVolume] = useState(0);
  const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    resetHideTimeout();
  }, [volume]);

  useEffect(() => {
    getVolume();
  }, []);

  listen("volume-change", (event) => {
    console.debug("Volume changed: ", event.payload);
    resetHideTimeout();
    setVolume(event.payload as number);
  });

  function resetHideTimeout() {
    console.debug("Resetting hide timeout");
    if (hideTimeoutRef.current) {
      clearTimeout(hideTimeoutRef.current);
    }
    hideTimeoutRef.current = setTimeout(() => {
      appWindow.hide();
    }, 2000);
  }

  async function getVolume() {
    const processName = "chrome";
    const volume = (await invoke("get_process_volume", {
      processName,
    })) as number;
    console.debug(`${processName} volume is ${volume}`);
    setVolume(volume);
  }

  async function updateVolume(newVolume: number) {
    if (newVolume === volume) {
      console.debug("Volume is the same, returning");
      return;
    }
    setVolume(newVolume);
    try {
      await invoke("set_process_volume", {
        processName: process,
        volume: newVolume,
      });
    } catch (error) {
      console.error("Error setting volume", error);
    }

    console.debug("New volume is", newVolume);
  }

  return (
    <div
      className="flex flex-row justify-center items-center h-screen p-4 bg-gray-700 bg-transparent"
      onMouseDown={resetHideTimeout}
      onMouseMove={resetHideTimeout}
      onMouseUp={resetHideTimeout}
    >
      <Slider defaultValue={[33]} max={100} step={2} value={[volume]} onValueChange={(v) => updateVolume(v[0])} />
    </div>
  );
}

export default App;
