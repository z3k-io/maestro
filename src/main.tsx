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

  e.stopPropagation();
  e.preventDefault();
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
