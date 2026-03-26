<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Plus, Settings as SettingsIcon, Check, ChevronLeft } from 'lucide-vue-next'
import TodoList from '@/components/TodoList.vue'
import NoteList from '@/components/NoteList.vue'
import TodoForm from '@/components/TodoForm.vue'
import NoteForm from '@/components/NoteForm.vue'
import Settings from '@/components/Settings.vue'
import WindowControls from '@/components/WindowControls.vue'
import Button from '@/components/ui/Button.vue'
import { Tabs } from '@/components/ui/tabs'
import LiquidGlassTabs from '@/components/ui/LiquidGlassTabs.vue'
import type { Todo, Note } from '@/types'
import { useI18n } from 'vue-i18n'
import { useTodos } from '@/composables/useTodos'
import { useNotes } from '@/composables/useNotes'

const { t } = useI18n()
const { loadTodos, saveTodo } = useTodos()
const { loadNotes, saveNote } = useNotes()

const activePage = ref<'main' | 'settings'>('main')
const activeView = ref<'todo' | 'note'>('todo')
const showTodoEditor = ref(false)
const showNoteEditor = ref(false)
const editingTodo = ref<Todo | null>(null)
const editingNote = ref<Note | null>(null)
const autosaveToastVisible = ref(false)
let autosaveToastTimeout: ReturnType<typeof setTimeout> | undefined

const viewTitle = computed(() => {
  if (activePage.value === 'settings') {
    return t('settings.title')
  }
  return activeView.value === 'todo' ? 'Todo' : t('settings.noteList')
})

const isTodoEditorVisible = computed(() => activeView.value === 'todo' && showTodoEditor.value)
const isNoteEditorVisible = computed(() => activeView.value === 'note' && showNoteEditor.value)
const selectedTodoId = computed(() => activeView.value === 'todo' ? editingTodo.value?.id ?? null : null)
const selectedNoteId = computed(() => activeView.value === 'note' ? editingNote.value?.id ?? null : null)
const openSettingsPage = () => {
  activePage.value = 'settings'
}

const closeSettingsPage = () => {
  activePage.value = 'main'
}
// ----------------------------------------------------------------------
// Window Drag Logic
// ----------------------------------------------------------------------
const startDrag = (e: MouseEvent) => {
  // Ignore clicks on buttons or interactive elements
  if ((e.target as HTMLElement).closest('button, a, input, [role="button"]')) {
    return
  }
  
  getCurrentWindow().startDragging()
}
// ----------------------------------------------------------------------

// 监听托盘事件
onMounted(async () => {
  // 从 localStorage 恢复上次的视图状态
  const savedView = localStorage.getItem('activeView')
  if (savedView === 'todo' || savedView === 'note') {
    activeView.value = savedView
  }

  // 监听 activeView 变化，保存到 localStorage 以便 popup 窗口同步
  watch(() => activeView.value, (newView) => {
    localStorage.setItem('activeView', newView)
  }, { immediate: true })

  // 监听托盘菜单事件
  await listen('tray-add-todo', () => {
    activePage.value = 'main'
    editingTodo.value = null
    showTodoEditor.value = true
    activeView.value = 'todo'
  })

  await listen('tray-add-note', () => {
    activePage.value = 'main'
    editingNote.value = null
    showNoteEditor.value = true
    activeView.value = 'note'
  })

  await listen('tray-settings', () => {
    openSettingsPage()
  })

  // 监听来自弹窗的编辑事件
  await listen('edit-todo', (event: any) => {
    activePage.value = 'main'
    editingTodo.value = event.payload
    showTodoEditor.value = true
    activeView.value = 'todo'
  })

  await listen('edit-note', (event: any) => {
    activePage.value = 'main'
    editingNote.value = event.payload
    showNoteEditor.value = true
    activeView.value = 'note'
  })

  // 监听刷新数据事件
  await listen('refresh-data', async () => {
    await loadTodos()
    await loadNotes()
  })
})

const handleAddNew = () => {
  if (activeView.value === 'todo') {
    editingTodo.value = null
    showTodoEditor.value = true
    showNoteEditor.value = false
  } else {
    editingNote.value = null
    showNoteEditor.value = true
    showTodoEditor.value = false
  }
}

const handleEditTodo = (todo: Todo) => {
  editingTodo.value = todo
  showTodoEditor.value = true
  showNoteEditor.value = false
}

const handleEditNote = (note: Note) => {
  editingNote.value = note
  showNoteEditor.value = true
  showTodoEditor.value = false
}

const closeEditor = () => {
  showTodoEditor.value = false
  showNoteEditor.value = false
  editingTodo.value = null
  editingNote.value = null
}

const showAutosaveToast = () => {
  autosaveToastVisible.value = true
  if (autosaveToastTimeout) clearTimeout(autosaveToastTimeout)
  autosaveToastTimeout = setTimeout(() => {
    autosaveToastVisible.value = false
  }, 1400)
}

const handleSaveTodo = async (todo: Partial<Todo>) => {
  await saveTodo(todo as Todo)
  editingTodo.value = todo as Todo
  showTodoEditor.value = true
  showAutosaveToast()
}

const handleSaveNote = async (note: Partial<Note>) => {
  await saveNote(note as Note)
  editingNote.value = note as Note
  showNoteEditor.value = true
  showAutosaveToast()
}
</script>

<template>
  <!-- Main Container: Transparent + Rounded + Border -->
  <Tabs
    v-model="activeView"
    class="flex flex-col h-screen bg-white/[0.06] dark:bg-black/[0.18] backdrop-blur-[40px] text-foreground rounded-[16px] border border-black/[0.08] dark:border-white/14 overflow-hidden relative"
  >
    <div class="absolute inset-0 rounded-[16px] bg-white/[0.02] dark:bg-black/[0.12] z-0 pointer-events-none"></div>
    <div class="absolute inset-0 rounded-[16px] bg-white/[0.44] dark:bg-black/[0.56] z-0 pointer-events-none"></div>

    <!-- Title Bar -->
    <div 
      class="titlebar absolute top-0 left-0 right-0 flex items-center justify-between z-20 px-4 pt-3 pb-1 select-none"
      @mousedown="startDrag"
    >
      <div class="flex items-center gap-3">
        <WindowControls />
        <Button
          v-if="activePage === 'settings'"
          variant="ghost"
          size="icon"
          @click="closeSettingsPage"
          class="h-8 w-8 rounded-lg hover:bg-white/10"
        >
          <ChevronLeft class="h-4 w-4" />
        </Button>
        <h1 class="text-xl font-bold select-none text-foreground/90 pointer-events-none">{{ viewTitle }}</h1>
      </div>
      
      <div class="flex items-center gap-2">
        <Button 
          v-if="activePage === 'main'"
          variant="ghost" 
          size="icon"
          @click="openSettingsPage"
          class="hover:bg-white/10 w-8 h-8 rounded-lg"
        >
          <SettingsIcon class="w-4 h-4" />
        </Button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="relative z-10 flex-1 min-h-0 bg-transparent">
      <div v-if="activePage === 'main'" class="flex h-full gap-3 overflow-hidden rounded-[16px] p-3">
        <div class="min-w-0 w-[360px] shrink-0 rounded-[16px] bg-white/[0.05] dark:bg-black/[0.18] backdrop-blur-[32px] overflow-hidden relative">
          <div class="h-full rounded-[16px] border border-black/[0.06] bg-white dark:border-white/18 dark:bg-black/[0.64]">
            <TodoList v-if="activeView === 'todo'" :selected-id="selectedTodoId" @edit="handleEditTodo" />
            <NoteList v-else :selected-id="selectedNoteId" @edit="handleEditNote" />
          </div>

          <div class="absolute bottom-5 left-3 right-3 z-30 pointer-events-none">
            <div class="pointer-events-auto flex items-center justify-center gap-2">
              <button
                type="button"
                class="inline-flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-black/[0.06] bg-white text-black shadow-[0_10px_24px_rgba(15,23,42,0.12)] transition-colors hover:bg-white/95 dark:border-white/16 dark:bg-black/[0.78] dark:text-white dark:hover:bg-black/[0.88] dark:shadow-[0_10px_24px_rgba(0,0,0,0.32)]"
                @click.stop="handleAddNew"
                :title="activeView === 'todo' ? $t('todo.newTodo') : $t('note.newNote')"
              >
                <Plus :size="24" :stroke-width="2.6" />
              </button>
              <LiquidGlassTabs 
                v-model="activeView" 
                :options="[
                  { value: 'todo', label: 'Todo' },
                  { value: 'note', label: $t('settings.noteList') }
                ]"
              />
            </div>
          </div>
        </div>

        <div class="min-w-0 flex-1 rounded-[16px] bg-white/[0.05] dark:bg-black/[0.18] backdrop-blur-[32px] overflow-hidden">
          <div class="h-full min-h-0 rounded-[16px] border border-black/[0.06] bg-white dark:border-white/18 dark:bg-black/[0.64] p-4 pt-16 relative">
            <TodoForm
              v-if="isTodoEditorVisible"
              :todo="editingTodo"
              @save="handleSaveTodo"
              @cancel="closeEditor"
            />

            <NoteForm
              v-else-if="isNoteEditorVisible"
              :note="editingNote"
              @save="handleSaveNote"
              @cancel="closeEditor"
            />

            <div v-else class="flex h-full flex-col items-center justify-center px-6">
              <div class="relative mb-8 h-[210px] w-full max-w-[360px]">
                <div class="absolute inset-x-0 top-8 mx-auto h-[148px] w-[240px] rounded-[24px] border border-black/[0.05] bg-white/82 dark:border-white/12 dark:bg-black/[0.72]">
                  <div class="absolute left-[22px] top-[24px] bottom-[24px] w-px bg-black/[0.06] dark:bg-white/10"></div>
                  <div class="absolute left-[42px] right-[22px] top-[40px] h-px bg-black/[0.06] dark:bg-white/10"></div>
                  <div class="absolute left-[42px] right-[22px] top-[68px] h-px bg-black/[0.06] dark:bg-white/10"></div>
                  <div class="absolute left-[42px] right-[22px] top-[96px] h-px bg-black/[0.06] dark:bg-white/10"></div>
                </div>

                <div class="absolute left-1/2 top-[58px] h-[14px] w-[150px] -translate-x-[6%] rotate-[26deg]">
                  <div class="absolute right-[16px] top-1/2 h-[8px] w-[102px] -translate-y-1/2 rounded-full bg-black/70 dark:bg-white/70"></div>
                  <div class="absolute right-[4px] top-1/2 h-[6px] w-[18px] -translate-y-1/2 rounded-r-full bg-black/35 dark:bg-white/35"></div>
                  <div class="absolute left-[12px] top-1/2 h-0 w-0 -translate-y-1/2 border-y-[5px] border-y-transparent border-r-[16px] border-r-[#e4d1b8] dark:border-r-[#cfcfcf]"></div>
                  <div class="absolute left-[3px] top-1/2 h-0 w-0 -translate-y-1/2 border-y-[2px] border-y-transparent border-r-[8px] border-r-black/80 dark:border-r-white/80"></div>
                </div>

                <div class="absolute left-1/2 top-[118px] h-px w-[72px] -translate-x-[8%] rotate-[8deg] bg-black/12 dark:bg-white/16"></div>
              </div>

              <blockquote class="max-w-[340px] border-l border-black/10 pl-4 text-left text-[20px] font-semibold leading-[1.65] text-black/35 dark:border-white/22 dark:text-zinc-300">
                你一定有很多想写的吧
              </blockquote>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="h-full">
        <Settings @close="closeSettingsPage" />
      </div>
    </div>
    <div id="app-portal"></div>
  </Tabs>

  <Transition
    enter-active-class="transition-all duration-300 ease-out"
    enter-from-class="opacity-0 translate-y-3"
    enter-to-class="opacity-100 translate-y-0"
    leave-active-class="transition-all duration-250 ease-in"
    leave-from-class="opacity-100 translate-y-0"
    leave-to-class="opacity-0 translate-y-2"
  >
    <div
      v-if="autosaveToastVisible"
      class="fixed bottom-4 left-1/2 z-50 -translate-x-1/2 pointer-events-none"
    >
      <div class="inline-flex items-center gap-2 rounded-full border border-white/45 dark:border-white/16 bg-white/68 dark:bg-black/[0.76] px-3.5 py-1.5 text-[11px] font-medium text-foreground/80 backdrop-blur-2xl shadow-[0_12px_30px_rgba(15,23,42,0.12)] dark:shadow-[0_12px_30px_rgba(0,0,0,0.28)]">
        <span class="inline-flex h-4 w-4 items-center justify-center rounded-full bg-black/6 dark:bg-white/8">
          <Check class="h-3 w-3" />
        </span>
        {{ $t('common.autoSaved') }}
      </div>
    </div>
  </Transition>
</template>
