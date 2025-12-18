<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { Plus, Settings as SettingsIcon } from 'lucide-vue-next'
import TodoList from '@/components/TodoList.vue'
import NoteList from '@/components/NoteList.vue'
import TodoEditor from '@/components/TodoEditor.vue'
import NoteEditor from '@/components/NoteEditor.vue'
import Settings from '@/components/Settings.vue'
import Button from '@/components/ui/Button.vue'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { useSettings } from '@/composables/useSettings'
import type { Todo, Note } from '@/types'

const { settings } = useSettings()

const activeView = ref<'todo' | 'note'>('todo')
const showTodoEditor = ref(false)
const showNoteEditor = ref(false)
const showSettings = ref(false)
const editingTodo = ref<Todo | null>(null)
const editingNote = ref<Note | null>(null)

const viewTitle = computed(() => {
  return activeView.value === 'todo' ? 'Todo' : '便签'
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
    class="flex flex-col h-screen bg-background/80 dark:bg-black/80 backdrop-blur-xl text-foreground rounded-2xl border border-white/20 overflow-hidden relative"
  >
    <!-- Title Bar -->
    <div class="titlebar absolute top-0 left-0 right-0 flex items-center justify-between px-4 py-3 border-b border-white/10 bg-white/5 backdrop-blur-xl z-20" data-tauri-drag-region>
      <div class="flex items-center gap-3 pointer-events-none">
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
        <TabsList class="bg-black/20">
          <TabsTrigger value="todo">
            Todo
          </TabsTrigger>
          <TabsTrigger value="note">
            便签
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

    <!-- Bottom Action Bar -->
    <div class="absolute bottom-0 left-0 right-0 border-t border-white/10 bg-white/5 p-3 backdrop-blur-xl z-20">
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