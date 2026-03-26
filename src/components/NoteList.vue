<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useNotes } from '@/composables/useNotes'
import { StickyNote, Trash2, Search } from 'lucide-vue-next'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import Input from '@/components/ui/Input.vue'
import type { Note } from '@/types'
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()
const { sortedNotes, loadNotes, deleteNote } = useNotes()

const props = defineProps<{
  selectedId?: string | null
}>()

const emit = defineEmits<{
  (e: 'edit', note: Note): void
}>()

onMounted(() => {
  loadNotes()
})

const showDeleteConfirm = ref(false)
const itemToDelete = ref<string | null>(null)

const confirmDelete = (id: string) => {
  itemToDelete.value = id
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  if (itemToDelete.value) {
    await deleteNote(itemToDelete.value)
    itemToDelete.value = null
  }
}

const searchQuery = ref('')

const filteredNotes = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  if (!query) return sortedNotes.value

  return sortedNotes.value.filter((note) => {
    const plainContent = (note.content || '').replace(/<[^>]*>/g, ' ')
    return `${note.title || ''} ${plainContent}`.toLowerCase().includes(query)
  })
})

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
    <div class="flex-1 overflow-y-auto p-4 pt-16 pb-24 space-y-3">
      <div class="sticky top-0 z-10 pb-3">
        <div class="relative">
          <Search class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-foreground/35" />
          <Input
            v-model="searchQuery"
            class="h-10 rounded-[16px] border-black/[0.06] bg-white/80 pl-9 pr-3 shadow-none dark:border-white/18 dark:bg-black/[0.72]"
            :placeholder="$t('note.searchPlaceholder')"
          />
        </div>
      </div>

      <div 
        v-for="note in filteredNotes" 
        :key="note.id"
        class="group p-4 rounded-[16px] border backdrop-blur-md transition-all duration-300 cursor-pointer animate-slide-in shadow-sm hover:shadow-md"
        :class="props.selectedId === note.id
          ? 'border-black/30 bg-[#f3efe7] shadow-[0_16px_32px_rgba(15,23,42,0.10),inset_0_0_0_1px_rgba(15,23,42,0.06)] dark:border-white/40 dark:bg-white/[0.14] dark:shadow-[0_16px_32px_rgba(0,0,0,0.30),inset_0_0_0_1px_rgba(255,255,255,0.16)]'
          : 'border-black/[0.05] dark:border-white/22 bg-white/50 dark:bg-black/[0.66] hover:bg-white/80 dark:hover:bg-black/[0.78] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08)]'"
        @click="emit('edit', note)"
      >
        <div class="flex items-start gap-3">
          <div class="flex-1 min-w-0 overflow-hidden">
            <h3 class="font-medium text-sm truncate">{{ note.title || $t('common.untitled') }}</h3>
            <div
              v-if="note.content"
              class="rich-preview mt-1.5 text-xs text-muted-foreground"
              v-html="note.content"
            ></div>
            <p class="text-xs text-muted-foreground mt-2">
              {{ formatDate(note.updated_at) }}
            </p>
          </div>
          <button
            class="opacity-0 group-hover:opacity-100 transition-all p-1 hover:bg-destructive hover:text-white dark:hover:bg-red-500 rounded-md"
            @click.stop="confirmDelete(note.id)"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Empty State -->
      <div v-if="filteredNotes.length === 0" class="flex flex-col items-center justify-center h-full text-center py-12">
        <div class="w-16 h-16 rounded-full bg-muted flex items-center justify-center mb-4">
          <StickyNote class="w-8 h-8 text-muted-foreground" />
        </div>
        <p class="text-sm text-muted-foreground">{{ $t('note.emptyState') }}</p>
        <p class="text-xs text-muted-foreground mt-1">{{ $t('note.emptyStateSub') }}</p>
      </div>
    </div>

    <ConfirmDialog
      v-model:open="showDeleteConfirm"
      :title="$t('common.delete')"
      :description="$t('note.deleteConfirm')"
      :confirm-text="$t('common.delete')"
      variant="destructive"
      @confirm="handleDelete"
    />
  </div>
</template>

<style scoped>
.rich-preview {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 3;
  overflow: hidden;
  line-height: 1.45;
}

.rich-preview :deep(p),
.rich-preview :deep(ul),
.rich-preview :deep(ol),
.rich-preview :deep(blockquote),
.rich-preview :deep(pre),
.rich-preview :deep(h1),
.rich-preview :deep(h2),
.rich-preview :deep(h3) {
  margin: 0;
}

.rich-preview :deep(ul),
.rich-preview :deep(ol) {
  padding-left: 1rem;
}

.rich-preview :deep(strong) {
  font-weight: 600;
}
</style>
