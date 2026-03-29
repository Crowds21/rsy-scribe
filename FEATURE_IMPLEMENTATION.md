# Vim-like 功能实现

## ✅ 已完成的功能

### 1. 模式系统 (`tui/src/mode.rs`)

实现了 Vim-like 的编辑器模式：

```rust
pub enum Mode {
    Normal,    // 正常模式 - 导航和命令
    Insert,    // 插入模式 - 编辑文本
    Command,   // 命令模式 - 输入 : 命令
    Search,    // 搜索模式 - 搜索
}
```

### 2. Leader 键支持

- **Leader 键：** 空格键（可配置）
- **超时：** 1000ms
- **用法：** `<Space>ff` 打开搜索框

### 3. :exit 命令

在 Normal 模式下：
- 输入 `:` 进入命令模式
- 输入 `exit` 或 `q` 退出应用
- 按 `Esc` 返回 Normal 模式

### 4. <leader>ff 搜索功能

在 Normal 模式下：
- 按 `<Space>` 然后按 `f` 两次
- 弹出搜索框
- 基于标题搜索文档

### 5. Markdown 元素渲染增强

新增支持的 Markdown 元素：

| 元素 | 渲染方式 | 状态 |
|------|----------|------|
| 段落 | 普通文本 | ✅ |
| 标题 (H1-H6) | 带装饰线 | ✅ |
| 分隔线 | `─` 字符 | ✅ |
| 引用块 | `│` 前缀 | ✅ |
| 代码块 | ``` 包裹，等宽字体 | ✅ |
| 列表 | `-` / `*` 前缀 | ✅ |
| 超级块 | 容器标记 | ✅ |
| 表格 | `│` 分隔单元格 | ✅ |

---

## 🎯 使用指南

### 基本操作

```
Normal 模式:
  j/k/h/l  - 上下左右移动
  <Space>  - Leader 键
  <Space>ff - 打开搜索
  :        - 进入命令模式

命令模式:
  :exit    - 退出应用
  :q       - 退出应用
  :ff      - 打开搜索
  <Esc>    - 返回 Normal 模式
```

### 搜索功能

1. 按 `<Space>ff`
2. 输入搜索关键词
3. 选择搜索结果
4. 打开对应文档

---

## 📝 代码结构

### 新增文件

- `tui/src/mode.rs` - 模式系统定义

### 修改文件

- `tui/src/lib.rs` - 添加 mode 模块
- `tui/src/component/editor.rs` - 实现模式系统和命令处理
- `view/src/document.rs` - 增强 Markdown 渲染

---

## 🔧 技术实现

### 模式切换

```rust
match self.mode {
    Mode::Normal => self.handle_normal_mode(event, cx),
    Mode::Command => self.handle_command_mode(event, cx),
    Mode::Insert => EventResult::Ignored(None),
    Mode::Search => EventResult::Ignored(None),
}
```

### Leader 键处理

```rust
KeyCode::Char(c) if c == self.leader_config.key && !self.leader_pending => {
    self.leader_pending = true;
    self.leader_time = Some(Instant::now());
    self.set_status("Leader...");
    return EventResult::Consumed(None);
}
```

### 命令解析

```rust
pub enum Command {
    Exit,
    Quit,
    Write,
    WriteQuit,
    SearchFile,
    SearchContent,
    Unknown(String),
}

impl Command {
    pub fn parse(cmd: &str) -> Self {
        let cmd = cmd.trim();
        match cmd {
            "exit" | "quit" | "q" => Command::Exit,
            "ff" | "find" => Command::SearchFile,
            _ => Command::Unknown(cmd.to_string()),
        }
    }
}
```

---

## 🎨 Markdown 渲染

### 代码块渲染

```rust
fn create_code_block_lines(&mut self, node: &Node, available_width: u16) -> Vec<DocumentLine> {
    // 获取代码语言
    let language = node.properties.get("language").cloned().unwrap_or_default();
    
    // 添加代码块标记
    lines.push(fence_line);
    
    // 添加代码内容
    for child in node.children.iter() {
        lines.push(code_line);
    }
    
    lines
}
```

### 引用块渲染

```rust
fn create_blockquote_lines(&mut self, node: &Node, available_width: u16) -> Vec<DocumentLine> {
    for line in &mut child_lines {
        line.container = NodeType::NodeBlockquote;
        // 添加引用符号
        line.content.insert(0, quote_marker);
    }
    lines
}
```

### 表格渲染

```rust
fn create_table_lines(&mut self, node: &Node, available_width: u16) -> Vec<DocumentLine> {
    for cell in child.children.iter() {
        row_content.push(cell_content);
        row_content.push(separator);
    }
    lines.push(row_line);
    lines.push(separator_line);
}
```

---

## 📋 TODO

### 近期目标

- [ ] 实现 DiaryApp/Draft 文档的完整渲染
- [ ] 优化搜索功能，支持模糊匹配
- [ ] 添加更多 Vim 命令（`:w`, `:save` 等）
- [ ] 实现 Insert 模式

### 中期目标

- [ ] 支持更多 Markdown 元素（数学公式、流程图等）
- [ ] 优化渲染性能
- [ ] 添加语法高亮
- [ ] 实现多文档编辑

### 长期目标

- [ ] 插件系统
- [ ] 自定义快捷键
- [ ] 主题系统
- [ ] 与 SiYuan 双向同步

---

## 🧪 测试

### 编译测试

```bash
cd /Users/crowds/RustroverProjects/scribe
cargo build --package tui
cargo build --package view
```

**结果：** ✅ 编译成功

### 单元测试

```bash
cargo test --package tui --lib mode::tests
```

**结果：** ✅ 所有测试通过

---

## 📚 参考

- [Neovim 操作指南](https://neovim.io/doc/user/)
- [Helix 编辑器架构](https://docs.helix-editor.com/architecture.html)
- [Ratatui 文档](https://ratatui.rs/)

---

**实现时间：** 2026-03-27  
**状态：** ✅ 完成  
**下一步：** 测试 DiaryApp/Draft 文档渲染
