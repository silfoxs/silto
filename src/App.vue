<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { Plus, Settings as SettingsIcon } from 'lucide-vue-next'
import TodoList from '@/components/TodoList.vue'
import NoteList from '@/components/NoteList.vue'
import TodoEditor from '@/components/TodoEditor.vue'
import NoteEditor from '@/components/NoteEditor.vue'
import Settings from '@/components/Settings.vue'
import WindowControls from '@/components/WindowControls.vue'
import Button from '@/components/ui/Button.vue'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useSettings } from '@/composables/useSettings'
import type { Todo, Note } from '@/types'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const { settings } = useSettings()

const activeView = ref<'todo' | 'note'>('todo')
const showTodoEditor = ref(false)
const showNoteEditor = ref(false)
const showSettings = ref(false)
const editingTodo = ref<Todo | null>(null)
const editingNote = ref<Note | null>(null)

const viewTitle = computed(() => {
  return activeView.value === 'todo' ? 'Todo' : t('settings.noteList')
})

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
  <Tabs
    v-model="activeView"
    class="flex flex-col h-screen bg-white/80 dark:bg-black/40 backdrop-blur-xl text-foreground rounded-2xl border border-white/20 overflow-hidden relative"
  >
    <!-- Gradient Blur Layers -->
    <div class="absolute top-0 left-0 right-0 h-20 z-10 pointer-events-none overflow-hidden select-none">
      <div class="absolute inset-0 backdrop-blur-[2px] [mask-image:linear-gradient(to_bottom,black_0%,transparent_25%)]"></div>
      <div class="absolute inset-0 backdrop-blur-[4px] [mask-image:linear-gradient(to_bottom,black_0%,transparent_50%)]"></div>
      <div class="absolute inset-0 backdrop-blur-[8px] [mask-image:linear-gradient(to_bottom,black_0%,transparent_75%)]"></div>
      <div class="absolute inset-0 backdrop-blur-[16px] [mask-image:linear-gradient(to_bottom,black_0%,black_25%,transparent_100%)]"></div>
      <div class="absolute inset-0 backdrop-blur-[24px] [mask-image:linear-gradient(to_bottom,black_0%,black_50%,transparent_100%)]"></div>
      <div class="absolute inset-0 backdrop-blur-[40px] [mask-image:linear-gradient(to_bottom,black_0%,black_75%,transparent_100%)]"></div>
    </div>

    <!-- Title Bar -->
    <div class="titlebar absolute top-0 left-0 right-0 flex items-center justify-between z-20 px-4 pt-4 pb-2" data-tauri-drag-region>
      <div class="flex items-center gap-3">
        <WindowControls />
        <h1 class="text-xl font-bold select-none text-foreground/90 pointer-events-none">{{ viewTitle }}</h1>
      </div>
      
      <div class="flex items-center gap-2">
        <Button 
          variant="ghost" 
          size="icon"
          @click="showSettings = true"
          class="hover:bg-white/10 w-8 h-8 rounded-lg"
        >
          <SettingsIcon class="w-4 h-4" />
        </Button>
        <TabsList class="bg-black/30 dark:bg-black/60 backdrop-blur-xl border border-white/20 rounded-lg p-0.5 h-7 overflow-hidden group shadow-inner">
          <TabsTrigger 
            value="todo"
            class="px-3 h-full text-[10px] font-bold rounded-md transition-all duration-300 data-[state=inactive]:text-white/40 hover:data-[state=inactive]:text-white hover:data-[state=inactive]:bg-white/5 data-[state=active]:bg-white data-[state=active]:text-black data-[state=active]:shadow-[0_0_0_1px_rgba(255,255,255,0.1),0_2px_4px_rgba(0,0,0,0.2)]"
          >
            Todo
          </TabsTrigger>
          <TabsTrigger 
            value="note"
            class="px-3 h-full text-[10px] font-bold rounded-md transition-all duration-300 data-[state=inactive]:text-white/40 hover:data-[state=inactive]:text-white hover:data-[state=inactive]:bg-white/5 data-[state=active]:bg-white data-[state=active]:text-black data-[state=active]:shadow-[0_0_0_1px_rgba(255,255,255,0.1),0_2px_4px_rgba(0,0,0,0.2)]"
          >
            {{ $t('settings.noteList') }}
          </TabsTrigger>
        </TabsList>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 min-h-0">
      <TabsContent value="todo" class="h-full mt-0">
        <TodoList @edit="handleEditTodo" />
      </TabsContent>
      <TabsContent value="note" class="h-full mt-0">
        <NoteList @edit="handleEditNote" />
      </TabsContent>
    </div>

    <!-- Bottom Action Bar: Floating Liquid Glass -->
    <div class="absolute bottom-6 left-0 right-0 px-6 pb-2 z-20 pointer-events-none">
      <Button 
        class="w-full h-12 bg-white/90 dark:bg-black/80 backdrop-blur-xl backdrop-saturate-150 border border-white dark:border-white/20 rounded-full shadow-[0_0_15px_rgba(0,0,0,0.1)] dark:shadow-[0_0_15px_rgba(255,255,255,0.1)] hover:bg-white dark:hover:bg-black hover:scale-[1.05] active:scale-95 transition-all duration-300 pointer-events-auto text-foreground font-semibold" 
        @click="handleAddNew"
      >
        <Plus class="w-5 h-5 mr-2" />
        {{ activeView === 'todo' ? $t('todo.newTodo') : $t('note.newNote') }}
      </Button>
    </div>

    <!-- Editors and Settings -->
    <TodoEditor 
      v-model:open="showTodoEditor"
      :todo="editingTodo"
    />
    <NoteEditor 
      v-model:open="showNoteEditor"
      :note="editingNote"
    />
    <Settings 
      v-model:open="showSettings"
    />
    <div id="app-portal"></div>
  </Tabs>
</template>