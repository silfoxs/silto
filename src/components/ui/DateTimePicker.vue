<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { PopoverRoot, PopoverTrigger, PopoverContent, PopoverPortal } from 'radix-vue'
import { Calendar as CalendarIcon, ChevronLeft, ChevronRight, Clock } from 'lucide-vue-next'
import { cn } from '@/lib/utils'
import Button from '@/components/ui/Button.vue'

const props = defineProps<{
  modelValue?: string | Date | null
  placeholder?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | null): void
}>()

const isOpen = ref(false)
const viewingDate = ref(new Date())

// Initialize viewing date
watch(() => props.modelValue, (val) => {
  if (val) {
    viewingDate.value = new Date(val)
  }
}, { immediate: true })

const selectedDate = computed(() => {
  if (!props.modelValue) return null
  return new Date(props.modelValue)
})

const daysOfWeek = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa']

const calendarDays = computed(() => {
  const year = viewingDate.value.getFullYear()
  const month = viewingDate.value.getMonth()
  
  const firstDayOfMonth = new Date(year, month, 1)
  const lastDayOfMonth = new Date(year, month + 1, 0)
  
  const days = []
  
  // Padding for start of week
  for (let i = 0; i < firstDayOfMonth.getDay(); i++) {
    days.push(null)
  }
  
  // Real days
  for (let i = 1; i <= lastDayOfMonth.getDate(); i++) {
    days.push(new Date(year, month, i))
  }
  
  return days
})

const currentMonthLabel = computed(() => {
  return viewingDate.value.toLocaleString('default', { month: 'long', year: 'numeric' })
})

const timeValue = computed({
  get: () => {
    if (!selectedDate.value) return ''
    return selectedDate.value.toTimeString().slice(0, 5)
  },
  set: (val: string) => {
    if (!val) return
    const [hours, minutes] = val.split(':').map(Number)
    const newDate = selectedDate.value ? new Date(selectedDate.value) : new Date()
    newDate.setHours(hours)
    newDate.setMinutes(minutes)
    emit('update:modelValue', newDate.toISOString())
  }
})

const moveMonth = (delta: number) => {
  const newDate = new Date(viewingDate.value)
  newDate.setMonth(newDate.getMonth() + delta)
  viewingDate.value = newDate
}

const selectDate = (date: Date) => {
  const newDate = new Date(date)
  if (selectedDate.value) {
    // Preserve time
    newDate.setHours(selectedDate.value.getHours())
    newDate.setMinutes(selectedDate.value.getMinutes())
  } else {
    // Default to now time or 09:00? Default to current time implies ease.
    const now = new Date()
    newDate.setHours(now.getHours())
    newDate.setMinutes(now.getMinutes())
  }
  emit('update:modelValue', newDate.toISOString())
}

const isSelected = (date: Date) => {
  if (!selectedDate.value) return false
  return date.getDate() === selectedDate.value.getDate() &&
         date.getMonth() === selectedDate.value.getMonth() &&
         date.getFullYear() === selectedDate.value.getFullYear()
}

const isToday = (date: Date) => {
  const today = new Date()
  return date.getDate() === today.getDate() &&
         date.getMonth() === today.getMonth() &&
         date.getFullYear() === today.getFullYear()
}

const formatDisplay = (dateStr: string | Date) => {
  const d = new Date(dateStr)
  return d.toLocaleString('default', { 
    month: 'short', 
    day: 'numeric', 
    hour: 'numeric', 
    minute: '2-digit' 
  })
}

const clear = () => {
  emit('update:modelValue', null)
  isOpen.value = false
}
</script>

<template>
  <PopoverRoot v-model:open="isOpen">
    <PopoverTrigger as-child>
      <Button
        variant="outline"
        :class="cn(
          'w-full justify-start text-left font-normal',
          !modelValue && 'text-muted-foreground'
        )"
      >
        <CalendarIcon class="mr-2 h-4 w-4" />
        {{ modelValue ? formatDisplay(modelValue) : (placeholder || 'Pick a date') }}
      </Button>
    </PopoverTrigger>
    <PopoverPortal>
      <PopoverContent class="w-auto p-0 bg-popover text-popover-foreground rounded-md border shadow-md z-[99999]" align="start">
        <div class="p-3">
          <!-- Header -->
          <div class="flex items-center justify-between mb-4">
            <Button variant="ghost" size="icon" class="h-7 w-7" @click="moveMonth(-1)">
              <ChevronLeft class="h-4 w-4" />
            </Button>
            <div class="font-medium text-sm">{{ currentMonthLabel }}</div>
            <Button variant="ghost" size="icon" class="h-7 w-7" @click="moveMonth(1)">
              <ChevronRight class="h-4 w-4" />
            </Button>
          </div>

          <!-- Calendar Grid -->
          <div class="grid grid-cols-7 gap-1 text-center text-xs mb-2">
            <div v-for="day in daysOfWeek" :key="day" class="text-muted-foreground font-medium w-8">
              {{ day }}
            </div>
          </div>
          <div class="grid grid-cols-7 gap-1">
            <div 
              v-for="(date, index) in calendarDays" 
              :key="index"
              class="w-full flex justify-center"
            >
              <Button
                v-if="date"
                variant="ghost"
                size="icon"
                :class="cn(
                  'h-8 w-8 p-0 font-normal hover:bg-muted focus:bg-muted',
                  isSelected(date) && 'bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground focus:bg-primary focus:text-primary-foreground',
                  isToday(date) && !isSelected(date) && 'text-primary font-bold bg-accent/20'
                )"
                @click="selectDate(date)"
              >
                {{ date.getDate() }}
              </Button>
              <div v-else class="h-8 w-8"></div>
            </div>
          </div>

          <!-- Time & Footer -->
          <div class="border-t mt-4 pt-3 flex items-center justify-between gap-2">
            <div class="flex items-center gap-2 flex-1">
              <Clock class="h-4 w-4 text-muted-foreground" />
              <input 
                type="time"
                v-model="timeValue"
                class="flex h-8 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-black/20 dark:focus-visible:ring-white/30 disabled:cursor-not-allowed disabled:opacity-50"
              />
            </div>
            <Button variant="ghost" size="sm" class="h-8 px-2 text-xs text-muted-foreground hover:text-foreground" @click="clear">
              Clear
            </Button>
          </div>
        </div>
      </PopoverContent>
    </PopoverPortal>
  </PopoverRoot>
</template>
