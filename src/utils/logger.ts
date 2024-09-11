import { invoke } from "@tauri-apps/api/core";

export class Logger {
  public info(message: string) {
    invoke("log", { message: message, level: "info" });
    console.log(message);
  }

  public debug(message: string) {
    invoke("log", { message: message, level: "debug" });
    console.debug(message);
  }

  public warn(message: string) {
    invoke("log", { message: message, level: "warn" });
    console.warn(message);
  }

  public error(message: string, error?: any) {
    invoke("log", { message: message, level: "error" });
    console.error(message, error);
  }
}

export const logger = new Logger();
