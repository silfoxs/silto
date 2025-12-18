<script setup lang="ts">

import { DialogRoot, DialogPortal, DialogOverlay, DialogContent, DialogTitle } from 'radix-vue'
import TodoForm from '@/components/TodoForm.vue'
import { useTodos } from '@/composables/useTodos'
import type { Todo } from '@/types'

defineProps<{
  open: boolean
  todo?: Todo | null
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
}>()

const { saveTodo } = useTodos()

const handleSave = async (todoData: Partial<Todo>) => {
  await saveTodo(todoData as Todo)
  emit('update:open', false)
}

const handleClose = () => {
  emit('update:open', false)
}


</script>

<template>
  <DialogRoot :open="open" @update:open="emit('update:open', $event)">
    <DialogPortal to="#app-portal">
      <DialogOverlay class="fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
      <DialogContent class="fixed bottom-4 left-4 right-4 z-50 grid gap-2 border border-white/40 bg-white/90 dark:bg-black/40 backdrop-blur-2xl dark:backdrop-saturate-[1.5] p-4 shadow-2xl duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-bottom-[48%] data-[state=open]:slide-in-from-bottom-[48%] rounded-3xl">
        <div class="flex items-center justify-center">
          <DialogTitle class="text-base font-semibold text-gray-900 dark:text-white">
            {{ todo ? $t('todo.newTodo').replace('新建', '编辑') : $t('todo.newTodo') }}
          </DialogTitle>
        </div>

        <TodoForm 
          :todo="todo"
          :is-rich-text="false"
          @save="handleSave" 
          @cancel="handleClose"
        />
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>
