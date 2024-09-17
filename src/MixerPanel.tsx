import { currentMonitor, getCurrentWindow, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import VolumeControl from "./components/VolumeControl";
import "./styles.css";
import { AudioSession } from "./types/audioSession";
import { Command, invokeCommand } from "./utils/commands";
import { AppEvent, listenToEvent } from "./utils/events";
import { logger } from "./utils/logger";

const VolumeMixerPanel = () => {
  const [sessions, setSessions] = useState<AudioSession[]>([]);

  useEffect(() => {
    fetchSessions();
  }, []);

  useEffect(() => {
    logger.debug(`Sessions changed`);
    setWindowSizeAndPosition();
  }, [sessions]);

  useEffect(() => {
    listenToEvent(AppEvent.MixerVisibilityChange, async (visible: boolean) => {
      const appWindow = getCurrentWindow();
      if (visible) {
        await fetchSessions();

        appWindow.show();
      } else {
        appWindow.hide();
      }
    });
  }, []);

  const fetchSessions = async () => {
    const sessions = await invokeCommand(Command.GetAllSessions);

    logger.debug(`Sessions: ${JSON.stringify(sessions)}`);

    sessions.sort((a: AudioSession, b: AudioSession) => {
      if (a.name.toLowerCase() === "master") return -1;
      if (b.name.toLowerCase() === "master") return 1;
      return a.name.localeCompare(b.name);
    });

    setSessions(sessions);
  };

  const setWindowSizeAndPosition = async () => {
    const monitor = await currentMonitor();
    const appWindow = getCurrentWindow();

    let screenWidth = monitor!.size.width;
    let screenHeight = monitor!.size.height;

    let scaleFactor = monitor!.scaleFactor;
    let windowWidth = Math.round(300 * scaleFactor);

    // Calculate height based on number of sessions
    const baseHeight = 75;
    const padding = 40;
    let windowHeight = Math.round((sessions.length * baseHeight + padding) * scaleFactor);

    logger.debug(`Setting window size: ${windowWidth} ${windowHeight}`);
    await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

    let x = screenWidth - (windowWidth + 20);
    let y = screenHeight - (windowHeight + 80);

    logger.debug(`Setting window position: ${x} ${y}`);
    await appWindow.setPosition(new PhysicalPosition(x, y));
  };

  return (
    <div id="container" className="flex flex-col h-screen w-screen bg-base-300 justify-center m-0 p-0">
      <h1 className="text-md font-bold pb-2 px-4 m-0">Volume Mixer</h1>
      <div className="flex flex-col gap-2">
        {sessions.map((session) => (
          <VolumeControl key={session.name} sessionName={session.name} volume={session.volume} icon={session.icon} />
        ))}
      </div>
    </div>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <VolumeMixerPanel />
  </React.StrictMode>,
);
