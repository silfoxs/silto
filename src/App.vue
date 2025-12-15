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
  return activeView.value === 'todo' ? 'Todo 列表' : '便签列表'
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
  <div class="flex flex-col h-screen bg-background text-foreground">
    <!-- Title Bar -->
    <div class="titlebar flex items-center justify-between px-4 py-3 border-b bg-card/50 backdrop-blur-sm">
      <div class="flex items-center gap-3">
        <h1 class="text-lg font-semibold">{{ viewTitle }}</h1>
      </div>
      <div class="flex items-center gap-2">
        <Button 
          variant="ghost" 
          size="icon"
          @click="showSettings = true"
        >
          <SettingsIcon class="w-4 h-4" />
        </Button>
        <div class="flex gap-1 bg-muted/50 rounded-md p-0.5">
          <Button
            :variant="activeView === 'todo' ? 'secondary' : 'ghost'"
            size="sm"
            @click="activeView = 'todo'"
          >
            Todo
          </Button>
          <Button
            :variant="activeView === 'note' ? 'secondary' : 'ghost'"
            size="sm"
            @click="activeView = 'note'"
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
    <div class="border-t bg-card/50 backdrop-blur-sm p-3">
      <Button 
        class="w-full" 
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
  </div>
</template>