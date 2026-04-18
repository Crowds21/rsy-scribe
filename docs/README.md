# Scribe 文档中心

欢迎使用 Scribe - 思源笔记的 TUI 客户端。

## 📚 文档分类

### 设计文档

设计文档记录了项目的架构决策、实现计划和技术细节。

| 编号 | 文档 | 状态 | 日期 |
|------|------|------|------|
| 001 | [代码块渲染设计](design/001-code-block-rendering.md) | 评审中 | 2026-03-31 |
| 002 | [行内样式重构](design/002-inline-styles.md) | 已实现 | 2026-03-31 |
| 003 | [配置系统设计](design/003-config-system.md) | 已实现 | 2026-03-31 |

### 用户文档

| 文档 | 说明 |
|------|------|
| [快速开始](user-guide/getting-started.md) | 安装和配置指南 |
| 配置说明 | 配置文件格式和选项 |

### API 文档

API 文档由 `cargo doc` 自动生成：

- [View API](../target/doc/view/) - 视图模块
- [Syservice API](../target/doc/syservice/) - 思源服务模块
- [TUI API](../target/doc/tui/) - TUI 组件模块

生成 API 文档：
```bash
cargo doc --open
```

## 🔧 本地开发文档

### 生成 API 文档

```bash
# 生成并打开文档
cargo doc --open

# 生成文档（包含私有项）
cargo doc --document-private-items
```

### 查看设计文档

设计文档是 Markdown 格式，可以用任何 Markdown 查看器打开：

```bash
# 使用 VS Code
code docs/README.md

# 使用浏览器（需要 Markdown 预览插件）
# 或使用 mdbook
mdbook serve docs
```

### 文档规范

编写新文档时，请参考 [文档指南](../DOC_GUIDE.md)。

## 📝 文档更新流程

1. **新增功能** → 创建设计文档（`docs/design/序号 - 功能.md`）
2. **实现完成** → 更新状态为"已实现"
3. **API 变更** → 更新代码注释，运行 `cargo doc`

## 🔗 相关资源

- [项目 README](../README.md)
- [贡献指南](../CONTRIBUTING.md)
- [Rust 文档规范](https://doc.rust-lang.org/rustdoc/)
- [mdBook 文档](https://rust-lang.github.io/mdBook/)

---

**最后更新：** 2026-03-31
