import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { appWindow, currentMonitor, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import VolumeControl from "./components/VolumeControl";
import { warn } from "./logger";
import "./styles.css";

window.addEventListener("DOMContentLoaded", async () => {
  invoke("apply_aero_theme");
});

listen<Boolean>(`visibility_change`, (event) => {
  // warn(`Visibility change event: ${event.payload}`);

  const visible = event.payload;

  if (visible) {
    appWindow.show();
    appWindow.setFocus();
  } else {
    appWindow.hide();
  }
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

  const fetchSessions = async () => {
    const rawSessions = await invoke("get_all_sessions");
    const sessionsArray = Object.entries(rawSessions as []);

    const sessions = new Array<SessionData>();

    sessionsArray.forEach((session) => {
      // warn(`Session: ${JSON.stringify(session)}`);
      sessions.push(new SessionData(session[1]));
    });

    warn(`Sessions: ${JSON.stringify(sessions)}`);

    sessions.sort((a, b) => {
      if (a.name.toLowerCase() === "master") return -1;
      if (b.name.toLowerCase() === "master") return 1;
      return a.name.localeCompare(b.name);
    });

    setSessions(sessions);
  };

  useEffect(() => {
    fetchSessions();

    const unlistenFocus = appWindow.onFocusChanged((event) => {
      warn(`Focus change event: ${event.payload}`);
      if (event) {
        fetchSessions();
      }
    });

    return () => {
      unlistenFocus.then((unlisten) => unlisten());
    };
  }, []);

  const setWindowSizeAndPosition = async () => {
    const monitor = await currentMonitor();

    let screenWidth = monitor!.size.width;
    let screenHeight = monitor!.size.height;

    // Arbitrary values, need to compute intelligently
    let scaleFactor = monitor!.scaleFactor;
    let windowWidth = Math.round(300 * scaleFactor);
    let windowHeight = Math.round(sessions.length * 92 * scaleFactor);

    // info(`Setting window size: ${windowWidth} ${windowHeight}`);
    await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

    let x = screenWidth - windowWidth;
    let y = screenHeight - (windowHeight + 80);

    // info(`Setting position: ${x} ${y}`);
    await appWindow.setPosition(new PhysicalPosition(x, y));

    appWindow.setFocus();
  };

  setTimeout(() => {
    setWindowSizeAndPosition();
  });

  return (
    <div className="flex flex-col h-screen w-screen bg-base-300">
      {sessions.map((session) => (
        <VolumeControl key={session.name} sessionName={session.name} volume={session.volume} />
      ))}
    </div>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <VolumeMixerPanel />
  </React.StrictMode>,
);
