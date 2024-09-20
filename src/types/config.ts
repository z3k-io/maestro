export interface Config {
  sessions: SessionConfig[];
  mixer: MixerConfig;
  arduino: ArduinoConfig;
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

export interface ArduinoConfig {
  enabled: boolean;
  com_port: string;
  baud_rate: number;
}

export interface SystemConfig {
  autostart: boolean;
  show_console: boolean;
  theme: string;
}
