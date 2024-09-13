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
  hotkey: string;
}

export interface ArduinoConfig {
  com_port: string;
  baud_rate: number;
}
