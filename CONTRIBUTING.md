# 贡献指南

感谢关注 Scribe 项目。

## 开发环境

```bash
cargo build
cargo test
cargo run -p app
```

## 分支与提交

- 功能开发请从 `main` 拉取分支，例如 `feat/your-feature`
- 提交信息遵循 [约定式提交](https://www.conventionalcommits.org/)（如 `feat(tui): ...`、`docs: ...`、`fix(view): ...`）

## 文档

- 设计文档：`docs/design/`（见 [文档中心](docs/README.md)）
- 新增功能请先更新或新增对应设计文档
- 编写规范见 [DOC_GUIDE.md](DOC_GUIDE.md)

## 配置与安全

- **不要**在 PR 中包含真实 API Token、个人路径或 `config.toml`
- 示例配置使用 [`syservice/config.example.toml`](syservice/config.example.toml) 中的占位符

## 代码结构

```
app → tui → view → syservice → infrastructure
```

模块设计见 `docs/design/005`–`009`。

## 问题与讨论

请通过 GitHub Issues 反馈 bug 或功能建议。
