<script setup lang="ts">
import { ref } from 'vue'
import { DialogRoot, DialogPortal, DialogOverlay, DialogContent, DialogTitle, DialogClose } from 'radix-vue'
import { X, Moon, Sun, Monitor, Languages, RefreshCw } from 'lucide-vue-next'
import { check } from '@tauri-apps/plugin-updater'
import { ask, message } from '@tauri-apps/plugin-dialog'
import Button from '@/components/ui/Button.vue'
import { useSettings } from '@/composables/useSettings'
import { useI18n } from 'vue-i18n'

const { locale, t } = useI18n()

defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
}>()

const { settings, saveSettings } = useSettings()

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

import { relaunch } from '@tauri-apps/plugin-process'

const isChecking = ref(false)
const isUpdating = ref(false)
const downloadProgress = ref(0)
const contentLength = ref(0)
const downloaded = ref(0)

const handleCheckUpdate = async () => {
  if (isChecking.value || isUpdating.value) return
  isChecking.value = true
  try {
    const update = await check()
    if (update) {
      const confirmed = await ask(t('settings.updateAvailable'), {
        title: t('settings.checkUpdate'),
        kind: 'info',
        okLabel: t('common.confirm'),
        cancelLabel: t('common.cancel'),
      })
      if (confirmed) {
        isUpdating.value = true
        downloadProgress.value = 0
        
        await update.downloadAndInstall((event) => {
          switch (event.event) {
            case 'Started':
              contentLength.value = event.data.contentLength || 0
              break
            case 'Progress':
              downloaded.value += event.data.chunkLength
              if (contentLength.value > 0) {
                downloadProgress.value = Math.round((downloaded.value / contentLength.value) * 100)
              }
              break
            case 'Finished':
              downloadProgress.value = 100
              break
          }
        })
        
        await relaunch()
      }
    } else {
      await message(t('settings.noUpdate'), {
        title: t('settings.checkUpdate'),
        kind: 'info',
        okLabel: t('common.done'),
      })
    }
  } catch (error) {
    console.error('Failed to check for updates:', error)
    // 当检查失败时（通常是因为已经是最新版本或网络问题），提示用户已是最新
    await message(t('settings.noUpdate'), {
      title: t('settings.checkUpdate'),
      kind: 'info',
      okLabel: t('common.done'),
    })
  } finally {
    isChecking.value = false
    isUpdating.value = false
    downloadProgress.value = 0
    contentLength.value = 0
    downloaded.value = 0
  }
}
</script>

<template>
  <DialogRoot :open="open" @update:open="emit('update:open', $event)">
    <DialogPortal to="#app-portal">
      <DialogOverlay class="fixed inset-0 z-50 bg-black/20 backdrop-blur-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0" />
      <DialogContent class="fixed bottom-4 left-4 right-4 z-50 grid gap-4 border border-white/40 bg-white dark:bg-zinc-950 backdrop-blur-xl p-6 shadow-2xl duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:slide-out-to-bottom-[48%] data-[state=open]:slide-in-from-bottom-[48%] rounded-3xl">
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
                @click="saveSettings({ ...settings, theme: 'light' })"
              >
                <Sun class="w-4 h-4 mr-2" />
                {{ $t('settings.themeLight') }}
              </Button>
              <Button
                :variant="settings.theme === 'dark' ? 'default' : 'outline'"
                class="flex-1"
                @click="saveSettings({ ...settings, theme: 'dark' })"
              >
                <Moon class="w-4 h-4 mr-2" />
                {{ $t('settings.themeDark') }}
              </Button>
              <Button
                :variant="settings.theme === 'system' ? 'default' : 'outline'"
                class="flex-1"
                @click="saveSettings({ ...settings, theme: 'system' })"
              >
                <Monitor class="w-4 h-4 mr-2" />
                {{ $t('settings.themeSystem') }}
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

          <!-- 检查更新 -->
          <div>
            <label class="text-sm font-medium mb-3 block">{{ $t('settings.checkUpdate') }}</label>
            <div class="relative w-full">
              <Button
                variant="outline"
                class="w-full relative overflow-hidden"
                @click="handleCheckUpdate"
                :disabled="isChecking || isUpdating"
              >
                <!-- 进度条背景 -->
                <div 
                  v-if="isUpdating && downloadProgress > 0"
                  class="absolute left-0 top-0 bottom-0 bg-primary/10 transition-all duration-300 ease-out"
                  :style="{ width: `${downloadProgress}%` }"
                ></div>
                
                <div class="relative flex items-center justify-center z-10 w-full">
                  <RefreshCw :class="['w-4 h-4 mr-2', (isChecking || (isUpdating && downloadProgress === 0)) ? 'animate-spin' : '']" />
                  <span v-if="isUpdating">
                    {{ downloadProgress > 0 ? `${$t('settings.updating')} ${downloadProgress}%` : $t('settings.updating') }}
                  </span>
                  <span v-else>
                    {{ isChecking ? $t('settings.checking') : $t('settings.checkUpdate') }}
                  </span>
                </div>
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
