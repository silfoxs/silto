# Silto 项目架构

这是一个 Mac 状态栏 Todo & 便签应用。

## 技术栈
- **后端**: Tauri 2.x + Rust
- **前端**: Vue 3 + TypeScript + Vite
- **UI**: Tailwind CSS + shadcn-vue
- **状态管理**: Vue Composition API
- **数据存储**: Tauri Store Plugin

## 项目结构

### 后端 (Rust)
- `src-tauri/src/models.rs` - 数据模型
- `src-tauri/src/commands.rs` - Tauri 命令 (API)
- `src-tauri/src/tray.rs` - 系统托盘
- `src-tauri/src/notification.rs` - 提醒系统

### 前端 (Vue)
- `src/components/` - Vue 组件
- `src/composables/` - 组合式函数 (数据管理)
- `src/types/` - TypeScript 类型
- `src/lib/` - 工具函数

## 核心模式

### 数据流
前端组件 → Composable → Tauri Command → Rust Backend → Store → 返回数据

### 事件系统
Rust (emit) → Vue (listen) 用于托盘事件

### 主题系统
CSS 变量 + `.dark` 类 + `useSettings` composable

## 开发规范
- 组件使用 `<script setup>` 和 TypeScript
- 样式使用 Tailwind CSS 类名
- 导入使用 `@/` 别名
- Rust 和 TypeScript 类型保持一致

## 数据存储
所有数据存储在 `store.json`:
```json
{
  "todos": [],
  "notes": [],
  "settings": {}
}
```

查看 `.cursorrules` 了解完整架构说明。
