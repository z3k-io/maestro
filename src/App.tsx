import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useRef, useState } from "react";
import SpeakerIcon from "./components/SpeakerIcon";

function App() {
  const [process, setProcess] = useState("master");
  const [volume, setVolume] = useState(0);
  const [mute, setMute] = useState(false);

  const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    const unlisten = listen<String>("mute-change", (event) => {
      console.debug(`Mute change event: ${event.payload}`);

      const [processName, mute] = event.payload.split(":");

      setProcess(processName);
      setMute(Boolean(mute));

      resetHideTimeout();
    });

    return () => {
      unlisten.then((r) => r());
    };
  }, []);

  useEffect(() => {
    const unlisten = listen<String>("volume-change", (event) => {
      console.debug(`Volume change event: ${event.payload}`);

      const [processName, volume] = event.payload.split(":");

      setProcess(processName);
      setVolume(Math.abs(Number(volume)));
      setMute(Number(volume) < 0);

      resetHideTimeout();
    });

    return () => {
      unlisten.then((r) => r());
    };
  }, []);

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

  async function updateVolume(newVolume: number) {
    if (newVolume === volume) {
      console.debug("Volume is the same, returning");
      return;
    }
    setVolume(newVolume);
    try {
      await invoke("set_session_volume", { sessionName: process, volume: newVolume });
    } catch (error) {
      console.error("Error setting volume", error);
    }

    console.debug("New volume is", newVolume);
  }

  const handleSliderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const volume = Number(event.target.value);
    updateVolume(volume);
  };

  const handleButtonClick = async () => {
    setMute(!mute);
    try {
      await invoke("toggle_session_mute", { sessionName: process });
    } catch (error) {
      console.error("Error setting mute", error);
    }
  };

  return (
    <div
      className="flex flex-col h-screen w-screen bg-base-300 justify-center"
      onMouseDown={resetHideTimeout}
      onMouseMove={resetHideTimeout}
      onMouseUp={resetHideTimeout}
      onMouseOver={resetHideTimeout}
      onContextMenu={() => setMute(!mute)}
    >
      <h1 className="flex justify-center capitalize font-semibold text-md">{process}</h1>
      <div className="flex flex-row items-center gap-2 px-4">
        <button
          className="flex h-8 w-8 flex-shrink-0 justify-center items-center hover:bg-base-200 rounded-md"
          onClick={() => handleButtonClick()}
        >
          <SpeakerIcon volume={volume} mute={mute} className="h-4 w-4" />
        </button>
        <input
          type="range"
          min={0}
          max="100"
          value={volume}
          className={`range range-xs ${mute ? "range-error" : "range-primary"}`}
          onChange={handleSliderChange}
        />
        <h2 className="text-lg w-12 text-center">{volume}</h2>
      </div>
    </div>
  );
}

export default App;
