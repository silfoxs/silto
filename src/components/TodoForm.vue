<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import { Calendar } from 'lucide-vue-next'
import Input from '@/components/ui/Input.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'
import DateTimePicker from '@/components/ui/DateTimePicker.vue'
import type { Todo } from '@/types'

const props = defineProps<{
  todo?: Todo | null
  initialTitle?: string
  initialContent?: string
  initialRemindTime?: string
}>()

const emit = defineEmits<{
  (e: 'save', todo: Partial<Todo>): void
}>()

const title = ref('')
const content = ref('')
const remindTime = ref('')
const draftId = ref('')
const draftCreatedAt = ref('')
const isHydrating = ref(true)
let autosaveTimeout: ReturnType<typeof setTimeout> | undefined

const toLocalDateTimeInputValue = (value: string) => {
  const date = new Date(value)
  const offset = date.getTimezoneOffset()
  return new Date(date.getTime() - offset * 60_000).toISOString().slice(0, 16)
}

// Initialize from props
onMounted(() => {
  if (props.todo) {
    // Initialize from props (handle both DB format and editor partial format)
    title.value = props.todo.title || ''
    content.value = props.todo.content || ''
    // Handle both snake_case (DB) and camelCase (Editor pass-through)
    const rt = (props.todo as any).remind_time || (props.todo as any).remindTime
    remindTime.value = rt ? toLocalDateTimeInputValue(rt) : ''
    draftId.value = props.todo.id
    draftCreatedAt.value = props.todo.created_at
  } else {
    // Or initialize from passed initial values (for detached mode)
    title.value = props.initialTitle || ''
    content.value = props.initialContent || ''
    remindTime.value = props.initialRemindTime || ''
    draftId.value = crypto.randomUUID()
    draftCreatedAt.value = new Date().toISOString()
  }
  isHydrating.value = false
})

watch(() => props.todo, (newTodo) => {
  isHydrating.value = true
  if (newTodo) {
    title.value = newTodo.title
    content.value = newTodo.content
    remindTime.value = newTodo.remind_time ? toLocalDateTimeInputValue(newTodo.remind_time) : ''
    draftId.value = newTodo.id
    draftCreatedAt.value = newTodo.created_at
  } else {
    title.value = props.initialTitle || ''
    content.value = props.initialContent || ''
    remindTime.value = props.initialRemindTime || ''
    draftId.value = crypto.randomUUID()
    draftCreatedAt.value = new Date().toISOString()
  }
  isHydrating.value = false
})

const emitAutosave = () => {
  if (!title.value.trim() && !content.value.trim()) {
    return
  }

  const plainContent = content.value.replace(/<[^>]*>/g, '').trim()
  const finalTitle = title.value.trim() || (plainContent ? plainContent.slice(0, 10) : '新建待办')

  emit('save', {
    id: draftId.value,
    title: finalTitle,
    content: content.value,
    remind_time: remindTime.value ? new Date(remindTime.value).toISOString() : null,
    completed: props.todo?.completed || false,
    created_at: draftCreatedAt.value,
  })
}

watch([title, content, remindTime], () => {
  if (isHydrating.value) return
  if (autosaveTimeout) clearTimeout(autosaveTimeout)
  autosaveTimeout = setTimeout(() => {
    emitAutosave()
  }, 500)
})

onBeforeUnmount(() => {
  if (autosaveTimeout) clearTimeout(autosaveTimeout)
})

defineExpose({
  title,
  content,
  remindTime
})
</script>

<template>
  <div class="flex flex-col h-full relative rounded-[18px] bg-white/70 dark:bg-black/[0.58] border border-transparent dark:border-white/16 dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.06)]">
    <div class="flex-1 space-y-5 overflow-y-auto p-2">
      <div>
        <RichTextEditor 
          v-model="content" 
          :scroll-key="draftId"
          :placeholder="$t('todo.contentPlaceholder')" 
        />
      </div>

      <div>
        <label class="text-sm font-medium mb-2 block text-foreground/85">{{ $t('todo.titleLabel') }}</label>
        <Input v-model="title" class="h-11 rounded-[18px] border-black/[0.06] dark:border-white/24 bg-white/50 dark:bg-black/[0.72] shadow-[0_10px_24px_rgba(15,23,42,0.05)] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)] focus-visible:border-black/20 focus-visible:ring-2 focus-visible:ring-black/10 dark:focus-visible:border-white/40 dark:focus-visible:ring-white/15" :placeholder="$t('todo.titlePlaceholder')" />
      </div>

      <div>
        <label class="text-sm font-medium mb-2 block flex items-center gap-2 text-foreground/85">
          <Calendar class="w-4 h-4" />
          {{ $t('todo.remindTime') }}
        </label>
        <DateTimePicker 
          v-model="remindTime" 
          :placeholder="$t('todo.remindTimePlaceholder')" 
        />
      </div>
    </div>
  </div>
</template>
