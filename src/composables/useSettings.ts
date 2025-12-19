import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Settings, Theme } from '../types'

const settings = ref<Settings>({
    left_click_action: 'todo',
    theme: 'light',
})

export function useSettings() {
    const loadSettings = async () => {
        try {
            const result = await invoke<Settings>('get_settings')
            settings.value = result
            await applyTheme(result.theme)
        } catch (e) {
            console.error('Failed to load settings:', e)
        }
    }

    const saveSettings = async (newSettings: Settings) => {
        try {
            await invoke('save_settings', { settings: newSettings })
            settings.value = newSettings
            await applyTheme(newSettings.theme)
        } catch (e) {
            console.error('Failed to save settings:', e)
            throw e
        }
    }

    const toggleTheme = async () => {
        let newTheme: Theme = 'light'
        if (settings.value.theme === 'light') {
            newTheme = 'dark'
        } else if (settings.value.theme === 'dark') {
            newTheme = 'system'
        } else {
            newTheme = 'light'
        }

        await saveSettings({
            ...settings.value,
            theme: newTheme,
        })
    }

    const applyTheme = async (theme: Theme) => {
        const { getCurrentWindow } = await import('@tauri-apps/api/window')
        const tauriWindow = getCurrentWindow()

        let effectiveTheme: 'light' | 'dark' = 'light'

        if (theme === 'system') {
            const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
            effectiveTheme = mediaQuery.matches ? 'dark' : 'light'

            // Add listener for system theme changes
            mediaQuery.onchange = () => {
                if (settings.value.theme === 'system') {
                    applyTheme('system')
                }
            }
        } else {
            effectiveTheme = theme
        }

        // Apply dark class for CSS
        if (effectiveTheme === 'dark') {
            document.documentElement.classList.add('dark')
        } else {
            document.documentElement.classList.remove('dark')
        }

        // Set window theme to fix macOS background issue
        // This ensures the vibrancy follows the correct theme
        try {
            await tauriWindow.setTheme(effectiveTheme)
        } catch (e) {
            console.error('Failed to set window theme:', e)
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
        applyTheme,
    }
}
