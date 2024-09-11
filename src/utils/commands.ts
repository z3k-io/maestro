import { invoke } from "@tauri-apps/api/core";
import { AudioSession } from "../types/audioSession";

export enum Command {
  GetAllSessions = "get_all_sessions",
  GetSession = "get_session",
  SetSessionVolume = "set_session_volume",
  ToggleSessionMute = "toggle_session_mute",
}

export interface CommandArgs {
  [Command.GetAllSessions]: undefined;
  [Command.GetSession]: { sessionName: string };
  [Command.SetSessionVolume]: { sessionName: string; volume: number };
  [Command.ToggleSessionMute]: { sessionName: string };
}

export interface CommandReturns {
  [Command.GetAllSessions]: AudioSession[];
  [Command.GetSession]: AudioSession;
  [Command.SetSessionVolume]: void;
  [Command.ToggleSessionMute]: void;
}

export async function invokeCommand<T extends Command>(command: T, args?: CommandArgs[T]): Promise<CommandReturns[T]> {
  return args === undefined ? invoke(command) : invoke(command, args);
}
