import { invoke, InvokeArgs } from "@tauri-apps/api/tauri";
import { AudioSession } from "../types/audioSession";

export enum Command {
  GetAllSessions = "get_all_sessions",
  GetSession = "get_session",
  ApplyAeroTheme = "apply_aero_theme",
  SetSessionVolume = "set_session_volume",
  ToggleSessionMute = "toggle_session_mute",
  // Add other commands here
}

export interface CommandArgs extends InvokeArgs {
  [Command.GetAllSessions]: undefined;
  [Command.GetSession]: { sessionName: string };
  [Command.ApplyAeroTheme]: undefined;
  [Command.SetSessionVolume]: { sessionName: string; volume: number };
  [Command.ToggleSessionMute]: { sessionName: string };
  // Add other command arguments here
}

export interface CommandReturns {
  [Command.GetAllSessions]: AudioSession[];
  [Command.GetSession]: AudioSession;
  [Command.ApplyAeroTheme]: void;
  [Command.SetSessionVolume]: void;
  [Command.ToggleSessionMute]: void;
  // Add other command return types here
}

export async function invokeCommand<T extends Command>(command: T, args?: CommandArgs[T]): Promise<CommandReturns[T]> {
  return args === undefined ? invoke(command) : invoke(command, args);
}
