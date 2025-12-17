<script setup lang="ts">
import { watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import { Maximize2 } from 'lucide-vue-next'


const props = defineProps<{
  modelValue: string
  placeholder?: string
  allowExpand?: boolean
}>()

const emitValue = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'expand'): void
}>()

const editor = useEditor({
  content: props.modelValue,
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: props.placeholder,
    }),
  ],
  editorProps: {
    attributes: {
      class: 'prose prose-sm dark:prose-invert max-w-none w-full h-full focus:outline-none min-h-[100px]',
    },
  },
  onUpdate: ({ editor }: { editor: any }) => {
    emitValue('update:modelValue', editor.getHTML())
  },
})

watch(() => props.modelValue, (newVal) => {
  if (editor.value && editor.value.getHTML() !== newVal) {
    editor.value.commands.setContent(newVal, { emitUpdate: false })
  }
})

onBeforeUnmount(() => {
  editor.value?.destroy()
})

const handleExpand = () => {
  emitValue('expand')
}
</script>

<template>
  <div class="relative w-full border border-input rounded-md bg-transparent transition-colors focus-within:ring-1 focus-within:ring-ring">
    <!-- Compact Editor -->
    <div class="relative min-h-[120px] p-3">
      <EditorContent :editor="editor" />
      
      <!-- Expand Button -->
      <button 
        v-if="allowExpand !== false"
        class="absolute top-2 right-2 p-1.5 rounded-md text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
        @click="handleExpand"
        type="button"
        title="独立窗口编辑"
      >
        <Maximize2 class="w-4 h-4" />
      </button>
    </div>
  </div>
</template>
