<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { currentMonitor, type Monitor } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'
import { Plus, ExternalLink, Clock, CheckCircle, StickyNote, Trash2, Copy, Check, X } from 'lucide-vue-next'
import Button from '@/components/ui/Button.vue'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import { useSettings } from '@/composables/useSettings'
import type { Todo, Note } from '@/types'

const appWindow = getCurrentWebviewWindow()

const todos = ref<Todo[]>([])
const notes = ref<Note[]>([])
const { settings, loadSettings } = useSettings()

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
  const estimatedHeight = 300 // Estimate average height
  
  if (top + estimatedHeight > winHeight) {
    top = Math.max(10, winHeight - estimatedHeight - 20)
  }

  const style: Record<string, string> = {
    top: `${top}px`,
  }
  
  const GAP = 10 // Reduced from 30
  
  if (tooltipSide.value === 'right') {
    style.left = `${mainRect.right + GAP}px`
    style.right = 'auto'
  } else {
    style.left = 'auto'
    style.right = `${window.innerWidth - mainRect.left + GAP}px`
  }
  
  return style
})

// 根据设置显示的内容类型
const displayMode = computed(() => {
  return settings.value.left_click_action === 'note' ? 'note' : 'todo'
})

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
      // 都没有提醒时间，按创建时间由近及远排序
      return new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
    })
})

// 获取所有便签，按更新时间由近及远排序
const sortedNotes = computed(() => {
  return notes.value
    .sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
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
  hoverTimeout = setTimeout(() => {
    hoveredItem.value = null
  }, 300) // Increased to 300ms for better usability
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
  loadSettings()
  updateMonitor()
  
  // 监听窗口获得焦点事件（显示时刷新数据和设置）
  appWindow.listen('tauri://focus', () => {
    loadSettings()
    loadData()
    updateMonitor()
  })
  
  // 点击窗口外部时隐藏
  appWindow.listen('tauri://blur', () => {
    appWindow.hide()
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

// 打开主窗口并添加 Todo
const addTodo = async () => {
  const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
  const mainWindow = await WebviewWindow.getByLabel('main')
  if (mainWindow) {
    await mainWindow.emit('tray-add-todo', {})
    await mainWindow.show()
    await mainWindow.setFocus()
  }
  appWindow.hide()
}

// 打开主窗口并添加便签
const addNote = async () => {
  const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
  const mainWindow = await WebviewWindow.getByLabel('main')
  if (mainWindow) {
    await mainWindow.emit('tray-add-note', {})
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
  <div class="popup-container relative h-screen w-screen flex flex-col p-4 box-border" @click="appWindow.hide()">
    <!-- 箭头指向状态栏图标 -->
    <div 
      class="arrow-up pointer-events-auto absolute top-[9px] left-1/2 w-4 h-4 -translate-x-1/2 rotate-45 z-50 rounded-tl-[4px] backdrop-blur-[24px] bg-gradient-to-br from-white/60 to-white/40 dark:from-black/60 dark:to-black/40"
      style="will-change: backdrop-filter; -webkit-backdrop-filter: blur(24px);"
    ></div>
    
    <!-- 主内容区域 - 液态玻璃效果 -->
    <div ref="mainContentRef" class="popup-content h-[420px] bg-white/60 dark:bg-black/60 backdrop-blur-3xl backdrop-saturate-150 text-foreground rounded-2xl overflow-hidden relative flex flex-col w-[340px] mx-auto" @click.stop style="will-change: backpack-filter; -webkit-backdrop-filter: blur(64px);">
      
      <!-- 顶部标题栏背景 (独立层，用于实现渐变高斯模糊) -->
      <div 
        class="absolute top-0 left-0 right-0 h-16 z-20 pointer-events-none"
        style="backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px); mask-image: linear-gradient(to bottom, black 0%, black 40%, transparent 100%); -webkit-mask-image: linear-gradient(to bottom, black 0%, black 40%, transparent 100%);"
      ></div>

      <!-- 顶部标题栏内容 (无背景，仅内容) -->
      <div class="absolute top-0 left-0 right-0 z-30 py-2.5 px-3 flex items-center justify-between pointer-events-none">
        <div class="w-8 pointer-events-auto"></div> <!-- 占位符 -->
        <h2 class="text-xs font-semibold tracking-wide opacity-90 text-foreground/90 pointer-events-auto">
          {{ displayMode === 'todo' ? '待办事项' : '便签' }}
        </h2>
        <Button 
          variant="ghost" 
          size="icon" 
          class="h-6 w-6 hover:bg-white/20 dark:hover:bg-black/20 rounded-full pointer-events-auto"
          @click="openMainWindow"
        >
          <ExternalLink class="w-3.5 h-3.5 opacity-70" />
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
              class="p-2.5 rounded-xl bg-white/40 dark:bg-black/40 hover:bg-white/50 dark:hover:bg-black/50 border border-white/40 dark:border-white/20 cursor-pointer transition-all duration-200 shadow hover:shadow-md group ring-1 ring-white/10 dark:ring-white/5"
              @click="handleTodoClick(todo)"
              @mouseenter="handleItemHover(todo, 'todo')"
              @mouseleave="handleItemLeave"
            >
              <div class="flex items-center gap-3">
                <div 
                  class="w-4 h-4 rounded ml-0.5 border-2 border-primary/40 group-hover:border-primary/60 transition-colors flex-shrink-0 cursor-pointer flex items-center justify-center hover:bg-primary/10 pointer-events-auto z-10"
                  @click.stop="toggleComplete(todo)"
                >
                  <Check v-if="todo.completed" class="w-2.5 h-2.5 text-primary" />
                </div>
                <div class="flex-1 min-w-0">
                  <div class="text-sm font-medium leading-none truncate">{{ todo.title }}</div>
                  <div v-if="todo.content" class="text-xs text-foreground/50 line-clamp-2 mt-1.5 leading-relaxed">{{ todo.content }}</div>
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
            <div class="w-12 h-12 rounded-full bg-black/5 dark:bg-white/5 flex items-center justify-center">
              <CheckCircle class="w-6 h-6 opacity-50" />
            </div>
            <span class="text-xs">暂无待办事项</span>
          </div>
        </template>

        <!-- 便签列表 -->
        <template v-else>
          <div v-if="sortedNotes.length > 0" class="space-y-2">
            <div 
              v-for="note in sortedNotes" 
              :key="note.id"
              class="p-2.5 rounded-xl bg-white/40 dark:bg-black/40 hover:bg-white/50 dark:hover:bg-black/50 border border-white/40 dark:border-white/20 cursor-pointer transition-all duration-200 shadow hover:shadow-md ring-1 ring-white/10 dark:ring-white/5 group"
              @click="handleNoteClick(note)"
              @mouseenter="handleItemHover(note, 'note')"
              @mouseleave="handleItemLeave"
            >
              <div class="flex gap-3">
                <div class="flex-1 min-w-0">
                  <div class="text-sm font-medium leading-relaxed line-clamp-3 text-foreground/90">{{ note.content }}</div>
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
            <div class="w-12 h-12 rounded-full bg-black/5 dark:bg-white/5 flex items-center justify-center">
              <StickyNote class="w-6 h-6 opacity-50" />
            </div>
            <span class="text-xs">暂无便签</span>
          </div>
        </template>
      </div>

    
      <!-- 内容详情悬浮窗 (Moved outside popup-content) -->
      
      <!-- 底部新建按钮 (悬浮 - 移除背景遮罩) -->
      <div class="absolute bottom-2 left-3 right-3 z-30 pointer-events-none">
        <div class="w-full pointer-events-auto">
          <Button 
            class="w-full h-10 bg-white/20 dark:bg-black/20 hover:bg-white/30 dark:hover:bg-black/30 text-foreground/90 rounded-full shadow-lg backdrop-blur-xl border border-white/50 dark:border-white/30 transition-all duration-300 hover:scale-[1.01] active:scale-[0.99] font-medium text-xs ring-1 ring-white/20 dark:ring-white/10"
            @click="displayMode === 'todo' ? addTodo() : addNote()"
          >
            <div class="flex items-center justify-center gap-2">
              <Plus class="w-4 h-4 opacity-90" />
              <span class="tracking-wide opacity-100 font-semibold">{{ displayMode === 'todo' ? '新建待办' : '新建便签' }}</span>
            </div>
          </Button>
        </div>
      </div>
      
      <!-- 确认删除对话框 -->
      <ConfirmDialog
        :open="confirmOpen"
        @update:open="confirmOpen = $event"
        :title="itemToDelete?.type === 'todo' ? '删除待办' : '删除便签'"
        :description="itemToDelete?.type === 'todo' ? '确定要删除这条待办事项吗？此操作无法撤销。' : '确定要删除这条便签吗？此操作无法撤销。'"
        confirm-text="删除"
        cancel-text="取消"
        variant="destructive"
        compact
        @confirm="handleConfirmDelete"
      />
    </div>

    <!-- 内容详情悬浮窗 (Now outside popup-content to avoid clipping) -->
    <div 
      v-if="hoveredItem" 
      class="absolute z-50 pointer-events-none transition-all duration-300 ease-out"
      :class="{ 
        'translate-x-[-5px]': tooltipSide === 'left', 
        'translate-x-[5px]': tooltipSide === 'right' 
      }"
      :style="tooltipStyle"
      @mouseenter="handleTooltipEnter"
      @mouseleave="handleTooltipLeave"
    >
      <div 
        class="relative flex flex-col bg-white/70 dark:bg-black/70 backdrop-blur-2xl backdrop-saturate-150 rounded-2xl w-[360px] max-h-[400px] overflow-hidden transform-gpu pointer-events-auto"
        style="will-change: backdrop-filter; -webkit-backdrop-filter: blur(40px);"
        @click.stop
      >
        <!-- Title Header -->
        <div class="px-4 py-3 border-b border-black/5 dark:border-white/5 bg-white/30 dark:bg-white/5 flex items-center justify-between">
           <div class="font-semibold text-sm leading-snug text-foreground/90 truncate flex-1 min-w-0 mr-2">
             {{ hoveredItem.type === 'todo' ? (hoveredItem.item as Todo).title : '便签详情' }}
           </div>
           
           <!-- Copy Button -->
           <button 
             class="flex-shrink-0 w-6 h-6 flex items-center justify-center rounded-md hover:bg-black/5 dark:hover:bg-white/10 text-foreground/50 hover:text-foreground transition-all duration-200"
             @click.stop="handleCopy"
             :title="copied ? '已复制' : (copyError ? '复制失败' : '复制内容')"
           >
             <Check v-if="copied" class="w-3.5 h-3.5 text-green-500" />
             <X v-else-if="copyError" class="w-3.5 h-3.5 text-red-500" />
             <Copy v-else class="w-3.5 h-3.5" />
           </button>
        </div>

        <div class="p-4 overflow-y-auto custom-scrollbar">
          <div class="text-[13px] leading-6 text-foreground/80 whitespace-pre-wrap break-words font-normal">
            {{ hoveredItem.type === 'todo' ? ((hoveredItem.item as Todo).content || '无详情') : (hoveredItem.item as Note).content }}
          </div>
          
          <div class="mt-4 pt-3 border-t border-black/5 dark:border-white/5 flex items-center justify-between group/meta">
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
  </div>
</template>

<style scoped>

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

:global(.dark) .custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}

:global(.dark) .custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
