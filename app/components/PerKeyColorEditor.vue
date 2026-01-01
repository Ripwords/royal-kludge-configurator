<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue";
import { resolveResource } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { Key, PerKeyColor, RgbColor } from "~/composables/useKeyboard";

interface Props {
  keyboard: Keyboard;
  modelValue: PerKeyColor[];
}

interface Keyboard {
  id: { vid: number; pid: number };
  path: string;
  name: string;
  image_path: string;
  keys: Key[];
  key_map_enabled: boolean;
  light_enabled: boolean;
  rgb: boolean;
  top_left_x: number;
  top_left_y: number;
  bottom_right_x: number;
  bottom_right_y: number;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  "update:modelValue": [value: PerKeyColor[]];
}>();

const colors = computed({
  get: () => props.modelValue,
  set: (value) => emit("update:modelValue", value),
});

const selectedKeyIndex = ref<number | null>(null);
const selectedColorHex = ref("#ffffff");
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

const keyboardImage = ref<HTMLImageElement | null>(null);
const imageLoaded = ref(false);
const imageWidth = ref(0);
const imageHeight = ref(0);
const imgElement = ref<HTMLImageElement | null>(null);

const onImageLoad = (event?: Event) => {
  const img = (event?.target as HTMLImageElement) || keyboardImage.value;
  if (img) {
    imgElement.value = img;
    imageWidth.value = img.naturalWidth;
    imageHeight.value = img.naturalHeight;
    imageLoaded.value = true;
  }
};

onMounted(() => {
  // Get reference to the image element
  const img = keyboardImage.value as HTMLImageElement | null;
  if (img) {
    imgElement.value = img;
    if (img.complete) {
      onImageLoad();
    }
  }
});

// Calculate scaling factors
const scaleX = computed(() => {
  if (!imageLoaded.value || !imgElement.value) return 1;
  const displayedWidth =
    imgElement.value.width || imgElement.value.naturalWidth;
  const naturalWidth = imgElement.value.naturalWidth;
  if (naturalWidth === 0) return 1;
  return displayedWidth / naturalWidth;
});

const scaleY = computed(() => {
  if (!imageLoaded.value || !imgElement.value) return 1;
  const displayedHeight =
    imgElement.value.height || imgElement.value.naturalHeight;
  const naturalHeight = imgElement.value.naturalHeight;
  if (naturalHeight === 0) return 1;
  return displayedHeight / naturalHeight;
});

function getScaledX(x: number): number {
  return x * scaleX.value;
}

function getScaledY(y: number): number {
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

function selectKey(bufferIndex: number) {
  selectedKeyIndex.value = bufferIndex;
  const existingColor = colors.value.find(
    (c) => c.buffer_index === bufferIndex
  );
  if (existingColor) {
    selectedColorHex.value = rgbToHex(
      existingColor.color.r,
      existingColor.color.g,
      existingColor.color.b
    );
  } else {
    // Reset to white if no color assigned
    selectedColorHex.value = "#ffffff";
  }
}

function hasColor(bufferIndex: number): boolean {
  return colors.value.some((c) => c.buffer_index === bufferIndex);
}

function getColorHex(bufferIndex: number): string {
  const colorEntry = colors.value.find((c) => c.buffer_index === bufferIndex);
  if (!colorEntry) return "#ffffff";
  return rgbToHex(colorEntry.color.r, colorEntry.color.g, colorEntry.color.b);
}

function rgbToHex(r: number, g: number, b: number): string {
  return `#${[r, g, b]
    .map((x) => {
      const hex = x.toString(16);
      return hex.length === 1 ? "0" + hex : hex;
    })
    .join("")}`;
}

function hexToRgb(hex: string): RgbColor {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  if (!result) {
    return { r: 255, g: 255, b: 255 };
  }
  return {
    r: parseInt(result[1] || "255", 16),
    g: parseInt(result[2] || "255", 16),
    b: parseInt(result[3] || "255", 16),
  };
}

function applyToSelectedKey() {
  if (selectedKeyIndex.value === null) return;

  const rgb = hexToRgb(selectedColorHex.value);
  const existingIndex = colors.value.findIndex(
    (c) => c.buffer_index === selectedKeyIndex.value
  );

  if (existingIndex >= 0) {
    colors.value[existingIndex] = {
      buffer_index: selectedKeyIndex.value,
      color: rgb,
    };
  } else {
    colors.value = [
      ...colors.value,
      {
        buffer_index: selectedKeyIndex.value,
        color: rgb,
      },
    ];
  }
}

function applyToAllKeys() {
  const rgb = hexToRgb(selectedColorHex.value);
  colors.value = props.keyboard.keys.map((key) => ({
    buffer_index: key.buffer_index,
    color: rgb,
  }));
}

function clearSelectedKey() {
  if (selectedKeyIndex.value === null) return;
  colors.value = colors.value.filter(
    (c) => c.buffer_index !== selectedKeyIndex.value
  );
}

function clearAllKeys() {
  colors.value = [];
}

function removeColor(bufferIndex: number) {
  colors.value = colors.value.filter((c) => c.buffer_index !== bufferIndex);
}

function getKeyByBufferIndex(bufferIndex: number): number | null {
  const key = props.keyboard.keys.find((k) => k.buffer_index === bufferIndex);
  return key ? key.key_code : null;
}

function getKeyName(keyCode: number | null): string {
  if (keyCode === null) return "Unknown";
  // Simple key name mapping - can be expanded
  const keyNames: Record<number, string> = {
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
    0x2800: "Enter",
    0x2900: "Escape",
    0x2a00: "Backspace",
    0x2b00: "Tab",
    0x2c00: "Space",
  };
  return keyNames[keyCode] || `0x${keyCode.toString(16)}`;
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
          style="pointer-events: none"
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
                ? 'border-primary bg-primary/20 ring-2 ring-primary ring-offset-1 z-20'
                : hasColor(key.buffer_index)
                ? 'border-warning bg-warning/20 z-10'
                : 'border-transparent hover:border-gray-400 hover:bg-gray-400/10 z-0',
            ]"
            :style="{
              left: getScaledX(key.top_x) + 'px',
              top: getScaledY(key.top_y) + 'px',
              width: getScaledWidth(key) + 'px',
              height: getScaledHeight(key) + 'px',
              backgroundColor:
                selectedKeyIndex === key.buffer_index
                  ? 'transparent'
                  : hasColor(key.buffer_index)
                  ? getColorHex(key.buffer_index) + '80'
                  : 'transparent',
            }"
            @click="selectKey(key.buffer_index)"
            :title="`Buffer ${key.buffer_index}: ${getKeyName(
              key.key_code
            )} - Click to select`"
          />
        </div>
      </div>
    </div>

    <!-- Per-Key Color Controls -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <UCard>
        <template #header>
          <h3 class="text-lg font-semibold">Color Selection</h3>
        </template>
        <div class="space-y-4 p-4">
          <UFormField label="Select Color">
            <div class="flex gap-2 items-center">
              <input
                v-model="selectedColorHex"
                type="color"
                class="w-16 h-10 rounded border border-default cursor-pointer"
              />
              <UInput
                v-model="selectedColorHex"
                placeholder="#ffffff"
                class="flex-1"
              />
            </div>
          </UFormField>

          <div class="flex gap-2">
            <UButton
              @click="applyToSelectedKey"
              color="primary"
              :disabled="!selectedKeyIndex"
              icon="i-lucide-paintbrush"
              class="flex-1"
            >
              Apply to Selected Key
            </UButton>
            <UButton
              @click="applyToAllKeys"
              color="primary"
              variant="outline"
              icon="i-lucide-paintbrush-2"
            >
              Apply to All
            </UButton>
          </div>

          <div class="flex gap-2">
            <UButton
              @click="clearSelectedKey"
              color="error"
              variant="outline"
              :disabled="!selectedKeyIndex || !hasColor(selectedKeyIndex!)"
              icon="i-lucide-x"
              class="flex-1"
            >
              Clear Selected
            </UButton>
            <UButton
              @click="clearAllKeys"
              color="error"
              variant="outline"
              :disabled="colors.length === 0"
              icon="i-lucide-trash-2"
            >
              Clear All
            </UButton>
          </div>

          <div
            v-if="selectedKeyIndex !== null"
            class="mt-4 p-3 bg-primary/10 border border-primary rounded-lg"
          >
            <p class="text-sm font-medium text-primary">
              ✓ Selected Key: Buffer Index {{ selectedKeyIndex }}
            </p>
            <p class="text-xs text-muted mt-1">
              Key: {{ getKeyName(getKeyByBufferIndex(selectedKeyIndex)) }}
            </p>
            <p
              v-if="hasColor(selectedKeyIndex)"
              class="text-xs text-muted mt-1"
            >
              Current Color: {{ getColorHex(selectedKeyIndex) }}
            </p>
            <p v-else class="text-xs text-muted mt-1">No color assigned yet</p>
          </div>
          <div v-else class="mt-4 p-3 bg-muted rounded-lg">
            <p class="text-sm text-muted">
              Click on a key in the keyboard image above to select it
            </p>
          </div>
        </div>
      </UCard>

      <UCard>
        <template #header>
          <h3 class="text-lg font-semibold">Colored Keys</h3>
        </template>
        <div class="p-4">
          <div v-if="colors.length === 0" class="text-sm text-muted">
            No keys colored. Click on a key and select a color to start.
          </div>
          <div v-else class="space-y-2 max-h-96 overflow-y-auto">
            <div
              v-for="colorEntry in colors"
              :key="colorEntry.buffer_index"
              class="flex items-center justify-between p-2 bg-muted rounded"
            >
              <div class="flex items-center gap-3">
                <div
                  class="w-8 h-8 rounded border border-default"
                  :style="{
                    backgroundColor: getColorHex(colorEntry.buffer_index),
                  }"
                />
                <div>
                  <p class="text-sm font-medium">
                    Buffer {{ colorEntry.buffer_index }}
                  </p>
                  <p class="text-xs text-muted">
                    {{
                      getKeyName(getKeyByBufferIndex(colorEntry.buffer_index))
                    }}
                  </p>
                </div>
              </div>
              <UButton
                @click="removeColor(colorEntry.buffer_index)"
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
