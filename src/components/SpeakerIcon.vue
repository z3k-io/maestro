<template>
  <svg :class="`stroke-2 stroke-current ${className}`" viewBox="0 0 24 24">
    <template v-if="mute">
      <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
      <line x1="23" y1="9" x2="17" y2="15" />
      <line x1="17" y1="9" x2="23" y2="15" />
    </template>
    <template v-else-if="volume === 0">
      <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
    </template>
    <template v-else-if="volume < 50">
      <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
      <path d="M15.54 8.46a5 5 0 0 1 0 7.07" />
    </template>
    <template v-else-if="volume >= 50">
      <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" />
      <path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07" />
    </template>
  </svg>
</template>

<script setup lang="ts">
interface Props {
  volume: number;
  mute: boolean;
  className?: string;
}

defineProps<Props>();
</script>
