import { listen } from "@tauri-apps/api/event";
import { AudioSession } from "../types/audioSession";

export enum AppEvent {
  VolumeChange = "volume-change-event",
  MixerVisibilityChange = "mixer-visibility-change-event",
  ThemeChange = "theme-change-event",
}

export interface EventPayloads {
  [AppEvent.VolumeChange]: AudioSession;
  [AppEvent.MixerVisibilityChange]: boolean;
  [AppEvent.ThemeChange]: string;
}

export function listenToEvent<T extends AppEvent>(event: T, callback: (payload: EventPayloads[T]) => void): Promise<() => void> {
  return listen(event, (event) => {
    callback(event.payload as EventPayloads[T]);
  });
}
