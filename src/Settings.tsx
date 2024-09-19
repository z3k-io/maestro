import { PhysicalPosition, PhysicalSize } from "@tauri-apps/api/dpi";
import { currentMonitor, getCurrentWindow } from "@tauri-apps/api/window";
import React, { useCallback, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { Bounce, toast, ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import "./styles.css";
import { Config } from "./types/config";
import { Command, invokeCommand } from "./utils/commands";
import { logger } from "./utils/logger";

const Settings = () => {
  const [config, setConfig] = useState<Config>();
  const [originalConfig, setOriginalConfig] = useState<Config>();

  const loadConfig = async () => {
    const config = await invokeCommand(Command.GetConfig);
    logger.warn(`Loaded config: ${JSON.stringify(config)}`);
    setConfig(config);
    setOriginalConfig(config);
  };

  const setWindowSizeAndPosition = async () => {
    const monitor = await currentMonitor();
    const appWindow = getCurrentWindow();

    const scaleFactor = monitor!.scaleFactor;

    const screenWidth = monitor!.size.width;
    const screenHeight = monitor!.size.height;

    const windowWidth = Math.round(600 * scaleFactor);
    const windowHeight = Math.round(800 * scaleFactor);
    let taskbarHeight = (await invokeCommand(Command.GetTaskbarHeight)) * scaleFactor;

    logger.debug(`Setting window size: ${windowWidth} ${windowHeight}`);
    await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

    let padding = 40 * scaleFactor;

    logger.debug(`Screen width: ${screenWidth}`);
    logger.debug(`Window width: ${windowWidth}`);
    logger.debug(`Taskbar height: ${taskbarHeight}`);
    logger.debug(`Padding: ${padding}`);

    taskbarHeight += 23; // Magic number

    const x = screenWidth - (windowWidth + padding);
    const y = screenHeight - (windowHeight + taskbarHeight + padding);

    logger.info(`Setting window position: ${x} ${y}`);

    await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));

    await appWindow.show();
  };

  const handleSessionChange = useCallback((index: number, field: keyof Config["sessions"][0], value: string | number) => {
    setConfig((prevConfig) => {
      if (!prevConfig) return prevConfig;
      const newSessions = [...prevConfig.sessions];
      newSessions[index] = { ...newSessions[index], [field]: value };
      return { ...prevConfig, sessions: newSessions };
    });
  }, []);

  useEffect(() => {
    setWindowSizeAndPosition();
    loadConfig();
  }, []);

  const handleMixerChange = useCallback((field: keyof Config["mixer"], value: string | boolean) => {
    setConfig((prevConfig) => {
      if (!prevConfig) return prevConfig;
      return { ...prevConfig, mixer: { ...prevConfig.mixer, [field]: value } };
    });
  }, []);

  const handleArduinoChange = useCallback((field: keyof Config["arduino"], value: string | boolean) => {
    setConfig((prevConfig) => {
      if (!prevConfig) return prevConfig;
      return { ...prevConfig, arduino: { ...prevConfig.arduino, [field]: value } };
    });
  }, []);

  const handleSystemChange = useCallback((field: keyof Config["system"], value: boolean) => {
    setConfig((prevConfig) => {
      if (!prevConfig) return prevConfig;
      return { ...prevConfig, system: { ...prevConfig.system, [field]: value } };
    });
  }, []);

  const handleReset = () => {
    setConfig(originalConfig);
  };

  const handleSave = async () => {
    logger.warn(`Saving config: ${JSON.stringify(config)}`);
    await invokeCommand(Command.SetConfig, { config: config! });

    toast.success("Settings saved", {
      position: "top-center",
      autoClose: 500,
      hideProgressBar: true,
      closeOnClick: true,
      pauseOnHover: false,
      draggable: true,
      theme: "dark",
      transition: Bounce,
    });
  };

  return (
    <div className="flex flex-col gap-4 p-4 bg-base-300 h-screen w-screen">
      <div className="flex flex-row justify-between">
        <h1 className="text-3xl font-bold text-center p-2">Settings</h1>
        <div className="flex flex-row gap-2">
          <button className="btn btn-outline btn-sm btn-info" onClick={handleReset}>
            Reset
          </button>
          <button className="btn btn-outline btn-sm btn-success" onClick={handleSave}>
            Save
          </button>
        </div>
      </div>
      <div id="sessions" className="bg-base-100 p-4 rounded-lg">
        <h2 className="text-xl font-bold text-left">Audio Mapping</h2>
        <div id="sessions" className="flex flex-row gap-2 justify-between">
          {config?.sessions.map((session, index) => (
            <div key={index} className="flex flex-col gap-2">
              <label className="form-control w-full max-w-xs">
                <div className="label">
                  <span className="label-text">Encoder ({index})</span>
                </div>
                <input
                  type="text"
                  placeholder="master"
                  className="input input-sm input-bordered w-24 max-w-xs"
                  value={session.name}
                  onChange={(e) => handleSessionChange(index, "name", e.target.value)}
                />
              </label>
            </div>
          ))}
        </div>
      </div>

      <div id="mixer" className="flex flex-col gap-2 p-4 bg-base-100 rounded-lg">
        <h2 className="text-xl font-bold text-left">Volume Mixer</h2>
        <div className="form-control w-52">
          <label className="label cursor-pointer">
            <span className="label-text">Enabled</span>
            <input
              type="checkbox"
              className="toggle toggle-primary"
              checked={config?.mixer.enabled}
              onChange={(e) => handleMixerChange("enabled", e.target.checked)}
            />
          </label>
        </div>
        <div className="flex flex-ro gap-2">
          <label className="label">
            <span className="label-text whitespace-nowrap">Hotkey</span>
          </label>
          <input
            type="text"
            placeholder="Ctrl+Shift+M"
            className="input input-sm input-bordered w-full max-w-xs"
            value={config?.mixer.hotkey}
            onChange={(e) => handleMixerChange("hotkey", e.target.value)}
          />
        </div>
      </div>
      <div id="arduino" className="flex flex-col gap-2 p-4 bg-base-100 rounded-lg">
        <h2 className="text-xl font-bold text-left">Serial Connection (Arduino)</h2>
        <label className="form-control w-full max-w-xs">
          <div className="flex flex-col">
            <div className="form-control w-52">
              <label className="label cursor-pointer">
                <span className="label-text">Enabled</span>
                <input
                  type="checkbox"
                  className="toggle toggle-primary"
                  checked={config?.arduino.enabled}
                  onChange={(e) => handleArduinoChange("enabled", e.target.checked)}
                />
              </label>
            </div>
            <div className="flex flex-row gap-2">
              <label className="label">
                <span className="label-text whitespace-nowrap">COM Port</span>
              </label>
              <input
                type="text"
                placeholder="COM3"
                className="input input-sm input-bordered w-full max-w-xs"
                value={config?.arduino.com_port}
                onChange={(e) => handleArduinoChange("com_port", e.target.value)}
              />
            </div>
          </div>
        </label>
      </div>

      <div id="system" className="flex flex-col gap-2 p-4 bg-base-100 rounded-lg">
        <h2 className="text-xl font-bold text-left">System Settings</h2>
        <div className="form-control w-52">
          <label className="label cursor-pointer">
            <span className="label-text">System Startup</span>
            <input
              type="checkbox"
              className="toggle toggle-primary"
              checked={config?.system.autostart}
              onChange={(e) => handleSystemChange("autostart", e.target.checked)}
            />
          </label>
          <label className="label cursor-pointer">
            <span className="label-text">Show Debug Console</span>
            <input
              type="checkbox"
              className="toggle toggle-primary"
              checked={config?.system.show_console}
              onChange={(e) => handleSystemChange("show_console", e.target.checked)}
            />
          </label>
        </div>
      </div>
    </div>
  );
};

createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Settings />
    <ToastContainer />
  </React.StrictMode>,
);
