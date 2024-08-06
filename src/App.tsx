import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import reactLogo from "./assets/react.svg";

function App() {
  const [process, setProcess] = useState("chrome");
  const [volume, setVolume] = useState("Unknown");

  async function getVolume() {
    const processName = process;
    console.log(processName);
    const volume = (await invoke("get_process_volume", {
      processName,
    })) as string;
    console.log(volume);
    setVolume(volume);
  }

  return (
    <div className="container">
      <h1 className="text-4xl text-red-700">Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
    </div>
  );
}

export default App;
