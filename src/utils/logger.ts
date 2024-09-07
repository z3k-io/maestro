import { invoke } from "@tauri-apps/api";

export function info(message: string) {
  invoke("log", { message: message, level: "info" });
  console.log(message);
}

export function debug(message: string) {
  invoke("log", { message: message, level: "debug" });
  console.debug(message);
}

export function warn(message: string) {
  invoke("log", { message: message, level: "warn" });
  console.warn(message);
}

export function error(message: string) {
  invoke("log", { message: message, level: "error" });
  console.error(message);
}
