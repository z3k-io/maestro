export interface Config {
  sessions: SessionConfig[];
  mixer: MixerConfig;
  system: SystemConfig;
}

export interface SessionConfig {
  name: string;
  encoder: number;
}

export interface MixerConfig {
  enabled: boolean;
  hotkey: string;
}


export interface SystemConfig {
  autostart: boolean;
  show_console: boolean;
  theme: string;
}
