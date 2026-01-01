<script setup lang="ts">
import {
  useKeyboard,
  type Keyboard,
  type KeyboardConfig,
  type LightModeConfig,
  type KeyMappingConfig,
  type PerKeyColor,
} from "~/composables/useKeyboard";
import { useDatabase, type Profile } from "~/composables/useDatabase";
import KeyMappingEditor from "~/components/KeyMappingEditor.vue";
import PerKeyColorEditor from "~/components/PerKeyColorEditor.vue";

const route = useRoute();
const toast = useToast();
const { scanKeyboards, sendKeyboardConfig, getLightingModes } = useKeyboard();
const {
  initDatabase,
  saveKeyboardConfig,
  getKeyboardConfig,
  getProfiles,
  saveProfile,
  deleteProfile,
  saveSelectedProfile,
  getSelectedProfile,
} = useDatabase();

const keyboard = ref<Keyboard | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);
const saving = ref(false);
const showResetConfirm = ref(false);
const lightModes = ref<Array<{ label: string; value: number }>>([]);
const activeTab = ref<string>("");

const isCustomMode = computed(() => {
  if (!keyboard.value) return false;
  const customModeBit = keyboard.value.rgb ? 0 : 2; // RGB Custom=0, Single Color Custom=2
  return lightConfig.value.mode_bit === customModeBit;
});

const lightConfig = ref<LightModeConfig>({
  mode_bit: 1,
  animation: 3,
  brightness: 5,
  color: { r: 255, g: 255, b: 255 },
  random_colors: false,
  sleep: 5,
});

const keyMappingConfig = ref<KeyMappingConfig>({
  mappings: [],
});

const perKeyColors = ref<PerKeyColor[]>([]);
const profiles = ref<Profile[]>([]);
const selectedProfileId = ref<string | undefined>(undefined);
const showProfileModal = ref(false);
const profileName = ref("");
const editingProfileId = ref<string | null>(null);
const showDeleteConfirm = ref(false);
const profileToDelete = ref<Profile | null>(null);

const deleteConfirmDescription = computed(() => {
  if (!profileToDelete.value) return "";
  return `Are you sure you want to delete profile "${profileToDelete.value.name}"? This action cannot be undone.`;
});

const tabItems = computed(() => {
  const items: Array<{
    label: string;
    icon: string;
    value: string;
    slot: string;
  }> = [];
  if (keyboard.value?.light_enabled) {
    items.push({
      label: "Lighting",
      icon: "i-lucide-palette",
      value: "lighting",
      slot: "lighting",
    });
  }
  if (keyboard.value?.key_map_enabled) {
    items.push({
      label: "Key Mapping",
      icon: "i-lucide-keyboard",
      value: "keymapping",
      slot: "keymapping",
    });
  }
  return items;
});

const colorHex = computed({
  get: () => {
    const c = lightConfig.value.color;
    if (!c) return "#ffffff";
    return `#${c.r.toString(16).padStart(2, "0")}${c.g
      .toString(16)
      .padStart(2, "0")}${c.b.toString(16).padStart(2, "0")}`;
  },
  set: (value: string) => {
    const hex = value.replace("#", "");
    lightConfig.value.color = {
      r: parseInt(hex.substring(0, 2), 16),
      g: parseInt(hex.substring(2, 4), 16),
      b: parseInt(hex.substring(4, 6), 16),
    };
  },
});

const loadKeyboard = async () => {
  loading.value = true;
  error.value = null;
  try {
    const keyboards = await scanKeyboards();
    const idParam = route.params.id as string;
    const parts = idParam.split("-");

    if (parts.length !== 2) {
      error.value = "Invalid keyboard ID format";
      loading.value = false;
      return;
    }

    const vid = parseInt(parts[0] || "0", 16);
    const pid = parseInt(parts[1] || "0", 16);

    if (isNaN(vid) || isNaN(pid)) {
      error.value = "Invalid keyboard ID format";
      loading.value = false;
      return;
    }

    keyboard.value =
      keyboards.find((k) => k.id.vid === vid && k.id.pid === pid) || null;

    if (!keyboard.value) {
      error.value = `Keyboard not found (VID: 0x${parts[0]}, PID: 0x${parts[1]})`;
      loading.value = false;
      return;
    }

    // Load lighting modes based on keyboard RGB capability
    const modes = await getLightingModes(keyboard.value.rgb);
    lightModes.value = modes.map((m) => ({
      label: m.name,
      value: m.mode_bit,
    }));
    // Initialize database and load saved configuration
    await initDatabase();
    const savedConfig = await getKeyboardConfig(
      keyboard.value.id.vid,
      keyboard.value.id.pid
    );
    if (savedConfig?.light_mode) {
      lightConfig.value = {
        mode_bit: savedConfig.light_mode.mode_bit,
        animation: savedConfig.light_mode.animation,
        brightness: savedConfig.light_mode.brightness,
        color: savedConfig.light_mode.color || { r: 255, g: 255, b: 255 },
        random_colors: savedConfig.light_mode.random_colors,
        sleep: savedConfig.light_mode.sleep,
      };
    } else {
      setDefaultConfig();
    }

    // Load saved key mappings
    if (savedConfig?.key_mapping) {
      keyMappingConfig.value = savedConfig.key_mapping;
    } else {
      keyMappingConfig.value = { mappings: [] };
    }

    // Load saved per-key colors
    if (savedConfig?.light_mode?.custom_colors) {
      perKeyColors.value = savedConfig.light_mode.custom_colors;
    } else {
      perKeyColors.value = [];
    }

    // Load profiles
    await loadProfiles();

    // Create default profile if none exist
    if (profiles.value.length === 0) {
      await createDefaultProfile();
      await loadProfiles();
    }

    // Load and auto-select the previously selected profile
    const savedProfileId = await getSelectedProfile(
      keyboard.value.id.vid,
      keyboard.value.id.pid
    );
    if (savedProfileId) {
      const profileExists = profiles.value.some((p) => p.id === savedProfileId);
      if (profileExists) {
        selectedProfileId.value = savedProfileId;
        await selectProfile(savedProfileId);
      } else {
        // If saved profile doesn't exist, select default profile
        const defaultProfile = profiles.value.find(
          (p) => p.name === "Default Profile"
        );
        if (defaultProfile) {
          selectedProfileId.value = defaultProfile.id;
          await selectProfile(defaultProfile.id);
        }
      }
    } else {
      // No saved profile, select default profile
      const defaultProfile = profiles.value.find(
        (p) => p.name === "Default Profile"
      );
      if (defaultProfile) {
        selectedProfileId.value = defaultProfile.id;
        await selectProfile(defaultProfile.id);
      }
    }

    // Set initial active tab to first available tab
    const firstTab = tabItems.value[0];
    if (firstTab) {
      activeTab.value = firstTab.value;
    }
  } catch (e) {
    error.value = e instanceof Error ? e.message : "Failed to load keyboard";
    console.error("Failed to load keyboard:", e);
  } finally {
    loading.value = false;
  }
};

const saveConfiguration = async () => {
  if (!keyboard.value) {
    console.warn("[RK-Configurator] Cannot save: no keyboard selected");
    return;
  }

  saving.value = true;
  try {
    const lightMode: LightModeConfig = {
      ...lightConfig.value,
      custom_colors:
        isCustomMode.value && perKeyColors.value.length > 0
          ? perKeyColors.value
          : undefined,
    };

    const config: KeyboardConfig = {
      light_mode: lightMode,
      key_mapping:
        keyMappingConfig.value.mappings.length > 0
          ? keyMappingConfig.value
          : undefined,
    };

    await sendKeyboardConfig(keyboard.value.path, config, keyboard.value);

    // Save configuration to database
    await saveKeyboardConfig(
      keyboard.value.id.vid,
      keyboard.value.id.pid,
      config
    );

    // Show success toast
    toast.add({
      title: "Configuration Saved",
      description: `Settings saved to ${keyboard.value.name}`,
      color: "success",
      icon: "i-lucide-check-circle",
    });
  } catch (e) {
    const errorMessage =
      e instanceof Error ? e.message : "Failed to save configuration";
    console.error("[RK-Configurator] ERROR: Failed to save configuration:", e);
    error.value = errorMessage;

    // Show error toast
    toast.add({
      title: "Save Failed",
      description: errorMessage,
      color: "error",
      icon: "i-lucide-alert-circle",
    });
  } finally {
    saving.value = false;
  }
};

const setDefaultConfig = () => {
  // Set default mode (Steady for RGB=16, for single-color=1)
  if (lightModes.value.length > 0 && keyboard.value) {
    const defaultMode = keyboard.value.rgb
      ? lightModes.value.find((m) => m.label === "Steady") ||
        lightModes.value[0]
      : lightModes.value.find((m) => m.label === "Steady") ||
        lightModes.value[0];
    if (defaultMode) {
      lightConfig.value.mode_bit = defaultMode.value;
    }
  }
};

const confirmReset = () => {
  lightConfig.value = {
    mode_bit: 1,
    animation: 3,
    brightness: 5,
    color: { r: 255, g: 255, b: 255 },
    random_colors: false,
    sleep: 5,
  };
  // Reset to default mode
  setDefaultConfig();
  perKeyColors.value = [];
  keyMappingConfig.value = { mappings: [] };

  // Close modal
  showResetConfirm.value = false;

  // Show reset toast
  toast.add({
    title: "Configuration Reset",
    description: "Settings have been reset to defaults",
    color: "info",
    icon: "i-lucide-refresh-cw",
  });
};

const loadProfiles = async () => {
  if (!keyboard.value) return;
  try {
    profiles.value = await getProfiles(
      keyboard.value.id.vid,
      keyboard.value.id.pid
    );
  } catch (e) {
    console.error("Failed to load profiles:", e);
  }
};

const createDefaultProfile = async () => {
  if (!keyboard.value) return;

  const lightMode: LightModeConfig = {
    ...lightConfig.value,
    custom_colors:
      isCustomMode.value && perKeyColors.value.length > 0
        ? perKeyColors.value
        : undefined,
  };

  const config: KeyboardConfig = {
    light_mode: lightMode,
    key_mapping:
      keyMappingConfig.value.mappings.length > 0
        ? keyMappingConfig.value
        : undefined,
  };

  const defaultProfile: Profile = {
    id: `default_${keyboard.value.id.vid}_${keyboard.value.id.pid}`,
    name: "Default Profile",
    keyboard_id: keyboard.value.id,
    config,
  };

  await saveProfile(defaultProfile);

  // Set as selected profile
  await saveSelectedProfile(
    keyboard.value.id.vid,
    keyboard.value.id.pid,
    defaultProfile.id
  );
};

const selectProfile = async (profileId: string) => {
  const profile = profiles.value.find((p) => p.id === profileId);
  if (!profile) return;

  selectedProfileId.value = profileId;

  // Save selected profile to database
  if (keyboard.value) {
    await saveSelectedProfile(
      keyboard.value.id.vid,
      keyboard.value.id.pid,
      profileId
    );
  }

  // Load profile configuration
  if (profile.config.light_mode) {
    lightConfig.value = {
      mode_bit: profile.config.light_mode.mode_bit,
      animation: profile.config.light_mode.animation,
      brightness: profile.config.light_mode.brightness,
      color: profile.config.light_mode.color || { r: 255, g: 255, b: 255 },
      random_colors: profile.config.light_mode.random_colors,
      sleep: profile.config.light_mode.sleep,
    };
    perKeyColors.value = profile.config.light_mode.custom_colors || [];
  } else {
    // Reset to defaults if no light_mode in profile
    setDefaultConfig();
    perKeyColors.value = [];
  }

  if (profile.config.key_mapping) {
    keyMappingConfig.value = profile.config.key_mapping;
  } else {
    keyMappingConfig.value = { mappings: [] };
  }

  toast.add({
    title: "Profile Loaded",
    description: `Loaded profile: ${profile.name}`,
    color: "success",
    icon: "i-lucide-check-circle",
  });
};

const openCreateProfileModal = () => {
  profileName.value = "";
  editingProfileId.value = null;
  showProfileModal.value = true;
};

const openEditProfileModal = (profile: Profile) => {
  profileName.value = profile.name;
  editingProfileId.value = profile.id;
  showProfileModal.value = true;
};

const saveProfileAction = async () => {
  if (!keyboard.value || !profileName.value.trim()) return;

  const lightMode: LightModeConfig = {
    ...lightConfig.value,
    custom_colors:
      isCustomMode.value && perKeyColors.value.length > 0
        ? perKeyColors.value
        : undefined,
  };

  const config: KeyboardConfig = {
    light_mode: lightMode,
    key_mapping:
      keyMappingConfig.value.mappings.length > 0
        ? keyMappingConfig.value
        : undefined,
  };

  const profile: Profile = {
    id: editingProfileId.value || `profile_${Date.now()}`,
    name: profileName.value.trim(),
    keyboard_id: keyboard.value.id,
    config,
  };

  await saveProfile(profile);
  await loadProfiles();
  showProfileModal.value = false;

  toast.add({
    title: "Profile Saved",
    description: `Profile "${profile.name}" saved successfully`,
    color: "success",
    icon: "i-lucide-check-circle",
  });
};

const saveCurrentToSelectedProfile = async () => {
  if (!keyboard.value || !selectedProfileId.value) return;

  const profile = profiles.value.find((p) => p.id === selectedProfileId.value);
  if (!profile) return;

  const lightMode: LightModeConfig = {
    ...lightConfig.value,
    custom_colors:
      isCustomMode.value && perKeyColors.value.length > 0
        ? perKeyColors.value
        : undefined,
  };

  const config: KeyboardConfig = {
    light_mode: lightMode,
    key_mapping:
      keyMappingConfig.value.mappings.length > 0
        ? keyMappingConfig.value
        : undefined,
  };

  const updatedProfile: Profile = {
    ...profile,
    config,
  };

  await saveProfile(updatedProfile);
  await loadProfiles();

  toast.add({
    title: "Profile Updated",
    description: `Profile "${profile.name}" has been updated with current settings`,
    color: "success",
    icon: "i-lucide-check-circle",
  });
};

const deleteProfileAction = (profileId: string) => {
  const profile = profiles.value.find((p) => p.id === profileId);
  if (!profile) return;

  profileToDelete.value = profile;
  showDeleteConfirm.value = true;
};

const confirmDeleteProfile = async () => {
  if (!profileToDelete.value) return;

  const profileId = profileToDelete.value.id;
  const profileName = profileToDelete.value.name;
  const wasSelected = selectedProfileId.value === profileId;

  await deleteProfile(profileId);
  await loadProfiles();

  if (wasSelected) {
    // If there are remaining profiles, select the first one
    if (profiles.value.length > 0) {
      const nextProfile = profiles.value[0];
      if (nextProfile) {
        selectedProfileId.value = nextProfile.id;
        await selectProfile(nextProfile.id);
      }
    } else {
      // No profiles remaining, clear selection
      selectedProfileId.value = undefined;
      if (keyboard.value) {
        await saveSelectedProfile(
          keyboard.value.id.vid,
          keyboard.value.id.pid,
          null
        );
      }
    }
  }

  showDeleteConfirm.value = false;
  profileToDelete.value = null;

  toast.add({
    title: "Profile Deleted",
    description: `Profile "${profileName}" deleted`,
    color: "info",
    icon: "i-lucide-trash-2",
  });
};

onMounted(() => {
  loadKeyboard();
});
</script>

<template>
  <div class="container mx-auto p-6 px-8">
    <div class="mb-6">
      <UButton to="/" icon="i-lucide-arrow-left" variant="ghost" class="mb-4">
        Back to Keyboards
      </UButton>

      <h1 class="text-3xl font-bold mb-2" v-if="keyboard">
        {{ keyboard.name }}
      </h1>
    </div>

    <UCard v-if="loading" class="mb-4">
      <div class="flex items-center justify-center p-8">
        <UIcon name="i-lucide-loader-2" class="w-6 h-6 animate-spin mr-2" />
        <span>Loading keyboard configuration...</span>
      </div>
    </UCard>

    <UCard v-else-if="error" class="mb-4">
      <UAlert
        color="error"
        variant="soft"
        :title="error"
        icon="i-lucide-alert-circle"
      />
    </UCard>

    <div v-else-if="keyboard" class="space-y-6">
      <!-- Profiles Section - Compact -->
      <div class="flex items-center gap-2">
        <span class="text-sm font-medium text-muted">Profile:</span>
        <USelectMenu
          v-model="selectedProfileId"
          value-key="id"
          :items="
            profiles.length > 0
              ? profiles.map((p) => ({
                  label: p.name,
                  id: p.id,
                }))
              : []
          "
          placeholder="Select profile"
          size="sm"
          class="min-w-[150px]"
          @update:model-value="(value) => value && selectProfile(value)"
        >
          <template #item="{ item }">
            <div class="flex items-center justify-between w-full gap-4">
              <span class="flex items-center gap-2">
                <UIcon
                  v-if="selectedProfileId === item.id"
                  name="i-lucide-check"
                  class="w-4 h-4"
                />
                {{ item.label }}
              </span>
              <div class="flex gap-1">
                <UButton
                  @click.stop="
                    openEditProfileModal(
                      profiles.find((p) => p.id === item.id)!
                    )
                  "
                  icon="i-lucide-edit"
                  size="xs"
                  variant="ghost"
                  :title="'Rename'"
                />
                <UButton
                  @click.stop="deleteProfileAction(item.id)"
                  icon="i-lucide-trash-2"
                  size="xs"
                  variant="ghost"
                  color="error"
                  :title="'Delete'"
                />
              </div>
            </div>
          </template>
        </USelectMenu>
        <UButton
          v-if="selectedProfileId"
          @click="saveCurrentToSelectedProfile"
          icon="i-lucide-save"
          size="sm"
          variant="outline"
          :title="'Save current settings to selected profile'"
        >
          Save
        </UButton>
        <UButton
          @click="openCreateProfileModal"
          icon="i-lucide-plus"
          size="sm"
          variant="ghost"
          :title="'Create new profile'"
        />
      </div>

      <!-- Tabs Navigation -->
      <UTabs v-model="activeTab" :items="tabItems" class="w-full">
        <template #lighting>
          <UCard v-if="keyboard.light_enabled" class="mt-4">
            <template #header>
              <h2 class="text-xl font-semibold">Lighting Configuration</h2>
            </template>

            <div class="space-y-4 p-4">
              <UFormField label="Brightness">
                <USlider
                  v-model="lightConfig.brightness"
                  :min="0"
                  :max="5"
                  :step="1"
                />
                <div class="text-sm text-muted mt-1">
                  Value: {{ lightConfig.brightness }}
                </div>
              </UFormField>

              <UFormField label="Animation Speed">
                <USlider
                  v-model="lightConfig.animation"
                  :min="1"
                  :max="5"
                  :step="1"
                />
                <div class="text-sm text-muted mt-1">
                  Value: {{ lightConfig.animation }}
                </div>
              </UFormField>

              <UFormField label="Sleep Time">
                <USlider
                  v-model="lightConfig.sleep"
                  :min="1"
                  :max="5"
                  :step="1"
                />
                <div class="text-sm text-muted mt-1">
                  Value: {{ lightConfig.sleep }}
                </div>
              </UFormField>

              <UFormField label="Mode">
                <USelect
                  v-model="lightConfig.mode_bit"
                  :items="lightModes"
                  value-key="value"
                  class="w-full min-w-[300px]"
                />
                <template v-if="isCustomMode" #description>
                  <p class="text-sm text-muted mt-1">
                    Custom mode selected - per-key colors can be configured
                  </p>
                </template>
              </UFormField>

              <UFormField v-if="keyboard.rgb" label="Random Colors">
                <USwitch v-model="lightConfig.random_colors" />
              </UFormField>

              <UFormField
                v-if="
                  keyboard.rgb && !lightConfig.random_colors && !isCustomMode
                "
                label="Color"
              >
                <div class="flex items-center gap-4">
                  <input
                    v-model="colorHex"
                    type="color"
                    class="w-16 h-16 rounded border"
                  />
                  <div class="text-sm">
                    <div>R: {{ lightConfig.color?.r ?? 255 }}</div>
                    <div>G: {{ lightConfig.color?.g ?? 255 }}</div>
                    <div>B: {{ lightConfig.color?.b ?? 255 }}</div>
                  </div>
                </div>
              </UFormField>

              <!-- Per-Key Color Editor (only in custom mode) -->
              <div v-if="isCustomMode && keyboard.rgb" class="mt-6">
                <PerKeyColorEditor
                  :keyboard="keyboard"
                  v-model="perKeyColors"
                />
              </div>
            </div>
          </UCard>
          <UCard v-else class="mt-4">
            <UAlert
              color="info"
              variant="soft"
              title="Lighting Not Available"
              description="This keyboard does not support lighting configuration."
              icon="i-lucide-info"
            />
          </UCard>
        </template>

        <template #keymapping>
          <UCard v-if="keyboard.key_map_enabled" class="mt-4">
            <template #header>
              <h2 class="text-xl font-semibold">Key Mapping</h2>
            </template>

            <div class="p-4">
              <KeyMappingEditor
                :keyboard="keyboard"
                v-model="keyMappingConfig"
              />
            </div>
          </UCard>
          <UCard v-else class="mt-4">
            <UAlert
              color="info"
              variant="soft"
              title="Key Mapping Not Available"
              description="This keyboard does not support key mapping configuration."
              icon="i-lucide-info"
            />
          </UCard>
        </template>
      </UTabs>

      <!-- Actions - Fixed at bottom -->
      <div class="flex gap-4 pt-4 border-t">
        <UButton
          @click="saveConfiguration"
          :loading="saving"
          icon="i-lucide-save"
          color="primary"
          size="lg"
        >
          Save to Keyboard
        </UButton>
        <UButton
          @click="showResetConfirm = true"
          icon="i-lucide-refresh-cw"
          variant="outline"
          size="lg"
        >
          Reset
        </UButton>
      </div>
    </div>

    <!-- Reset Confirmation Modal -->
    <UModal
      v-model:open="showResetConfirm"
      title="Confirm Reset"
      description="Are you sure you want to reset the configuration to defaults? This action cannot be undone."
    >
      <template #footer>
        <div class="flex justify-end gap-2">
          <UButton
            color="neutral"
            variant="ghost"
            @click="showResetConfirm = false"
          >
            Cancel
          </UButton>
          <UButton color="error" @click="confirmReset"> Reset </UButton>
        </div>
      </template>
    </UModal>

    <!-- Profile Modal -->
    <UModal
      v-model:open="showProfileModal"
      :title="editingProfileId ? 'Edit Profile' : 'Create Profile'"
      :description="
        editingProfileId
          ? 'Update the profile name'
          : 'Enter a name for your new profile'
      "
    >
      <template #body>
        <UFormField label="Profile Name">
          <UInput
            v-model="profileName"
            placeholder="Enter profile name"
            autofocus
            @keyup.enter="profileName.trim() && saveProfileAction()"
          />
        </UFormField>
      </template>
      <template #footer>
        <div class="flex justify-end gap-2">
          <UButton
            color="neutral"
            variant="ghost"
            @click="showProfileModal = false"
          >
            Cancel
          </UButton>
          <UButton
            color="primary"
            @click="saveProfileAction"
            :disabled="!profileName.trim()"
          >
            {{ editingProfileId ? "Update" : "Create" }}
          </UButton>
        </div>
      </template>
    </UModal>

    <!-- Delete Profile Confirmation Modal -->
    <UModal
      v-model:open="showDeleteConfirm"
      title="Confirm Delete Profile"
      :description="deleteConfirmDescription"
    >
      <template #footer>
        <div class="flex justify-end gap-2">
          <UButton
            color="neutral"
            variant="ghost"
            @click="showDeleteConfirm = false"
          >
            Cancel
          </UButton>
          <UButton color="error" @click="confirmDeleteProfile">
            Delete
          </UButton>
        </div>
      </template>
    </UModal>
  </div>
</template>
