<template>
  <div class="flex flex-col gap-4 p-4 bg-base-300 h-full w-full">
    <div class="flex flex-row justify-between">
      <h1 class="text-3xl font-bold text-center p-2">Settings</h1>
      <div class="flex flex-row gap-2">
        <button class="btn btn-outline btn-sm btn-info" @click="handleReset">
          Reset
        </button>
        <button class="btn btn-outline btn-sm btn-success" @click="handleSave">
          Save
        </button>
      </div>
    </div>
    <div id="sessions" class="bg-base-100 p-4 rounded-lg">
      <h2 class="text-xl font-bold text-left">Audio Mapping</h2>
      <div id="sessions" class="flex flex-row gap-2 justify-between">
        <div v-for="(session, index) in config?.sessions" :key="index" class="flex flex-col gap-2">
          <label class="form-control w-full max-w-xs">
            <div class="label">
              <span class="label-text">Encoder ({{ index }})</span>
            </div>
            <input
              type="text"
              placeholder="master"
              class="input input-sm input-bordered w-24 max-w-xs"
              :value="session.name"
              @input="handleSessionChange(index, 'name', ($event.target as HTMLInputElement).value)"
            />
          </label>
        </div>
      </div>
    </div>

    <div id="mixer" class="flex flex-col gap-2 p-4 bg-base-100 rounded-lg">
      <h2 class="text-xl font-bold text-left">Volume Mixer</h2>
      <div class="form-control w-52">
        <label class="label cursor-pointer">
          <span class="label-text">Enabled</span>
          <input
            type="checkbox"
            class="toggle toggle-primary"
            :checked="config?.mixer.enabled"
            @change="handleMixerChange('enabled', ($event.target as HTMLInputElement).checked)"
          />
        </label>
      </div>
      <div class="flex flex-ro gap-2">
        <label class="label">
          <span class="label-text whitespace-nowrap">Hotkey</span>
        </label>
        <input
          type="text"
          placeholder="Ctrl+Shift+M"
          class="input input-sm input-bordered w-full max-w-xs"
          :value="config?.mixer.hotkey"
          @input="handleMixerChange('hotkey', ($event.target as HTMLInputElement).value)"
        />
      </div>
    </div>

    <div id="system" class="flex flex-col gap-2 p-4 bg-base-100 rounded-lg">
      <h2 class="text-xl font-bold text-left">System Settings</h2>
      <div class="form-control w-52">
        <label class="label cursor-pointer">
          <span class="label-text">System Startup</span>
          <input
            type="checkbox"
            class="toggle toggle-primary"
            :checked="config?.system.autostart"
            @change="handleSystemChange('autostart', ($event.target as HTMLInputElement).checked)"
          />
        </label>
        <label class="label cursor-pointer">
          <span class="label-text">Show Debug Console</span>
          <input
            type="checkbox"
            class="toggle toggle-primary"
            :checked="config?.system.show_console"
            @change="handleSystemChange('show_console', ($event.target as HTMLInputElement).checked)"
          />
        </label>
      </div>
      <ThemePicker 
        :theme="config?.system.theme || 'light'" 
        :setTheme="(newTheme) => handleSystemChange('theme', newTheme)" 
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { PhysicalPosition, PhysicalSize } from "@tauri-apps/api/dpi";
import { currentMonitor, getCurrentWindow } from "@tauri-apps/api/window";
import { useToast } from "vue-toastification";
import ThemePicker from "./components/ThemePicker.vue";
import "./styles.css";
import type { Config } from "./types/config";
import { Command, invokeCommand } from "./utils/commands";
import { logger } from "./utils/logger";

const config = ref<Config>();
const originalConfig = ref<Config>();
const toast = useToast();

const loadConfig = async () => {
  const configData = await invokeCommand(Command.GetConfig);
  logger.info(`Loaded config: ${JSON.stringify(configData)}`);
  config.value = configData;
  originalConfig.value = configData;
  let theme = configData!.system.theme;
  document.documentElement.setAttribute("data-theme", theme);
};

const setWindowSizeAndPosition = async () => {
  const monitor = await currentMonitor();
  const appWindow = getCurrentWindow();

  const scaleFactor = monitor!.scaleFactor;

  const screenWidth = monitor!.size.width;
  const screenHeight = monitor!.size.height;

  const windowWidth = Math.round(600 * scaleFactor);
  const windowHeight = Math.round(875 * scaleFactor);
  let taskbarHeight = (await invokeCommand(Command.GetTaskbarHeight)) * scaleFactor;

  logger.debug(`Setting window size: ${windowWidth} ${windowHeight}`);
  await appWindow.setSize(new PhysicalSize(windowWidth, windowHeight));

  let padding = 40 * scaleFactor;

  logger.debug(`Screen width: ${screenWidth}`);
  logger.debug(`Window width: ${windowWidth}`);
  logger.debug(`Taskbar height: ${taskbarHeight}`);
  logger.debug(`Padding: ${padding}`);

  taskbarHeight += 23; // Magic number

  const x = screenWidth - (windowWidth + padding);
  const y = screenHeight - (windowHeight + taskbarHeight + padding);

  logger.info(`Setting window position: ${x} ${y}`);

  await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));

  await appWindow.show();
};

const handleSessionChange = (index: number, field: keyof Config["sessions"][0], value: string | number) => {
  if (!config.value) return;
  const newSessions = [...config.value.sessions];
  newSessions[index] = { ...newSessions[index], [field]: value };
  config.value = { ...config.value, sessions: newSessions };
};

const handleMixerChange = (field: keyof Config["mixer"], value: string | boolean) => {
  if (!config.value) return;
  config.value = { ...config.value, mixer: { ...config.value.mixer, [field]: value } };
};

const handleSystemChange = (field: keyof Config["system"], value: string | boolean) => {
  logger.debug(`Setting ${field} to ${value}`);
  if (!config.value) return;
  config.value = { ...config.value, system: { ...config.value.system, [field]: value } };
};

const handleReset = () => {
  config.value = originalConfig.value;
};

const handleSave = async () => {
  logger.debug(`Saving config: ${JSON.stringify(config.value)}`);
  await invokeCommand(Command.SetConfig, { config: config.value! });

  toast.success("Settings saved", {
    position: "top-center",
    timeout: 500,
    hideProgressBar: true,
    closeOnClick: true,
    pauseOnHover: false,
    draggable: true,
    theme: "dark",
  });
};

onMounted(async () => {
  await setWindowSizeAndPosition();
  await loadConfig();
});
</script>
