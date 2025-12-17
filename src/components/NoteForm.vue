<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import Input from '@/components/ui/Input.vue'
import RichTextEditor from '@/components/ui/RichTextEditor.vue'
import Button from '@/components/ui/Button.vue'
import type { Note } from '@/types'

const props = defineProps<{
  note?: Note | null
  allowExpand?: boolean
  initialTitle?: string
  initialContent?: string
  isRichText?: boolean
}>()

const emit = defineEmits<{
  (e: 'save', note: Partial<Note>): void
  (e: 'cancel'): void
  (e: 'expand', data: { title: string, content: string }): void
}>()

const title = ref('')
const content = ref('')

onMounted(() => {
  if (props.note) {
    title.value = props.note.title
    content.value = props.note.content
  } else {
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
    alert('请输入内容')
    return
  }

  const finalTitle = title.value.trim() || content.value.trim().slice(0, 5)
  const now = new Date().toISOString()

  emit('save', {
    id: props.note?.id || crypto.randomUUID(),
    title: finalTitle,
    content: content.value,
    created_at: props.note?.created_at || now,
    updated_at: now,
  })
}

const handleExpand = () => {
  emit('expand', {
    title: title.value,
    content: content.value
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
        <label class="text-sm font-medium mb-2 block">内容</label>
        
        <RichTextEditor 
          v-if="isRichText !== false"
          v-model="content" 
          placeholder="输入内容..." 
          :allow-expand="allowExpand"
          @expand="handleExpand"
        />
        
        <textarea
          v-else
          v-model="content"
          class="flex w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 min-h-[150px] resize-none"
          placeholder="输入内容..."
        ></textarea>
      </div>

      <div>
        <label class="text-sm font-medium mb-2 block">标题（可选）</label>
        <Input v-model="title" placeholder="输入标题..." />
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
