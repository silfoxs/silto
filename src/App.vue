<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { listen, emit } from '@tauri-apps/api/event'
import { Window, getCurrentWindow } from '@tauri-apps/api/window'
import { PhysicalPosition, PhysicalSize } from '@tauri-apps/api/dpi'
import { Plus, Settings as SettingsIcon } from 'lucide-vue-next'
import TodoList from '@/components/TodoList.vue'
import NoteList from '@/components/NoteList.vue'
import TodoEditor from '@/components/TodoEditor.vue'
import NoteEditor from '@/components/NoteEditor.vue'
import Settings from '@/components/Settings.vue'
import Button from '@/components/ui/Button.vue'
import { useSettings } from '@/composables/useSettings'
import { useTodos } from '@/composables/useTodos'
import { useNotes } from '@/composables/useNotes'
import type { Todo, Note } from '@/types'

const { settings } = useSettings()
const { saveTodo } = useTodos()
const { saveNote } = useNotes()

const activeView = ref<'todo' | 'note'>('todo')
const showTodoEditor = ref(false)
const showNoteEditor = ref(false)
const showSettings = ref(false)
const editingTodo = ref<Todo | null>(null)
const editingNote = ref<Note | null>(null)
const sideEditorUnlisten = ref<(() => void) | null>(null)

const viewTitle = computed(() => {
  return activeView.value === 'todo' ? 'Todo 列表' : '便签列表'
})

const handleOpenSideEditor = async (payload: { type: 'todo' | 'note', data: any }) => {
  const label = `editor-side`
  const mainWindow = getCurrentWindow()
  
const mainSize = await mainWindow.innerSize()
  const mainPos = await mainWindow.innerPosition()
  const factor = await mainWindow.scaleFactor()
  
  // Clean up previous listener to prevent stale data
  if (sideEditorUnlisten.value) {
    sideEditorUnlisten.value()
    sideEditorUnlisten.value = null
  }
  
  // Calculate side window position (Right side of main window)
  const gap = 20 * factor
  const sideX = mainPos.x + mainSize.width + gap
  const sideY = mainPos.y
  
  // Dimensions: Height matches main window, Width is wider (e.g. 800 logical pixels)
  // Note: mainSize is PhysicalSize (pixels). We need to be careful with units.
  // if we set width: 800 in constructor, it's roughly logical.
  // Let's use physical values derived from factor to be precise or rely on logical.
  
  // Just use reasonable defaults for now but ensure Y matches.
  // Logical height = mainSize.height / factor
  const logicalHeight = mainSize.height / factor
  const logicalWidth = 800 // Wider than probably ~400px main window

  // Check if window already exists
  let sideWindow = await Window.getByLabel(label)
  
  if (sideWindow) {
    await sideWindow.setFocus()
    // Update content
    await emit(`editor-init-${label}`, payload)
    // Also update position/size if already open?
    // User might have moved it, so maybe respecting their position is better.
    // But forcing alignment on open is also good "snapping" behavior.
    // User requirement: "Open a ... page".
    // I will enforce position/size on fresh open, but minimal interference if already open?
    // Let's enforce it for consistent experience per request.
     await sideWindow.setPosition(new PhysicalPosition(sideX, sideY))
     await sideWindow.setSize(new PhysicalSize(logicalWidth * factor, mainSize.height))
  } else {
    // Setup listener BEFORE creating window
    // We keep listening to handle retries from the side window
    sideEditorUnlisten.value = await listen(`editor-ready-${label}`, async () => {
       console.log(`Received ready from ${label}, sending init...`)
       await emit(`editor-init-${label}`, payload)
       
       // Re-fetch window to ensure positioning
       const win = await Window.getByLabel(label)
       if (win) {
         await win.setPosition(new PhysicalPosition(sideX, sideY))
       }
    })

     sideWindow = new Window(label, {
      url: 'index.html?window=editor',
      title: payload.type === 'todo' ? 'Edit Todo' : 'Edit Note',
      x: sideX / factor,
      y: sideY / factor,
      width: logicalWidth,
      height: logicalHeight,
      focus: true,
      skipTaskbar: false,
      transparent: true, // Enable transparency for blur effect
      decorations: false, // Custom titlebar
      resizable: true,
      // On macOS, we might want to ensure 'vibrancy' if supported by the plugin/logic, 
      // but 'transparent: true' is the prerequisite.
    } as any)
  }
}

// 监听托盘事件
onMounted(async () => {
  // 根据设置初始化视图
  activeView.value = settings.value.left_click_action

  // 监听托盘菜单事件
  await listen('tray-add-todo', () => {
    editingTodo.value = null
    showTodoEditor.value = true
    activeView.value = 'todo'
  })

  await listen('tray-add-note', () => {
    editingNote.value = null
    showNoteEditor.value = true
    activeView.value = 'note'
  })

  await listen('tray-settings', () => {
    showSettings.value = true
  })

   // Listen for Save from Side Editor
   await listen<{ type: 'todo' | 'note', data: any }>('side-editor-save', async (event) => {
      const { type, data } = event.payload
      if (type === 'todo') {
        await saveTodo(data)
      } else {
        await saveNote(data)
      }
      const sideWin = await Window.getByLabel('editor-side')
      await sideWin?.close()
   })
})

const handleAddNew = () => {
  if (activeView.value === 'todo') {
    editingTodo.value = null
    showTodoEditor.value = true
  } else {
    editingNote.value = null
    showNoteEditor.value = true
  }
}

const handleEditTodo = (todo: Todo) => {
  editingTodo.value = todo
  showTodoEditor.value = true
}

const handleEditNote = (note: Note) => {
  editingNote.value = note
  showNoteEditor.value = true
}
</script>

<template>
  <!-- Main Container: Transparent + Rounded + Border -->
  <div class="flex flex-col h-screen bg-background/80 dark:bg-black/80 backdrop-blur-xl text-foreground rounded-2xl border border-white/20 overflow-hidden">
    <!-- Title Bar -->
    <!-- Use bg-transparent or very light overlay -->
    <div class="titlebar flex items-center justify-between px-4 py-3 border-b border-white/10 bg-white/5">
      <div class="flex items-center gap-3">
        <h1 class="text-lg font-semibold">{{ viewTitle }}</h1>
      </div>
      <div class="flex items-center gap-2">
        <Button 
          variant="ghost" 
          size="icon"
          @click="showSettings = true"
          class="hover:bg-white/10"
        >
          <SettingsIcon class="w-4 h-4" />
        </Button>
        <div class="flex gap-1 bg-black/20 rounded-md p-0.5">
          <Button
            :variant="activeView === 'todo' ? 'secondary' : 'ghost'"
            size="sm"
            @click="activeView = 'todo'"
            class="data-[variant=ghost]:hover:bg-white/10"
          >
            Todo
          </Button>
          <Button
            :variant="activeView === 'note' ? 'secondary' : 'ghost'"
            size="sm"
            @click="activeView = 'note'"
            class="data-[variant=ghost]:hover:bg-white/10"
          >
            便签
          </Button>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 overflow-hidden">
      <TodoList 
        v-if="activeView === 'todo'" 
        @edit="handleEditTodo"
      />
      <NoteList 
        v-else 
        @edit="handleEditNote"
      />
    </div>

    <!-- Bottom Action Bar -->
    <div class="border-t border-white/10 bg-white/5 p-3">
      <Button 
        class="w-full bg-primary/90 hover:bg-primary shadow-lg" 
        @click="handleAddNew"
      >
        <Plus class="w-4 h-4 mr-2" />
        {{ activeView === 'todo' ? '新建 Todo' : '新建便签' }}
      </Button>
    </div>

    <!-- Editors and Settings -->
    <TodoEditor 
      v-model:open="showTodoEditor"
      :todo="editingTodo"
      @open-side-editor="handleOpenSideEditor"
    />
    <NoteEditor 
      v-model:open="showNoteEditor"
      :note="editingNote"
      @open-side-editor="handleOpenSideEditor"
    />
    <Settings 
      v-model:open="showSettings"
    />
    <div id="app-portal"></div>
  </div>
</template>