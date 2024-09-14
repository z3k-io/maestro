import { Config } from "@/types/config";
import { invoke } from "@tauri-apps/api/core";
import { AudioSession } from "../types/audioSession";

export enum Command {
  GetAllSessions = "get_all_sessions",
  GetSession = "get_session",
  SetSessionVolume = "set_session_volume",
  ToggleSessionMute = "toggle_session_mute",
  GetConfig = "get_config",
  SetConfig = "set_config",
  GetTaskbarHeight = "get_taskbar_height",
}

export interface CommandArgs {
  [Command.GetAllSessions]: undefined;
  [Command.GetSession]: { sessionName: string };
  [Command.SetSessionVolume]: { sessionName: string; volume: number };
  [Command.ToggleSessionMute]: { sessionName: string };
  [Command.GetConfig]: undefined;
  [Command.SetConfig]: { config: Config };
  [Command.GetTaskbarHeight]: undefined;
}

export interface CommandReturns {
  [Command.GetAllSessions]: AudioSession[];
  [Command.GetSession]: AudioSession;
  [Command.SetSessionVolume]: void;
  [Command.ToggleSessionMute]: void;
  [Command.GetConfig]: Config;
  [Command.SetConfig]: void;
  [Command.GetTaskbarHeight]: number;
}

export async function invokeCommand<T extends Command>(command: T, args?: CommandArgs[T]): Promise<CommandReturns[T]> {
  return args === undefined ? invoke(command) : invoke(command, args);
}
