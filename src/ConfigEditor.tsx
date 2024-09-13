import { PhysicalPosition, PhysicalSize } from "@tauri-apps/api/dpi";
import { currentMonitor, getCurrentWindow } from "@tauri-apps/api/window";
import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import { Config } from "./types/config";
import { Command, invokeCommand } from "./utils/commands";
import { logger } from "./utils/logger";

const ConfigEditor = () => {
  const [config, setConfig] = useState<Config>();

  const loadConfig = async () => {
    const config = await invokeCommand(Command.GetConfig);
    logger.warn(`Loaded config: ${JSON.stringify(config)}`);
    setConfig(config);
  };

  const setWindowSizeAndPosition = async () => {
    const monitor = await currentMonitor();
    const appWindow = getCurrentWindow();

    const screenWidth = monitor!.size.width;
    const screenHeight = monitor!.size.height;

    const scaleFactor = monitor!.scaleFactor;
    const windowWidth = Math.round(600 * scaleFactor);
    const windowHeight = Math.round(600 * scaleFactor);

    logger.debug(`Setting window size: ${windowWidth} ${windowHeight}`);
    await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

    // Bottom right of screen, 50px from each edge
    const x = screenWidth - windowWidth - 150;
    const y = screenHeight - windowHeight - 200;

    logger.debug(`Setting window position: ${x} ${y}`);
    await appWindow.setPosition(new PhysicalPosition(x, y));

    await appWindow.show();
    await appWindow.setFocus();
  };

  useEffect(() => {
    setWindowSizeAndPosition();
    loadConfig();
  }, []);

  return (
    <div className="flex flex-col gap-4 p-4">
      <h1 className="text-2xl font-bold text-center">Config Editor</h1>
      <h2 className="text-xl font-bold text-left">Sessions</h2>
      <div id="sessions" className="flex flex-row gap-2 p-2 bg-blue-600">
        {config?.sessions.map((session) => (
          <div key={session.name} className="flex flex-col gap-2">
            <label className="form-control w-full max-w-xs">
              <div className="label">
                <span className="label-text">Name</span>
              </div>
              <input
                type="text"
                placeholder="master"
                className="input input-bordered w-32 max-w-xs"
                value={session.name}
                onChange={(e) =>
                  setConfig({
                    ...config,
                    sessions: config?.sessions.map((s) => (s.name === session.name ? { ...s, name: e.target.value } : s)),
                  })
                }
              />

              <div className="label">
                <span className="label-text">Encoder Index</span>
              </div>
              <input
                type="number"
                className="input input-bordered w-32 max-w-xs"
                value={session.encoder}
                onChange={(e) =>
                  setConfig({
                    ...config,
                    sessions: config?.sessions.map((s) =>
                      s.encoder === session.encoder ? { ...s, encoder: parseInt(e.target.value) } : s,
                    ),
                  })
                }
              />
            </label>
          </div>
        ))}
      </div>
      <div id="mixer">
        <h2>Mixer</h2>
        <input
          type="text"
          className="input input-bordered w-full max-w-xs"
          value={config?.mixer.hotkey}
          onChange={(e) => setConfig({ ...config, mixer: { ...config?.mixer, hotkey: e.target.value } })}
        />
      </div>
      <div id="arduino" className="flex flex-col gap-2 p-4">
        <h2>Arduino</h2>
        {/* <input
          type="text"
          className="input input-bordered w-full max-w-xs"
          placeholder="COM3"
          value={config?.arduino.com_port}
          onChange={(e) =>
            setConfig({
              ...config,
              arduino: { ...config?.arduino, com_port: config?.arduino.com_port ?? "", baud_rate: parseInt(e.target.value) },
            })
          }
        />
        <input
          type="number"
          className="input input-bordered w-full max-w-xs"
          value={config?.arduino.baud_rate}
          placeholder="9600"
          onChange={(e) => setConfig({ ...config, arduino: { ...config?.arduino, baud_rate: parseInt(e.target.value) } })}
        /> */}
      </div>
    </div>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ConfigEditor />
  </React.StrictMode>,
);
