import { Slider } from "@/components/ui/slider";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

function App() {
  const [process, setProcess] = useState("chrome");
  const [volume, setVolume] = useState(0);

  // const [currentVolume, setCurrentVolume] = useState<number>(0);

  // const [processes, setProcesses] = useState<string[]>([
  //   "master",
  //   "chrome",
  //   "discord",
  // ]);
  // const [volumes, setVolumes] = useState<number[]>([20, 30, 50]);

  useEffect(() => {
    getVolume();
  }, [process]);

  async function getVolume() {
    const processName = process;
    const volume = (await invoke("get_process_volume", {
      processName,
    })) as number;
    console.debug(`${processName} volume is ${volume}`);
    setVolume(volume);
  }

  async function updateVolume(value: number[]) {
    if (value[0] === volume) {
      console.debug("Volume is the same, returning");
      return;
    }

    let updatedVolume = value[0];
    try {
      await invoke("set_process_volume", {
        processName: process,
        volume: value[0],
      });
      setVolume(value[0]);
    } catch (error) {
      console.error("Error setting volume", error);
    }

    console.debug("New volume is", updatedVolume);
  }

  return (
    <div className="container p-10">
      <Slider
        defaultValue={[33]}
        max={100}
        step={2}
        value={[volume]}
        onValueChange={updateVolume}
      />
    </div>
  );
}

export default App;
