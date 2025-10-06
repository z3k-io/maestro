<template>
  <div class="flex flex-col items-center gap-0 bg-base-100 mx-2 p-2 rounded-md h-14 justify-center">
    <div class="flex flex-row items-center gap-2">
      <SessionButton 
        :name="sessionName" 
        :icon="icon" 
        :volume="volume" 
        :mute="mute" 
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
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { Command, invokeCommand } from "@/utils/commands";
import { AppEvent, listenToEvent } from "@/utils/events";
import { logger } from "@/utils/logger";
import SessionButton from "./SessionButton.vue";
import type { AudioSession } from "@/types/audioSession";

interface Props {
  sessionName: string;
  volume: number;
  icon: string | undefined;
}

const props = defineProps<Props>();

const volume = ref(Math.abs(props.volume));
const mute = ref(props.volume < 0);
const icon = ref(props.icon ? `data:image/png;base64,${props.icon}` : "/master-speaker-512.png");

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listenToEvent(AppEvent.VolumeChange, (payload: AudioSession) => {
    if (payload.name !== props.sessionName) {
      return;
    }

    logger.debug(`Volume change event: ${payload}`);

    volume.value = Math.abs(payload.volume);
    mute.value = payload.mute;
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});

async function updateVolume(newVolume: number) {
  if (newVolume === volume.value) {
    logger.debug(`Volume unchanged: ${newVolume}`);
    return;
  }

  volume.value = newVolume;

  logger.info(`Setting ${props.sessionName} volume to ${newVolume}`);

  try {
    await invokeCommand(Command.SetSessionVolume, { sessionName: props.sessionName, volume: newVolume });
  } catch (error) {
    logger.error(`Error setting volume: ${error}`, error);
  }
}

const handleSliderChange = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const volumeValue = Number(target.value);
  updateVolume(volumeValue);
};
</script>
