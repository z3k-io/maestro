import { appWindow } from "@tauri-apps/api/window";
import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import SpeakerIcon from "./components/SpeakerIcon";
import "./styles.css";
import { Command, invokeCommand } from "./utils/commands";
import { AppEvent, listenToEvent } from "./utils/events";
import { logger } from "./utils/logger";

window.addEventListener("DOMContentLoaded", async () => {
  await invokeCommand(Command.ApplyAeroTheme);
});

window.addEventListener("keydown", async (e) => {
  logger.debug(e.key);

  e.stopPropagation();
  e.preventDefault();
});

const VolumeOverlay = () => {
  const [sessionName, setSessionName] = useState("master");
  const [volume, setVolume] = useState(0);
  const [mute, setMute] = useState(false);
  const [icon, _setIcon] = useState("");

  const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    logger.debug(`Mute is ${mute}`);
  }, [mute]);

  useEffect(() => {
    const unlisten = listenToEvent(AppEvent.VolumeChange, (payload: string) => {
      logger.debug(`Volume change event: ${payload}`);

      const [processName, volume] = payload.split(":");

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
    logger.debug("Resetting hide timeout");
    if (hideTimeoutRef.current) {
      clearTimeout(hideTimeoutRef.current);
    }
    hideTimeoutRef.current = setTimeout(() => {
      logger.debug("Hiding window");
      appWindow.hide();
    }, 1500);
  }

  resetHideTimeout();

  async function updateVolume(newVolume: number) {
    if (newVolume === volume) {
      logger.debug("Volume unchanged");
      return;
    }

    setVolume(newVolume);

    logger.info(`Setting ${sessionName} volume to ${newVolume}`);

    try {
      await invokeCommand(Command.SetSessionVolume, { sessionName: sessionName, volume: newVolume });
    } catch (error) {
      logger.error("Error setting volume", error);
    }
  }

  const handleSliderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const volume = Number(event.target.value);
    updateVolume(volume);
  };

  const handleButtonClick = async () => {
    logger.info(`Toggling mute: ${sessionName} ${mute} -> ${!mute}`);
    setMute(!mute);
    try {
      await invokeCommand(Command.ToggleMute, { sessionName: sessionName });
    } catch (error) {
      logger.error("Error setting mute", error);
    }
  };

  const iconSrc = icon ? `data:image/png;base64,${icon}` : "/speaker-128.png";

  return (
    <div
      className="flex flex-col h-screen w-screen bg-base-300 justify-center"
      onMouseDown={resetHideTimeout}
      onMouseMove={resetHideTimeout}
      onMouseUp={resetHideTimeout}
      onMouseOver={resetHideTimeout}
    >
      <div className="flex flex-col items-center gap-0 bg-base-200 mx-2 p-2 rounded-md h-14 justify-center">
        <div className="flex flex-row items-center gap-2">
          <img src={iconSrc} className="h-5 w-5 flex-shrink-0 justify-center items-center hover:bg-base-300 rounded-md" />
          <button
            className="flex h-8 w-8 flex-shrink-0 justify-center items-center hover:bg-base-300 rounded-md"
            onClick={() => handleButtonClick()}
          >
            <SpeakerIcon volume={volume} mute={mute} className="h-5 w-5" />
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
    </div>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <VolumeOverlay />
  </React.StrictMode>,
);
