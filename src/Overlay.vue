<template>
  <div
    class="flex flex-col h-screen w-screen bg-base-300 justify-center align-middle"
    @mousedown="resetHideTimeout"
    @mousemove="resetHideTimeout"
    @mouseup="resetHideTimeout"
    @mouseover="resetHideTimeout"
  >
    <div class="flex items-center gap-0 m-2 rounded-md justify-center">
      <div class="flex flex-row items-center gap-2">
        <SessionButton 
          :name="sessionName" 
          :icon="icon" 
          :volume="volume" 
          :mute="mute" 
          hoverStyle="bg-base-100" 
        />
        <input
          type="range"
          min="0"
          max="100"
          :value="volume"
          :class="`range range-xs ${mute ? 'range-error' : 'range-primary'}`"
          @input="handleSliderChange"
          @change="handleSliderChange"
        />
        <h2 class="text-lg w-12 text-center cursor-default">{{ volume }}</h2>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { currentMonitor, getCurrentWindow, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window";
import SessionButton from "./components/SessionButton.vue";
import type { AudioSession } from "./types/audioSession";
import { Command, invokeCommand } from "./utils/commands";
import { AppEvent, listenToEvent } from "./utils/events";
import { logger } from "./utils/logger";

const sessionName = ref("master");
const volume = ref(0);
const mute = ref(false);
const icon = ref<string>("");

let hideTimeout: NodeJS.Timeout | null = null;

const setSession = (session: AudioSession) => {
  sessionName.value = session.name;
  volume.value = session.volume;
  mute.value = session.mute;
  icon.value = session.icon ? `data:image/png;base64,${session.icon}` : "/master-speaker-512.png";
};

function resetHideTimeout() {
  logger.debug("Resetting hide timeout");
  if (hideTimeout) {
    clearTimeout(hideTimeout);
  }
  hideTimeout = setTimeout(() => {
    logger.debug("Hiding window");
    getCurrentWindow().hide();
  }, 1000);
}

async function updateVolume(newVolume: number) {
  if (newVolume === volume.value) {
    logger.debug("Volume unchanged");
    return;
  }

  volume.value = newVolume;

  logger.info(`Setting ${sessionName.value} volume to ${newVolume}`);

  try {
    await invokeCommand(Command.SetSessionVolume, { sessionName: sessionName.value, volume: newVolume });
  } catch (error) {
    logger.error("Error setting volume", error);
  }
}

const handleSliderChange = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const volumeValue = Number(target.value);
  updateVolume(volumeValue);
};

let unlisten: (() => void) | null = null;

onMounted(async () => {
  const session = await invokeCommand(Command.GetSession, { sessionName: sessionName.value });
  setSession(session);

  const config = await invokeCommand(Command.GetConfig);
  document.documentElement.setAttribute("data-theme", config.system.theme);

  listenToEvent(AppEvent.ThemeChange, (theme) => {
    document.documentElement.setAttribute("data-theme", theme);
  });

  const initWindow = async () => {
    let window = getCurrentWindow();
    let monitor = await currentMonitor();

    if (!monitor) {
      logger.error("No monitor found");
      return;
    }

    let scaleFactor = monitor.scaleFactor;

    let width = 300 * scaleFactor;
    let height = 60 * scaleFactor;

    let x = Math.round((monitor.size.width - width) / 2);
    let y = Math.round(20 * scaleFactor);

    logger.debug(`Setting window size to ${width}x${height}, location to ${x}, ${y}`);

    window.setSize(new PhysicalSize(width, height));
    window.setPosition(new PhysicalPosition(x, y));
  };

  await initWindow();

  unlisten = await listenToEvent(AppEvent.VolumeChange, (session: AudioSession) => {
    logger.debug(`Volume change event: ${session.name} ${session.volume} ${session.mute}`);

    setSession(session);

    resetHideTimeout();
  });

  resetHideTimeout();
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});
</script>
