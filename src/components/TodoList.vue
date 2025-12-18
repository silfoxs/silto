<script setup lang="ts">
import { onMounted } from 'vue'
import { useTodos } from '@/composables/useTodos'
import { Check, Clock, Trash2 } from 'lucide-vue-next'
import type { Todo } from '@/types'
import { stripHtml } from '@/lib/utils'

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

const handleDelete = async (id: string) => {
  if (confirm('确定要删除这个 Todo 吗？')) {
    await deleteTodo(id)
  }
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleString('zh-CN', { 
    month: '2-digit', 
    day: '2-digit', 
    hour: '2-digit', 
    minute: '2-digit' 
  })
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
          class="group p-3 rounded-xl border border-white/10 bg-white/5 dark:bg-white/5 backdrop-blur-md hover:bg-white/10 dark:hover:bg-white/10 transition-all duration-300 cursor-pointer animate-slide-in shadow-sm hover:shadow-md"
          @click="emit('edit', todo)"
        >
          <div class="flex items-center gap-3">
            <button
              class="flex-shrink-0 w-5 h-5 rounded border-2 border-primary hover:bg-primary/10 transition-colors"
              @click.stop="handleToggle(todo)"
            >
              <Check v-if="todo.completed" class="w-4 h-4 text-primary" />
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
              class="opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-destructive/10 rounded"
              @click.stop="handleDelete(todo.id)"
            >
              <Trash2 class="w-4 h-4 text-destructive" />
            </button>
          </div>
        </div>
      </div>

      <!-- Completed Todos -->
      <div v-if="completedTodos.length > 0" class="mt-4">
        <h3 class="text-xs text-muted-foreground mb-2 px-1">已完成</h3>
        <div class="space-y-2">
          <div 
            v-for="todo in completedTodos" 
            :key="todo.id"
            class="group p-3 rounded-xl border border-white/5 bg-white/5 dark:bg-white/5 backdrop-blur-sm hover:bg-white/10 dark:hover:bg-white/10 transition-all duration-300 cursor-pointer opacity-60 hover:opacity-100"
            @click="emit('edit', todo)"
          >
            <div class="flex items-center gap-3">
              <button
                class="flex-shrink-0 w-5 h-5 rounded border-2 bg-primary border-primary hover:bg-primary/80 transition-colors flex items-center justify-center"
                @click.stop="handleToggle(todo)"
              >
                <Check class="w-4 h-4 text-primary-foreground" />
              </button>
              <div class="flex-1 min-w-0">
                <h3 class="font-medium text-sm truncate line-through">{{ todo.title }}</h3>
                <p v-if="todo.content" class="text-xs text-muted-foreground mt-1 line-clamp-2 line-through">
                  {{ stripHtml(todo.content) }}
                </p>
              </div>
              <button
                class="opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-destructive/10 rounded"
                @click.stop="handleDelete(todo.id)"
              >
                <Trash2 class="w-4 h-4 text-destructive" />
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
        <p class="text-sm text-muted-foreground">还没有任何 Todo</p>
        <p class="text-xs text-muted-foreground mt-1">点击下方按钮添加新的任务</p>
      </div>
    </div>
  </div>
</template>
