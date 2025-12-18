<script setup lang="ts">
import { watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'

const props = defineProps<{
  modelValue: string
  placeholder?: string
}>()

const emitValue = defineEmits<{
  (e: 'update:modelValue', value: string): void
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


</script>

<template>
  <div class="relative w-full border border-input rounded-md bg-transparent transition-colors focus-within:ring-1 focus-within:ring-black/20 dark:focus-within:ring-white/30">
    <!-- Compact Editor -->
    <div class="relative min-h-[120px] p-3">
      <EditorContent :editor="editor" />
      

    </div>
  </div>
</template>
