# Silto

一个现代化的 Mac 状态栏 Todo & 便签管理应用，使用 Tauri + Rust + Vue3 构建。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)

## ✨ 特性

- 🎯 **系统托盘集成** - 常驻 Mac 菜单栏，快速访问
- ✅ **Todo 管理** - 创建、编辑、完成任务，支持定时提醒
- 📝 **便签系统** - 快速记录想法和笔记
- ⏰ **智能提醒** - 到时自动发送系统通知
- 🌓 **主题切换** - 支持深色和浅色两种模式
- 💾 **数据持久化** - 本地安全存储，应用重启数据不丢失
- 🎨 **现代 UI** - 基于 shadcn-vue，美观且易用

## 🚀 快速开始

### 环境要求

- Node.js 16+
- pnpm 8+
- Rust 1.70+
- macOS 11+

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建应用

```bash
pnpm tauri build
```

构建完成后，应用位于 `src-tauri/target/release/bundle/`

## 📖 使用说明

### 基本操作

1. **启动应用** - 应用会自动托管到菜单栏
2. **查看列表** - 左键点击菜单栏图标
3. **快速添加** - 右键点击菜单栏图标 → 选择"添加 Todo"或"添加便签"
4. **编辑项目** - 点击列表中的任意项目
5. **设置提醒** - 编辑 Todo 时选择提醒时间
6. **切换主题** - 右键菜单 → 设置 → 选择主题

### 快捷功能

- **Todo 列表** - 显示所有待办事项，区分已完成和未完成
- **便签列表** - 按更新时间排序显示所有便签
- **视图切换** - 顶部标签页快速切换 Todo 和便签
- **设置面板** - 配置默认显示内容和主题

## 🏗️ 技术架构

### 后端 (Rust)

- **Tauri 2.x** - 跨平台应用框架
- **tauri-plugin-store** - 数据持久化
- **tauri-plugin-notification** - 系统通知
- **chrono** - 时间处理
- **tokio** - 异步运行时

### 前端 (Vue 3)

- **Vue 3** + **TypeScript** - 渐进式框架
- **Vite** - 快速构建工具
- **Tailwind CSS 3** - 实用优先的 CSS 框架
- **shadcn-vue** - 高质量 UI 组件
- **radix-vue** - 无样式组件基础
- **lucide-vue-next** - 现代图标库

## 📁 项目结构

```
silto/
├── src/                    # Vue3 前端代码
│   ├── components/         # Vue 组件
│   ├── composables/        # 组合式函数
│   ├── types/              # TypeScript 类型
│   ├── lib/                # 工具函数
│   └── styles/             # 全局样式
├── src-tauri/              # Rust 后端代码
│   ├── src/
│   │   ├── models.rs       # 数据模型
│   │   ├── commands.rs     # Tauri 命令
│   │   ├── tray.rs         # 系统托盘
│   │   └── notification.rs # 提醒系统
│   └── icons/              # 应用图标
├── .cursorrules            # AI 开发指南
├── ARCHITECTURE.md         # 架构文档
└── README.md               # 项目说明
```

## 🔧 配置说明

### 数据存储位置

所有数据存储在：
```
~/.local/share/com.smile.silto/store.json
```

### 配置文件

- `src-tauri/tauri.conf.json` - Tauri 应用配置
- `tailwind.config.js` - Tailwind CSS 配置
- `vite.config.ts` - Vite 构建配置
- `tsconfig.json` - TypeScript 配置

## 🛠️ 开发指南

### 添加新功能

1. **后端** - 在 `src-tauri/src/` 中添加 Rust 代码
2. **前端** - 在 `src/components/` 中创建 Vue 组件
3. **数据管理** - 使用 composables 模式

### 代码规范

- 使用 TypeScript 严格模式
- 遵循 Vue 3 Composition API
- 使用 Tailwind CSS 类名
- Rust 代码使用 rustfmt 格式化

详细开发指南请查看 [.cursorrules](.cursorrules) 和 [ARCHITECTURE.md](ARCHITECTURE.md)

## 🐛 已知问题

- ~~Input/Textarea 组件 v-model 不工作~~ ✅ 已修复
- 自定义图标格式需要手动调整

历史问题请查看 [BUGFIX-save-issue.md](BUGFIX-save-issue.md)

## 🗺️ 路线图

- [ ] 添加 Todo 优先级功能
- [ ] 支持便签分类/标签
- [ ] 全局搜索功能
- [ ] 数据导入/导出
- [ ] 快捷键支持
- [ ] 云同步功能

## 📄 许可证

MIT License

## 🙏 致谢

- [Tauri](https://tauri.app/) - 强大的应用框架
- [Vue 3](https://vuejs.org/) - 渐进式 JavaScript 框架
- [shadcn-vue](https://www.shadcn-vue.com/) - 精美的 UI 组件
- [Tailwind CSS](https://tailwindcss.com/) - 实用优先的 CSS 框架

## 📮 反馈

如有问题或建议，欢迎提交 Issue 或 Pull Request。

---

**Enjoy using Silto!** 🎉
