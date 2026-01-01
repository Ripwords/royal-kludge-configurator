<script setup>
const { checkForUpdates } = useUpdater();

onMounted(() => {
  checkForUpdates();
});
</script>

<template>
  <UApp>
    <UMain>
      <NuxtPage />
    </UMain>
  </UApp>
</template>
