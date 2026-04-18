# InLineMarkType 复合样式解析方案

## 问题描述

当前 `InLineMarkType` 是单一枚举，但 `node.text_mark_type` 可能是复合样式（如 `"strong em"`），需要支持多种样式的组合解析。

---

## 方案对比

### 方案 1：枚举中增加组合类型 ❌

```rust
enum InLineMarkType {
    Default,
    Strong,
    Em,
    StrongEm,      // 新增
    StrongCode,    // 新增
    EmCode,        // 新增
    // ... 所有可能的组合
}
```

**问题：**
- 组合爆炸：n 个基础样式 → 2^n 种组合
- 维护成本高，每次新增基础样式都要更新所有组合
- 无法支持动态/未知组合

**结论：不推荐**

---

### 方案 2：BitFlags 位标志 ✅

```rust
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Debug, Default)]
    pub struct InlineMarkFlags: u16 {
        const STRONG  = 1 << 0;
        const EM      = 1 << 1;
        const MARK    = 1 << 2;
        const CODE    = 1 << 3;
        const BLOCK_REF = 1 << 4;
        const A       = 1 << 5;
        const TAG     = 1 << 6;
        const U       = 1 << 7;
        const S       = 1 << 8;
    }
}
```

**解析逻辑：**
```rust
fn parse_mark_flags(input: &str) -> InlineMarkFlags {
    let mut flags = InlineMarkFlags::empty();
    for part in input.split_whitespace() {
        match part {
            "strong" => flags |= InlineMarkFlags::STRONG,
            "em" => flags |= InlineMarkFlags::EM,
            // ... 其他匹配
            _ => {}
        }
    }
    flags
}
```

**优点：**
- 高效（位运算）
- 类型安全
- 组合清晰，无爆炸问题
- 易于判断某个样式是否存在

**缺点：**
- 需要引入 `bitflags` crate
- 样式顺序丢失（某些场景可能需要顺序，如嵌套标签）
- 渲染时需要遍历所有可能的 flag

**适用场景：** CSS 类名生成、样式覆盖逻辑、**TUI 应用**

---

### 方案 3：Vec<InLineMarkType> 保持顺序 ✅

```rust
// 保持原有枚举不变
enum InLineMarkType {
    Default, Strong, Em, Mark, Code, BlockRef, A, Tag, U, S,
}

// 解析为向量
fn parse_mark_types(input: &str) -> Vec<InLineMarkType> {
    input.split_whitespace()
        .filter_map(|s| s.parse::<InLineMarkType>().ok())
        .collect()
}
```

**渲染逻辑：**
```rust
fn apply_marks(text: Element, marks: &[InLineMarkType]) -> Element {
    let mut result = text;
    for mark in marks {
        result = apply_single_mark(result, mark);
    }
    result
}
```

**优点：**
- 最小改动（枚举本身不变）
- 保留样式顺序（支持嵌套语义）
- 灵活，支持任意组合
- 易于理解和调试

**缺点：**
- 需要修改 `create_node_text_mark` 返回类型
- 渲染时需要循环处理
- 需要处理样式覆盖/冲突逻辑（如同时有 `S` 和 `U`）

**适用场景：** 需要保持样式顺序、嵌套标签语义（如 HTML）

---

### 方案 4：混合方案 - 解析时去重 + 排序 ⭐

```rust
// 解析函数
fn parse_and_normalize_marks(input: &str) -> Vec<InLineMarkType> {
    let mut marks: Vec<InLineMarkType> = input.split_whitespace()
        .filter_map(|s| s.parse::<InLineMarkType>().ok())
        .collect();
    
    // 去重
    marks.sort();
    marks.dedup();
    
    // 可选：定义渲染顺序（如 strong 总是在最外层）
    marks.sort_by_key(|m| mark_render_order(m));
    
    marks
}

fn mark_render_order(m: &InLineMarkType) -> u8 {
    match m {
        InLineMarkType::Strong => 0,  // 最外层
        InLineMarkType::Em => 1,
        InLineMarkType::Code => 2,
        // ... 其他
        InLineMarkType::Default => 255,  // 最内层
    }
}
```

**优点：**
- 结合方案 3 的灵活性
- 避免重复样式
- 可控的渲染顺序
- 代码改动适中

---

### 方案 5：字符串直接映射到 CSS 类（最简单）

```rust
fn mark_type_to_css_class(input: &str) -> String {
    // "strong em" → "mark-strong mark-em"
    input.split_whitespace()
        .map(|s| format!("mark-{}", s))
        .collect::<Vec<_>>()
        .join(" ")
}
```

**优点：**
- 最简单，几乎无需修改
- 直接映射到 CSS

**缺点：**
- 失去类型安全
- 无法在 Rust 层做样式逻辑判断

**适用场景：** 纯展示，无需 Rust 层样式逻辑

---

## 🖥️ TUI 场景专项分析

### TUI 特点

- **无嵌套概念**：TUI（如 Ratatui）使用 `Span`，样式是扁平叠加的
- **样式互斥需求**：某些基础样式（如背景色、链接）互相排斥，只能选一个
- **样式叠加需求**：装饰样式（粗体、斜体、下划线）可以共存

### 推荐方案：分组 BitFlags ⭐⭐⭐⭐⭐

```rust
bitflags! {
    /// 基础样式 - 互斥组（同一时间只能有一个）
    #[derive(Clone, Debug, Default)]
    pub struct BaseMark: u8 {
        const DEFAULT   = 0;
        const MARK      = 1 << 0;
        const CODE      = 1 << 1;
        const BLOCK_REF = 1 << 2;
        const A         = 1 << 3;
        const TAG       = 1 << 4;
    }
}

bitflags! {
    /// 装饰样式 - 叠加组（可以多个共存）
    #[derive(Clone, Debug, Default)]
    pub struct DecorMark: u8 {
        const STRONG = 1 << 0;
        const EM     = 1 << 1;
        const U      = 1 << 2;
        const S      = 1 << 3;
    }
}

/// 完整样式
pub struct InlineMarks {
    pub base: BaseMark,
    pub decor: DecorMark,
}
```

### 解析逻辑（互斥 + 叠加）

```rust
fn parse_marks(input: &str) -> InlineMarks {
    let mut base = BaseMark::empty();
    let mut decor = DecorMark::empty();
    
    for part in input.split_whitespace() {
        match part {
            // 互斥组 - 后出现的覆盖先前的
            "mark"      => base = BaseMark::MARK,
            "code"      => base = BaseMark::CODE,
            "block-ref" => base = BaseMark::BLOCK_REF,
            "a"         => base = BaseMark::A,
            "tag"       => base = BaseMark::TAG,
            
            // 叠加组 - 累积
            "strong" => decor |= DecorMark::STRONG,
            "em"     => decor |= DecorMark::EM,
            "u"      => decor |= DecorMark::U,
            "s"      => decor |= DecorMark::S,
            
            _ => {}
        }
    }
    
    InlineMarks { base, decor }
}
```

**示例：**
```
"strong em code"  → base=CODE,        decor=STRONG|EM
"mark strong u"   → base=MARK,        decor=STRONG|U
"code a"          → base=A,           decor=∅  (a 覆盖了 code)
```

### TUI 渲染示例（Ratatui）

```rust
use ratatui::style::{Style, Color, Modifier};
use ratatui::text::Span;

fn apply_marks(span: Span, marks: &InlineMarks) -> Span {
    let mut style = Style::default();
    
    // 应用基础样式（互斥）
    style = match marks.base {
        b if b.contains(BaseMark::CODE) => style.fg(Color::Green),
        b if b.contains(BaseMark::MARK) => style.bg(Color::Yellow),
        b if b.contains(BaseMark::A)    => style.fg(Color::Blue).underlined(),
        b if b.contains(BaseMark::BLOCK_REF) => style.fg(Color::Cyan),
        b if b.contains(BaseMark::TAG)  => style.fg(Color::Magenta),
        _ => style,
    };
    
    // 应用装饰样式（叠加）
    if marks.decor.contains(DecorMark::STRONG) {
        style = style.add_modifier(Modifier::BOLD);
    }
    if marks.decor.contains(DecorMark::EM) {
        style = style.add_modifier(Modifier::ITALIC);
    }
    if marks.decor.contains(DecorMark::U) {
        style = style.add_modifier(Modifier::UNDERLINED);
    }
    if marks.decor.contains(DecorMark::S) {
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }
    
    span.style(style)
}
```

### 简化方案：单一 BitFlags

如果不想分组，也可以用单一 BitFlags，解析时处理互斥：

```rust
bitflags! {
    #[derive(Clone, Debug, Default)]
    pub struct InlineMarks: u16 {
        // 互斥组
        const MARK      = 1 << 0;
        const CODE      = 1 << 1;
        const BLOCK_REF = 1 << 2;
        const A         = 1 << 3;
        const TAG       = 1 << 4;
        
        // 叠加组
        const STRONG = 1 << 8;
        const EM     = 1 << 9;
        const U      = 1 << 10;
        const S      = 1 << 11;
    }
}

fn parse_marks(input: &str) -> InlineMarks {
    let mut marks = InlineMarks::empty();
    const MUTUALLY_EXCLUSIVE: InlineMarks = 
        InlineMarks::MARK | InlineMarks::CODE | 
        InlineMarks::BLOCK_REF | InlineMarks::A | InlineMarks::TAG;
    
    for part in input.split_whitespace() {
        match part {
            "mark" | "code" | "block-ref" | "a" | "tag" => {
                // 清除其他互斥项
                marks.remove(MUTUALLY_EXCLUSIVE);
                marks |= parse_single_base(part);
            }
            "strong" => marks |= InlineMarks::STRONG,
            "em"     => marks |= InlineMarks::EM,
            "u"      => marks |= InlineMarks::U,
            "s"      => marks |= InlineMarks::S,
            _ => {}
        }
    }
    
    marks
}
```

---

## 方案选择决策树

```
是否需要保持样式顺序（HTML 嵌套）？
├── 是 → 方案 3（Vec<Enum>）或 方案 4（混合）
└── 否 → 是否有互斥样式需求？
    ├── 是（TUI 场景）→ 分组 BitFlags ⭐
    ├── 否 → 单一 BitFlags
    └── 纯展示 → 方案 5（CSS 映射）
```

---

## 总结对比

| 方案 | 改动量 | 灵活性 | 类型安全 | 性能 | 适用场景 |
|------|--------|--------|----------|------|----------|
| 1. 枚举组合 | 中 | 低 | 高 | 高 | ❌ 不推荐 |
| 2. BitFlags（单一） | 中 | 中 | 高 | ⭐ | 简单叠加 |
| 2. BitFlags（分组） | 中 | 高 | 高 | ⭐ | **TUI 应用** ⭐⭐⭐⭐⭐ |
| 3. Vec<Enum> | 低 | 高 | 高 | 中 | HTML 嵌套 |
| 4. 混合方案 | 中 | 高 | 高 | 中 | 需顺序控制 |
| 5. CSS 映射 | 最低 | 中 | 低 | 高 | 纯展示 |

---

## 最终推荐

### TUI 应用（当前场景）
**分组 BitFlags** - 清晰表达互斥/叠加语义，高性能，类型安全

### Web/HTML 应用
**Vec<InLineMarkType>** - 保留嵌套顺序，改动最小

---

## 依赖

```toml
[dependencies]
bitflags = "2.4"
```
