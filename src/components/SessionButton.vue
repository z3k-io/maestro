<template>
  <button
    :class="`relative flex h-8 w-14 flex-shrink-0 justify-between items-center rounded-md group`"
    :data-tip="name.charAt(0).toUpperCase() + name.slice(1)"
    @click="handleButtonClick"
  >
    <span
      :class="`absolute inset-0 -m-1 ${hoverStyle} opacity-0 group-hover:opacity-100 rounded-lg transition-opacity duration-200`"
    ></span>
    <img :src="icon" class="h-6 w-6 relative z-10" />
    <SpeakerIcon :volume="volume" :mute="mute" class="h-5 w-5 relative z-10" />
  </button>
</template>

<script setup lang="ts">
import { Command, invokeCommand } from "../utils/commands";
import { logger } from "../utils/logger";
import SpeakerIcon from "./SpeakerIcon.vue";

interface Props {
  name: string;
  icon: string;
  volume: number;
  mute: boolean;
  hoverStyle?: string;
}

const props = withDefaults(defineProps<Props>(), {
  hoverStyle: "bg-base-300"
});

const handleButtonClick = async () => {
  logger.info(`Toggling mute: ${props.name} ${props.mute} -> ${!props.mute}`);

  try {
    await invokeCommand(Command.ToggleSessionMute, { sessionName: props.name });
  } catch (error) {
    logger.error("Error setting mute", error);
  }
};
</script>
