<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'

interface TabOption {
  value: string
  label: string
}

const props = defineProps<{
  modelValue: string
  options: TabOption[]
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const containerRef = ref<HTMLElement | null>(null)

// 滑块位置 (0-1)
const sliderPosition = ref(0)

// 当前选中的索引
const activeIndex = computed(() => {
  return props.options.findIndex(opt => opt.value === props.modelValue)
})

// 每个选项的宽度比例
const optionWidth = computed(() => 1 / props.options.length)

// 计算滑块的实际位置
const computedSliderPosition = computed(() => {
  const idx = activeIndex.value >= 0 ? activeIndex.value : 0
  return idx * optionWidth.value
})

// 滑块样式
const sliderStyle = computed(() => {
  const widthPercent = 100 / props.options.length
  // 计算滑块左边距（考虑 padding）
  const position = computedSliderPosition.value / optionWidth.value
  return {
    width: `calc(${widthPercent}% - 2px)`,
    left: `calc(${position * widthPercent}% + 2px)`,
    transform: 'none',
  }
})

// 监听 modelValue 变化，更新滑块位置
watch(() => props.modelValue, () => {
  sliderPosition.value = activeIndex.value * optionWidth.value
}, { immediate: true })



// 处理点击
const handleClick = (value: string) => {
  emit('update:modelValue', value)
}

// 初始化位置
onMounted(() => {
  sliderPosition.value = activeIndex.value * optionWidth.value
})
</script>

<template>
  <div
    :ref="(el) => containerRef = el as HTMLElement"
    class="liquid-glass-tabs relative inline-flex items-center bg-white dark:bg-black/60 backdrop-blur-xl border border-black/10 dark:border-white/20 rounded-full p-0.5 h-7 overflow-hidden shadow-[inset_0_1px_3px_rgba(0,0,0,0.1),inset_0_0_0_1px_rgba(0,0,0,0.05)] dark:shadow-[inset_0_1px_3px_rgba(255,255,255,0.05)] cursor-pointer"
  >
    <!-- 液态玻璃滑块 -->
    <div
      class="absolute top-0.5 bottom-0.5 rounded-full transition-all ease-out pointer-events-none duration-300"
      :style="sliderStyle"
    >
      <!-- 玻璃效果层 - 透明 + 细边框 -->
      <div class="absolute inset-0 rounded-full bg-transparent border border-black/30 dark:border-white/40 shadow-[0_1px_2px_rgba(0,0,0,0.1)] dark:shadow-[0_1px_2px_rgba(255,255,255,0.1)]">
      </div>
    </div>
    
    <!-- 选项文字 -->
    <div 
      v-for="option in options" 
      :key="option.value"
      class="relative z-10 flex-1 flex items-center justify-center h-full px-2 text-xs font-bold transition-colors duration-200 select-none"
      :class="[
        modelValue === option.value 
          ? 'text-black dark:text-white font-semibold' 
          : 'text-black/40 dark:text-white/40 hover:text-black/70 dark:hover:text-white/70'
      ]"
      @click="handleClick(option.value)"
    >
      {{ option.label }}
    </div>
  </div>
</template>

<style scoped>
.liquid-glass-tabs {
  /* 确保圆角效果正确 */
  -webkit-backdrop-filter: blur(24px);
}
</style>
