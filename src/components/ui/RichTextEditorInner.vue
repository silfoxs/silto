<script setup lang="ts">
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import { Bold, Italic, List, ListOrdered, Quote, Code, Heading1, Heading2 } from 'lucide-vue-next'
import Button from '@/components/ui/Button.vue'
import { onBeforeUnmount, watch } from 'vue'

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const editor = useEditor({
  content: props.modelValue,
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: '输入内容...',
    }),
  ],
  editorProps: {
    attributes: {
      class: 'prose prose-sm dark:prose-invert max-w-none w-full h-full focus:outline-none min-h-[300px]',
    },
  },
  onUpdate: ({ editor }: { editor: any }) => {
    emit('update:modelValue', editor.getHTML())
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
  <div class="flex flex-col h-full bg-background">
    <!-- Toolbar -->
    <div v-if="editor" class="flex flex-wrap items-center gap-1 p-2 border-b bg-muted/10">
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleBold().run()"
        :class="{ 'bg-muted': editor.isActive('bold') }"
      >
        <Bold class="w-4 h-4" />
      </Button>
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleItalic().run()"
        :class="{ 'bg-muted': editor.isActive('italic') }"
      >
        <Italic class="w-4 h-4" />
      </Button>
      <div class="w-px h-6 bg-border mx-1"></div>
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
        :class="{ 'bg-muted': editor.isActive('heading', { level: 1 }) }"
      >
        <Heading1 class="w-4 h-4" />
      </Button>
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
        :class="{ 'bg-muted': editor.isActive('heading', { level: 2 }) }"
      >
        <Heading2 class="w-4 h-4" />
      </Button>
      <div class="w-px h-6 bg-border mx-1"></div>
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleBulletList().run()"
        :class="{ 'bg-muted': editor.isActive('bulletList') }"
      >
        <List class="w-4 h-4" />
      </Button>
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleOrderedList().run()"
        :class="{ 'bg-muted': editor.isActive('orderedList') }"
      >
        <ListOrdered class="w-4 h-4" />
      </Button>
      <div class="w-px h-6 bg-border mx-1"></div>
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleBlockquote().run()"
        :class="{ 'bg-muted': editor.isActive('blockquote') }"
      >
        <Quote class="w-4 h-4" />
      </Button>
      <Button 
        type="button"
        variant="ghost" 
        size="icon" 
        @mousedown.prevent
        @click="editor.chain().focus().toggleCodeBlock().run()"
        :class="{ 'bg-muted': editor.isActive('codeBlock') }"
      >
        <Code class="w-4 h-4" />
      </Button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-4 cursor-text" @click="editor?.commands.focus()">
      <EditorContent :editor="editor" />
    </div>
  </div>
</template>
