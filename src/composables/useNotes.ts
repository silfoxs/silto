import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Note } from '../types'

const notes = ref<Note[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

export function useNotes() {
    const loadNotes = async () => {
        loading.value = true
        error.value = null
        try {
            const result = await invoke<Note[]>('get_notes')
            notes.value = result
        } catch (e) {
            error.value = e as string
            console.error('Failed to load notes:', e)
        } finally {
            loading.value = false
        }
    }

    const saveNote = async (note: Note) => {
        error.value = null
        try {
            await invoke('save_note', { note })
            await loadNotes()
        } catch (e) {
            error.value = e as string
            console.error('Failed to save note:', e)
            throw e
        }
    }

    const deleteNote = async (id: string) => {
        error.value = null
        try {
            await invoke('delete_note', { id })
            await loadNotes()
        } catch (e) {
            error.value = e as string
            console.error('Failed to delete note:', e)
            throw e
        }
    }

    const sortedNotes = computed(() =>
        [...notes.value].sort((a, b) =>
            new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
        )
    )

    return {
        notes,
        loading,
        error,
        sortedNotes,
        loadNotes,
        saveNote,
        deleteNote,
    }
}
