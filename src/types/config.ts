export interface Config {
  sessions: SessionConfig[];
  mixer: MixerConfig;
  arduino: ArduinoConfig;
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
