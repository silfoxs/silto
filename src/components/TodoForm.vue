<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { Calendar } from 'lucide-vue-next'
import Input from '@/components/ui/Input.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'
import Button from '@/components/ui/Button.vue'
import DateTimePicker from '@/components/ui/DateTimePicker.vue'
import type { Todo } from '@/types'

const props = defineProps<{
  todo?: Todo | null
  initialTitle?: string
  initialContent?: string
  initialRemindTime?: string
  isRichText?: boolean
}>()

const emit = defineEmits<{
  (e: 'save', todo: Partial<Todo>): void
  (e: 'cancel'): void
}>()

const title = ref('')
const content = ref('')
const remindTime = ref('')

// Initialize from props
onMounted(() => {
  if (props.todo) {
    // Initialize from props (handle both DB format and editor partial format)
    title.value = props.todo.title || ''
    content.value = props.todo.content || ''
    // Handle both snake_case (DB) and camelCase (Editor pass-through)
    const rt = (props.todo as any).remind_time || (props.todo as any).remindTime
    remindTime.value = rt ? new Date(rt).toISOString().slice(0, 16) : ''
  } else {
    // Or initialize from passed initial values (for detached mode)
    title.value = props.initialTitle || ''
    content.value = props.initialContent || ''
    remindTime.value = props.initialRemindTime || ''
  }
})

watch(() => props.todo, (newTodo) => {
  if (newTodo) {
    title.value = newTodo.title
    content.value = newTodo.content
    remindTime.value = newTodo.remind_time ? new Date(newTodo.remind_time).toISOString().slice(0, 16) : ''
  }
})

const handleSave = () => {
  if (!title.value.trim() && !content.value.trim()) {
    alert('请输入标题或内容')
    return
  }

  const finalTitle = title.value.trim() || (content.value.trim() ? content.value.trim().slice(0, 5) : '新建代办')

  emit('save', {
    id: props.todo?.id || crypto.randomUUID(),
    title: finalTitle,
    content: content.value,
    remind_time: remindTime.value ? new Date(remindTime.value).toISOString() : null,
    completed: props.todo?.completed || false,
    created_at: props.todo?.created_at || new Date().toISOString(),
  })
}



defineExpose({
  title,
  content,
  remindTime
})
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex-1 space-y-4 overflow-y-auto p-1">
      <div>

        
        <!-- Rich Text Mode -->
        <RichTextEditor 
          v-if="isRichText !== false"
          v-model="content" 
          placeholder="输入内容..." 
        />
        
        <!-- Plain Text Mode -->
        <textarea
          v-else
          v-model="content"
          class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-black dark:focus-visible:ring-white disabled:cursor-not-allowed disabled:opacity-50 min-h-[150px] resize-none"
          placeholder="输入内容..."
        ></textarea>
      </div>

      <div>
        <label class="text-sm font-medium mb-2 block">标题（可选）</label>
        <Input v-model="title" placeholder="输入标题..." />
      </div>

      <div>
        <label class="text-sm font-medium mb-2 block flex items-center gap-2">
          <Calendar class="w-4 h-4" />
          提醒时间（可选）
        </label>
        <DateTimePicker 
          v-model="remindTime" 
          placeholder="设置提醒时间..." 
        />
      </div>
    </div>

    <div class="flex justify-end gap-2 mt-4 pt-4 border-t border-border/50">
      <Button variant="outline" @click="$emit('cancel')">
        取消
      </Button>
      <Button @click="handleSave">
        保存
      </Button>
    </div>
  </div>
</template>
