# 终端恢复功能实现

## ✅ 实现内容

### 问题描述
当 TUI 应用退出时（通过 `:exit`、Ctrl+C 或 panic），需要恢复终端到正常状态，否则终端显示会混乱。

### 解决方案

#### 1. 主退出函数 (`app/src/main.rs`)

```rust
/// 恢复终端到正常状态
/// 在应用退出时必须调用此函数
pub fn restore_tui() -> io::Result<()> {
    let result = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    result
}

fn main() -> io::Result<()> {
    let result = main_impl();
    // 确保退出时恢复终端
    let _ = restore_tui();
    result
}
```

**关键点：**
- 在 `main()` 函数末尾调用 `restore_tui()`
- 即使 `main_impl()` 返回错误，也会执行恢复

---

#### 2. Panic Hook (`app/src/main.rs`)

```rust
pub fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();  // panic 时恢复终端
        original_hook(panic_info);
    }));
}
```

**作用：** 当应用 panic 时，先恢复终端再显示 panic 信息

---

#### 3. Application 退出 (`app/src/application.rs`)

```rust
use crate::restore_tui;

impl Application {
    pub fn exit_app(&mut self) {
        let _ = restore_tui();
        std::process::exit(0);
    }
}
```

**使用场景：** Ctrl+C 退出时调用

---

#### 4. TUI 库退出 (`tui/src/lib.rs`)

```rust
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use std::io;
use std::io::stdout;

/// 恢复终端到正常状态
pub fn restore_tui() -> io::Result<()> {
    let result = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    result
}
```

---

#### 5. :exit 命令处理 (`tui/src/component/editor.rs`)

```rust
use crate::restore_tui;

Command::Exit | Command::Quit => {
    // 退出应用前恢复终端
    return EventResult::Consumed(Some(Box::new(|_, _| {
        let _ = restore_tui();
        std::process::exit(0);
    })));
}
```

**关键点：**
- 通过回调函数执行恢复和退出
- 确保在 Compositor 处理完事件后执行

---

## 🔄 退出流程

```
用户操作 → 退出命令 → restore_tui() → process::exit(0)
   ↓
1. Ctrl+C
2. :exit / :q
3. panic
   ↓
   恢复终端
   ↓
1. disable_raw_mode()
2. LeaveAlternateScreen
   ↓
   终端恢复正常
```

---

## 📝 退出路径

| 退出方式 | 处理位置 | restore_tui 调用 |
|----------|----------|-----------------|
| **Ctrl+C** | `application.rs::handle_terminal_events()` | ✅ `exit_app()` |
| **:exit 命令** | `editor.rs::handle_command_mode()` | ✅ 回调函数 |
| **Panic** | `main.rs::init_panic_hook()` | ✅ panic hook |
| **正常退出** | `main.rs::main()` | ✅ main 末尾 |

---

## ⚠️ 注意事项

### 1. 错误处理

```rust
let _ = restore_tui();  // 忽略错误，确保退出继续
```

即使恢复终端失败，也要继续退出流程。

### 2. 调用顺序

```rust
// ✅ 正确
restore_tui();
std::process::exit(0);

// ❌ 错误 - exit 后不会执行
std::process::exit(0);
restore_tui();  // 永远不会执行
```

### 3. 回调函数

在 TUI 事件处理中，使用回调确保在正确的时机恢复终端：

```rust
EventResult::Consumed(Some(Box::new(|_, _| {
    let _ = restore_tui();
    std::process::exit(0);
})))
```

---

## 🧪 测试方法

### 1. 测试 :exit 命令

```bash
cd /Users/crowds/RustroverProjects/scribe
cargo run --package app
# 输入 :exit
# 检查终端是否正常显示
```

### 2. 测试 Ctrl+C

```bash
cargo run --package app
# 按 Ctrl+C
# 检查终端是否正常显示
```

### 3. 测试 Panic

在代码中故意添加 panic：

```rust
// 在某个地方添加
panic!("Test panic");
```

检查 panic 信息是否正常显示，终端是否恢复。

---

## 📊 对比

### 未恢复终端（错误）

```bash
$ cargo run
# TUI 应用
# 退出后...
$  # ← 命令行提示符可能不显示或显示混乱
```

### 已恢复终端（正确）

```bash
$ cargo run
# TUI 应用
# 退出后...
$  # ← 命令行提示符正常显示
```

---

## 🔗 相关文件

- `app/src/main.rs` - 主退出函数和 panic hook
- `app/src/application.rs` - Application 退出方法
- `tui/src/lib.rs` - TUI 库恢复函数
- `tui/src/component/editor.rs` - :exit 命令处理

---

**实现时间：** 2026-03-27  
**状态：** ✅ 完成  
**测试：** 待验证（需要修复 view 包编译错误）
