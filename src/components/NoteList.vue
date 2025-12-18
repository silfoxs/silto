<script setup lang="ts">
import { onMounted } from 'vue'
import { useNotes } from '@/composables/useNotes'
import { StickyNote, Trash2 } from 'lucide-vue-next'
import type { Note } from '@/types'
import { stripHtml } from '@/lib/utils'

const { sortedNotes, loadNotes, deleteNote } = useNotes()

const emit = defineEmits<{
  (e: 'edit', note: Note): void
}>()

onMounted(() => {
  loadNotes()
})

const handleDelete = async (id: string) => {
  if (confirm('确定要删除这个便签吗？')) {
    await deleteNote(id)
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
      <div 
        v-for="note in sortedNotes" 
        :key="note.id"
        class="group p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors cursor-pointer animate-slide-in"
        @click="emit('edit', note)"
      >
        <div class="flex items-start gap-3">
          <div class="flex-1 min-w-0">
            <h3 class="font-medium text-sm truncate">{{ note.title || '无标题' }}</h3>
            <p v-if="note.content" class="text-xs text-muted-foreground mt-1.5 line-clamp-3">
              {{ stripHtml(note.content) }}
            </p>
            <p class="text-xs text-muted-foreground mt-2">
              {{ formatDate(note.updated_at) }}
            </p>
          </div>
          <button
            class="opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-destructive/10 rounded"
            @click.stop="handleDelete(note.id)"
          >
            <Trash2 class="w-4 h-4 text-destructive" />
          </button>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="sortedNotes.length === 0" class="flex flex-col items-center justify-center h-full text-center py-12">
        <div class="w-16 h-16 rounded-full bg-muted flex items-center justify-center mb-4">
          <StickyNote class="w-8 h-8 text-muted-foreground" />
        </div>
        <p class="text-sm text-muted-foreground">还没有任何便签</p>
        <p class="text-xs text-muted-foreground mt-1">点击下方按钮添加新的便签</p>
      </div>
    </div>
  </div>
</template>
