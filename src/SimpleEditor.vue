<template>
  <div style="background: white; color: black; height: 100vh; padding: 20px; font-family: monospace;">
    <h1>SIMPLE EDITOR MODE</h1>
    <p>If you can see this, Vue is working.</p>
    <p>Current Time: {{ new Date().toISOString() }}</p>
    <p>URL: {{ url }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
const url = ref(window.location.href);
console.log('SimpleEditor mounted');
</script>
