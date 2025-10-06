<template>
  <div id="container" class="flex flex-col h-screen w-screen bg-base-300 justify-center m-0 p-0">
    <h1 class="text-md font-bold pb-2 px-4 m-0">Maestro</h1>
    <div class="flex flex-col gap-2">
      <VolumeControl 
        v-for="session in sessions" 
        :key="session.name" 
        :sessionName="session.name" 
        :volume="session.volume" 
        :icon="session.icon" 
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { currentMonitor, getCurrentWindow, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import VolumeControl from "./components/VolumeControl.vue";
import type { AudioSession } from "./types/audioSession";
import { Command, invokeCommand } from "./utils/commands";
import { AppEvent, listenToEvent } from "./utils/events";
import { logger } from "./utils/logger";

const sessions = ref<AudioSession[]>([]);

const fetchConfig = async () => {
  const config = await invokeCommand(Command.GetConfig);
  logger.debug(`Loaded config: ${JSON.stringify(config)}`);
  let theme = config!.system.theme;
  document.documentElement.setAttribute("data-theme", theme);
};

const fetchSessions = async () => {
  const sessionsData = await invokeCommand(Command.GetAllSessions);

  sessionsData.sort((a: AudioSession, b: AudioSession) => {
    if (a.name.toLowerCase() === "master") return -1;
    if (b.name.toLowerCase() === "master") return 1;
    return a.name.localeCompare(b.name);
  });

  sessions.value = sessionsData;
};

const setWindowSizeAndPosition = async () => {
  const monitor = await currentMonitor();
  const appWindow = getCurrentWindow();

  let screenWidth = monitor!.size.width;
  let screenHeight = monitor!.size.height;

  let scaleFactor = monitor!.scaleFactor;
  let windowWidth = Math.round(300 * scaleFactor);

  // Calculate height based on number of sessions
  const baseHeight = 75;
  const padding = 40;
  let windowHeight = Math.round((sessions.value.length * baseHeight + padding) * scaleFactor);

  logger.debug(`Setting window size: ${windowWidth} ${windowHeight}`);
  await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

  let x = screenWidth - (windowWidth + 20);
  let y = screenHeight - (windowHeight + 80);

  logger.debug(`Setting window position: ${x} ${y}`);
  await appWindow.setPosition(new PhysicalPosition(x, y));
};

// Watch for sessions changes and resize window accordingly
watch(sessions, async () => {
  logger.debug(`Sessions changed`);
  await setWindowSizeAndPosition();
}, { deep: true });

onMounted(async () => {
  await fetchSessions();
  await fetchConfig();

  listenToEvent(AppEvent.MixerVisibilityChange, async (visible: boolean) => {
    const appWindow = getCurrentWindow();
    if (visible) {
      await fetchSessions();
      appWindow.show();
    } else {
      appWindow.hide();
    }
  });

  listenToEvent(AppEvent.ThemeChange, (theme: string) => {
    logger.info(`Theme changed to ${theme}`);
    document.documentElement.setAttribute("data-theme", theme);
  });
});
</script>
