<script setup lang="ts">
import { ref } from 'vue'
import { DialogRoot, DialogPortal, DialogOverlay, DialogContent, DialogTitle } from 'radix-vue'
import { Maximize } from 'lucide-vue-next'
import TodoForm from '@/components/TodoForm.vue'
import { useTodos } from '@/composables/useTodos'
import type { Todo } from '@/types'

const props = defineProps<{
  open: boolean
  todo?: Todo | null
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'open-side-editor', data: { type: 'todo', data: any }): void
}>()

const { saveTodo } = useTodos()
const formRef = ref<any>(null)

const handleSave = async (todoData: Partial<Todo>) => {
  await saveTodo(todoData as Todo)
  emit('update:open', false)
}

const handleClose = () => {
  emit('update:open', false)
}

const handleExpand = (formData?: any) => {
  // If formData is passed (from emit), use it.
  // Otherwise try to get from ref (from header button click)
  let data = formData
  
  if (!data && formRef.value) {
    data = {
      title: formRef.value.title,
      content: formRef.value.content,
      remindTime: formRef.value.remindTime
    }
  }

  // Pass current form data to side editor
  emit('open-side-editor', {
    type: 'todo',
    data: {
      ...props.todo, // Keep original ID if editing
      ...data // Override with current form inputs
    }
  })
  // Close local dialog
  emit('update:open', false)
}
</script>

<template>
  <DialogRoot :open="open" @update:open="emit('update:open', $event)">
    <DialogPortal to="#app-portal">
      <DialogOverlay class="fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
      <DialogContent class="fixed bottom-4 left-4 right-4 z-50 grid gap-4 border border-white/20 bg-background/90 backdrop-blur-xl p-6 shadow-2xl duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-bottom-[48%] data-[state=open]:slide-in-from-bottom-[48%] rounded-3xl">
        <div class="flex items-center justify-between mb-4">
          <DialogTitle class="text-lg font-semibold">
            {{ todo ? '编辑 Todo' : '新建 Todo' }}
          </DialogTitle>
          <button 
            @click="handleExpand()"
            class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 p-1 hover:bg-white/10"
          >
            <Maximize class="h-4 w-4" />
            <span class="sr-only">全屏编辑</span>
          </button>
        </div>

        <TodoForm 
          ref="formRef"
          :todo="todo"
          :is-rich-text="false"
          @save="handleSave" 
          @cancel="handleClose"
          @expand="handleExpand"
        />
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>
