# Silto

Silto 是一个面向 macOS 的菜单栏 Todo / 便签应用，基于 Tauri 2、Rust、Vue 3 和 TipTap 构建。

它包含两套核心界面：

- 主窗口：宽屏双栏布局，左侧列表，右侧原地编辑
- 状态栏悬浮窗：快速查看、切换和进入详情

## 项目状态

当前版本已经具备这些核心能力：

- 菜单栏常驻与悬浮窗交互
- Todo 与便签双模式
- 富文本编辑与富文本展示
- 自动保存
- 提醒时间
- 深色 / 浅色主题
- 中英文切换
- 应用内更新检查
- 内置 `Maple Mono` 字体

## 技术栈

前端：

- Vue 3
- TypeScript
- Vite
- Tailwind CSS
- Radix Vue
- TipTap
- vue-i18n

桌面端：

- Tauri 2
- Rust
- Tauri Updater
- Tauri Dialog / Process / Clipboard 等插件

## 环境要求

- macOS
- Node.js 18+
- pnpm 8+
- Rust stable
- Xcode Command Line Tools

## 快速开始

安装依赖：

```bash
pnpm install
```

启动开发环境：

```bash
pnpm tauri dev
```

构建前端：

```bash
pnpm build
```

构建桌面应用：

```bash
pnpm tauri build
```

打包产物默认位于：

```bash
src-tauri/target/release/bundle/
```

## 常用命令

```bash
pnpm dev          # 仅启动前端 Vite
pnpm build        # 构建前端
pnpm tauri dev    # 启动 Tauri 开发环境
pnpm tauri build  # 打包桌面应用
```

## 目录结构

```text
silto/
├── src/
│   ├── components/        # UI 组件
│   ├── composables/       # 数据与状态逻辑
│   ├── locales/           # 中英文文案
│   ├── styles/            # 全局样式
│   ├── types/             # TypeScript 类型
│   └── assets/fonts/      # 内置字体
├── src-tauri/
│   ├── src/               # Rust / Tauri 逻辑
│   ├── icons/             # 应用图标
│   └── tauri.conf.json    # Tauri 配置
├── ARCHITECTURE.md        # 架构说明
└── README.md
```

## 当前界面约定

- 主窗口尺寸默认在 [tauri.conf.json](/Users/smile/work/silto/src-tauri/tauri.conf.json)
- 主窗口为无边框透明窗口
- 主编辑区使用 TipTap 富文本编辑器
- 状态栏 popup 使用独立 `popup.html`
- 设置页已改为主窗口内的独立页面，而不是弹层

## 数据与存储

Silto 使用本地持久化存储。

- Todo、便签、设置都保存在本机
- 具体存储路径由 Tauri 的应用数据目录决定
- 不依赖云端服务

如果你要调试数据读写逻辑，优先查看：

- [useTodos.ts](/Users/smile/work/silto/src/composables/useTodos.ts)
- [useNotes.ts](/Users/smile/work/silto/src/composables/useNotes.ts)
- [useSettings.ts](/Users/smile/work/silto/src/composables/useSettings.ts)

## 关键文件

前端主入口：

- [App.vue](/Users/smile/work/silto/src/App.vue)

设置页：

- [Settings.vue](/Users/smile/work/silto/src/components/Settings.vue)

富文本编辑器：

- [RichTextEditor.vue](/Users/smile/work/silto/src/components/ui/RichTextEditor.vue)

状态栏悬浮窗：

- [PopupView.vue](/Users/smile/work/silto/src/components/PopupView.vue)

托盘与窗口控制：

- [tray.rs](/Users/smile/work/silto/src-tauri/src/tray.rs)

## 开发建议

- UI 调整优先从 `src/components/` 和 `src/App.vue` 入手
- 数据行为优先查看 `src/composables/`
- 桌面窗口、托盘、原生行为优先查看 `src-tauri/src/`
- 大改 UI 前，先确认你看到的是开发版，不是旧的已安装 `.app`

## 注意事项

- 当前项目只面向 macOS 菜单栏场景设计
- 内置 `Maple Mono` 字体目前直接打包了 `ttf`，体积较大
- 如果后续要继续优化安装包大小，建议把字体转成 `woff2` 或减少字重
- popup 和主窗口是两套页面，需要分别注意主题和样式一致性

## 相关文档

- [ARCHITECTURE.md](/Users/smile/work/silto/ARCHITECTURE.md)

## License

MIT
