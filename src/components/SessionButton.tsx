import { Command, invokeCommand } from "../utils/commands";
import { logger } from "../utils/logger";
import SpeakerIcon from "./SpeakerIcon";

interface SessionButtonProps {
  name: string;
  icon: string;
  volume: number;
  mute: boolean;
  style?: string;
}

export default function SessionButton({ name, icon, volume, mute, style }: SessionButtonProps) {
  const handleButtonClick = async () => {
    logger.info(`Toggling mute: ${name} ${mute} -> ${!mute}`);

    try {
      await invokeCommand(Command.ToggleSessionMute, { sessionName: name });
    } catch (error) {
      logger.error("Error setting mute", error);
    }
  };

  return (
    <button
      className={`relative flex h-8 w-14 flex-shrink-0 justify-between items-center rounded-md group ${style}`}
      data-tip={name.charAt(0).toUpperCase() + name.slice(1)}
      onClick={() => handleButtonClick()}
    >
      <span className="absolute inset-0 -m-1 bg-base-300 opacity-0 group-hover:opacity-100 rounded-lg transition-opacity duration-200"></span>
      <img src={icon} className="h-6 w-6 relative z-10" />
      <SpeakerIcon volume={volume} mute={mute} className="h-5 w-5 relative z-10" />
    </button>
  );
}
