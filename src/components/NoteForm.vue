<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import Input from '@/components/ui/Input.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'
import Button from '@/components/ui/Button.vue'
import type { Note } from '@/types'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps<{
  note?: Note | null
  initialTitle?: string
  initialContent?: string
  isRichText?: boolean
}>()

const emit = defineEmits<{
  (e: 'save', note: Partial<Note>): void
  (e: 'cancel'): void
}>()

const title = ref('')
const content = ref('')

onMounted(() => {
  if (props.note) {
    // Initialize from props (handle both DB format and editor partial format)
    title.value = props.note.title || ''
    content.value = props.note.content || ''
  } else {
    // Or initialize from passed initial values
    title.value = props.initialTitle || ''
    content.value = props.initialContent || ''
  }
})

watch(() => props.note, (newNote) => {
  if (newNote) {
    title.value = newNote.title
    content.value = newNote.content
  }
})

const handleSave = () => {
  if (!content.value.trim()) {
    alert(t('note.inputError'))
    return
  }

  const plainContent = content.value.replace(/<[^>]*>/g, '').trim()
  const finalTitle = title.value.trim() || (plainContent ? plainContent.slice(0, 10) : t('note.newNote'))
  const now = new Date().toISOString()

  emit('save', {
    id: props.note?.id || crypto.randomUUID(),
    title: finalTitle,
    content: content.value,
    created_at: props.note?.created_at || now,
    updated_at: now,
  })
}



defineExpose({
  title,
  content
})
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex-1 space-y-4 overflow-y-auto p-1">
      <div>

        
        <RichTextEditor 
          v-if="isRichText !== false"
          v-model="content" 
          :placeholder="$t('note.contentPlaceholder')" 
        />
        
        <textarea
          v-else
          v-model="content"
          class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-black/20 dark:focus-visible:ring-white/30 disabled:cursor-not-allowed disabled:opacity-50 min-h-[150px] resize-none"
          :placeholder="$t('note.contentPlaceholder')"
        ></textarea>
      </div>

      <div>
        <label class="text-sm font-medium mb-2 block">{{ $t('note.titleLabel') }}</label>
        <Input v-model="title" :placeholder="$t('note.titlePlaceholder')" />
      </div>
    </div>

    <div class="flex justify-end gap-2 mt-4 pt-4 border-t border-border/50">
      <Button variant="outline" @click="$emit('cancel')">
        {{ $t('common.cancel') }}
      </Button>
      <Button @click="handleSave">
        {{ $t('common.save') }}
      </Button>
    </div>
  </div>
</template>
