import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { appWindow, currentMonitor, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import VolumeControl from "./components/VolumeControl";
import { info } from "./logger";
import "./styles.css";

window.addEventListener("DOMContentLoaded", async () => {
  invoke("apply_aero_theme");
});

class Session {
  name: string;
  volume: number;
  mute: boolean;

  constructor(name: string, volume: number, mute: boolean) {
    this.name = name;
    this.volume = volume;
    this.mute = mute;
  }
}

const VolumeMixerPanel = () => {
  const [sessions, setSessions] = useState<Session[]>([]);

  useEffect(() => {
    const unlisten = listen<Boolean>(`visibility_change`, (event) => {
      info(`Visibility change event: ${event.payload}`);

      const visible = event.payload;

      if (visible) {
        appWindow.show();
        appWindow.setFocus();
      } else {
        appWindow.hide();
      }
    });

    return () => {
      unlisten.then((r) => r());
    };
  }, []);

  const fetchSessions = async () => {
    const rawSessions = await invoke("get_all_sessions");
    const sessionsObj = rawSessions as Record<string, number>;
    const sessionsArray = Object.entries(sessionsObj);

    const sessions = new Array<Session>();

    sessionsArray.forEach(([key, value]) => {
      console.log(key, value);
      sessions.push(new Session(key, value, false));
    });

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

    info(`Setting window size: ${windowWidth} ${windowHeight}`);
    await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

    let x = screenWidth - windowWidth;
    let y = screenHeight - (windowHeight + 80);

    info(`Setting position: ${x} ${y}`);
    await appWindow.setPosition(new PhysicalPosition(x, y));
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
