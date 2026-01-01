<script setup>
import { listen } from "@tauri-apps/api/event";

const { checkForUpdates } = useUpdater();

onMounted(async () => {
  await checkForUpdates();

  // Listen for menu event to check for updates
  await listen("menu-check-updates", async () => {
    await checkForUpdates();
  });
});
</script>

<template>
  <UApp>
    <UMain>
      <NuxtPage />
    </UMain>
  </UApp>
</template>
