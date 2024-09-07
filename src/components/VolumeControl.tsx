import { AudioSession } from "@/types/audioSession";
import { Command, invokeCommand } from "@/utils/commands";
import { AppEvent, listenToEvent } from "@/utils/events";
import { logger } from "@/utils/logger";
import { useEffect, useState } from "react";
import SpeakerIcon from "./SpeakerIcon";

function VolumeControl(props: { sessionName: string; volume: number; icon: string | null }) {
  const [volume, setVolume] = useState(Math.abs(props.volume));
  const [mute, setMute] = useState(props.volume < 0);

  logger.debug(`VolumeControl: ${props.sessionName} ${props.volume}`);

  useEffect(() => {
    const unlisten = listenToEvent(AppEvent.VolumeChange, (payload: AudioSession) => {
      if (payload.name !== props.sessionName) {
        return;
      }

      logger.debug(`Volume change event: ${payload}`);

      setVolume(Math.abs(payload.volume));
      setMute(payload.muted);
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

  const handleButtonClick = async () => {
    logger.info(`Toggling mute: ${props.sessionName} ${mute} -> ${!mute}`);
    setMute(!mute);
    try {
      await invokeCommand(Command.ToggleMute, { sessionName: props.sessionName });
    } catch (error) {
      logger.error(`Error setting mute: ${error}`, error);
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
