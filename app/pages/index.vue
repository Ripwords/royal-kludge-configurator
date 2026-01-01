<script setup lang="ts">
import { useKeyboard, type Keyboard } from "~/composables/useKeyboard";
import { usePermissions } from "~/composables/usePermissions";
import { resolveResource } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

const { scanKeyboards } = useKeyboard();
const {
  ensureHIDPermissions,
  requestAccessibility,
  requestInputMonitoring,
  isMacOS,
} = usePermissions();

const keyboards = ref<Keyboard[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const checkingPermissions = ref(true); // Track if we're still checking permissions on initial load
const permissionStatus = ref<{
  accessibility: boolean;
  inputMonitoring: boolean;
  allGranted: boolean;
} | null>(null);
const requestingPermission = ref(false);

const requestPermission = async () => {
  requestingPermission.value = true;
  try {
    console.log("[Permissions] Requesting permissions...");
    const isMac = await isMacOS();
    if (!isMac) {
      // Not macOS, no permissions needed
      permissionStatus.value = {
        accessibility: true,
        inputMonitoring: true,
        allGranted: true,
      };
      await refreshKeyboards();
      return;
    }

    // Directly request permissions (this will show system dialogs if not previously denied)
    // Note: If permissions were previously denied, macOS won't show the dialog again
    // and the user will need to go to System Settings manually
    console.log("[Permissions] Requesting Accessibility permission...");
    await requestAccessibility();

    console.log("[Permissions] Requesting Input Monitoring permission...");
    await requestInputMonitoring();

    // Wait a moment for the dialogs to appear/be processed
    await new Promise((resolve) => setTimeout(resolve, 500));

    // Re-check permissions after requesting
    const status = await ensureHIDPermissions();
    permissionStatus.value = status;
    console.log("[Permissions] Permission status after request:", status);

    if (status.allGranted) {
      await refreshKeyboards();
    } else {
      const missing = [];
      if (!status.accessibility) missing.push("Accessibility");
      if (!status.inputMonitoring) missing.push("Input Monitoring");
      const missingText = missing.join(" and ");
      error.value = `${missingText} permission${
        missing.length > 1 ? "s are" : " is"
      } required. ${
        missing.length > 1
          ? "If dialogs didn't appear, please grant these permissions"
          : "If the dialog didn't appear, please grant this permission"
      } in System Settings > Privacy & Security > ${missingText}.`;
    }
  } catch (e) {
    error.value =
      "Failed to request permission. Please check System Settings > Privacy & Security.";
    console.error("[Permissions] Failed to request permission:", e);
  } finally {
    requestingPermission.value = false;
  }
};

const refreshKeyboards = async () => {
  // Always re-check permissions on macOS when refreshing
  const isMac = await isMacOS();
  if (isMac) {
    // Re-check permissions to get latest status
    const status = await ensureHIDPermissions();
    permissionStatus.value = status;
    if (status && !status.allGranted) {
      const missing = [];
      if (!status.accessibility) missing.push("Accessibility");
      if (!status.inputMonitoring) missing.push("Input Monitoring");
      error.value = `${missing.join(" and ")} permission${
        missing.length > 1 ? "s are" : " is"
      } required to scan for keyboards. Please grant ${
        missing.length > 1 ? "these permissions" : "this permission"
      } in System Settings > Privacy & Security.`;
      // Don't return - allow the scan to attempt anyway, it will fail gracefully
    }
  }

  loading.value = true;
  error.value = null;
  try {
    keyboards.value = await scanKeyboards();

    // Resolve image paths for all keyboards
    await Promise.all(
      keyboards.value.map((keyboard) => resolveKeyboardImage(keyboard))
    );

    // If scan succeeds, clear any permission errors and update permission status
    // This is a fallback: if HID scan works, permissions must be granted
    if (keyboards.value.length > 0) {
      error.value = null;
      // Update permission status to reflect success - if scan works, permissions are granted
      if (permissionStatus.value) {
        console.log(
          "[Permissions] HID scan succeeded, updating permission status to granted"
        );
        permissionStatus.value.accessibility = true;
        permissionStatus.value.inputMonitoring = true;
        permissionStatus.value.allGranted = true;
      } else {
        // If permission status wasn't set yet, set it now
        permissionStatus.value = {
          accessibility: true,
          inputMonitoring: true,
          allGranted: true,
        };
      }
    } else if (
      isMac &&
      permissionStatus.value &&
      permissionStatus.value.allGranted
    ) {
      // Permissions granted but no keyboards found - this is normal
      error.value = null;
    }
  } catch (e) {
    const errorMessage =
      e instanceof Error ? e.message : "Failed to scan keyboards";
    error.value = errorMessage;
    console.error("Failed to scan keyboards:", e);
    console.error("Error details:", {
      message: errorMessage,
      permissions: permissionStatus.value,
    });
  } finally {
    loading.value = false;
  }
};

const selectKeyboard = (keyboard: Keyboard) => {
  // Use hex format for VID and PID to match parsing in detail page
  const vidHex = keyboard.id.vid.toString(16).padStart(4, "0");
  const pidHex = keyboard.id.pid.toString(16).padStart(4, "0");
  navigateTo(`/keyboard/${vidHex}-${pidHex}`);
};

const keyboardImagePaths = ref<Map<string, string>>(new Map());

const getKeyboardImagePath = (keyboard: Keyboard): string => {
  const key = `${keyboard.id.vid}-${keyboard.id.pid}`;
  return keyboardImagePaths.value.get(key) || "";
};

const resolveKeyboardImage = async (keyboard: Keyboard): Promise<void> => {
  // The image_path from Rust is like "keyboards/258a/images/52.png"
  const resourcePath = keyboard.image_path;
  const key = `${keyboard.id.vid}-${keyboard.id.pid}`;

  // Check cache first
  if (keyboardImagePaths.value.has(key)) {
    return;
  }

  try {
    // Use Tauri's resolveResource to get the proper resource path
    const resolvedPath = await resolveResource(resourcePath);
    // Convert the resolved path to a URL using convertFileSrc
    const convertedPath = convertFileSrc(resolvedPath);
    keyboardImagePaths.value.set(key, convertedPath);
  } catch (e) {
    console.error(
      "[RK-Configurator] Failed to resolve image path:",
      e,
      keyboard.image_path
    );
    // Fallback: try using asset protocol directly
    const fallbackPath = `asset://localhost/${resourcePath}`;
    keyboardImagePaths.value.set(key, fallbackPath);
  }
};

const handleImageError = (event: Event) => {
  const img = event.target as HTMLImageElement;
  console.error("[RK-Configurator] Image failed to load:", img.src);
  img.style.display = "none";
};

// Check permissions immediately on component creation (before mount)
onBeforeMount(async () => {
  // Check permissions first to set initial status
  const isMac = await isMacOS();
  if (isMac) {
    // Check permissions without requesting
    const status = await ensureHIDPermissions();
    permissionStatus.value = status;
    checkingPermissions.value = false;
    if (status.allGranted) {
      await refreshKeyboards();
    }
  } else {
    // Not macOS, set permissions as granted
    permissionStatus.value = {
      accessibility: true,
      inputMonitoring: true,
      allGranted: true,
    };
    checkingPermissions.value = false;
    await refreshKeyboards();
  }
});
</script>

<template>
  <div class="container mx-auto p-6">
    <div class="mb-6">
      <h1 class="text-3xl font-bold mb-2">Royal Kludge Configurator</h1>
      <p class="text-muted">Configure your Royal Kludge keyboard</p>
    </div>

    <UCard v-if="checkingPermissions" class="mb-4">
      <div class="flex items-center justify-center p-8">
        <UIcon name="i-lucide-loader-2" class="w-6 h-6 animate-spin mr-2" />
        <span>Checking permissions...</span>
      </div>
    </UCard>

    <UCard v-if="loading && !checkingPermissions" class="mb-4">
      <div class="flex items-center justify-center p-8">
        <UIcon name="i-lucide-loader-2" class="w-6 h-6 animate-spin mr-2" />
        <span>Scanning for keyboards...</span>
      </div>
    </UCard>

    <UCard
      v-if="
        !checkingPermissions &&
        permissionStatus &&
        !permissionStatus.allGranted &&
        !requestingPermission &&
        !loading
      "
      class="mb-4"
    >
      <UAlert
        color="warning"
        variant="soft"
        title="macOS Permissions Required"
        :description="`This app needs ${
          !permissionStatus.accessibility ? 'Accessibility' : ''
        }${
          !permissionStatus.accessibility && !permissionStatus.inputMonitoring
            ? ' and '
            : ''
        }${
          !permissionStatus.inputMonitoring ? 'Input Monitoring' : ''
        } permission${
          !permissionStatus.accessibility && !permissionStatus.inputMonitoring
            ? 's'
            : ''
        } to detect and configure your keyboard. After granting permissions in System Settings, click Refresh to try again.`"
        icon="i-lucide-shield-alert"
      >
        <template #actions>
          <div class="flex gap-2">
            <UButton
              @click="requestPermission"
              color="primary"
              icon="i-lucide-shield-check"
            >
              Request Permissions
            </UButton>
            <UButton
              @click="refreshKeyboards"
              variant="outline"
              icon="i-lucide-refresh-cw"
            >
              Refresh
            </UButton>
          </div>
        </template>
      </UAlert>
    </UCard>

    <UCard v-if="error && !loading" class="mb-4">
      <UAlert
        color="error"
        variant="soft"
        :title="error"
        icon="i-lucide-alert-circle"
      >
        <template #actions>
          <UButton
            @click="refreshKeyboards"
            variant="outline"
            icon="i-lucide-refresh-cw"
          >
            Retry
          </UButton>
        </template>
      </UAlert>
    </UCard>

    <div
      v-if="
        keyboards.length === 0 &&
        !loading &&
        (!permissionStatus || permissionStatus.allGranted)
      "
      class="text-center py-12"
    >
      <UIcon
        name="i-lucide-keyboard"
        class="w-16 h-16 mx-auto mb-4 text-muted"
      />
      <h2 class="text-xl font-semibold mb-2">No keyboards found</h2>
      <p class="text-muted mb-4">
        Connect a Royal Kludge keyboard and click refresh
      </p>
      <UButton @click="refreshKeyboards" icon="i-lucide-refresh-cw">
        Refresh
      </UButton>
    </div>

    <div
      v-if="keyboards.length > 0 && !loading"
      class="flex flex-col items-center gap-4 max-w-2xl mx-auto"
    >
      <UCard
        v-for="keyboard in keyboards"
        :key="`${keyboard.id.vid}-${keyboard.id.pid}`"
        class="cursor-pointer hover:shadow-lg transition-shadow w-full"
        @click="selectKeyboard(keyboard)"
      >
        <template #header>
          <div class="flex items-center justify-between">
            <h3 class="font-semibold text-lg">{{ keyboard.name }}</h3>
            <UBadge
              :color="keyboard.rgb ? 'primary' : 'neutral'"
              variant="soft"
            >
              {{ keyboard.rgb ? "RGB" : "Single Color" }}
            </UBadge>
          </div>
        </template>

        <div class="space-y-4">
          <div
            class="aspect-video bg-muted rounded-lg flex items-center justify-center overflow-hidden p-4"
          >
            <img
              :src="getKeyboardImagePath(keyboard)"
              :alt="keyboard.name"
              class="max-w-full max-h-full object-contain"
              @error="handleImageError"
            />
          </div>

          <div class="text-sm space-y-1">
            <div class="flex justify-between">
              <span class="text-muted">VID:</span>
              <span class="font-mono"
                >0x{{ keyboard.id.vid.toString(16).padStart(4, "0") }}</span
              >
            </div>
            <div class="flex justify-between">
              <span class="text-muted">PID:</span>
              <span class="font-mono"
                >0x{{ keyboard.id.pid.toString(16).padStart(4, "0") }}</span
              >
            </div>
            <div class="flex justify-between">
              <span class="text-muted">Keys:</span>
              <span>{{ keyboard.keys.length }}</span>
            </div>
          </div>
        </div>

        <template #footer>
          <UButton
            block
            @click.stop="selectKeyboard(keyboard)"
            icon="i-lucide-settings"
          >
            Configure
          </UButton>
        </template>
      </UCard>
    </div>

    <div
      v-if="keyboards.length > 0 && !loading"
      class="mt-6 flex justify-center"
    >
      <UButton
        @click="refreshKeyboards"
        icon="i-lucide-refresh-cw"
        variant="outline"
      >
        Refresh Keyboards
      </UButton>
    </div>
  </div>
</template>
