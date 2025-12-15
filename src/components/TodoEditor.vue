<script setup lang="ts">
import { ref, watch } from 'vue'
import { DialogRoot, DialogPortal, DialogOverlay, DialogContent, DialogTitle, DialogClose } from 'radix-vue'
import { X, Calendar } from 'lucide-vue-next'
import Input from '@/components/ui/Input.vue'
import Textarea from '@/components/ui/Textarea.vue'
import Button from '@/components/ui/Button.vue'
import { useTodos } from '@/composables/useTodos'
import type { Todo } from '@/types'

const props = defineProps<{
  open: boolean
  todo?: Todo | null
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
}>()

const { saveTodo } = useTodos()

const title = ref('')
const content = ref('')
const remindTime = ref('')

watch(() => props.todo, (newTodo) => {
  if (newTodo) {
    title.value = newTodo.title
    content.value = newTodo.content
    remindTime.value = newTodo.remind_time ? new Date(newTodo.remind_time).toISOString().slice(0, 16) : ''
  } else {
    title.value = ''
    content.value = ''
    remindTime.value = ''
  }
}, { immediate: true })

const handleSave = async () => {
  if (!title.value.trim()) {
    alert('请输入标题')
    return
  }

  const todo: Todo = {
    id: props.todo?.id || crypto.randomUUID(),
    title: title.value,
    content: content.value,
    remind_time: remindTime.value ? new Date(remindTime.value).toISOString() : null,
    completed: props.todo?.completed || false,
    created_at: props.todo?.created_at || new Date().toISOString(),
  }

  await saveTodo(todo)
  emit('update:open', false)
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <DialogRoot :open="open" @update:open="emit('update:open', $event)">
    <DialogPortal>
      <DialogOverlay class="fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
      <DialogContent class="fixed left-[50%] top-[50%] z-50 grid w-full max-w-md translate-x-[-50%] translate-y-[-50%] gap-4 border bg-background p-6 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 sm:rounded-lg">
        <div class="flex items-center justify-between">
          <DialogTitle class="text-lg font-semibold">
            {{ todo ? '编辑 Todo' : '新建 Todo' }}
          </DialogTitle>
          <DialogClose class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100">
            <X class="h-4 w-4" />
            <span class="sr-only">关闭</span>
          </DialogClose>
        </div>

        <div class="space-y-4">
          <div>
            <label class="text-sm font-medium mb-2 block">标题</label>
            <Input v-model="title" placeholder="输入标题..." />
          </div>

          <div>
            <label class="text-sm font-medium mb-2 block">内容</label>
            <Textarea v-model="content" placeholder="输入内容..." rows="4" />
          </div>

          <div>
            <label class="text-sm font-medium mb-2 block flex items-center gap-2">
              <Calendar class="w-4 h-4" />
              提醒时间（可选）
            </label>
            <input 
              v-model="remindTime"
              type="datetime-local"
              class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
            />
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-2">
          <Button variant="outline" @click="handleClose">
            取消
          </Button>
          <Button @click="handleSave">
            保存
          </Button>
        </div>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>
