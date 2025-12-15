import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Settings } from '../types'

const settings = ref<Settings>({
    left_click_action: 'todo',
    theme: 'light',
})

export function useSettings() {
    const loadSettings = async () => {
        try {
            const result = await invoke<Settings>('get_settings')
            settings.value = result
            applyTheme(result.theme)
        } catch (e) {
            console.error('Failed to load settings:', e)
        }
    }

    const saveSettings = async (newSettings: Settings) => {
        try {
            await invoke('save_settings', { settings: newSettings })
            settings.value = newSettings
            applyTheme(newSettings.theme)
        } catch (e) {
            console.error('Failed to save settings:', e)
            throw e
        }
    }

    const toggleTheme = async () => {
        const newTheme = settings.value.theme === 'light' ? 'dark' : 'light'
        await saveSettings({
            ...settings.value,
            theme: newTheme,
        })
    }

    const applyTheme = (theme: 'light' | 'dark') => {
        if (theme === 'dark') {
            document.documentElement.classList.add('dark')
        } else {
            document.documentElement.classList.remove('dark')
        }
    }

    onMounted(() => {
        loadSettings()
    })

    return {
        settings,
        loadSettings,
        saveSettings,
        toggleTheme,
    }
}
