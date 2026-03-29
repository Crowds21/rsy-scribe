# Ratatui 样式测试工具

## 📦 测试工具说明

### 1. 可视化测试（真实终端）

**运行方式：**
```bash
cd /Users/crowds/RustroverProjects/scribe
cargo run --package view --example test_styles
```

**功能：**
- 在真实终端中展示各种样式组合
- 按 `q` 键退出
- 支持测试：粗体、斜体、下划线、删除线、高亮、代码等

**注意事项：**
- 需要在真实终端中运行（不能在 IDE 的 Run 窗口）
- 某些终端模拟器可能不支持所有样式
- 推荐使用：iTerm2、Alacritty、Kitty、Windows Terminal

---

### 2. 单元测试（自动验证）

**运行方式：**
```bash
cargo test --package view --test style_nesting_test
```

**测试覆盖：**
- ✅ 粗体 + 斜体嵌套
- ✅ 粗体 + 下划线嵌套
- ✅ 三样式组合
- ✅ 删除线样式
- ✅ 样式叠加
- ✅ 多 Span 不同样式
- ✅ 字符串解析样式
- ✅ 冲突样式处理

---

### 3. 查看测试结果详情

```bash
# 运行测试并显示输出
cargo test --package view --test style_nesting_test -- --nocapture

# 生成测试报告
cargo test --package view --test style_nesting_test -- --format json > test_results.json
```

---

## 🎨 预期效果

### 单一粗体
```
┌─ 粗体 ─────────────────────────────┐
│ 这是粗体文本 (BOLD)                 │
└─────────────────────────────────────┘
```
（文本应该显示为**粗体**）

### 单一斜体
```
┌─ 斜体 ─────────────────────────────┐
│ 这是斜体文本 (ITALIC)               │
└─────────────────────────────────────┘
```
（文本应该显示为*斜体*）

### 粗体 + 斜体（关键测试）
```
┌─ 粗体 + 斜体 ───────────────────────┐
│ 粗体 + 斜体 (BOLD + ITALIC)          │
└─────────────────────────────────────┘
```
（文本应该**同时显示为粗体和斜体**）

### 粗体 + 下划线
```
┌─ 粗体 + 下划线 ─────────────────────┐
│ 粗体 + 下划线 (BOLD + UNDERLINE)     │
└─────────────────────────────────────┘
```
（文本应该**同时显示为粗体和下划线**）

### 三样式组合
```
┌─ 三样式组合 ────────────────────────┐
│ 粗体 + 斜体 + 下划线 (BOLD + ITALIC + UNDERLINE) │
└─────────────────────────────────────┘
```
（文本应该**同时显示三种样式**）

### 删除线
```
┌─ 删除线 ────────────────────────────┐
│ 这是删除线文本 (CROSSED_OUT)         │
└─────────────────────────────────────┘
```
（文本应该显示为~~删除线~~）

### 高亮
```
┌─ 高亮 ──────────────────────────────┐
│ 这是高亮文本 (MARK)                  │
│ (黄色背景，黑色文字)                  │
└─────────────────────────────────────┘
```
（文本应该有**黄色背景**）

### 行内代码
```
┌─ 行内代码 ──────────────────────────┐
│ let x = 10; // 行内代码              │
│ (绿色文字，深色背景)                  │
└─────────────────────────────────────┘
```
（文本应该有**代码样式**）

---

## 🔍 如何验证样式是否生效

### 方法 1：肉眼观察
运行 `test_styles` 示例，直接观察终端显示效果。

### 方法 2：检查测试输出
```bash
cargo test --package view --test style_nesting_test -- --nocapture
```

你会看到类似输出：
```
Buffer cell style: Style { fg: None, bg: None, underline: None, add_modifier: BOLD | ITALIC, sub_modifier: NONE }
Expected style: Style { fg: None, bg: None, underline: None, add_modifier: BOLD | ITALIC, sub_modifier: NONE }
```

**关键看 `add_modifier` 字段：**
- `BOLD | ITALIC` - 表示同时有粗体和斜体 ✅
- `BOLD | UNDERLINED` - 表示同时有粗体和下划线 ✅
- `BOLD | ITALIC | UNDERLINED` - 表示三种样式都有 ✅

### 方法 3：查看 Style 结构

在测试代码中添加：
```rust
println!("Cell style: {:?}", cell.style());
println!("Add modifiers: {:?}", cell.style().add_modifier);
```

---

## 🐛 常见问题

### Q: 看不到斜体效果？
A: 某些终端/字体不支持斜体。尝试：
- 更换终端（推荐 iTerm2、Alacritty）
- 更换字体（推荐 Fira Code、JetBrains Mono）

### Q: 删除线不显示？
A: 删除线支持度较低。可以：
- 检查终端是否支持
- 使用其他样式组合代替

### Q: 高亮颜色不对？
A: 终端颜色配置可能影响显示。可以：
- 调整终端配色方案
- 使用 RGB 颜色代替预设颜色

### Q: 测试通过但看不到效果？
A: 单元测试在内存中运行，不会实际渲染。请使用：
```bash
cargo run --package view --example test_styles
```

---

## 📊 终端兼容性参考

| 终端 | 粗体 | 斜体 | 下划线 | 删除线 | 高亮 |
|------|------|------|--------|--------|------|
| iTerm2 | ✅ | ✅ | ✅ | ✅ | ✅ |
| Alacritty | ✅ | ✅ | ✅ | ⚠️ | ✅ |
| Kitty | ✅ | ✅ | ✅ | ✅ | ✅ |
| Windows Terminal | ✅ | ✅ | ✅ | ⚠️ | ✅ |
| macOS Terminal | ✅ | ⚠️ | ✅ | ❌ | ✅ |
| xterm | ✅ | ⚠️ | ✅ | ❌ | ⚠️ |

✅ 完全支持 | ⚠️ 部分支持 | ❌ 不支持

---

## 🎯 下一步

运行测试验证 ratatui 的样式嵌套能力：

```bash
# 1. 运行单元测试（验证逻辑）
cargo test --package view --test style_nesting_test

# 2. 运行可视化测试（查看效果）
cargo run --package view --example test_styles
```

如果一切正常，你就可以在 Scribe 中实现嵌套样式了！
