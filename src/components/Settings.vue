<script setup lang="ts">
import { DialogRoot, DialogPortal, DialogOverlay, DialogContent, DialogTitle, DialogClose } from 'radix-vue'
import { X, Moon, Sun, Languages } from 'lucide-vue-next'
import Button from '@/components/ui/Button.vue'
import { useSettings } from '@/composables/useSettings'
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()

defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
}>()

const { settings, saveSettings, toggleTheme } = useSettings()

const handleLeftClickActionChange = async (action: 'todo' | 'note') => {
  await saveSettings({
    ...settings.value,
    left_click_action: action,
  })
}

const handleLanguageChange = (lang: string) => {
  locale.value = lang
  localStorage.setItem('language', lang)
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <DialogRoot :open="open" @update:open="emit('update:open', $event)">
    <DialogPortal to="#app-portal">
      <DialogOverlay class="fixed inset-0 z-50 bg-black/80 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
      <DialogContent class="fixed bottom-4 left-4 right-4 z-50 grid gap-4 border border-white/40 bg-white/90 dark:bg-black/40 backdrop-blur-2xl dark:backdrop-saturate-[1.5] p-6 shadow-2xl duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-bottom-[48%] data-[state=open]:slide-in-from-bottom-[48%] rounded-3xl">
        <div class="flex items-center justify-between">
          <DialogTitle class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ $t('settings.title') }}
          </DialogTitle>
          <DialogClose class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 text-gray-900 dark:text-white">
            <X class="h-4 w-4" />
            <span class="sr-only">{{ $t('common.close') }}</span>
          </DialogClose>
        </div>

        <div class="space-y-6">
          <!-- 主题设置 -->
          <div>
            <label class="text-sm font-medium mb-3 block">{{ $t('settings.theme') }}</label>
            <div class="flex gap-2">
              <Button
                :variant="settings.theme === 'light' ? 'default' : 'outline'"
                class="flex-1"
                @click="settings.theme === 'dark' ? toggleTheme() : null"
              >
                <Sun class="w-4 h-4 mr-2" />
                {{ $t('settings.themeLight') }}
              </Button>
              <Button
                :variant="settings.theme === 'dark' ? 'default' : 'outline'"
                class="flex-1"
                @click="settings.theme === 'light' ? toggleTheme() : null"
              >
                <Moon class="w-4 h-4 mr-2" />
                {{ $t('settings.themeDark') }}
              </Button>
            </div>
          </div>

          <!-- 左键点击行为 -->
          <div>
            <label class="text-sm font-medium mb-3 block">{{ $t('settings.leftClickAction') }}</label>
            <div class="flex gap-2">
              <Button
                :variant="settings.left_click_action === 'todo' ? 'default' : 'outline'"
                class="flex-1"
                @click="handleLeftClickActionChange('todo')"
              >
                {{ $t('settings.todoList') }}
              </Button>
              <Button
                :variant="settings.left_click_action === 'note' ? 'default' : 'outline'"
                class="flex-1"
                @click="handleLeftClickActionChange('note')"
              >
                {{ $t('settings.noteList') }}
              </Button>
            </div>
          </div>

          <!-- 语言设置 -->
          <div>
            <label class="text-sm font-medium mb-3 block">{{ $t('settings.language') }}</label>
            <div class="flex gap-2">
              <Button
                :variant="locale === 'zh-CN' ? 'default' : 'outline'"
                class="flex-1"
                @click="handleLanguageChange('zh-CN')"
              >
                <Languages class="w-4 h-4 mr-2" />
                {{ $t('locales.zh-CN') }}
              </Button>
              <Button
                :variant="locale === 'en-US' ? 'default' : 'outline'"
                class="flex-1"
                @click="handleLanguageChange('en-US')"
              >
                <Languages class="w-4 h-4 mr-2" />
                {{ $t('locales.en-US') }}
              </Button>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-2">
          <Button @click="handleClose">
            {{ $t('common.done') }}
          </Button>
        </div>
      </DialogContent>
    </DialogPortal>
  </DialogRoot>
</template>
