<script setup lang="ts">
import { ref, watch, onBeforeUnmount, nextTick } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import { Bold, Italic, List, ListOrdered, Quote, Code, Heading1, Heading2 } from 'lucide-vue-next'
import Button from '@/components/ui/Button.vue'

const scrollPositions = new Map<string, number>()

const props = defineProps<{
  modelValue: string
  placeholder?: string
  scrollKey?: string
}>()

const emitValue = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const scrollContainerRef = ref<HTMLElement | null>(null)

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
      class: 'rich-editor-prosemirror w-full h-full focus:outline-none min-h-[260px] text-[15px] leading-7 text-foreground',
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

const saveScrollPosition = (key?: string) => {
  if (!key || !scrollContainerRef.value) return
  scrollPositions.set(key, scrollContainerRef.value.scrollTop)
}

const restoreScrollPosition = async (key?: string) => {
  if (!key || !scrollContainerRef.value) return
  await nextTick()
  scrollContainerRef.value.scrollTop = scrollPositions.get(key) ?? 0
}

watch(() => props.scrollKey, async (newKey, oldKey) => {
  saveScrollPosition(oldKey)
  await restoreScrollPosition(newKey)
})

onBeforeUnmount(() => {
  saveScrollPosition(props.scrollKey)
  editor.value?.destroy()
})


</script>

<template>
  <div class="relative flex h-[420px] w-full flex-col border border-black/[0.06] dark:border-white/24 rounded-[18px] bg-white/50 dark:bg-black/[0.72] backdrop-blur-xl transition-all duration-200 focus-within:border-black/20 focus-within:ring-2 focus-within:ring-black/10 dark:focus-within:border-white/40 dark:focus-within:ring-white/15 overflow-hidden shadow-[0_10px_24px_rgba(15,23,42,0.05)] dark:shadow-[inset_0_0_0_1px_rgba(255,255,255,0.10)]">
    <div v-if="editor" class="flex flex-wrap items-center gap-1.5 px-3 py-2.5 border-b border-black/[0.05] dark:border-white/20 bg-white/55 dark:bg-black/[0.84]">
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleBold().run()"
        :class="{ 'bg-black/8 text-foreground dark:bg-white/12 dark:text-white': editor.isActive('bold') }"
      >
        <Bold class="w-4 h-4" />
      </Button>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleItalic().run()"
        :class="{ 'bg-black/8 text-foreground dark:bg-white/12 dark:text-white': editor.isActive('italic') }"
      >
        <Italic class="w-4 h-4" />
      </Button>
      <div class="w-px h-5 bg-black/8 dark:bg-white/8 mx-0.5"></div>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
        :class="{ 'bg-black/8 text-foreground dark:bg-white/12 dark:text-white': editor.isActive('heading', { level: 1 }) }"
      >
        <Heading1 class="w-4 h-4" />
      </Button>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
        :class="{ 'bg-black/8 text-foreground dark:bg-white/12 dark:text-white': editor.isActive('heading', { level: 2 }) }"
      >
        <Heading2 class="w-4 h-4" />
      </Button>
      <div class="w-px h-5 bg-black/8 dark:bg-white/8 mx-0.5"></div>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleBulletList().run()"
        :class="{ 'rounded-md border border-black/15 bg-transparent text-foreground dark:border-white/20 dark:text-white': editor.isActive('bulletList') }"
      >
        <List class="w-4 h-4" />
      </Button>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleOrderedList().run()"
        :class="{ 'rounded-md border border-black/15 bg-transparent text-foreground dark:border-white/20 dark:text-white': editor.isActive('orderedList') }"
      >
        <ListOrdered class="w-4 h-4" />
      </Button>
      <div class="w-px h-5 bg-black/8 dark:bg-white/8 mx-0.5"></div>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleBlockquote().run()"
        :class="{ 'bg-black/8 text-foreground dark:bg-white/12 dark:text-white': editor.isActive('blockquote') }"
      >
        <Quote class="w-4 h-4" />
      </Button>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        class="h-8 w-8 rounded-full text-foreground/75"
        @mousedown.prevent
        @click="editor.chain().focus().toggleCodeBlock().run()"
        :class="{ 'bg-black/8 text-foreground dark:bg-white/12 dark:text-white': editor.isActive('codeBlock') }"
      >
        <Code class="w-4 h-4" />
      </Button>
    </div>

    <div ref="scrollContainerRef" class="rich-editor-content relative flex-1 overflow-y-auto p-5 cursor-text" @click="editor?.commands.focus()" @scroll="saveScrollPosition(props.scrollKey)">
      <EditorContent :editor="editor" />
    </div>
  </div>
</template>

<style>
.rich-editor-content .rich-editor-prosemirror {
  min-height: 100%;
}

.rich-editor-content .rich-editor-prosemirror p {
  margin: 0 0 0.85rem;
}

.rich-editor-content .rich-editor-prosemirror blockquote {
  margin: 1rem 0;
  padding: 0.75rem 1rem;
  border-left: 3px solid rgba(15, 23, 42, 0.18);
  border-radius: 0 14px 14px 0;
  background: rgba(15, 23, 42, 0.04);
  color: inherit;
}

.rich-editor-content .rich-editor-prosemirror pre {
  margin: 1rem 0;
  padding: 0.9rem 1rem;
  border: 1px solid rgba(15, 23, 42, 0.08);
  border-radius: 14px;
  background: rgba(15, 23, 42, 0.05);
  white-space: pre-wrap;
  word-break: break-word;
}

.rich-editor-content .rich-editor-prosemirror pre code {
  background: transparent;
  padding: 0;
  font-size: 0.85rem;
  color: inherit;
}

.rich-editor-content .rich-editor-prosemirror code {
  border-radius: 8px;
  background: rgba(15, 23, 42, 0.06);
  padding: 0.12rem 0.38rem;
  font-size: 0.85em;
}

.rich-editor-content .rich-editor-prosemirror strong {
  font-weight: 700;
}

.rich-editor-content .rich-editor-prosemirror em {
  font-style: italic;
}

.rich-editor-content .rich-editor-prosemirror h1 {
  margin: 1rem 0 0.75rem;
  font-size: 1.4rem;
  font-weight: 700;
  line-height: 1.25;
}

.rich-editor-content .rich-editor-prosemirror h2 {
  margin: 0.9rem 0 0.65rem;
  font-size: 1.15rem;
  font-weight: 700;
  line-height: 1.3;
}

.rich-editor-content .rich-editor-prosemirror ul,
.rich-editor-content .rich-editor-prosemirror ol {
  margin: 1rem 0;
  margin-left: 0 !important;
  padding-left: 0 !important;
  list-style: none !important;
}

.rich-editor-content .rich-editor-prosemirror ol {
  counter-reset: rich-editor-item;
}

.rich-editor-content .rich-editor-prosemirror ul > li,
.rich-editor-content .rich-editor-prosemirror ol > li {
  position: relative;
  margin: 0.35rem 0;
  padding-left: 1.9rem !important;
  list-style: none !important;
  min-height: 1.5rem;
  display: block;
}

.rich-editor-content .rich-editor-prosemirror ul > li::marker,
.rich-editor-content .rich-editor-prosemirror ol > li::marker {
  content: '' !important;
  color: transparent !important;
  font-size: 0 !important;
}

.rich-editor-content .rich-editor-prosemirror ul > li::before {
  content: '•';
  position: absolute;
  left: 0.45rem;
  top: 0.02rem;
  background: none !important;
  border: 0 !important;
  border-radius: 0 !important;
  box-shadow: none !important;
  padding: 0 !important;
  color: rgba(15, 23, 42, 0.76);
  font-weight: 700;
  line-height: 1;
}

.rich-editor-content .rich-editor-prosemirror ol > li {
  counter-increment: rich-editor-item;
}

.rich-editor-content .rich-editor-prosemirror ol > li::before {
  content: counter(rich-editor-item) '.';
  position: absolute;
  left: 0;
  top: 0.02rem;
  width: 1.35rem;
  background: none !important;
  border: 0 !important;
  border-radius: 0 !important;
  box-shadow: none !important;
  padding: 0 !important;
  color: rgba(15, 23, 42, 0.76);
  font-weight: 700;
  line-height: 1;
  text-align: right;
}

.rich-editor-content .rich-editor-prosemirror li > p {
  margin: 0;
}

.dark .rich-editor-content .rich-editor-prosemirror blockquote {
  border-left-color: rgba(255, 255, 255, 0.28);
  background: rgba(255, 255, 255, 0.06);
}

.dark .rich-editor-content .rich-editor-prosemirror pre {
  border-color: rgba(255, 255, 255, 0.16);
  background: rgba(255, 255, 255, 0.05);
}

.dark .rich-editor-content .rich-editor-prosemirror code {
  background: rgba(255, 255, 255, 0.08);
}

.dark .rich-editor-content .rich-editor-prosemirror ul > li::before,
.dark .rich-editor-content .rich-editor-prosemirror ol > li::before {
  color: rgba(255, 255, 255, 0.94);
}
</style>
