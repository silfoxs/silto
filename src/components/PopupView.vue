<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { currentMonitor, type Monitor } from '@tauri-apps/api/window'
import { emit, listen } from '@tauri-apps/api/event'
import { ExternalLink, Clock, CheckCircle, StickyNote, Trash2, Copy, Check, X } from 'lucide-vue-next'
import Button from '@/components/ui/Button.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import type { Todo, Note, Settings, Theme } from '@/types'

const appWindow = getCurrentWebviewWindow()
const { t, locale } = useI18n()

const todos = ref<Todo[]>([])
const notes = ref<Note[]>([])
const popupTheme = ref<'light' | 'dark'>('light')

const monitor = ref<Monitor | null>(null)
const updateMonitor = async () => {
  try {
    monitor.value = await currentMonitor()
  } catch (e) {
    console.error('Failed to get monitor info:', e)
  }
}

const mainContentRef = ref<HTMLElement | null>(null)

// Tooltip state
const hoveredItem = ref<{ item: Todo | Note, type: 'todo' | 'note', mainRect: DOMRect } | null>(null)
const tooltipSide = ref<'left' | 'right'>('right')

const tooltipStyle = computed(() => {
  if (!hoveredItem.value) return {}
  
  const { mainRect } = hoveredItem.value
  
  // Calculate vertical position (prevent overflow)
  let top = mainRect.top
  const winHeight = window.innerHeight
  const estimatedHeight = 420
  
  if (top + estimatedHeight > winHeight) {
    top = Math.max(10, winHeight - estimatedHeight - 20)
  }

  const style: Record<string, string> = {
    top: `${top}px`,
  }
  
  const GAP = 6 // Reduced from 10
  
  if (tooltipSide.value === 'right') {
    style.left = `${mainRect.right + GAP}px`
    style.right = 'auto'
  } else {
    style.left = 'auto'
    style.right = `${window.innerWidth - mainRect.left + GAP}px`
  }
  
  return style
})

// 根据主界面显示的内容类型（从 localStorage 读取）
const displayMode = ref<'todo' | 'note'>('todo')

const loadDisplayMode = () => {
  const savedView = localStorage.getItem('activeView')
  if (savedView === 'todo' || savedView === 'note') {
    displayMode.value = savedView
  }
}

const resolveEffectiveTheme = (theme: Theme): 'light' | 'dark' => {
  if (theme === 'system') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }
  return theme
}

const applyPopupTheme = (theme: 'light' | 'dark') => {
  popupTheme.value = theme
  document.documentElement.classList.toggle('dark', theme === 'dark')
}

const loadPopupTheme = async () => {
  try {
    const settings = await invoke<Settings>('get_settings')
    applyPopupTheme(resolveEffectiveTheme(settings.theme))
  } catch (error) {
    console.error('Failed to load popup theme:', error)
  }
}

// 获取所有未完成的 Todo，按时间排序（有提醒时间的优先，然后按创建时间由近及远）
const sortedTodos = computed(() => {
  return todos.value
    .filter(t => !t.completed)
    .sort((a, b) => {
      // 如果都有提醒时间，按提醒时间由近及远排序
      if (a.remind_time && b.remind_time) {
        return new Date(a.remind_time).getTime() - new Date(b.remind_time).getTime()
      }
      // 有提醒时间的排在前面
      if (a.remind_time) return -1
      if (b.remind_time) return 1
      // 都没有提醒时间，按创建时间由近及远排序 (Newer first)
      return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    })
})

// 获取所有便签，按更新时间由近及远排序
const sortedNotes = computed(() => {
  return notes.value
    .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
})

// 加载数据
const loadData = async () => {
  try {
    todos.value = await invoke<Todo[]>('get_todos')
    notes.value = await invoke<Note[]>('get_notes')
  } catch (error) {
    console.error('Failed to load data:', error)
  }
}

// 删除确认状态
const confirmOpen = ref(false)
const itemToDelete = ref<{ id: number, type: 'todo' | 'note' } | null>(null)

const confirmDelete = (id: number, type: 'todo' | 'note') => {
  itemToDelete.value = { id, type }
  confirmOpen.value = true
}

const handleConfirmDelete = async () => {
  if (!itemToDelete.value) return
  
  try {
    if (itemToDelete.value.type === 'todo') {
      await invoke('delete_todo', { id: itemToDelete.value.id })
    } else {
      await invoke('delete_note', { id: itemToDelete.value.id })
    }
    await loadData()
    await emit('refresh-data')
    itemToDelete.value = null
  } catch (error) {
    console.error('Failed to delete item:', error)
  }
}

// Hover handlers for tooltip
let hoverTimeout: ReturnType<typeof setTimeout> | undefined

const handleItemHover = (item: Todo | Note, type: 'todo' | 'note') => {
  if (hoverTimeout) {
    clearTimeout(hoverTimeout)
    hoverTimeout = undefined
  }
  
  const mainRect = mainContentRef.value?.getBoundingClientRect()
  if (!mainRect) return
  
  // Default to right
  let side: 'left' | 'right' = 'right'

  if (monitor.value) {
    const scaleFactor = monitor.value.scaleFactor
    const screenX = monitor.value.position.x / scaleFactor
    const screenWidth = monitor.value.size.width / scaleFactor
    const screenRight = screenX + screenWidth
    
    // Window global (logical) X. 
    const winX = window.screenX
    const tooltipWidth = 360 // Wider width
    
    // Check if showing on right would overflow screen
    // Gap 10 + Width
    const GAP = 10
    const rightEdge = winX + mainRect.right + GAP + tooltipWidth
    
    if (rightEdge > screenRight) {
      side = 'left'
    } else {
      side = 'right'
    }
  } else {
    // Fallback if monitor info missing
    const winWidth = window.innerWidth
    const centerX = winWidth / 2
    side = mainRect.left < centerX ? 'right' : 'left'
  }
  
  tooltipSide.value = side
  hoveredItem.value = { item, type, mainRect }
}

const handleItemLeave = () => {
    // hoverTimeout = setTimeout(() => {
    //   hoveredItem.value = null
    // }, 300) // Keep commented out or remove to disable auto-hide
}

const handleTooltipEnter = () => {
  if (hoverTimeout) {
    clearTimeout(hoverTimeout)
    hoverTimeout = undefined
  }
}

const handleTooltipLeave = () => {
  handleItemLeave()
}

// Copy functionality
const copied = ref(false)
const copyError = ref(false)
const copyTimeout = ref<ReturnType<typeof setTimeout> | undefined>(undefined)

const handleCopy = async () => {
  if (!hoveredItem.value) return
  
  const content = hoveredItem.value.type === 'todo' 
    ? ((hoveredItem.value.item as Todo).content || '') 
    : (hoveredItem.value.item as Note).content
    
  if (!content) return

  try {
    const { writeText } = await import('@tauri-apps/plugin-clipboard-manager')
    await writeText(content)
    copied.value = true
    
    if (copyTimeout.value) clearTimeout(copyTimeout.value)
    
    copyTimeout.value = setTimeout(() => {
      copied.value = false
    }, 2000)
  } catch (err) {
    console.error('Failed to copy via plugin:', err)
    alert('Copy Failed: ' + err) // Debugging
    copyError.value = true
    if (copyTimeout.value) clearTimeout(copyTimeout.value)
    copyTimeout.value = setTimeout(() => {
      copyError.value = false
    }, 2000)
  }
}

onMounted(() => {
  loadData()
  loadDisplayMode()
  updateMonitor()
  loadPopupTheme()

  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
  mediaQuery.onchange = () => {
    loadPopupTheme()
  }

  listen<Settings>('settings-changed', (event) => {
    applyPopupTheme(resolveEffectiveTheme(event.payload.theme))
  })
  
  // 监听窗口获得焦点事件（显示时刷新数据和设置）
  appWindow.listen('tauri://focus', () => {
    loadDisplayMode()
    loadData()
    updateMonitor()
    loadPopupTheme()
    const saved = localStorage.getItem('language')
    if (saved) {
      locale.value = saved
    }
  })
  
  // 点击窗口外部时隐藏
  appWindow.listen('tauri://blur', () => {
    appWindow.hide()
    hoveredItem.value = null // Reset tooltip
  })
})

// 打开主窗口
const openMainWindow = async () => {
  const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
  const mainWindow = await WebviewWindow.getByLabel('main')
  if (mainWindow) {
    await mainWindow.show()
    await mainWindow.setFocus()
  }
  appWindow.hide()
}

// 点击 Todo 项
const handleTodoClick = async (todo: Todo) => {
  const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
  const mainWindow = await WebviewWindow.getByLabel('main')
  if (mainWindow) {
    await mainWindow.emit('edit-todo', todo)
    await mainWindow.show()
    await mainWindow.setFocus()
  }
  appWindow.hide()
}

// 点击便签项
const handleNoteClick = async (note: Note) => {
  const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
  const mainWindow = await WebviewWindow.getByLabel('main')
  if (mainWindow) {
    await mainWindow.emit('edit-note', note)
    await mainWindow.show()
    await mainWindow.setFocus()
  }
  appWindow.hide()
}

// 切换 Todo 完成状态
const toggleComplete = async (todo: Todo) => {
  try {
    const updatedTodo = { ...todo, completed: !todo.completed }
    await invoke('save_todo', { todo: updatedTodo })
    await loadData()
    await emit('refresh-data')
    
    // 如果是完成操作，显示一个小提示或者只是简单的让它消失（通过 filter）
    // 目前列表会自动过滤掉已完成的
  } catch (error) {
    console.error('Failed to toggle todo status:', error)
  }
}
</script>

<template>
  <div :class="['popup-container', popupTheme === 'dark' ? 'dark' : '', 'relative h-screen w-screen flex flex-col p-4 box-border']" @click="appWindow.hide()">
    <!-- 箭头指向状态栏图标 -->
    <div 
      class="arrow-up popup-glass-arrow pointer-events-auto absolute top-[9px] left-1/2 w-4 h-4 -translate-x-1/2 rotate-45 z-50 rounded-tl-[4px]"
    ></div>
    
    <!-- 主内容区域 - 液态玻璃效果 -->
    <div ref="mainContentRef" class="popup-content popup-glass-panel h-[420px] text-foreground rounded-2xl overflow-hidden relative flex flex-col w-[340px] mx-auto" @click.stop>
      
      <!-- 顶部标题栏背景 (独立层，用于实现渐变高斯模糊) -->
      <div 
        class="absolute top-0 left-0 right-0 h-16 z-20 pointer-events-none"
        style="backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px); mask-image: linear-gradient(to bottom, black 0%, black 40%, transparent 100%); -webkit-mask-image: linear-gradient(to bottom, black 0%, black 40%, transparent 100%);"
      ></div>

      <!-- 顶部标题栏内容 (无背景，仅内容) -->
      <div class="absolute top-0 left-0 right-0 z-30 py-2.5 px-3 flex items-center justify-between pointer-events-none">
        <div class="w-8 pointer-events-auto"></div> <!-- 占位符 -->
        <h2 class="text-xs font-semibold tracking-wide opacity-90 text-foreground/90 pointer-events-auto">
          {{ displayMode === 'todo' ? t('settings.todoList') : t('settings.noteList') }}
        </h2>
        <Button 
          variant="ghost"
          class="h-7 px-2.5 gap-1.5 hover:bg-white/20 dark:hover:bg-white/[0.06] rounded-full pointer-events-auto text-[11px] font-medium text-foreground/80"
          @click="openMainWindow"
          :title="t('popup.openMainWindow')"
        >
          <ExternalLink class="w-3.5 h-3.5 opacity-70" />
          <span>{{ t('popup.openMainWindowShort') }}</span>
        </Button>
      </div>

      <!-- 内容列表区域（可滚动，充满容器） -->
      <div class="flex-1 overflow-y-auto custom-scrollbar px-1.5 pt-11 pb-14 w-full" style="mask-image: linear-gradient(to bottom, black 0%, black calc(100% - 16px), transparent 100%); -webkit-mask-image: linear-gradient(to bottom, black 0%, black calc(100% - 16px), transparent 100%);">
        <!-- Todo 列表 -->
        <template v-if="displayMode === 'todo'">
          <div v-if="sortedTodos.length > 0" class="space-y-2">
            <div 
              v-for="todo in sortedTodos" 
              :key="todo.id"
              class="popup-item-card p-2.5 rounded-xl cursor-pointer transition-all duration-200 shadow hover:shadow-md group border border-black/10 dark:border-white/28"
              @click="handleTodoClick(todo)"
              @mouseenter="handleItemHover(todo, 'todo')"
            >
              <div class="flex items-center gap-3">
                <div 
                  class="w-4 h-4 rounded ml-0.5 border-2 border-primary/40 group-hover:border-primary/60 transition-colors flex-shrink-0 cursor-pointer flex items-center justify-center hover:bg-primary/10 pointer-events-auto z-10"
                  @click.stop="toggleComplete(todo)"
                >
                  <Check v-if="todo.completed" class="w-2.5 h-2.5 text-primary" />
                </div>
                <div class="flex-1 min-w-0 overflow-hidden">
                  <div class="text-sm font-medium leading-none truncate">{{ todo.title }}</div>
                  <div v-if="todo.content" class="popup-rich-preview text-xs text-foreground/50 mt-1.5 leading-relaxed" v-html="todo.content"></div>
                  <div v-if="todo.remind_time" class="text-[10px] text-primary/70 mt-1.5 flex items-center">
                    <Clock class="w-3 h-3 mr-1" />
                    {{ new Date(todo.remind_time).toLocaleString() }}
                  </div>
                </div>
                <!-- 删除按钮 -->
                <button 
                  class="p-1.5 rounded-md hover:bg-red-500/10 text-foreground/40 hover:text-red-500 opacity-0 group-hover:opacity-100 transition-all duration-200 cursor-pointer pointer-events-auto relative z-10"
                  @click.stop="confirmDelete(Number(todo.id), 'todo')"
                >
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
          <div v-else class="h-full flex flex-col items-center justify-center text-foreground/40 space-y-2">
            <div class="popup-empty-badge w-12 h-12 rounded-full flex items-center justify-center">
              <CheckCircle class="w-6 h-6 opacity-50" />
            </div>
            <span class="text-xs">{{ t('popup.noTodos') }}</span>
          </div>
        </template>

        <!-- 便签列表 -->
        <template v-else>
          <div v-if="sortedNotes.length > 0" class="space-y-2">
            <div 
              v-for="note in sortedNotes" 
              :key="note.id"
              class="popup-item-card p-2.5 rounded-xl cursor-pointer transition-all duration-200 shadow hover:shadow-md group border border-black/10 dark:border-white/28"
              @click="handleNoteClick(note)"
              @mouseenter="handleItemHover(note, 'note')"
            >
              <div class="flex gap-3">
                <div class="flex-1 min-w-0 overflow-hidden">
                  <div class="text-sm font-medium leading-none truncate">{{ note.title || '无标题' }}</div>
                  <div class="popup-rich-preview text-xs text-foreground/50 mt-1.5 leading-relaxed" v-html="note.content"></div>
                  <div class="text-[10px] text-foreground/30 mt-2 text-right">
                    {{ new Date(note.created_at).toLocaleDateString() }}
                  </div>
                </div>
                <!-- 删除按钮 -->
                <button 
                  class="p-1.5 rounded-md hover:bg-red-500/10 text-foreground/40 hover:text-red-500 opacity-0 group-hover:opacity-100 transition-all duration-200 h-fit self-start cursor-pointer pointer-events-auto relative z-10"
                  @click.stop="confirmDelete(Number(note.id), 'note')"
                >
                  <Trash2 class="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>
          <div v-else class="h-full flex flex-col items-center justify-center text-foreground/40 space-y-2">
            <div class="popup-empty-badge w-12 h-12 rounded-full flex items-center justify-center">
              <StickyNote class="w-6 h-6 opacity-50" />
            </div>
            <span class="text-xs">{{ t('popup.noNotes') }}</span>
          </div>
        </template>
      </div>

    
      <!-- 内容详情悬浮窗 (Moved outside popup-content) -->
      
      <!-- 确认删除对话框 -->
      <ConfirmDialog
        :open="confirmOpen"
        @update:open="confirmOpen = $event"
        :title="itemToDelete?.type === 'todo' ? `${t('common.delete')} ${t('settings.todoList')}` : `${t('common.delete')} ${t('settings.noteList')}`"
        :description="itemToDelete?.type === 'todo' ? t('todo.deleteConfirm') : t('note.deleteConfirm')"
        :confirm-text="t('common.delete')"
        :cancel-text="t('common.cancel')"
        variant="destructive"
        compact
        @confirm="handleConfirmDelete"
      />
    </div>

    <!-- 内容详情悬浮窗 (Now outside popup-content to avoid clipping) -->
    <div 
      v-if="hoveredItem" 
      class="absolute z-50 pointer-events-none transition-all duration-300 ease-out"
      :style="tooltipStyle"
      @mouseenter="handleTooltipEnter"
      @mouseleave="handleTooltipLeave"
    >
      <div 
        class="popup-tooltip-panel popup-detail-shell relative flex flex-col rounded-2xl w-[360px] h-[420px] overflow-hidden transform-gpu pointer-events-auto"
        @click.stop
      >
        <!-- Title Header -->
        <div class="popup-detail-header px-4 py-3 border-b flex items-center justify-between">
           <div class="font-semibold text-sm leading-snug text-foreground/90 truncate flex-1 min-w-0 mr-2">
             {{ hoveredItem.type === 'todo' ? (hoveredItem.item as Todo).title : ((hoveredItem.item as Note).title || t('popup.noteDetails')) }}
           </div>
           
           <!-- Copy Button -->
           <button 
             class="popup-detail-action flex-shrink-0 w-6 h-6 flex items-center justify-center rounded-md text-foreground/50 hover:text-foreground transition-all duration-200"
             @click.stop="handleCopy"
             :title="copied ? t('popup.copied') : (copyError ? t('popup.copyFailed') : t('popup.copyContent'))"
           >
             <Check v-if="copied" class="w-3.5 h-3.5 text-green-500" />
             <X v-else-if="copyError" class="w-3.5 h-3.5 text-red-500" />
             <Copy v-else class="w-3.5 h-3.5" />
           </button>
        </div>

        <div class="flex-1 min-h-0 overflow-y-auto custom-scrollbar px-4 py-4">
          <div
            v-if="hoveredItem.type === 'todo' ? !!((hoveredItem.item as Todo).content) : !!((hoveredItem.item as Note).content)"
            class="popup-rich-content text-[13px] text-foreground/80 font-normal"
            v-html="hoveredItem.type === 'todo' ? (((hoveredItem.item as Todo).content || '')) : ((hoveredItem.item as Note).content)"
          ></div>
          <div v-else class="text-[13px] leading-6 text-foreground/60 font-normal">
            {{ t('popup.noDetails') }}
          </div>
        </div>

        <div class="popup-detail-footer px-4 py-3 border-t flex items-center justify-between group/meta">
           <div class="flex items-center text-[11px] text-foreground/40 font-medium">
              <Clock class="w-3.5 h-3.5 mr-1.5 opacity-70" />
              {{ new Date(hoveredItem.type === 'todo' ? (hoveredItem.item as Todo).created_at : (hoveredItem.item as Note).updated_at).toLocaleString() }}
           </div>
           <div v-if="hoveredItem.type === 'todo' && (hoveredItem.item as Todo).remind_time" class="flex items-center text-[11px] text-primary/80 font-medium bg-primary/5 px-2 py-0.5 rounded-full">
              <Clock class="w-3 h-3 mr-1" />
              {{ new Date((hoveredItem.item as Todo).remind_time!).toLocaleTimeString() }}
           </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.popup-container {
  isolation: isolate;
  --popup-panel-bg: rgba(255, 255, 255, 0.78);
  --popup-panel-border: rgba(255, 255, 255, 0.5);
  --popup-panel-highlight: rgba(255, 255, 255, 0.35);
  --popup-arrow-from: rgba(255, 255, 255, 0.82);
  --popup-arrow-to: rgba(255, 255, 255, 0.64);
  --popup-item-bg: rgba(255, 255, 255, 0.4);
  --popup-item-hover-bg: rgba(255, 255, 255, 0.5);
  --popup-item-border: rgba(255, 255, 255, 0.52);
  --popup-item-ring: rgba(255, 255, 255, 0.16);
  --popup-detail-strip-bg: rgba(255, 255, 255, 0.3);
  --popup-detail-strip-border: rgba(0, 0, 0, 0.05);
  --popup-detail-action-hover: rgba(0, 0, 0, 0.05);
  --popup-empty-bg: rgba(0, 0, 0, 0.05);
}

.popup-glass-panel {
  background: var(--popup-panel-bg);
  border: 1px solid var(--popup-panel-border);
  box-shadow:
    0 20px 50px rgba(15, 23, 42, 0.16),
    inset 0 1px 0 var(--popup-panel-highlight);
  backdrop-filter: blur(28px) saturate(140%);
  -webkit-backdrop-filter: blur(28px) saturate(140%);
  transform: translateZ(0);
}

.popup-glass-arrow {
  background: linear-gradient(to bottom right, var(--popup-arrow-from), var(--popup-arrow-to));
  border-top: 1px solid var(--popup-panel-border);
  border-left: 1px solid var(--popup-panel-border);
  box-shadow: -6px -6px 20px rgba(15, 23, 42, 0.06);
  backdrop-filter: blur(18px) saturate(135%);
  -webkit-backdrop-filter: blur(18px) saturate(135%);
}

.popup-tooltip-panel {
  background: var(--popup-panel-bg);
  border: 1px solid var(--popup-panel-border);
  box-shadow:
    0 18px 42px rgba(15, 23, 42, 0.14),
    inset 0 1px 0 var(--popup-panel-highlight);
  backdrop-filter: blur(24px) saturate(140%);
  -webkit-backdrop-filter: blur(24px) saturate(140%);
  transform: translateZ(0);
}

.popup-item-card {
  background: var(--popup-item-bg);
  box-shadow:
    0 0 0 1px var(--popup-item-ring) inset,
    0 8px 18px rgba(15, 23, 42, 0.08);
}

.popup-item-card:hover {
  background: var(--popup-item-hover-bg);
}

.popup-detail-header,
.popup-detail-footer {
  border-color: var(--popup-detail-strip-border);
  background: var(--popup-detail-strip-bg);
}

.popup-detail-action:hover {
  background: var(--popup-detail-action-hover);
}

.popup-empty-badge {
  background: var(--popup-empty-bg);
}

.popup-container.dark {
  --popup-panel-bg: rgba(0, 0, 0, 0.88);
  --popup-panel-border: rgba(255, 255, 255, 0.16);
  --popup-panel-highlight: rgba(255, 255, 255, 0.05);
  --popup-arrow-from: rgba(8, 8, 8, 0.92);
  --popup-arrow-to: rgba(0, 0, 0, 0.82);
  --popup-item-bg: rgba(0, 0, 0, 0.82);
  --popup-item-hover-bg: rgba(0, 0, 0, 0.92);
  --popup-item-border: rgba(255, 255, 255, 0.22);
  --popup-item-ring: rgba(255, 255, 255, 0.12);
  --popup-detail-strip-bg: rgba(0, 0, 0, 0.90);
  --popup-detail-strip-border: rgba(255, 255, 255, 0.14);
  --popup-detail-action-hover: rgba(255, 255, 255, 0.08);
  --popup-empty-bg: rgba(255, 255, 255, 0.06);
}

.popup-container.dark .popup-item-card {
  box-shadow:
    0 0 0 1px rgba(255, 255, 255, 0.12) inset,
    0 10px 22px rgba(0, 0, 0, 0.28);
}

.popup-rich-preview {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  overflow: hidden;
}

.popup-rich-preview :deep(p),
.popup-rich-preview :deep(ul),
.popup-rich-preview :deep(ol),
.popup-rich-preview :deep(blockquote),
.popup-rich-preview :deep(pre),
.popup-rich-preview :deep(h1),
.popup-rich-preview :deep(h2),
.popup-rich-preview :deep(h3) {
  margin: 0;
}

.popup-rich-preview :deep(ul),
.popup-rich-preview :deep(ol) {
  padding-left: 1rem;
}

.popup-rich-content {
  line-height: 1.65;
  word-break: break-word;
}

.popup-rich-content :deep(strong) {
  font-weight: 700;
}

.popup-rich-content :deep(em) {
  font-style: italic;
}

.popup-rich-content :deep(p),
.popup-rich-content :deep(ul),
.popup-rich-content :deep(ol),
.popup-rich-content :deep(blockquote),
.popup-rich-content :deep(pre),
.popup-rich-content :deep(h1),
.popup-rich-content :deep(h2),
.popup-rich-content :deep(h3) {
  margin-top: 0;
  margin-bottom: 0.75rem;
}

.popup-rich-content :deep(ul),
.popup-rich-content :deep(ol) {
  padding-left: 1.25rem;
}

.popup-rich-content :deep(ul) {
  list-style-type: disc;
}

.popup-rich-content :deep(ol) {
  list-style-type: decimal;
}

.popup-rich-content :deep(li) {
  margin: 0.3rem 0;
}

.popup-rich-content :deep(h1) {
  font-size: 1.2rem;
  font-weight: 700;
  line-height: 1.3;
}

.popup-rich-content :deep(h2) {
  font-size: 1.05rem;
  font-weight: 700;
  line-height: 1.35;
}

.popup-rich-content :deep(blockquote) {
  padding: 0.7rem 0.9rem;
  border-left: 3px solid rgba(15, 23, 42, 0.16);
  border-radius: 0 12px 12px 0;
  background: rgba(15, 23, 42, 0.04);
}

.popup-rich-content :deep(pre) {
  padding: 0.8rem 0.9rem;
  border: 1px solid rgba(15, 23, 42, 0.08);
  border-radius: 12px;
  background: rgba(15, 23, 42, 0.05);
  white-space: pre-wrap;
}

.popup-rich-content :deep(pre code) {
  background: transparent;
  padding: 0;
  font-size: 0.84rem;
}

.popup-rich-content :deep(code) {
  border-radius: 8px;
  background: rgba(15, 23, 42, 0.06);
  padding: 0.1rem 0.32rem;
  font-size: 0.84em;
}

.popup-container.dark .popup-rich-content :deep(blockquote) {
  border-left-color: rgba(255, 255, 255, 0.24);
  background: rgba(255, 255, 255, 0.05);
}

.popup-container.dark .popup-rich-content :deep(pre) {
  border-color: rgba(255, 255, 255, 0.14);
  background: rgba(255, 255, 255, 0.05);
}

.popup-container.dark .popup-rich-content :deep(code) {
  background: rgba(255, 255, 255, 0.08);
}

/* 隐藏滚动条但保留功能 */
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

/* 自定义滚动条 */
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}

.popup-container.dark .custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}

.popup-container.dark .custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
