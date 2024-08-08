import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useRef, useState } from "react";
import SpeakerIcon from "./components/SpeakerIcon";

appWindow.show();

function App() {
  const [process, setProcess] = useState("master");
  const [volume, setVolume] = useState(0);
  const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    resetHideTimeout();
  }, [volume]);

  useEffect(() => {
    getVolume();
  }, []);

  listen<String>("volume-change", (event) => {
    console.debug(`Volume change event received: ${event}`);
    // event.payload = <session>:<volume>
    const sessionName = event.payload.split(":")[0];
    const volume = Number(event.payload.split(":")[1]);

    setProcess(sessionName);
    setVolume(volume * 100);

    resetHideTimeout();
  });

  // TODO: This isn't working
  function resetHideTimeout() {
    console.debug("Resetting hide timeout");
    if (hideTimeoutRef.current) {
      clearTimeout(hideTimeoutRef.current);
    }
    hideTimeoutRef.current = setTimeout(() => {
      console.debug("Hiding window");
      appWindow.hide();
    }, 3000);
  }

  async function getVolume() {
    const processName = process;
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
      await invoke("set_session_volume", {
        session_name: process,
        volume: newVolume,
      });
    } catch (error) {
      console.error("Error setting volume", error);
    }

    console.debug("New volume is", newVolume);
  }

  const handleSliderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const volume = Number(event.target.value);
    updateVolume(volume);
  };

  return (
    <div
      className="flex flex-col h-screen w-screen bg-base-300 justify-center"
      onMouseDown={resetHideTimeout}
      onMouseMove={resetHideTimeout}
      onMouseUp={resetHideTimeout}
      onMouseOver={resetHideTimeout}
    >
      <h1 className="flex justify-center capitalize font-semibold text-md">{process}</h1>
      <div className="flex flex-row items-center gap-2 px-4">
        <button
          className="flex h-8 w-8 flex-shrink-0 justify-center items-center hover:bg-base-200 rounded-md"
          onClick={() => updateVolume(0)}
        >
          <SpeakerIcon volume={volume} className="h-4 w-4" />
        </button>
        <input
          type="range"
          min={0}
          max="100"
          value={volume}
          className="range range-xs range-primary"
          onChange={handleSliderChange}
        />
        <h2 className="text-lg w-12 text-center">{volume}</h2>
      </div>
    </div>
  );
}

export default App;
