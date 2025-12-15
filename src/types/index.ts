export interface Todo {
    id: string
    title: string
    content: string
    remind_time: string | null
    completed: boolean
    created_at: string
}

export interface Note {
    id: string
    title: string
    content: string
    created_at: string
    updated_at: string
}

export type LeftClickAction = 'todo' | 'note'
export type Theme = 'light' | 'dark'

export interface Settings {
    left_click_action: LeftClickAction
    theme: Theme
}
