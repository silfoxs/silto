import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { Settings, Theme } from '../types'

const settings = ref<Settings>({
    left_click_action: 'todo',
    theme: 'system',
    language: 'zh-CN',
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

        // Set window theme and vibrancy to fix macOS background issue
        try {
            await tauriWindow.setTheme(effectiveTheme)
            // Re-apply vibrancy with theme-specific material to avoid gray background
            await invoke('apply_vibrancy', { theme: effectiveTheme })
        } catch (e) {
            console.error('Failed to set window theme or vibrancy:', e)
        }
    }

    onMounted(() => {
        loadSettings()

        // Listen for settings changes from other windows
        const unlisten = listen<Settings>('settings-changed', async (event) => {
            console.log('Settings changed event received:', event.payload)
            settings.value = event.payload
            await applyTheme(event.payload.theme)
        })

        return () => {
            unlisten.then(f => f())
        }
    })

    return {
        settings,
        loadSettings,
        saveSettings,
        toggleTheme,
        applyTheme,
    }
}
