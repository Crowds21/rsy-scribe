# SiYuan 配置指南

## 配置方式（优先级从高到低）

### 1. 环境变量（最高优先级）

```bash
# 可选
export SIYUAN_API_TOKEN="your-api-token"
export SIYUAN_API_URL="http://127.0.0.1:6806"
export SIYUAN_TIMEOUT_SECS="30"
```

### 2. 配置文件

#### 配置文件路径

系统会自动从以下路径加载配置文件（按优先级从高到低）：

| 操作系统 | 配置文件路径（优先级从高到低） |
|----------|---------------------------|
| **Linux/macOS** | 1. `$XDG_CONFIG_HOME/scribe/config.toml`<br>2. `~/.config/scribe/config.toml` |
| **Windows** | 1. `%APPDATA%\scribe\config.toml` |

**统一路径设计：** Linux 和 macOS 使用相同的配置路径（`~/.config/scribe/config.toml`），方便跨平台共享配置文件（如通过 dotfiles 仓库同步）。

#### 配置文件格式

```toml
# SiYuan API 基础 URL
base_url = "http://127.0.0.1:6806"

# API 令牌（可选）
# 在思源笔记中：设置 -> 关于 -> API token
token = "your-api-token-here"

# HTTP 请求超时时间（秒）
timeout_secs = 30
```

## 获取 API Token

1. 打开思源笔记
2. 进入 **设置** → **关于**
3. 找到 **API token** 字段
4. 复制 token 值

## 使用示例

### Rust 代码

```rust
use syservice::prelude::*;

fn main() -> anyhow::Result<()> {
    // 方式 1：自动从环境变量或配置文件加载
    let config = Config::load()?;
    
    // 方式 2：仅从环境变量加载
    let config = Config::from_env()?;
    
    // 方式 3：仅从配置文件加载
    let config = Config::from_file();  // Option<Config>
    
    // 方式 4：加载失败时使用默认配置
    let config = Config::load_or_default();
    
    // 创建 HTTP 客户端
    let client = HttpClient::new(&config)?;
    
    Ok(())
}
```

### 运行集成测试

```bash
# 方式 1：使用环境变量
export SIYUAN_API_TOKEN="your-token"
cargo test --test integration -- --ignored

# 方式 2：使用配置文件
# 确保配置文件已创建在正确路径
cargo test --test integration -- --ignored
```

## 配置项说明

| 配置项 | 环境变量 | 默认值 | 说明 |
|--------|---------|--------|------|
| `base_url` | `SIYUAN_API_URL` | `http://127.0.0.1:6806` | SiYuan API 基础 URL |
| `token` | `SIYUAN_API_TOKEN` | (空) | API 认证令牌（可选） |
| `timeout_secs` | `SIYUAN_TIMEOUT_SECS` | `30` | HTTP 请求超时时间（秒） |

## 安全提示

⚠️ **重要：** API Token 是敏感信息，请妥善保管：

1. **不要**将 token 提交到版本控制系统
2. **不要**在公开场合分享配置文件
3. 建议使用环境变量管理生产环境的 token
4. 配置文件权限应设置为仅所有者可读写：
   ```bash
   chmod 600 ~/.config/scribe/config.toml
   ```

## 故障排除

### 无法加载配置

如果配置加载失败，会使用默认配置（`http://127.0.0.1:6806`，无 token）。

**检查配置是否生效：**
1. 检查是否设置了 `SIYUAN_API_TOKEN` 环境变量
2. 检查配置文件是否存在于正确路径
3. 检查配置文件中是否包含 `token` 字段

### 配置文件路径

查看当前系统的配置文件路径：

```rust
use syservice::config::get_config_path;

if let Some(path) = get_config_path() {
    println!("Config path: {}", path.display());
}
```

### 测试配置是否生效

```bash
# 使用 cargo test 验证配置
cargo test -p syservice config -- --nocapture
```

## 示例配置文件

项目包含示例配置文件：`syservice/config.example.toml`

```bash
# 复制示例配置
cp syservice/config.example.toml ~/.config/scribe/config.toml

# 编辑配置
nano ~/.config/scribe/config.toml
```

## 跨平台支持

配置模块自动检测操作系统并选择正确的配置路径：

- **Linux**: 遵循 XDG Base Directory 规范
- **macOS**: 使用 `~/Library/Application Support/`
- **Windows**: 使用 `%APPDATA%`

## 迁移指南

### 从旧版本迁移

旧版本使用硬编码常量：

```rust
// 旧代码（已弃用）
use syservice::{SIYUAN_BASE, API_TOKEN};
let url = SIYUAN_BASE;
let token = API_TOKEN;
```

新代码应使用 `Config`：

```rust
// 新代码
use syservice::prelude::*;
let config = Config::load()?;
let url = config.base_url();
let token = config.token();
```

## 更多信息

- API 文档：`cargo doc -p syservice --open`
- 示例代码：`syservice/tests/integration.rs`
- 配置示例：`syservice/config.example.toml`
