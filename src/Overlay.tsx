import { currentMonitor, getCurrentWindow, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import { register, ShortcutEvent } from "@tauri-apps/plugin-global-shortcut";
import React, { useEffect, useRef, useState } from "react";
import ReactDOM from "react-dom/client";
import SessionButton from "./components/SessionButton";
import "./styles.css";
import { AudioSession } from "./types/audioSession";
import { Command, invokeCommand } from "./utils/commands";
import { AppEvent, listenToEvent } from "./utils/events";
import { logger } from "./utils/logger";

const VolumeOverlay = () => {
  const [sessionName, setSessionName] = useState("master");
  const [volume, setVolume] = useState(0);
  const [mute, setMute] = useState(false);
  const [icon, setIcon] = useState<string>("");

  const hideTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    const registerShortcut = async () => {
      await register("AudioVolumeUp", (event: ShortcutEvent) => {
        if (event.state === "Pressed") {
          logger.info("Shortcut triggered");
        }
      });
    };

    registerShortcut();
  }, []);

  useEffect(() => {
    invokeCommand(Command.GetSession, { sessionName: sessionName }).then((session) => {
      setSession(session);
    });

    const initWindow = async () => {
      let window = getCurrentWindow();
      let monitor = await currentMonitor();

      if (!monitor) {
        logger.error("No monitor found");
        return;
      }

      let scaleFactor = monitor.scaleFactor;

      let width = 300 * scaleFactor;
      let height = 60 * scaleFactor;

      let x = Math.round((monitor.size.width - width) / 2);
      let y = Math.round(20 * scaleFactor);

      logger.debug(`Setting window size to ${width}x${height}, location to ${x}, ${y}`);

      window.setSize(new PhysicalSize(width, height));
      window.setPosition(new PhysicalPosition(x, y));
    };

    initWindow();
  }, []);

  useEffect(() => {
    const unlisten = listenToEvent(AppEvent.VolumeChange, (session: AudioSession) => {
      logger.debug(`Volume change event: ${session.name} ${session.volume} ${session.mute}`);

      setSession(session);

      resetHideTimeout();
    });

    return () => {
      unlisten.then((r) => r());
    };
  }, []);

  const setSession = (session: AudioSession) => {
    setSessionName(session.name);
    setVolume(session.volume);
    setMute(session.mute);
    setIcon(session.icon ? `data:image/png;base64,${session.icon}` : "/master-speaker-512.png");
  };

  function resetHideTimeout() {
    logger.debug("Resetting hide timeout");
    if (hideTimeoutRef.current) {
      clearTimeout(hideTimeoutRef.current);
    }
    hideTimeoutRef.current = setTimeout(() => {
      logger.debug("Hiding window");
      getCurrentWindow().hide();
    }, 1000);
  }

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

  resetHideTimeout();

  return (
    <div
      className="flex flex-col h-screen w-screen bg-base-300 justify-center align-middle"
      onMouseDown={resetHideTimeout}
      onMouseMove={resetHideTimeout}
      onMouseUp={resetHideTimeout}
      onMouseOver={resetHideTimeout}
    >
      <div className="flex items-center gap-0 m-2 rounded-md justify-center">
        <div className="flex flex-row items-center gap-2">
          <SessionButton name={sessionName} icon={icon} volume={volume} mute={mute} />
          <input
            type="range"
            min={0}
            max="100"
            value={volume}
            className={`range range-xs ${mute ? "range-error" : "range-primary"}`}
            onChange={handleSliderChange}
          />
          <h2 className="text-lg w-12 text-center cursor-default">{volume}</h2>
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
