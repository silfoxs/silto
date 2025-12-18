<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Minus, Square, X } from 'lucide-vue-next'

const appWindow = getCurrentWindow()

const minimize = () => appWindow.minimize()
const toggleMaximize = () => appWindow.toggleMaximize()
const close = () => appWindow.hide() // Changed from close() to hide()
</script>

<template>
  <div class="flex items-center gap-0.5 px-0.5 group/controls">
    <!-- Close -->
    <button
      @click="close"
      class="relative flex items-center justify-center w-6 h-6 transition-all hover:scale-110 active:scale-95 group/btn"
      title="Close"
    >
      <div class="w-3.5 h-3.5 rounded-full bg-[#ff5f57] border border-black/10 flex items-center justify-center">
        <X class="w-2.5 h-2.5 text-black/50 opacity-0 group-hover/controls:opacity-100 transition-opacity" />
      </div>
    </button>
    
    <!-- Minimize -->
    <button
      @click="minimize"
      class="relative flex items-center justify-center w-6 h-6 transition-all hover:scale-110 active:scale-95 group/btn"
      title="Minimize"
    >
      <div class="w-3.5 h-3.5 rounded-full bg-[#febc2e] border border-black/10 flex items-center justify-center">
        <Minus class="w-2.5 h-2.5 text-black/50 opacity-0 group-hover/controls:opacity-100 transition-opacity" />
      </div>
    </button>

    <!-- Maximize -->
    <button
      @click="toggleMaximize"
      class="relative flex items-center justify-center w-6 h-6 transition-all hover:scale-110 active:scale-95 group/btn"
      title="Maximize"
    >
      <div class="w-3.5 h-3.5 rounded-full bg-[#28c940] border border-black/10 flex items-center justify-center">
        <Square class="w-2 h-2 text-black/50 opacity-0 group-hover/controls:opacity-100 transition-opacity" />
      </div>
    </button>
  </div>
</template>

<style scoped>
button {
  -webkit-app-region: no-drag;
  cursor: pointer;
  pointer-events: auto;
}
</style>
