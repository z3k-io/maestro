import { appWindow } from "@tauri-apps/api/window";
import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import SpeakerIcon from "./components/SpeakerIcon";
import "./styles.css";
import { AudioSession } from "./types/audioSession";
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
  const [icon, setIcon] = useState<string>("");

  const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    invokeCommand(Command.GetSession, { sessionName: sessionName }).then((session) => {
      setSessionName(session.name);
      setVolume(session.volume);
      setMute(session.mute);
      setIcon(session.icon ? `data:image/png;base64,${session.icon}` : "/speaker-128.png");
    });
  }, []);

  useEffect(() => {
    const unlisten = listenToEvent(AppEvent.VolumeChange, (session: AudioSession) => {
      logger.info(`Volume change event: ${session.name} ${session.volume} ${session.mute}`);

      setSessionName(session.name);
      setVolume(session.volume);
      setMute(session.mute);
      setIcon(session.icon ? `data:image/png;base64,${session.icon}` : "/speaker-128.png");

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
      await invokeCommand(Command.ToggleSessionMute, { sessionName: sessionName });
    } catch (error) {
      logger.error("Error setting mute", error);
    }
  };

  return (
    <div
      className="flex flex-col h-screen w-screen bg-base-300 justify-center align-middle"
      onMouseDown={resetHideTimeout}
      onMouseMove={resetHideTimeout}
      onMouseUp={resetHideTimeout}
      onMouseOver={resetHideTimeout}
    >
      <div className="flex items-center gap-0 mx-2 p-2 rounded-md justify-center">
        <div className="flex flex-row items-center gap-2">
          <img src={icon} className="h-5 w-5" />
          <button
            className="flex h-8 w-8 flex-shrink-0 justify-center items-center hover:bg-base-100 rounded-md"
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
