import { AudioSession } from "@/types/audioSession";
import { Command, invokeCommand } from "@/utils/commands";
import { AppEvent, listenToEvent } from "@/utils/events";
import { logger } from "@/utils/logger";
import { useEffect, useState } from "react";
import SessionButton from "./SessionButton";

function VolumeControl(props: { sessionName: string; volume: number; icon: string | undefined }) {
  const [volume, setVolume] = useState(Math.abs(props.volume));
  const [mute, setMute] = useState(props.volume < 0);
  const [icon, _setIcon] = useState(props.icon ? `data:image/png;base64,${props.icon}` : "/master-speaker-512.png");

  useEffect(() => {
    const unlisten = listenToEvent(AppEvent.VolumeChange, (payload: AudioSession) => {
      if (payload.name !== props.sessionName) {
        return;
      }

      logger.debug(`Volume change event: ${payload}`);

      setVolume(Math.abs(payload.volume));
      setMute(payload.mute);
    });

    return () => {
      unlisten.then((r) => r());
    };
  }, []);

  async function updateVolume(newVolume: number) {
    if (newVolume === volume) {
      logger.debug(`Volume unchanged: ${newVolume}`);
      return;
    }

    setVolume(newVolume);

    logger.info(`Setting ${props.sessionName} volume to ${newVolume}`);

    try {
      await invokeCommand(Command.SetSessionVolume, { sessionName: props.sessionName, volume: newVolume });
    } catch (error) {
      logger.error(`Error setting volume: ${error}`, error);
    }
  }

  const handleSliderChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const volume = Number(event.target.value);
    updateVolume(volume);
  };

  return (
    <div className="flex flex-col items-center gap-0 bg-base-100 mx-2 p-2 rounded-md h-14 justify-center">
      <div className="flex flex-row items-center gap-2">
        <SessionButton name={props.sessionName} icon={icon} volume={volume} mute={mute} />
        <input
          type="range"
          min={0}
          max="100"
          value={volume}
          className={`range range-xs ${mute ? "range-error" : "range-primary"}`}
          onChange={handleSliderChange}
        />
        <h2 className="text-lg w-12 text-center cursor-default">{volume}</h2>
      </div>
    </div>
  );
}

export default VolumeControl;
