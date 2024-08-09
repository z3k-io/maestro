import { invoke } from "@tauri-apps/api";
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

window.addEventListener("DOMContentLoaded", () => {
  invoke("apply_aero_theme");
});

window.addEventListener("keydown", async (e) => {
  console.log(e.key);
  if (e.key === "AudioVolumeUp") {
    await invoke("master_volume_up");
  }
  if (e.key === "AudioVolumeDown") {
    await invoke("master_volume_down");
  }
  if (e.key === "Mute") {
    await invoke("master_volume_mute");
  }

  e.preventDefault();
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
