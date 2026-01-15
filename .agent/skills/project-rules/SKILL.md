---
name: project-rules
description: AI 开发项目规范指南 - 包含项目架构说明和 UI 设计规范
---

# AI 开发项目规范指南

本文档定义了 AI 在开发此 Mac 状态栏应用 (Silto) 时需要遵循的规则和规范。

---

## 一、项目架构

### 1.1 技术栈

| 技术 | 说明 |
|------|------|
| **核心框架** | Tauri 2.0 (Rust + Webview) |
| **前端框架** | Vue 3 + TypeScript + Vite |
| **UI 框架** | Tailwind CSS + shadcn-vue |
| **图标库** | lucide-vue-next |
| **状态管理** | Vue Composition API (composables) |
| **数据持久化** | tauri-plugin-store (store.json) |
| **包管理器** | pnpm |

### 1.2 目录结构

```
silto/
├── src-tauri/                 # 后端 (Rust)
│   ├── src/
│   │   ├── models.rs          # 数据模型定义
│   │   ├── commands.rs        # Tauri 命令 (API)
│   │   ├── tray.rs            # 系统托盘逻辑
│   │   └── notification.rs    # 提醒系统
│   ├── tauri.conf.json        # Tauri 配置文件
│   └── Cargo.toml             # Rust 依赖
├── src/                       # 前端 (Vue)
│   ├── components/            # Vue 组件
│   │   └── ui/                # shadcn-vue 基础组件
│   ├── composables/           # 组合式函数 (逻辑复用)
│   ├── types/                 # TypeScript 类型定义
│   ├── lib/                   # 工具函数
│   ├── styles/                # 全局样式
│   ├── App.vue                # 根组件
│   └── main.ts                # 入口文件
├── .cursorrules               # AI 行为详细规范
└── tailwind.config.js         # Tailwind 配置
```

### 1.3 核心数据流

1.  **组件调用**: 前端组件不直接调用 API，而是通过 `composables/*`。
2.  **API 通信**: `Usage: invoke('command_name', { args })`。
3.  **事件驱动**: 后端通过 `emit` 推送事件 (如托盘点击)，前端在 `App.vue` 或 composables 中 `listen`。
4.  **存储**: 数据统一存储在 `~/.local/share/com.smile.silto/store.json`。

---

## 二、UI 设计规范：液态玻璃 (Liquid Glass)

本项目采用独特的"液态玻璃"设计风格，追求通透、流动且高级的视觉体验。

### 2.1 核心设计理念

*   **通透感**: 使用高模糊度和半透明背景。
*   **流动感**: 柔和的边框光泽和渐变。
*   **沉浸感**: 无边框窗口，深度融合系统环境。

### 2.2 Tailwind 样式预设

在编写组件时，优先使用以下 Tailwind 类组合来实现设计效果：

#### 玻璃容器 (Glass Container)
适用于卡片、弹窗或主要内容区域：
```html
<div class="bg-background/80 backdrop-blur-xl border border-white/20 shadow-lg ...">
```
*   **背景**: `bg-background/80` (配合 CSS 变量)
*   **模糊**: `backdrop-blur-xl` 或 `backdrop-blur-2xl`
*   **边框**: `border-white/20` (亮色) 或 `border-white/10` (深色)

#### 液态高光 (Liquid Highlight)
用于增加质感的内部高光或 hover 效果：
```html
<div class="hover:bg-white/10 transition-colors duration-300 ...">
```

#### 圆角系统
*   **窗口/主面板**: `rounded-xl` 或 `rounded-2xl`
*   **按钮/输入框**: `rounded-lg`
*   **标签/小元素**: `rounded-md`

### 2.3 颜色规范 (CSS Variables)

主题色定义在 `src/styles/index.css`，支持深色/浅色模式切换。

| 变量 | 说明 | 对应 Tailwind 类 |
|------|------|------------------|
| `--background` | 玻璃背景色 | `bg-background` |
| `--foreground` | 主要文字 | `text-foreground` |
| `--primary` | 主题色 | `bg-primary`, `text-primary` |
| `--muted` | 次要文字/背景 | `text-muted-foreground` |
| `--border` | 边框颜色 | `border-border` |

### 2.4 字体与排版
*   **字体**: 使用系统默认无衬线字体，保持原生感。
*   **字号**: 状态栏应用字号偏小，常用的有 `text-xs`, `text-sm`。

---

## 三、开发规范

### 3.1 代码风格
*   **Vue**: 使用 `<script setup lang="ts">`。
*   **TypeScript**: 保持强类型，所有数据模型需在 `src/types/index.ts` 定义接口，并与 Rust `models.rs` 对应。
*   **Rust**: 遵循 Rust 官方风格指南，使用 `cargo fmt` 格式化。

### 3.2 组件开发
1.  **原子化**: 简单的 UI 元素尽量复用 `src/components/ui/` 下的组件。
2.  **样式优先**: 优先使用 Tailwind Utility Classes，避免写行内 style 或单独的 css 文件。
3.  **国际化**: 所有用户可见文本需支持 i18n (虽然目前可能未完全实装，保留扩展性)。

### 3.3 状态管理
*   避免使用 Pinia 等复杂库，优先使用 Composition API (`ref`, `reactive`) 配合 `composables` 封装状态。
*   复杂全局状态可使用 Vue 的 `provide/inject`。

### 3.4 错误处理
*   Tauri 命令调用需使用 `.catch()` 或 `try/catch` 处理错误。
*   UI 上需给予用户适当的反馈 (如 Toast 提示)。

---

## 四、常用开发命令

```bash
# 启动开发服务器 (前端 + 后端)
pnpm tauri dev

# 仅构建前端
pnpm build

# 构建生产版本应用
pnpm tauri build

# 运行类型检查
pnpm vue-tsc --noEmit

# 快速修复 Lint 问题
pnpm eslint --fix
```

## 五、注意事项

> [!WARNING]
> 修改 `src-tauri/tauri.conf.json` 中的窗口配置可能会导致应用无法唤起或显示异常，请谨慎修改。

> [!IMPORTANT]
> 涉及数据结构的修改，必须同时更新 Frontend (`src/types`) 和 Backend (`src-tauri/src/models.rs`)，否则会导致序列化错误。
