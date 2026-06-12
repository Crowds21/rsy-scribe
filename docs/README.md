# Scribe 文档中心

欢迎使用 Scribe — 思源笔记的 TUI 客户端。

## 设计文档

| 编号 | 文档 | 状态 | 说明 |
|------|------|------|------|
| 001 | [代码块渲染](design/001-code-block-rendering.md) | 已实现 | Phase 1 代码块布局与主题键 |
| 002 | [行内样式](design/002-inline-styles.md) | 已实现 | BitFlags / 复合 mark |
| 003 | [配置系统](design/003-config-system.md) | 已实现 | config.toml 加载 |
| 004 | [项目结构审查](design/004-project-structure-review.md) | 已更新 | 架构与 backlog |
| 005 | [app 模块](design/005-app-module.md) | 已实现 | 应用入口与主循环 |
| 006 | [tui 模块](design/006-tui-module.md) | 已实现 | Compositor、Editor、Search |
| 007 | [view 模块](design/007-view-module.md) | 已实现 | DocumentModel、heading、code_block |
| 008 | [syservice 模块](design/008-syservice-module.md) | 已实现 | API、配置、.sy 读取 |
| 009 | [infrastructure 模块](design/009-infrastructure-module.md) | 已实现 | 文件日志 |
| 010 | [笔记本访问控制](design/010-notebook-access-control.md) | 规划中 | 多笔记本白名单 |

## 用户文档

| 文档 | 说明 |
|------|------|
| [快速开始](user-guide/getting-started.md) | 安装、配置、运行 |
| [项目 README](../README.md) | 概览与快捷键 |
| [配置指南](../syservice/CONFIG.md) | config.toml 字段说明 |
| [主题说明](../THEME.md) | theme.toml / icons.toml |

## 开发者文档

| 文档 | 说明 |
|------|------|
| [文档编写规范](../DOC_GUIDE.md) | 如何撰写设计文档 |
| [贡献指南](../CONTRIBUTING.md) | 分支、提交约定 |
| [syservice README](../syservice/README.md) | HttpClient 快速上手 |
| [扩展指南](../syservice/EXTENSIBILITY_GUIDE.md) | Trait 与中间件 |
| [集成测试](../syservice/tests/README.md) | 如何跑 `--ignored` 测试 |
| [样式示例](../view/examples/README.md) | view 样式测试工具 |

## API 文档

```bash
cargo doc --open
```

## 文档更新流程

1. 新功能 → 在 `docs/design/` 新增或更新设计文档
2. 实现完成 → 更新文档「状态」列
3. API 变更 → 更新 rustdoc 注释

---

**最后更新：** 2026-06-11
