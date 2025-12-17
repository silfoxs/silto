<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen, emit } from '@tauri-apps/api/event'
import TodoForm from '@/components/TodoForm.vue'
import NoteForm from '@/components/NoteForm.vue'
import { X, Loader2 } from 'lucide-vue-next'
import Button from '@/components/ui/Button.vue'

const currentType = ref<'todo' | 'note' | null>(null)
const currentData = ref<any>(null)
const windowLabel = ref('')
const status = ref('Initializing...')
const isReady = ref(false)

// Optional: Standalone specific data for testing if needed
// const isStandalone = ref(false) 

onMounted(async () => {
  console.log('[EditorWindow] Mounted')
  const appWindow = getCurrentWindow()
  windowLabel.value = appWindow.label
  
  // Set window title dynamically
  await appWindow.setTitle('Editor')
  
  // Setup listener
  const unlistenInit = await listen<{ type: 'todo' | 'note', data: any }>(`editor-init-${windowLabel.value}`, (event) => {
    console.log('[EditorWindow] Received init payload:', event.payload)
    currentType.value = event.payload.type
    currentData.value = event.payload.data
    isReady.value = true
    status.value = 'Ready'
  })

  // Signal readiness to main window
  console.log('[EditorWindow] Signalling ready...')
  await emit(`editor-ready-${windowLabel.value}`)
  
  // Fallback: If we don't get data quickly (e.g. reload), ask again periodically
  const interval = setInterval(async () => {
    if (!isReady.value) {
      console.log('[EditorWindow] Retry signalling ready...')
      await emit(`editor-ready-${windowLabel.value}`)
    } else {
      clearInterval(interval)
    }
  }, 1000)

  onUnmounted(() => {
    unlistenInit()
    clearInterval(interval)
  })
})

const handleSave = async (data: any) => {
  console.log('[EditorWindow] Saving data...')
  await emit(`side-editor-save`, { 
    type: currentType.value, 
    data: data 
  })
}

const handleCancel = async () => {
  const appWindow = getCurrentWindow()
  await appWindow.close()
}
</script>

<template>
  <div class="h-screen w-full flex flex-col bg-background/80 dark:bg-black/80 backdrop-blur-xl text-foreground overflow-hidden">
    <!-- Toolbar (Always visible) -->
    <div class="flex items-center justify-between px-4 py-3 border-b border-border/50 bg-muted/30 backdrop-blur-sm drag-region">
      <div class="flex items-center gap-3">
        <div class="w-2 h-2 rounded-full" :class="isReady ? 'bg-green-500' : 'bg-yellow-500'"></div>
        <h2 class="font-medium text-sm text-foreground/90">
          {{ currentType === 'todo' ? 'Edit Todo' : (currentType === 'note' ? 'Edit Note' : 'Loading Editor...') }}
        </h2>
      </div>
      <div class="flex items-center gap-2">
         <Button variant="ghost" size="icon" @click="handleCancel" class="h-7 w-7 hover:bg-destructive/10 hover:text-destructive transition-colors">
          <X class="w-4 h-4" />
        </Button>
      </div>
    </div>

    <!-- Content Area -->
    <div class="flex-1 overflow-hidden relative">
      <!-- Loading State -->
      <div v-if="!isReady" class="absolute inset-0 flex flex-col items-center justify-center bg-background/50 backdrop-blur-[1px] z-50">
        <Loader2 class="w-8 h-8 animate-spin text-primary mb-4" />
        <p class="text-sm text-muted-foreground">{{ status }}</p>
      </div>

      <!-- Forms -->
      <div class="h-full overflow-y-auto p-6 animate-in fade-in duration-300" v-if="isReady">
        <div class="max-w-2xl mx-auto">
          <TodoForm 
            v-if="currentType === 'todo'"
            :todo="currentData"
            :allow-expand="false"
            @save="handleSave"
            @cancel="handleCancel"
          />
          
          <NoteForm 
            v-if="currentType === 'note'"
            :note="currentData"
            :allow-expand="false"
            @save="handleSave"
            @cancel="handleCancel"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.drag-region {
  -webkit-app-region: drag;
}
.drag-region button {
  -webkit-app-region: no-drag;
}
</style>
