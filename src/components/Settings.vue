<script setup lang="ts">
import { computed, ref } from 'vue'
import { Moon, Sun, Monitor, Languages, RefreshCw, Sparkles, Palette, Globe2, Download } from 'lucide-vue-next'
import { check } from '@tauri-apps/plugin-updater'
import { ask, message } from '@tauri-apps/plugin-dialog'
import { relaunch } from '@tauri-apps/plugin-process'
import Button from '@/components/ui/Button.vue'
import { useSettings } from '@/composables/useSettings'
import { useI18n } from 'vue-i18n'

const { locale, t } = useI18n()
const { settings, saveSettings } = useSettings()

const selectedThemeLabel = computed(() => {
  if (settings.value.theme === 'light') return t('settings.themeLight')
  if (settings.value.theme === 'dark') return t('settings.themeDark')
  return t('settings.themeSystem')
})

const selectedLanguageLabel = computed(() => {
  return locale.value === 'zh-CN' ? t('locales.zh-CN') : t('locales.en-US')
})

const handleLanguageChange = async (lang: string) => {
  locale.value = lang
  localStorage.setItem('language', lang)

  await saveSettings({
    ...settings.value,
    language: lang,
  })
}

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
      const confirmed = await ask(`${t('settings.updateAvailable')} (${update.version})`, {
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

        await message(t('settings.updateCompleted', { version: update.version }), {
          title: t('settings.checkUpdate'),
          kind: 'info',
          okLabel: t('common.confirm'),
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
  <div class="h-full overflow-y-auto px-5 pb-5 pt-16">
    <div class="mx-auto flex h-full max-w-5xl flex-col">
      <div class="mb-4 grid gap-3 md:grid-cols-[minmax(0,1.2fr)_minmax(280px,0.8fr)]">
        <section class="rounded-[20px] border border-black/[0.06] bg-white/72 px-6 py-5 shadow-[0_16px_40px_rgba(15,23,42,0.08)] backdrop-blur-2xl dark:border-white/20 dark:bg-black/[0.58] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08),0_16px_40px_rgba(0,0,0,0.28)]">
          <div class="mb-3 inline-flex h-10 w-10 items-center justify-center rounded-2xl border border-black/[0.06] bg-white/70 text-foreground/80 dark:border-white/18 dark:bg-black/[0.7]">
            <Sparkles class="h-5 w-5" />
          </div>
          <h2 class="text-[28px] font-semibold tracking-tight text-foreground/92">{{ $t('settings.title') }}</h2>
          <p class="mt-2 max-w-xl text-sm leading-6 text-foreground/62">
            {{ $t('settings.pageIntro') }}
          </p>
        </section>

        <aside class="rounded-[20px] border border-black/[0.06] bg-white/72 px-5 py-5 shadow-[0_16px_40px_rgba(15,23,42,0.08)] backdrop-blur-2xl dark:border-white/20 dark:bg-black/[0.58] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08),0_16px_40px_rgba(0,0,0,0.28)]">
          <p class="text-xs font-medium uppercase tracking-[0.18em] text-foreground/42">{{ $t('settings.current') }}</p>
          <div class="mt-4 space-y-4">
            <div class="flex items-center justify-between rounded-[16px] border border-black/[0.05] bg-white/54 px-4 py-3 dark:border-white/16 dark:bg-black/[0.68]">
              <div class="flex items-center gap-3">
                <Palette class="h-4 w-4 text-foreground/60" />
                <span class="text-sm text-foreground/76">{{ $t('settings.theme') }}</span>
              </div>
              <span class="text-sm font-medium text-foreground/90">{{ selectedThemeLabel }}</span>
            </div>
            <div class="flex items-center justify-between rounded-[16px] border border-black/[0.05] bg-white/54 px-4 py-3 dark:border-white/16 dark:bg-black/[0.68]">
              <div class="flex items-center gap-3">
                <Globe2 class="h-4 w-4 text-foreground/60" />
                <span class="text-sm text-foreground/76">{{ $t('settings.language') }}</span>
              </div>
              <span class="text-sm font-medium text-foreground/90">{{ selectedLanguageLabel }}</span>
            </div>
          </div>
        </aside>
      </div>

      <div class="grid flex-1 gap-3 md:grid-cols-[minmax(0,1.2fr)_minmax(280px,0.8fr)]">
        <div class="space-y-3">
          <section class="rounded-[20px] border border-black/[0.06] bg-white/72 p-6 shadow-[0_16px_40px_rgba(15,23,42,0.08)] backdrop-blur-2xl dark:border-white/20 dark:bg-black/[0.58] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08),0_16px_40px_rgba(0,0,0,0.28)]">
            <div class="mb-5 flex items-start justify-between gap-4">
              <div>
                <p class="text-base font-semibold text-foreground/90">{{ $t('settings.theme') }}</p>
                <p class="mt-1 text-sm leading-6 text-foreground/60">{{ $t('settings.themeDesc') }}</p>
              </div>
              <div class="inline-flex h-10 w-10 items-center justify-center rounded-2xl border border-black/[0.06] bg-white/65 dark:border-white/18 dark:bg-black/[0.7]">
                <Palette class="h-5 w-5 text-foreground/72" />
              </div>
            </div>

            <div class="grid gap-3 md:grid-cols-3">
              <button
                type="button"
                class="rounded-[18px] border px-4 py-4 text-left transition-all duration-200"
                :class="settings.theme === 'light'
                  ? 'border-black/10 bg-white shadow-[0_14px_32px_rgba(15,23,42,0.08)] dark:border-white/24 dark:bg-black/[0.74]'
                  : 'border-black/[0.06] bg-white/54 hover:bg-white/72 dark:border-white/16 dark:bg-black/[0.68] dark:hover:bg-black/[0.76]'"
                @click="saveSettings({ ...settings, theme: 'light' })"
              >
                <Sun class="mb-4 h-5 w-5 text-foreground/72" />
                <p class="text-sm font-medium text-foreground/90">{{ $t('settings.themeLight') }}</p>
                <p class="mt-1 text-xs leading-5 text-foreground/58">{{ $t('settings.themeLightDesc') }}</p>
              </button>

              <button
                type="button"
                class="rounded-[18px] border px-4 py-4 text-left transition-all duration-200"
                :class="settings.theme === 'dark'
                  ? 'border-black/10 bg-white shadow-[0_14px_32px_rgba(15,23,42,0.08)] dark:border-white/24 dark:bg-black/[0.74]'
                  : 'border-black/[0.06] bg-white/54 hover:bg-white/72 dark:border-white/16 dark:bg-black/[0.68] dark:hover:bg-black/[0.76]'"
                @click="saveSettings({ ...settings, theme: 'dark' })"
              >
                <Moon class="mb-4 h-5 w-5 text-foreground/72" />
                <p class="text-sm font-medium text-foreground/90">{{ $t('settings.themeDark') }}</p>
                <p class="mt-1 text-xs leading-5 text-foreground/58">{{ $t('settings.themeDarkDesc') }}</p>
              </button>

              <button
                type="button"
                class="rounded-[18px] border px-4 py-4 text-left transition-all duration-200"
                :class="settings.theme === 'system'
                  ? 'border-black/10 bg-white shadow-[0_14px_32px_rgba(15,23,42,0.08)] dark:border-white/24 dark:bg-black/[0.74]'
                  : 'border-black/[0.06] bg-white/54 hover:bg-white/72 dark:border-white/16 dark:bg-black/[0.68] dark:hover:bg-black/[0.76]'"
                @click="saveSettings({ ...settings, theme: 'system' })"
              >
                <Monitor class="mb-4 h-5 w-5 text-foreground/72" />
                <p class="text-sm font-medium text-foreground/90">{{ $t('settings.themeSystem') }}</p>
                <p class="mt-1 text-xs leading-5 text-foreground/58">{{ $t('settings.themeSystemDesc') }}</p>
              </button>
            </div>
          </section>

          <section class="rounded-[20px] border border-black/[0.06] bg-white/72 p-6 shadow-[0_16px_40px_rgba(15,23,42,0.08)] backdrop-blur-2xl dark:border-white/20 dark:bg-black/[0.58] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08),0_16px_40px_rgba(0,0,0,0.28)]">
            <div class="mb-5 flex items-start justify-between gap-4">
              <div>
                <p class="text-base font-semibold text-foreground/90">{{ $t('settings.language') }}</p>
                <p class="mt-1 text-sm leading-6 text-foreground/60">{{ $t('settings.languageDesc') }}</p>
              </div>
              <div class="inline-flex h-10 w-10 items-center justify-center rounded-2xl border border-black/[0.06] bg-white/65 dark:border-white/18 dark:bg-black/[0.7]">
                <Languages class="h-5 w-5 text-foreground/72" />
              </div>
            </div>

            <div class="grid gap-3 md:grid-cols-2">
              <button
                type="button"
                class="flex items-center justify-between rounded-[18px] border px-4 py-4 text-left transition-all duration-200"
                :class="locale === 'zh-CN'
                  ? 'border-black/10 bg-white shadow-[0_14px_32px_rgba(15,23,42,0.08)] dark:border-white/24 dark:bg-black/[0.74]'
                  : 'border-black/[0.06] bg-white/54 hover:bg-white/72 dark:border-white/16 dark:bg-black/[0.68] dark:hover:bg-black/[0.76]'"
                @click="handleLanguageChange('zh-CN')"
              >
                <div>
                  <p class="text-sm font-medium text-foreground/90">{{ $t('locales.zh-CN') }}</p>
                  <p class="mt-1 text-xs text-foreground/58">{{ $t('settings.languageZhDesc') }}</p>
                </div>
                <span class="text-xs font-medium text-foreground/48">ZH</span>
              </button>

              <button
                type="button"
                class="flex items-center justify-between rounded-[18px] border px-4 py-4 text-left transition-all duration-200"
                :class="locale === 'en-US'
                  ? 'border-black/10 bg-white shadow-[0_14px_32px_rgba(15,23,42,0.08)] dark:border-white/24 dark:bg-black/[0.74]'
                  : 'border-black/[0.06] bg-white/54 hover:bg-white/72 dark:border-white/16 dark:bg-black/[0.68] dark:hover:bg-black/[0.76]'"
                @click="handleLanguageChange('en-US')"
              >
                <div>
                  <p class="text-sm font-medium text-foreground/90">{{ $t('locales.en-US') }}</p>
                  <p class="mt-1 text-xs text-foreground/58">{{ $t('settings.languageEnDesc') }}</p>
                </div>
                <span class="text-xs font-medium text-foreground/48">EN</span>
              </button>
            </div>
          </section>
        </div>

        <aside class="space-y-3">
          <section class="rounded-[20px] border border-black/[0.06] bg-white/72 p-6 shadow-[0_16px_40px_rgba(15,23,42,0.08)] backdrop-blur-2xl dark:border-white/20 dark:bg-black/[0.58] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08),0_16px_40px_rgba(0,0,0,0.28)]">
            <div class="mb-5 flex items-start justify-between gap-4">
              <div>
                <p class="text-base font-semibold text-foreground/90">{{ $t('settings.checkUpdate') }}</p>
                <p class="mt-1 text-sm leading-6 text-foreground/60">{{ $t('settings.updateDesc') }}</p>
              </div>
              <div class="inline-flex h-10 w-10 items-center justify-center rounded-2xl border border-black/[0.06] bg-white/65 dark:border-white/18 dark:bg-black/[0.7]">
                <Download class="h-5 w-5 text-foreground/72" />
              </div>
            </div>

            <div class="relative">
              <Button
                variant="outline"
                class="relative h-12 w-full overflow-hidden rounded-[16px] border-black/[0.06] bg-white/56 text-foreground shadow-[0_12px_30px_rgba(15,23,42,0.08)] hover:bg-white/76 dark:border-white/18 dark:bg-black/[0.7] dark:hover:bg-black/[0.8]"
                @click="handleCheckUpdate"
                :disabled="isChecking || isUpdating"
              >
                <div
                  v-if="isUpdating && downloadProgress > 0"
                  class="absolute inset-y-0 left-0 bg-primary/10 transition-all duration-300 ease-out"
                  :style="{ width: `${downloadProgress}%` }"
                ></div>

                <div class="relative z-10 flex w-full items-center justify-center">
                  <RefreshCw :class="['mr-2 h-4 w-4', (isChecking || (isUpdating && downloadProgress === 0)) ? 'animate-spin' : '']" />
                  <span v-if="isUpdating">
                    {{ downloadProgress > 0 ? `${$t('settings.updating')} ${downloadProgress}%` : $t('settings.updating') }}
                  </span>
                  <span v-else>
                    {{ isChecking ? $t('settings.checking') : $t('settings.checkUpdate') }}
                  </span>
                </div>
              </Button>
            </div>
          </section>

          <section class="rounded-[20px] border border-black/[0.06] bg-white/72 p-6 shadow-[0_16px_40px_rgba(15,23,42,0.08)] backdrop-blur-2xl dark:border-white/20 dark:bg-black/[0.58] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.08),0_16px_40px_rgba(0,0,0,0.28)]">
            <p class="text-xs font-medium uppercase tracking-[0.18em] text-foreground/42">{{ $t('settings.tips') }}</p>
            <div class="mt-4 space-y-3 text-sm leading-6 text-foreground/62">
              <p>{{ $t('settings.tipTheme') }}</p>
              <p>{{ $t('settings.tipLanguage') }}</p>
              <p>{{ $t('settings.tipUpdate') }}</p>
            </div>
          </section>
        </aside>
      </div>
    </div>
  </div>
</template>
