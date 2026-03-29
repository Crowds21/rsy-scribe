# 主题系统文档

## 概述

Scribe 使用混合主题系统：
- **默认主题**：代码内置，开箱即用
- **自定义主题**：通过 `~/.config/scribe/theme.toml` 覆盖默认值

## 目录结构

```
~/.config/scribe/
├── theme.toml          # 自定义主题（可选）
└── config.toml         # 其他配置（未来功能）
```

## 自定义主题

### 1. 创建主题文件

```bash
mkdir -p ~/.config/scribe
cp theme.example.toml ~/.config/scribe/theme.toml
```

### 2. 编辑主题

只需定义你想覆盖的样式：

```toml
# 修改编辑器背景
"editor.bg" = { bg = "#1a1a2e" }

# 修改 Normal 模式颜色
"mode.normal" = { fg = "#00ff00", modifier = "bold" }

# 修改代码块样式
"code.block.content" = { fg = "#98c379", bg = "#26292c" }
```

### 3. 重启 Scribe

主题会在启动时自动加载。

## 样式键参考

### 基础样式
- `editor.bg` - 编辑器背景
- `editor.fg` - 编辑器前景（默认文本颜色）

### 模式显示
- `mode.normal` - Normal 模式
- `mode.insert` - Insert 模式
- `mode.command` - Command 模式
- `mode.search` - Search 模式

### 标题样式
- `node.heading.h1` ~ `node.heading.h6` - 各级标题

### 代码样式
- `code.block.fence` - 代码块围栏（```）
- `code.block.content` - 代码块内容
- `code.inline` - 行内代码

### 引用和列表
- `blockquote.marker` - 引用块标记（│）
- `blockquote.content` - 引用块内容
- `list.bullet` - 列表标记（•）

### 行内元素
- `strong` - 粗体
- `em` - 斜体
- `strong.em` - 粗体 + 斜体
- `mark` - 高亮
- `link` - 链接
- `tag` - 标签

### 其他
- `table.separator` - 表格分隔线
- `divider` - 水平分隔线
- `ui.gutter` - 侧边栏

## 样式格式

每个样式可以包含以下字段：

```toml
"style.name" = { 
    fg = "#颜色值",           # 前景色（可选）
    bg = "#颜色值",           # 背景色（可选）
    modifier = "修饰器"       # 修饰器（可选）
}
```

### 颜色值
- 十六进制：`"#ffffff"`, `"#282c34"`
- 颜色名称：`"black"`, `"red"`, `"green"`, `"yellow"`, `"blue"`, `"magenta"`, `"cyan"`, `"white"`
- 亮色：`"light_red"`, `"light_green"` 等
- 暗色：`"dark_gray"`

### 修饰器
- `bold` - 粗体
- `italic` - 斜体
- `underlined` - 下划线
- `reversed` - 反色
- `crossed_out` - 删除线
- 组合：`"bold italic"`（空格分隔）

## 示例

### 深色主题
```toml
"editor.bg" = { bg = "#1a1a2e" }
"editor.fg" = { fg = "#eaeaea" }
"mode.normal" = { fg = "#00ff00", modifier = "bold" }
```

### 高对比度主题
```toml
"editor.bg" = { bg = "#000000" }
"editor.fg" = { fg = "#ffffff" }
"strong" = { fg = "#ffff00", modifier = "bold" }
"link" = { fg = "#00ffff", modifier = "underlined" }
```

## 向后兼容

主题系统保留了旧版 theme.toml 的样式键名：
- `node.heading.title`
- `node.text.strong`
- `node.text.code`
- `ui.gutter`
- 等等

旧主题文件仍然可用，但建议使用新键名。

## 运行时重新加载（未来功能）

计划支持 `:reload-theme` 命令来重新加载主题而无需重启。

## 故障排除

### 主题未生效
1. 检查文件路径：`~/.config/scribe/theme.toml`
2. 检查 TOML 语法是否正确
3. 查看启动时的日志输出

### 部分样式未生效
未定义的样式会使用默认值。确保你定义了正确的样式键名。

### 颜色显示不正确
某些终端可能不支持所有颜色。尝试使用基本的 16 色名称而非十六进制值。
