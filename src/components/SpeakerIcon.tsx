interface SpeakerIconProps {
  volume: number;
  mute: boolean;
  className?: string;
}

export default function SpeakerIcon({ volume, mute, className = "" }: SpeakerIconProps) {
  const getIcon = () => {
    if (mute) {
      return (
        <>
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
          <line x1="23" y1="9" x2="17" y2="15" />
          <line x1="17" y1="9" x2="23" y2="15" />
        </>
      );
    }
    if (volume === 0) {
      return (
        <>
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
        </>
      );
    }
    if (volume < 50) {
      return (
        <>
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
          <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
        </>
      );
    }
    if (volume >= 50) {
      return (
        <>
          <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
          <path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07" />
        </>
      );
    }
  };

  return (
    <svg className={`stroke-2 stroke-current ${className}`} viewBox="0 0 24 24">
      {getIcon()}
    </svg>
  );
}
