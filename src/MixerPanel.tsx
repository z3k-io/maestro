import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { appWindow, currentMonitor, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import VolumeControl from "./components/VolumeControl";
import { debug } from "./logger";
import "./styles.css";

window.addEventListener("DOMContentLoaded", async () => {
  invoke("apply_aero_theme");
});

class SessionData {
  name: string;
  volume: number;
  icon: string | null;

  constructor(data: any) {
    this.name = data["name"];
    this.volume = data["volume"];
    this.icon = data["icon"];
  }
}

const VolumeMixerPanel = () => {
  const [sessions, setSessions] = useState<SessionData[]>([]);

  useEffect(() => {
    fetchSessions();
  }, []);

  useEffect(() => {
    debug(`Sessions changed`);
    setWindowSizeAndPosition();
  }, [sessions]);

  useEffect(() => {
    listen<Boolean>(`visibility_change`, async (event) => {
      const visible = event.payload;

      if (visible) {
        await fetchSessions();

        appWindow.show();
        appWindow.setFocus();
      } else {
        appWindow.hide();
      }
    });
  }, []);

  const fetchSessions = async () => {
    const rawSessions = await invoke("get_all_sessions");
    const sessionsArray = Object.entries(rawSessions as []);

    const sessions = new Array<SessionData>();

    sessionsArray.forEach((session) => {
      sessions.push(new SessionData(session[1]));
    });

    debug(`Sessions: ${JSON.stringify(sessions)}`);

    sessions.sort((a, b) => {
      if (a.name.toLowerCase() === "master") return -1;
      if (b.name.toLowerCase() === "master") return 1;
      return a.name.localeCompare(b.name);
    });

    setSessions(sessions);
  };

  const setWindowSizeAndPosition = async () => {
    const monitor = await currentMonitor();

    let screenWidth = monitor!.size.width;
    let screenHeight = monitor!.size.height;

    let scaleFactor = monitor!.scaleFactor;
    let windowWidth = Math.round(300 * scaleFactor);

    // Calculate height based on number of sessions
    const baseHeight = 75;
    const padding = 40;
    let windowHeight = Math.round((sessions.length * baseHeight + padding) * scaleFactor);

    debug(`Setting window size: ${windowWidth} ${windowHeight}`);
    await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

    let x = screenWidth - (windowWidth + 20);
    let y = screenHeight - (windowHeight + 80);

    debug(`Setting window position: ${x} ${y}`);
    await appWindow.setPosition(new PhysicalPosition(x, y));
  };

  return (
    <div id="container" className="flex flex-col h-screen w-screen bg-base-300 justify-center">
      <h1 className="text-md font-bold py-2 px-4">Volume Mixer</h1>
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
