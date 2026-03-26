<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useTodos } from '@/composables/useTodos'
import { Check, Clock, Trash2, Search } from 'lucide-vue-next'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import Input from '@/components/ui/Input.vue'
import type { Todo } from '@/types'
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()
const { activeTodos, completedTodos, loadTodos, toggleTodo, deleteTodo } = useTodos()

const props = defineProps<{
  selectedId?: string | null
}>()

const emit = defineEmits<{
  (e: 'edit', todo: Todo): void
}>()

onMounted(() => {
  loadTodos()
})

const handleToggle = async (todo: Todo) => {
  await toggleTodo(todo)
}

const showDeleteConfirm = ref(false)
const itemToDelete = ref<string | null>(null)

const confirmDelete = (id: string) => {
  itemToDelete.value = id
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  if (itemToDelete.value) {
    await deleteTodo(itemToDelete.value)
    itemToDelete.value = null
  }
}

const searchQuery = ref('')

const matchesQuery = (todo: Todo) => {
  const query = searchQuery.value.trim().toLowerCase()
  if (!query) return true

  const plainContent = (todo.content || '').replace(/<[^>]*>/g, ' ')
  return `${todo.title} ${plainContent}`.toLowerCase().includes(query)
}

const filteredActiveTodos = computed(() => activeTodos.value.filter(matchesQuery))
const filteredCompletedTodos = computed(() => completedTodos.value.filter(matchesQuery))

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleString(locale.value, { 
    month: '2-digit', 
    day: '2-digit', 
    hour: '2-digit', 
    minute: '2-digit' 
  })
}

const isOverdue = (todo: Todo) => {
  if (!todo.remind_time || todo.completed) return false
  return new Date(todo.remind_time).getTime() < Date.now()
}
</script>

<template>
  <div class="flex flex-col h-full overflow-hidden">
    <div class="flex-1 overflow-y-auto p-4 pt-16 pb-24 space-y-3">
      <div class="sticky top-0 z-10 pb-3">
        <div class="relative">
          <Search class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-foreground/35" />
          <Input
            v-model="searchQuery"
            class="h-10 rounded-[16px] border-black/[0.06] bg-white/80 pl-9 pr-3 shadow-none dark:border-white/18 dark:bg-black/[0.72]"
            :placeholder="$t('todo.searchPlaceholder')"
          />
        </div>
      </div>

      <!-- Active Todos -->
      <div v-if="filteredActiveTodos.length > 0" class="space-y-2">
        <div 
          v-for="todo in filteredActiveTodos" 
          :key="todo.id"
          class="group p-4 rounded-[16px] border transition-all duration-300 cursor-pointer animate-slide-in shadow-sm hover:shadow-md backdrop-blur-md"
          :class="[
            props.selectedId === todo.id
              ? 'border-black/30 bg-[#f3efe7] shadow-[0_16px_32px_rgba(15,23,42,0.10),inset_0_0_0_1px_rgba(15,23,42,0.06)] dark:border-white/40 dark:bg-white/[0.14] dark:shadow-[0_16px_32px_rgba(0,0,0,0.30),inset_0_0_0_1px_rgba(255,255,255,0.16)]'
              : isOverdue(todo) 
              ? 'border-red-500/55 bg-white/50 dark:bg-black/[0.66] shadow-[0_0_15px_rgba(239,68,68,0.16)] dark:shadow-[0_0_15px_rgba(239,68,68,0.28)]' 
              : 'border-black/[0.05] dark:border-white/22 bg-white/50 dark:bg-black/[0.66] hover:bg-white/80 dark:hover:bg-black/[0.78] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]'
          ]"
          @click="emit('edit', todo)"
        >
          <div class="flex items-center gap-3">
            <button
              class="flex-shrink-0 w-5 h-5 rounded border-2 border-muted-foreground/40 hover:bg-muted-foreground/10 transition-colors flex items-center justify-center"
              @click.stop="handleToggle(todo)"
            >
              <Check v-if="todo.completed" class="w-4 h-4 text-muted-foreground" />
            </button>
            <div class="flex-1 min-w-0 overflow-hidden">
              <h3 class="font-medium text-sm truncate">{{ todo.title }}</h3>
              <div
                v-if="todo.content"
                class="rich-preview mt-1 text-xs text-muted-foreground"
                v-html="todo.content"
              ></div>
              <div v-if="todo.remind_time" class="flex items-center gap-1 mt-2 text-xs text-primary">
                <Clock class="w-3 h-3" />
                <span>{{ formatDate(todo.remind_time) }}</span>
              </div>
            </div>
            <button
              class="opacity-0 group-hover:opacity-100 transition-all p-1 hover:bg-destructive hover:text-white dark:hover:bg-red-500 rounded-md"
              @click.stop="confirmDelete(todo.id)"
            >
              <Trash2 class="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      <!-- Completed Todos -->
      <div v-if="filteredCompletedTodos.length > 0" class="mt-4">
        <h3 class="text-xs text-muted-foreground mb-2">{{ $t('todo.completed') }}</h3>
        <div class="space-y-2">
          <div 
            v-for="todo in filteredCompletedTodos" 
            :key="todo.id"
            class="group p-4 rounded-[16px] border backdrop-blur-sm transition-all duration-300 cursor-pointer opacity-60 hover:opacity-100"
            :class="props.selectedId === todo.id
              ? 'border-black/30 bg-[#f0ebe2] shadow-[0_14px_30px_rgba(15,23,42,0.08),inset_0_0_0_1px_rgba(15,23,42,0.05)] dark:border-white/36 dark:bg-white/[0.12] dark:shadow-[0_14px_30px_rgba(0,0,0,0.26),inset_0_0_0_1px_rgba(255,255,255,0.14)]'
              : 'border-black/[0.03] dark:border-white/18 bg-white/30 dark:bg-black/[0.60] hover:bg-white/50 dark:hover:bg-black/[0.72] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.06)]'"
            @click="emit('edit', todo)"
          >
            <div class="flex items-center gap-3">
              <button
                class="flex-shrink-0 w-5 h-5 rounded border-2 bg-muted-foreground/40 border-muted-foreground/40 hover:bg-muted-foreground/50 transition-colors flex items-center justify-center"
                @click.stop="handleToggle(todo)"
              >
                <Check class="w-4 h-4 text-white dark:text-gray-900" />
              </button>
              <div class="flex-1 min-w-0 overflow-hidden">
                <h3 class="font-medium text-sm truncate line-through">{{ todo.title }}</h3>
                <div
                  v-if="todo.content"
                  class="rich-preview mt-1 text-xs text-muted-foreground line-through"
                  v-html="todo.content"
                ></div>
              </div>
              <button
                class="opacity-0 group-hover:opacity-100 transition-all p-1 hover:bg-destructive hover:text-white dark:hover:bg-red-500 rounded-md"
                @click.stop="confirmDelete(todo.id)"
              >
                <Trash2 class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="filteredActiveTodos.length === 0 && filteredCompletedTodos.length === 0" class="flex flex-col items-center justify-center h-full text-center py-12">
        <div class="w-16 h-16 rounded-full bg-muted flex items-center justify-center mb-4">
          <Check class="w-8 h-8 text-muted-foreground" />
        </div>
        <p class="text-sm text-muted-foreground">{{ $t('todo.emptyState') }}</p>
        <p class="text-xs text-muted-foreground mt-1">{{ $t('todo.emptyStateSub') }}</p>
      </div>
    </div>

    <ConfirmDialog
      v-model:open="showDeleteConfirm"
      :title="$t('common.delete')"
      :description="$t('todo.deleteConfirm')"
      :confirm-text="$t('common.delete')"
      variant="destructive"
      @confirm="handleDelete"
    />
  </div>
</template>

<style scoped>
.rich-preview {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 2;
  overflow: hidden;
  line-height: 1.45;
}

.rich-preview :deep(p),
.rich-preview :deep(ul),
.rich-preview :deep(ol),
.rich-preview :deep(blockquote),
.rich-preview :deep(pre),
.rich-preview :deep(h1),
.rich-preview :deep(h2),
.rich-preview :deep(h3) {
  margin: 0;
}

.rich-preview :deep(ul),
.rich-preview :deep(ol) {
  padding-left: 1rem;
}

.rich-preview :deep(strong) {
  font-weight: 600;
}
</style>
