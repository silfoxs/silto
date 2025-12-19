<script setup lang="ts">
import { onMounted } from 'vue'
import { useNotes } from '@/composables/useNotes'
import { StickyNote, Trash2 } from 'lucide-vue-next'
import type { Note } from '@/types'
import { stripHtml } from '@/lib/utils'
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()
const { sortedNotes, loadNotes, deleteNote } = useNotes()

const emit = defineEmits<{
  (e: 'edit', note: Note): void
}>()

onMounted(() => {
  loadNotes()
})

const handleDelete = async (id: string) => {
  if (confirm(t('note.deleteConfirm'))) {
    await deleteNote(id)
  }
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleString(locale.value, { 
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
        class="group p-4 rounded-xl border border-black/[0.05] dark:border-white/10 bg-white/50 dark:bg-white/5 backdrop-blur-md hover:bg-white/80 dark:hover:bg-white/10 transition-all duration-300 cursor-pointer animate-slide-in shadow-sm hover:shadow-md"
        @click="emit('edit', note)"
      >
        <div class="flex items-start gap-3">
          <div class="flex-1 min-w-0">
            <h3 class="font-medium text-sm truncate">{{ note.title || $t('common.untitled') }}</h3>
            <p v-if="note.content" class="text-xs text-muted-foreground mt-1.5 line-clamp-3">
              {{ stripHtml(note.content) }}
            </p>
            <p class="text-xs text-muted-foreground mt-2">
              {{ formatDate(note.updated_at) }}
            </p>
          </div>
          <button
            class="opacity-0 group-hover:opacity-100 transition-all p-1 hover:bg-destructive hover:text-white dark:hover:bg-red-500 rounded-md"
            @click.stop="handleDelete(note.id)"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="sortedNotes.length === 0" class="flex flex-col items-center justify-center h-full text-center py-12">
        <div class="w-16 h-16 rounded-full bg-muted flex items-center justify-center mb-4">
          <StickyNote class="w-8 h-8 text-muted-foreground" />
        </div>
        <p class="text-sm text-muted-foreground">{{ $t('note.emptyState') }}</p>
        <p class="text-xs text-muted-foreground mt-1">{{ $t('note.emptyStateSub') }}</p>
      </div>
    </div>
  </div>
</template>
