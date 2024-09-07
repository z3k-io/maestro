import { listen } from "@tauri-apps/api/event";

export enum AppEvent {
  VolumeChange = "volume-change-event",
  MixerVisibilityChange = "mixer-visibility-change-event",
}

export interface EventPayloads {
  [AppEvent.VolumeChange]: string;
  [AppEvent.MixerVisibilityChange]: boolean;
}

export function listenToEvent<T extends AppEvent>(event: T, callback: (payload: EventPayloads[T]) => void): Promise<() => void> {
  return listen(event, (event) => {
    callback(event.payload as EventPayloads[T]);
  });
}
