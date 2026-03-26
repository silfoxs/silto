<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import Input from '@/components/ui/Input.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'
import type { Note } from '@/types'

const props = defineProps<{
  note?: Note | null
  initialTitle?: string
  initialContent?: string
}>()

const emit = defineEmits<{
  (e: 'save', note: Partial<Note>): void
}>()

const title = ref('')
const content = ref('')
const draftId = ref('')
const draftCreatedAt = ref('')
const isHydrating = ref(true)
let autosaveTimeout: ReturnType<typeof setTimeout> | undefined

onMounted(() => {
  if (props.note) {
    // Initialize from props (handle both DB format and editor partial format)
    title.value = props.note.title || ''
    content.value = props.note.content || ''
    draftId.value = props.note.id
    draftCreatedAt.value = props.note.created_at
  } else {
    // Or initialize from passed initial values
    title.value = props.initialTitle || ''
    content.value = props.initialContent || ''
    draftId.value = crypto.randomUUID()
    draftCreatedAt.value = new Date().toISOString()
  }
  isHydrating.value = false
})

watch(() => props.note, (newNote) => {
  isHydrating.value = true
  if (newNote) {
    title.value = newNote.title
    content.value = newNote.content
    draftId.value = newNote.id
    draftCreatedAt.value = newNote.created_at
  } else {
    title.value = props.initialTitle || ''
    content.value = props.initialContent || ''
    draftId.value = crypto.randomUUID()
    draftCreatedAt.value = new Date().toISOString()
  }
  isHydrating.value = false
})

const emitAutosave = () => {
  if (!content.value.trim()) {
    return
  }

  const plainContent = content.value.replace(/<[^>]*>/g, '').trim()
  const finalTitle = title.value.trim() || (plainContent ? plainContent.slice(0, 10) : '新建便签')
  const now = new Date().toISOString()

  emit('save', {
    id: draftId.value,
    title: finalTitle,
    content: content.value,
    created_at: draftCreatedAt.value,
    updated_at: now,
  })
}

watch([title, content], () => {
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
  content
})
</script>

<template>
  <div class="flex flex-col h-full relative rounded-[18px] bg-white/70 dark:bg-black/[0.58] border border-transparent dark:border-white/16 dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.06)]">
    <div class="flex-1 space-y-5 overflow-y-auto p-2">
      <div>
        <RichTextEditor 
          v-model="content" 
          :scroll-key="draftId"
          :placeholder="$t('note.contentPlaceholder')" 
        />
      </div>

      <div>
        <label class="text-sm font-medium mb-2 block text-foreground/85">{{ $t('note.titleLabel') }}</label>
        <Input v-model="title" class="h-11 rounded-[18px] border-black/[0.06] dark:border-white/24 bg-white/50 dark:bg-black/[0.72] shadow-[0_10px_24px_rgba(15,23,42,0.05)] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)] focus-visible:border-black/20 focus-visible:ring-2 focus-visible:ring-black/10 dark:focus-visible:border-white/40 dark:focus-visible:ring-white/15" :placeholder="$t('note.titlePlaceholder')" />
      </div>
    </div>
  </div>
</template>
