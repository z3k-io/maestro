import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import SpeakerIcon from "./SpeakerIcon";

function VolumeControl(props: { sessionName: string; volume: number; icon: string | null }) {
  const [volume, setVolume] = useState(Math.abs(props.volume));
  const [mute, setMute] = useState(props.volume < 0);

  console.log(`VolumeControl: ${props.sessionName} ${props.volume}`);

  useEffect(() => {
    const unlisten = listen<String>(`volume-change`, (event) => {
      const [process, volume] = event.payload.split(":");

      if (process !== props.sessionName) {
        return;
      }

      console.debug(`Volume change event: ${event.payload}`);

      setVolume(Math.abs(Number(volume)));
      setMute(Number(volume) < 0);
    });

    return () => {
      unlisten.then((r) => r());
    };
  }, []);

  async function updateVolume(newVolume: number) {
    if (newVolume === volume) {
      console.debug("Volume unchanged");
      return;
    }

    setVolume(newVolume);

    console.info(`Setting ${props.sessionName} volume to ${newVolume}`);

    try {
      await invoke("set_session_volume", { sessionName: props.sessionName, volume: newVolume });
    } catch (error) {
      console.error("Error setting volume", error);
    }
  }

  const handleSliderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const volume = Number(event.target.value);
    updateVolume(volume);
  };

  const handleButtonClick = async () => {
    console.info(`Toggling mute: ${props.sessionName} ${mute} -> ${!mute}`);
    setMute(!mute);
    try {
      await invoke("toggle_session_mute", { sessionName: props.sessionName });
    } catch (error) {
      console.error("Error setting mute", error);
    }
  };

  const iconSrc = props.icon ? `data:image/png;base64,${props.icon}` : "/speaker-128.png";

  return (
    <div className="flex flex-col items-center gap-0 bg-base-200 mx-2 p-2 rounded-md h-14 justify-center">
      <div className="flex flex-row items-center gap-2">
        <img src={iconSrc} className="h-5 w-5 flex-shrink-0 justify-center items-center hover:bg-base-300 rounded-md" />
        <button
          className="flex h-8 w-8 flex-shrink-0 justify-center items-center hover:bg-base-300 rounded-md"
          onClick={() => handleButtonClick()}
        >
          <SpeakerIcon volume={volume} mute={mute} className="h-5 w-5" />
        </button>
        <input
          type="range"
          min={0}
          max="100"
          value={volume}
          className={`range range-xs ${mute ? "range-error" : "range-primary"}`}
          onChange={handleSliderChange}
        />
        <h2 className="text-lg w-12 text-center">{volume}</h2>
      </div>
    </div>
  );
}

export default VolumeControl;
