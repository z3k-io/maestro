import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import VolumeControl from "./components/VolumeControl";
import { debug } from "./logger";
import "./styles.css";

window.addEventListener("DOMContentLoaded", () => {
  invoke("apply_aero_theme");
});

window.addEventListener("keydown", async (e) => {
  console.log(e.key);

  e.stopPropagation();
  e.preventDefault();
});

const VolumeOverlay = () => {
  const [sessionName, setSessionName] = useState("master");
  const [volume, setVolume] = useState(0);
  const [mute, setMute] = useState(false);

  const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    console.debug("Mute is", mute);
  }, [mute]);

  useEffect(() => {
    const unlisten = listen<String>("volume-change", (event) => {
      console.debug(`Volume change event: ${event.payload}`);

      const [processName, volume] = event.payload.split(":");

      debug(`Volume change event: ${event.payload}`);

      setSessionName(processName);
      setVolume(Math.abs(Number(volume)));
      setMute(Number(volume) < 0);

      resetHideTimeout();
    });

    return () => {
      unlisten.then((r) => r());
    };
  }, []);

  function resetHideTimeout() {
    console.debug("Resetting hide timeout");
    if (hideTimeoutRef.current) {
      clearTimeout(hideTimeoutRef.current);
    }
    hideTimeoutRef.current = setTimeout(() => {
      console.debug("Hiding window");
      appWindow.hide();
    }, 1500);
  }

  resetHideTimeout();

  return (
    <div
      className="flex flex-col h-screen w-screen bg-base-300 justify-center"
      onMouseDown={resetHideTimeout}
      onMouseMove={resetHideTimeout}
      onMouseUp={resetHideTimeout}
      onMouseOver={resetHideTimeout}
    >
      <VolumeControl sessionName={sessionName} volume={volume} icon={null} />
    </div>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <VolumeOverlay />
  </React.StrictMode>,
);
