# Scribe 项目文档指南

## 1. Rust 项目文档最佳实践

### 1.1 Rust 官方推荐的文档组织方式

Rust 生态系统通常采用**分层文档**策略：

```
scribe/
├── README.md                 # 项目概述（面向用户）
├── CONTRIBUTING.md           # 贡献指南
├── DOC_GUIDE.md              # 本文档（文档规范）
│
├── docs/                     # 用户文档（可选）
│   ├── user-guide.md
│   └── faq.md
│
├── view/doc/                 # 模块设计文档
│   ├── CODE_BLOCK_DESIGN.md  # 代码块设计
│   ├── inline-mark-type-refactor.md
│   └── IMPLEMENTATION.md
│
├── syservice/doc/            # 模块设计文档
│   └── CONFIG_IMPLEMENTATION.md
│
└── src/                      # 代码即文档
    ├── lib.rs                # 包含 API 文档注释
    └── ...
```

### 1.2 Rust 原生文档支持

#### A. 文档注释（`rustdoc`）

Rust 内置文档系统，代码即文档：

```rust
/// 代码块模型
/// 
/// 用于表示思源笔记中的代码块节点，包含语言标识和代码内容。
/// 
/// # Examples
/// 
/// ```
/// let code_block = CodeBlockModel::from_node(&node);
/// println!("Language: {}", code_block.language);
/// ```
/// 
/// # Fields
/// 
/// * `language` - 编程语言标识（如 "rust", "python"）
/// * `content` - 原始代码内容
#[derive(Clone, Debug)]
pub struct CodeBlockModel {
    pub language: String,
    pub content: String,
    // ...
}
```

**生成文档：**
```bash
cargo doc --open
```

#### B. 设计文档（RFC 风格）

对于复杂功能，使用独立的设计文档：

```markdown
docs/design/
├── 001-code-block-rendering.md    # 代码块渲染
├── 002-syntax-highlighting.md     # 语法高亮
└── 003-scrolling-support.md       # 滚动支持
```

**命名规范：** `序号 - 简短描述.md`

#### C. 模块级文档

每个模块的 `lib.rs` 或 `mod.rs` 包含模块说明：

```rust
//! 代码块处理模块
//!
//! 提供代码块的解析、渲染和语法高亮功能。
//!
//! # Architecture
//!
//! ```text
//! NodeCodeBlock → CodeBlockModel → Rendered Lines
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! let code_block = CodeBlockModel::from_node(&node);
//! let lines = render_code_block(&code_block, width);
//! ```

pub mod code_block;
```

---

## 2. 设计文档模板

### 2.1 标准模板

```markdown
# [功能名称] 设计文档

**状态：** 草案 | 评审中 | 已批准 | 已实现  
**日期：** YYYY-MM-DD  
**作者：** [姓名]  
**相关 Issue：** #[编号]

## 1. 概述

### 1.1 背景
为什么需要这个功能？解决什么问题？

### 1.2 目标
具体要实现什么功能？

### 1.3 非目标
什么不在本次实现范围内？

## 2. 设计

### 2.1 架构图
```text
[组件 A] → [组件 B] → [组件 C]
```

### 2.2 数据结构
```rust
pub struct MyStruct {
    // 字段说明
}
```

### 2.3 API 设计
```rust
pub fn my_function(arg: Type) -> Result<T, E>;
```

## 3. 实现计划

### Phase 1: ...
- [ ] 任务 1
- [ ] 任务 2

### Phase 2: ...

## 4. 替代方案

### 方案 A
优点：...
缺点：...

### 方案 B（推荐）
优点：...
缺点：...

## 5. 测试计划

- 单元测试
- 集成测试
- 性能测试

## 6. 参考资料

- [链接 1]
- [链接 2]

## 7. 修订历史

| 日期 | 版本 | 作者 | 变更 |
|------|------|------|------|
| ... | ... | ... | ... |
```

---

## 3. 文档维护规范

### 3.1 何时创建设计文档

**需要文档：**
- ✅ 新功能开发（Phase 1 之前）
- ✅ 架构变更
- ✅ 公共 API 修改
- ✅ 复杂算法实现

**不需要文档：**
- ❌ 简单 Bug 修复
- ❌ 重构（行为不变）
- ❌ 文档/注释更新

### 3.2 文档更新时机

1. **设计阶段** - 创建草案文档
2. **实现阶段** - 更新实现细节
3. **完成阶段** - 标记为"已实现"，添加示例

### 3.3 文档审查

重要设计文档需要团队审查：

```markdown
<!-- 在文档顶部添加 -->
**审查状态：**
- [ ] @reviewer1
- [ ] @reviewer2
```

---

## 4. Rust 工具支持

### 4.1 rustdoc

```bash
# 生成文档
cargo doc

# 生成并打开
cargo doc --open

# 包含私有项（开发用）
cargo doc --document-private-items

# 自定义输出
cargo doc --html-in-header docs/header.html
```

### 4.2 rustfmt

```bash
# 格式化代码（包括文档注释）
cargo fmt
```

### 4.3 clippy

```bash
# 检查文档完整性
cargo clippy -- -W missing-docs
```

### 4.4 cargo-docs

第三方工具，增强文档功能：

```toml
# Cargo.toml
[dev-dependencies]
cargo-docs = "0.1"
```

---

## 5. 文档位置规范

### 5.1 项目根目录

| 文件 | 用途 |
|------|------|
| `README.md` | 项目介绍、快速开始 |
| `CONTRIBUTING.md` | 贡献指南 |
| `CHANGELOG.md` | 变更日志 |
| `LICENSE` | 许可证 |

### 5.2 模块目录

| 位置 | 用途 |
|------|------|
| `view/doc/` | View 模块设计文档 |
| `syservice/doc/` | Syservice 模块设计文档 |
| `tui/doc/` | TUI 模块设计文档 |

### 5.3 代码目录

| 位置 | 用途 |
|------|------|
| `src/lib.rs` | Crate 级文档 |
| `src/mod.rs` | 模块级文档 |
| `pub struct/enum/fn` | 项目级文档注释 |

---

## 6. 文档版本控制

### 6.1 Git 标签

```bash
# 发布时打标签
git tag -a v0.1.0 -m "Release v0.1.0"

# 查看标签
git tag -l
```

### 6.2 文档版本

在文档中包含版本信息：

```markdown
**适用版本：** v0.1.0+
**最后更新：** 2026-03-31
```

---

## 7. 示例：代码块功能文档组织

```
scribe/
├── view/
│   ├── doc/
│   │   └── CODE_BLOCK_DESIGN.md    # 详细设计文档
│   └── src/
│       ├── document.rs             # 集成代码
│       └── code_block/
│           ├── mod.rs              # 模块文档
│           ├── parser.rs           # 解析器（带文档注释）
│           ├── renderer.rs         # 渲染器（带文档注释）
│           └── highlight.rs        # 高亮接口（带文档注释）
└── docs/
    └── user-guide.md               # 用户指南（提及代码块功能）
```

---

## 8. 总结

### 推荐实践

1. **代码即文档** - 使用 `rustdoc` 注释
2. **设计先行** - 复杂功能先写设计文档
3. **分层组织** - 用户文档、设计文档、API 文档分离
4. **定期更新** - 实现完成后更新文档
5. **工具辅助** - 使用 `cargo doc`、`clippy` 等工具

### 避免的陷阱

1. ❌ 文档与代码不同步
2. ❌ 过度文档化（简单功能也写长文档）
3. ❌ 文档散乱（无统一位置）
4. ❌ 缺少示例代码

---

## 9. 参考资源

- [Rust Book - Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)
- [rustdoc Book](https://doc.rust-lang.org/rustdoc/)
- [RFC Process](https://github.com/rust-lang/rfcs)
- [Keep a Changelog](https://keepachangelog.com/)
