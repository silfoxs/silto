import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Todo } from '../types'

const todos = ref<Todo[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

export function useTodos() {
    const loadTodos = async () => {
        loading.value = true
        error.value = null
        try {
            const result = await invoke<Todo[]>('get_todos')
            todos.value = result
        } catch (e) {
            error.value = e as string
            console.error('Failed to load todos:', e)
        } finally {
            loading.value = false
        }
    }

    const saveTodo = async (todo: Todo) => {
        error.value = null
        try {
            await invoke('save_todo', { todo })
            await loadTodos()
        } catch (e) {
            error.value = e as string
            console.error('Failed to save todo:', e)
            throw e
        }
    }

    const deleteTodo = async (id: string) => {
        error.value = null
        try {
            await invoke('delete_todo', { id })
            await loadTodos()
        } catch (e) {
            error.value = e as string
            console.error('Failed to delete todo:', e)
            throw e
        }
    }

    const toggleTodo = async (todo: Todo) => {
        const updated = { ...todo, completed: !todo.completed }
        await saveTodo(updated)
    }

    const activeTodos = computed(() =>
        todos.value.filter(t => !t.completed).sort((a, b) =>
            new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
        )
    )

    const completedTodos = computed(() =>
        todos.value.filter(t => t.completed).sort((a, b) =>
            new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
        )
    )

    return {
        todos,
        loading,
        error,
        activeTodos,
        completedTodos,
        loadTodos,
        saveTodo,
        deleteTodo,
        toggleTodo,
    }
}
