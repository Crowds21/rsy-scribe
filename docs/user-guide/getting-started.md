# 快速开始

本文档是 [README](../README.md) 中「快速开始」的详细版。

## 1. 环境

- Rust 1.70+（edition 2021）
- 运行中的 [思源笔记](https://b3log.org/siyuan/)（API 默认 `http://127.0.0.1:6806`）
- 支持 Nerd Font 的终端（Gutter 图标显示更佳）

## 2. 构建

```bash
git clone https://github.com/Crowds21/rsy-scribe.git
cd rsy-scribe
cargo build --release
```

## 3. 配置

```bash
mkdir -p ~/.config/scribe
cp syservice/config.example.toml ~/.config/scribe/config.toml
```

编辑 `config.toml`：

| 字段 | 说明 |
|------|------|
| `token` | 思源 **设置 → 关于 → API token** |
| `workspace_dir` | 工作空间下的 `data` 目录绝对路径 |

macOS 也可使用：`~/Library/Application Support/scribe/config.toml`

或使用交互脚本：

```bash
./syservice/scripts/init-config.sh
```

详见 [syservice/CONFIG.md](../../syservice/CONFIG.md)。

## 4. 运行

```bash
cargo run -p app
```

## 5. 基本操作

| 按键 | 作用 |
|------|------|
| `Space` | 打开搜索框 |
| `Enter` | 打开选中笔记 |
| `Esc` | 关闭搜索框 |
| `↑` `↓` | 列表选择 / 文档滚动 |
| `PageUp` / `PageDown` | 翻页 |
| `Home` / `End` | 文档首尾 |
| `Ctrl+C` | 退出 |

## 6. 主题

编辑仓库根目录 [`theme.toml`](../../theme.toml) 与 [`icons.toml`](../../icons.toml) 后重新 `cargo build`。见 [THEME.md](../../THEME.md)。

## 7. 故障排除

| 现象 | 检查 |
|------|------|
| 搜索无结果 | 思源是否运行、`token` 是否正确 |
| 打不开文档 | `workspace_dir` 是否指向 `data` 目录 |
| 无 Gutter 图标 | 终端字体是否支持 Nerd Font |

日志目录：项目根 `logs/`（git 已忽略）。

## 相关文档

- [文档中心](../README.md)
- [008 — syservice 模块](../design/008-syservice-module.md)
