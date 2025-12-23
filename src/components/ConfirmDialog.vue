<script setup lang="ts">
import { DialogRoot, DialogOverlay, DialogContent, DialogTitle, DialogDescription } from 'radix-vue'
import Button from '@/components/ui/Button.vue'
import { useI18n } from 'vue-i18n'

defineProps<{
  open: boolean
  title: string
  description?: string
  confirmText?: string
  cancelText?: string
  variant?: 'default' | 'destructive'
  compact?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'confirm'): void
  (e: 'cancel'): void
}>()

const { t } = useI18n()

const handleConfirm = () => {
  emit('confirm')
  emit('update:open', false)
}

const handleCancel = () => {
  emit('cancel')
  emit('update:open', false)
}
</script>

<template>
  <DialogRoot :open="open" @update:open="emit('update:open', $event)">
    <DialogOverlay class="absolute inset-0 z-50 bg-black/20 backdrop-blur-sm data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 rounded-2xl" />
    <DialogContent 
      class="absolute left-[50%] top-[50%] z-50 grid w-full translate-x-[-50%] translate-y-[-50%] gap-4 border border-white/20 dark:border-white/10 bg-white/10 dark:bg-zinc-950/10 backdrop-blur-xl backdrop-saturate-150 shadow-lg duration-200 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[state=closed]:slide-out-to-left-1/2 data-[state=closed]:slide-out-to-top-[48%] data-[state=open]:slide-in-from-left-1/2 data-[state=open]:slide-in-from-top-[48%] rounded-3xl md:w-full"
      :class="[
        compact ? 'max-w-[260px] p-4 text-sm' : 'max-w-[320px] p-6'
      ]"
    >
      <div class="flex flex-col space-y-2 text-center sm:text-left">
        <DialogTitle 
          class="font-semibold leading-none tracking-tight"
          :class="[compact ? 'text-base' : 'text-lg']"
        >
          {{ title }}
        </DialogTitle>
        <DialogDescription 
          v-if="description" 
          class="text-muted-foreground"
          :class="[compact ? 'text-xs' : 'text-sm']"
        >
          {{ description }}
        </DialogDescription>
      </div>

      <div class="grid grid-cols-2 gap-3">
        <Button variant="outline" @click="handleCancel" :size="compact ? 'sm' : 'default'">
          {{ cancelText || t('common.cancel') }}
        </Button>
        <Button :variant="variant || 'default'" @click="handleConfirm" :size="compact ? 'sm' : 'default'">
          {{ confirmText || t('common.confirm') }}
        </Button>
      </div>
    </DialogContent>
  </DialogRoot>
</template>
