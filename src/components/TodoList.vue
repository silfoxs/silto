<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useTodos } from '@/composables/useTodos'
import { Check, Clock, Trash2 } from 'lucide-vue-next'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import type { Todo } from '@/types'
import { stripHtml } from '@/lib/utils'
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()
const { activeTodos, completedTodos, loadTodos, toggleTodo, deleteTodo } = useTodos()

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
    <div class="flex-1 overflow-y-auto p-4 pt-20 pb-20 space-y-2">
      <!-- Active Todos -->
      <div v-if="activeTodos.length > 0" class="space-y-2">
        <div 
          v-for="todo in activeTodos" 
          :key="todo.id"
          class="group p-3 rounded-xl border transition-all duration-300 cursor-pointer animate-slide-in shadow-sm hover:shadow-md backdrop-blur-md"
          :class="[
            isOverdue(todo) 
              ? 'border-red-500/50 bg-red-500/5 shadow-[0_0_15px_rgba(239,68,68,0.25)] dark:shadow-[0_0_15px_rgba(239,68,68,0.4)]' 
              : 'border-black/[0.05] dark:border-white/10 bg-white/50 dark:bg-white/5 hover:bg-white/80 dark:hover:bg-white/10'
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
            <div class="flex-1 min-w-0">
              <h3 class="font-medium text-sm truncate">{{ todo.title }}</h3>
              <p v-if="todo.content" class="text-xs text-muted-foreground mt-1 line-clamp-2">
                {{ stripHtml(todo.content) }}
              </p>
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
      <div v-if="completedTodos.length > 0" class="mt-4">
        <h3 class="text-xs text-muted-foreground mb-2 px-1">{{ $t('todo.completed') }}</h3>
        <div class="space-y-2">
          <div 
            v-for="todo in completedTodos" 
            :key="todo.id"
            class="group p-3 rounded-xl border border-black/[0.03] dark:border-white/5 bg-white/30 dark:bg-white/5 backdrop-blur-sm hover:bg-white/50 dark:hover:bg-white/10 transition-all duration-300 cursor-pointer opacity-60 hover:opacity-100"
            @click="emit('edit', todo)"
          >
            <div class="flex items-center gap-3">
              <button
                class="flex-shrink-0 w-5 h-5 rounded border-2 bg-muted-foreground/40 border-muted-foreground/40 hover:bg-muted-foreground/50 transition-colors flex items-center justify-center"
                @click.stop="handleToggle(todo)"
              >
                <Check class="w-4 h-4 text-white dark:text-gray-900" />
              </button>
              <div class="flex-1 min-w-0">
                <h3 class="font-medium text-sm truncate line-through">{{ todo.title }}</h3>
                <p v-if="todo.content" class="text-xs text-muted-foreground mt-1 line-clamp-2 line-through">
                  {{ stripHtml(todo.content) }}
                </p>
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
      <div v-if="activeTodos.length === 0 && completedTodos.length === 0" class="flex flex-col items-center justify-center h-full text-center py-12">
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
