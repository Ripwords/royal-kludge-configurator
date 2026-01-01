<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue";
import { resolveResource } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import type {
  Key,
  KeyMapping,
  KeyMappingConfig,
} from "~/composables/useKeyboard";

interface Props {
  keyboard: {
    keys: Key[];
    name: string;
    image_path: string;
    top_left_x: number;
    top_left_y: number;
    bottom_right_x: number;
    bottom_right_y: number;
  };
  modelValue?: KeyMappingConfig;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  "update:modelValue": [value: KeyMappingConfig];
}>();

const selectedKeyIndex = ref<number | null>(null);
const selectedKeyCode = ref<number | null>(null);
const imageLoaded = ref(false);
const imageWidth = ref(0);
const imageHeight = ref(0);
const imgElement = ref<HTMLImageElement | null>(null);

const keyboardImagePath = ref<string>("");

onMounted(async () => {
  try {
    const resolvedPath = await resolveResource(props.keyboard.image_path);
    keyboardImagePath.value = convertFileSrc(resolvedPath);
  } catch (e) {
    console.error("Failed to resolve image path:", e);
    keyboardImagePath.value = `asset://localhost/${props.keyboard.image_path}`;
  }
});

const mappings = computed(() => {
  return props.modelValue?.mappings || [];
});

const keyCodeOptions = computed(() => {
  // Generate key code options from KeyCode enum values
  const options = [
    // Numbers
    { label: "1", value: 0x1e00 },
    { label: "2", value: 0x1f00 },
    { label: "3", value: 0x2000 },
    { label: "4", value: 0x2100 },
    { label: "5", value: 0x2200 },
    { label: "6", value: 0x2300 },
    { label: "7", value: 0x2400 },
    { label: "8", value: 0x2500 },
    { label: "9", value: 0x2600 },
    { label: "0", value: 0x2700 },
    // Letters
    { label: "A", value: 0x0400 },
    { label: "B", value: 0x0500 },
    { label: "C", value: 0x0600 },
    { label: "D", value: 0x0700 },
    { label: "E", value: 0x0800 },
    { label: "F", value: 0x0900 },
    { label: "G", value: 0x0a00 },
    { label: "H", value: 0x0b00 },
    { label: "I", value: 0x0c00 },
    { label: "J", value: 0x0d00 },
    { label: "K", value: 0x0e00 },
    { label: "L", value: 0x0f00 },
    { label: "M", value: 0x1000 },
    { label: "N", value: 0x1100 },
    { label: "O", value: 0x1200 },
    { label: "P", value: 0x1300 },
    { label: "Q", value: 0x1400 },
    { label: "R", value: 0x1500 },
    { label: "S", value: 0x1600 },
    { label: "T", value: 0x1700 },
    { label: "U", value: 0x1800 },
    { label: "V", value: 0x1900 },
    { label: "W", value: 0x1a00 },
    { label: "X", value: 0x1b00 },
    { label: "Y", value: 0x1c00 },
    { label: "Z", value: 0x1d00 },
    // Symbols
    { label: "Hyphen (-)", value: 0x2d00 },
    { label: "Equals (=)", value: 0x2e00 },
    { label: "Left Bracket ([)", value: 0x2f00 },
    { label: "Right Bracket (])", value: 0x3000 },
    { label: "Backslash (\\)", value: 0x3100 },
    { label: "Semicolon (;)", value: 0x3300 },
    { label: "Quote (')", value: 0x3400 },
    { label: "Back Quote (`)", value: 0x3500 },
    { label: "Comma (,)", value: 0x3600 },
    { label: "Dot (.)", value: 0x3700 },
    { label: "Slash (/)", value: 0x3800 },
    // Special keys
    { label: "Escape", value: 0x2900 },
    { label: "Tab", value: 0x2b00 },
    { label: "Enter", value: 0x2800 },
    { label: "Space", value: 0x2c00 },
    { label: "Backspace", value: 0x2a00 },
    { label: "Caps Lock", value: 0x3900 },
    { label: "Menu", value: 0x6500 },
    { label: "Print Screen", value: 0x4600 },
    // Arrow keys
    { label: "Arrow Up", value: 0x5200 },
    { label: "Arrow Down", value: 0x5100 },
    { label: "Arrow Left", value: 0x5000 },
    { label: "Arrow Right", value: 0x4f00 },
    // Navigation keys
    { label: "Home", value: 0x4a00 },
    { label: "End", value: 0x4d00 },
    { label: "Page Up", value: 0x4b00 },
    { label: "Page Down", value: 0x4e00 },
    { label: "Insert", value: 0x4900 },
    { label: "Delete", value: 0x4c00 },
    { label: "Pause", value: 0x4800 },
    // Function keys
    { label: "F1", value: 0x3a00 },
    { label: "F2", value: 0x3b00 },
    { label: "F3", value: 0x3c00 },
    { label: "F4", value: 0x3d00 },
    { label: "F5", value: 0x3e00 },
    { label: "F6", value: 0x3f00 },
    { label: "F7", value: 0x4000 },
    { label: "F8", value: 0x4100 },
    { label: "F9", value: 0x4200 },
    { label: "F10", value: 0x4300 },
    { label: "F11", value: 0x4400 },
    { label: "F12", value: 0x4500 },
    // Modifiers
    { label: "Left Control", value: 0x010000 },
    { label: "Right Control", value: 0x100000 },
    { label: "Left Shift", value: 0x020000 },
    { label: "Right Shift", value: 0x200000 },
    { label: "Left Alt", value: 0x040000 },
    { label: "Right Alt", value: 0x400000 },
    { label: "Left Super (Win)", value: 0x080000 },
    { label: "Right Super (Win)", value: 0x800000 },
    // Numpad
    { label: "Num 1", value: 0x5900 },
    { label: "Num 2", value: 0x5a00 },
    { label: "Num 3", value: 0x5b00 },
    { label: "Num 4", value: 0x5c00 },
    { label: "Num 5", value: 0x5d00 },
    { label: "Num 6", value: 0x5e00 },
    { label: "Num 7", value: 0x5f00 },
    { label: "Num 8", value: 0x6000 },
    { label: "Num 9", value: 0x6100 },
    { label: "Num 0", value: 0x6200 },
    { label: "Num +", value: 0x5700 },
    { label: "Num -", value: 0x5600 },
    { label: "Num *", value: 0x5500 },
    { label: "Num /", value: 0x5400 },
    { label: "Num .", value: 0x6300 },
    { label: "Num Enter", value: 0x5800 },
    { label: "Num Lock", value: 0x5300 },
    // Function key
    { label: "Fn", value: 0xb000 },
  ];
  return options.sort((a, b) => a.label.localeCompare(b.label));
});

const keyCodeMap: Record<number, string> = {
  0x1e00: "1",
  0x1f00: "2",
  0x2000: "3",
  0x2100: "4",
  0x2200: "5",
  0x2300: "6",
  0x2400: "7",
  0x2500: "8",
  0x2600: "9",
  0x2700: "0",
  0x0400: "A",
  0x0500: "B",
  0x0600: "C",
  0x0700: "D",
  0x0800: "E",
  0x0900: "F",
  0x0a00: "G",
  0x0b00: "H",
  0x0c00: "I",
  0x0d00: "J",
  0x0e00: "K",
  0x0f00: "L",
  0x1000: "M",
  0x1100: "N",
  0x1200: "O",
  0x1300: "P",
  0x1400: "Q",
  0x1500: "R",
  0x1600: "S",
  0x1700: "T",
  0x1800: "U",
  0x1900: "V",
  0x1a00: "W",
  0x1b00: "X",
  0x1c00: "Y",
  0x1d00: "Z",
  0x2d00: "-",
  0x2e00: "=",
  0x2f00: "[",
  0x3000: "]",
  0x3100: "\\",
  0x3300: ";",
  0x3400: "'",
  0x3500: "`",
  0x3600: ",",
  0x3700: ".",
  0x3800: "/",
  0x2900: "Esc",
  0x2b00: "Tab",
  0x2800: "Enter",
  0x2c00: "Space",
  0x2a00: "Backspace",
  0x3900: "Caps",
  0x6500: "Menu",
  0x4600: "PrtSc",
  0x5200: "↑",
  0x5100: "↓",
  0x5000: "←",
  0x4f00: "→",
  0x4a00: "Home",
  0x4d00: "End",
  0x4b00: "PgUp",
  0x4e00: "PgDn",
  0x4900: "Ins",
  0x4c00: "Del",
  0x4800: "Pause",
  0x3a00: "F1",
  0x3b00: "F2",
  0x3c00: "F3",
  0x3d00: "F4",
  0x3e00: "F5",
  0x3f00: "F6",
  0x4000: "F7",
  0x4100: "F8",
  0x4200: "F9",
  0x4300: "F10",
  0x4400: "F11",
  0x4500: "F12",
  0x010000: "LCtrl",
  0x100000: "RCtrl",
  0x020000: "LShift",
  0x200000: "RShift",
  0x040000: "LAlt",
  0x400000: "RAlt",
  0x080000: "LWin",
  0x800000: "RWin",
  0x5900: "Num1",
  0x5a00: "Num2",
  0x5b00: "Num3",
  0x5c00: "Num4",
  0x5d00: "Num5",
  0x5e00: "Num6",
  0x5f00: "Num7",
  0x6000: "Num8",
  0x6100: "Num9",
  0x6200: "Num0",
  0x5700: "Num+",
  0x5600: "Num-",
  0x5500: "Num*",
  0x5400: "Num/",
  0x6300: "Num.",
  0x5800: "NumEnter",
  0x5300: "NumLock",
  0xb000: "Fn",
};

function getKeyName(keyCode: number | null | undefined): string {
  if (keyCode === null || keyCode === undefined) return "Unknown";
  return keyCodeMap[keyCode] || `0x${keyCode.toString(16)}`;
}

function getCurrentKeyCode(bufferIndex: number | null): number | null {
  if (bufferIndex === null) return null;
  const key = props.keyboard.keys.find((k) => k.buffer_index === bufferIndex);
  return key?.key_code ?? null;
}

function hasMapping(bufferIndex: number): boolean {
  return mappings.value.some((m) => m.buffer_index === bufferIndex);
}

function selectKey(bufferIndex: number) {
  selectedKeyIndex.value = bufferIndex;
  const existingMapping = mappings.value.find(
    (m) => m.buffer_index === bufferIndex
  );
  if (existingMapping) {
    selectedKeyCode.value = existingMapping.key_code;
  } else {
    const currentCode = getCurrentKeyCode(bufferIndex);
    selectedKeyCode.value = currentCode !== null ? currentCode : null;
  }
}

function addMapping() {
  if (selectedKeyIndex.value === null || selectedKeyCode.value === null) {
    return;
  }

  const newMappings = [...mappings.value];
  const existingIndex = newMappings.findIndex(
    (m) => m.buffer_index === selectedKeyIndex.value
  );

  if (existingIndex >= 0) {
    newMappings[existingIndex] = {
      buffer_index: selectedKeyIndex.value,
      key_code: selectedKeyCode.value,
    };
  } else {
    newMappings.push({
      buffer_index: selectedKeyIndex.value,
      key_code: selectedKeyCode.value,
    });
  }

  emit("update:modelValue", { mappings: newMappings });
}

function removeMapping() {
  if (selectedKeyIndex.value === null) {
    return;
  }
  removeMappingByIndex(selectedKeyIndex.value);
}

function removeMappingByIndex(bufferIndex: number) {
  const newMappings = mappings.value.filter(
    (m) => m.buffer_index !== bufferIndex
  );
  emit("update:modelValue", { mappings: newMappings });
  if (selectedKeyIndex.value === bufferIndex) {
    selectedKeyIndex.value = null;
    selectedKeyCode.value = null;
  }
}

function onImageLoad(event: Event) {
  const img = event.target as HTMLImageElement;
  imageWidth.value = img.naturalWidth;
  imageHeight.value = img.naturalHeight;
  imageLoaded.value = true;
  imgElement.value = img;
}

// Calculate scaling factors
// Key coordinates are absolute in the full image (including border)
// We need to scale from natural image size to displayed size
const scaleX = computed(() => {
  if (!imageLoaded.value || !imgElement.value) return 1;
  const displayedWidth =
    imgElement.value.width || imgElement.value.naturalWidth;
  const naturalWidth = imgElement.value.naturalWidth;
  if (naturalWidth === 0) return 1;
  // Scale factor from natural image size to displayed size
  return displayedWidth / naturalWidth;
});

const scaleY = computed(() => {
  if (!imageLoaded.value || !imgElement.value) return 1;
  const displayedHeight =
    imgElement.value.height || imgElement.value.naturalHeight;
  const naturalHeight = imgElement.value.naturalHeight;
  if (naturalHeight === 0) return 1;
  // Scale factor from natural image size to displayed size
  return displayedHeight / naturalHeight;
});

function getScaledX(x: number): number {
  // Key coordinates are already absolute in the natural image coordinate system
  // Just scale them to the displayed image size
  return x * scaleX.value;
}

function getScaledY(y: number): number {
  // Key coordinates are already absolute in the natural image coordinate system
  // Just scale them to the displayed image size
  return y * scaleY.value;
}

function getScaledWidth(key: Key): number {
  return (key.bottom_x - key.top_x) * scaleX.value;
}

function getScaledHeight(key: Key): number {
  return (key.bottom_y - key.top_y) * scaleY.value;
}

function getOverlayWidth(): number {
  if (!imgElement.value) return imageWidth.value;
  return (
    imgElement.value.width || imgElement.value.naturalWidth || imageWidth.value
  );
}

function getOverlayHeight(): number {
  if (!imgElement.value) return imageHeight.value;
  return (
    imgElement.value.height ||
    imgElement.value.naturalHeight ||
    imageHeight.value
  );
}
</script>

<template>
  <div class="space-y-6">
    <!-- Keyboard Image with Clickable Keys -->
    <div class="w-full flex justify-center">
      <div class="relative inline-block">
        <img
          ref="keyboardImage"
          :src="keyboardImagePath"
          :alt="keyboard.name"
          class="max-w-full h-auto block"
          @load="onImageLoad"
        />
        <!-- Overlay for clickable keys -->
        <div
          v-if="imageLoaded && imgElement"
          class="absolute top-0 left-0 pointer-events-none"
          :style="{
            width: getOverlayWidth() + 'px',
            height: getOverlayHeight() + 'px',
          }"
        >
          <button
            v-for="key in keyboard.keys"
            :key="key.buffer_index"
            :class="[
              'absolute border-2 rounded transition-all pointer-events-auto cursor-pointer',
              selectedKeyIndex === key.buffer_index
                ? 'border-primary bg-primary/20 z-10'
                : hasMapping(key.buffer_index)
                ? 'border-warning bg-warning/10'
                : 'border-transparent hover:border-gray-400',
            ]"
            :style="{
              left: getScaledX(key.top_x) + 'px',
              top: getScaledY(key.top_y) + 'px',
              width: getScaledWidth(key) + 'px',
              height: getScaledHeight(key) + 'px',
            }"
            @click="selectKey(key.buffer_index)"
            :title="`Buffer ${key.buffer_index}: ${getKeyName(key.key_code)}`"
          />
        </div>
      </div>
    </div>

    <!-- Key Mapping Controls -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <UCard v-if="selectedKeyIndex !== null">
        <template #header>
          <h3 class="text-lg font-semibold">Map Key</h3>
        </template>
        <div class="space-y-4 p-4">
          <div>
            <p class="text-sm text-muted mb-2">
              Selected Key: Buffer Index {{ selectedKeyIndex }}
            </p>
            <p class="text-sm text-muted">
              Current:
              {{ getKeyName(getCurrentKeyCode(selectedKeyIndex)) || "N/A" }}
            </p>
          </div>

          <UFormField label="Map to Key Code">
            <USelectMenu
              v-model="selectedKeyCode!"
              :items="keyCodeOptions"
              value-key="value"
              placeholder="Select a key code"
              class="w-full min-w-[300px]"
            />
          </UFormField>

          <div class="flex gap-2">
            <UButton
              @click="addMapping"
              color="primary"
              :disabled="!selectedKeyCode"
              icon="i-lucide-plus"
            >
              Add
            </UButton>
            <UButton
              @click="removeMapping"
              color="error"
              variant="outline"
              :disabled="!hasMapping(selectedKeyIndex)"
              icon="i-lucide-trash-2"
            >
              Remove
            </UButton>
          </div>
        </div>
      </UCard>

      <UCard>
        <template #header>
          <h3 class="text-lg font-semibold">Mapped Keys</h3>
        </template>
        <div class="p-4">
          <div v-if="mappings.length === 0" class="text-sm text-muted">
            No keys mapped. Click on a key to start mapping.
          </div>
          <div v-else class="space-y-2">
            <div
              v-for="mapping in mappings"
              :key="`${mapping.buffer_index}-${mapping.key_code}`"
              class="flex items-center justify-between p-2 bg-muted rounded"
            >
              <div>
                <p class="text-sm font-medium">
                  Buffer {{ mapping.buffer_index }}
                </p>
                <p class="text-xs text-muted">
                  {{ getKeyName(mapping.key_code) }}
                </p>
              </div>
              <UButton
                @click="removeMappingByIndex(mapping.buffer_index)"
                color="error"
                variant="ghost"
                size="xs"
                icon="i-lucide-x"
              />
            </div>
          </div>
        </div>
      </UCard>
    </div>
  </div>
</template>
