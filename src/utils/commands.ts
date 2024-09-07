import { invoke, InvokeArgs } from "@tauri-apps/api/tauri";
import { SessionData } from "../types/audioSession";

export enum Command {
  GetAllSessions = "get_all_sessions",
  ApplyAeroTheme = "apply_aero_theme",
  SetSessionVolume = "set_session_volume",
  ToggleMute = "toggle_mute",
  // Add other commands here
}

export interface CommandArgs extends InvokeArgs {
  [Command.GetAllSessions]: undefined;
  [Command.ApplyAeroTheme]: undefined;
  [Command.SetSessionVolume]: { sessionName: string; volume: number };
  [Command.ToggleMute]: { sessionName: string };
  // Add other command arguments here
}

export interface CommandReturns {
  [Command.GetAllSessions]: Record<string, SessionData>;
  [Command.ApplyAeroTheme]: void;
  [Command.SetSessionVolume]: void;
  [Command.ToggleMute]: void;
  // Add other command return types here
}

export async function invokeCommand<T extends Command>(command: T, args?: CommandArgs[T]): Promise<CommandReturns[T]> {
  return args === undefined ? invoke(command) : invoke(command, args);
}
