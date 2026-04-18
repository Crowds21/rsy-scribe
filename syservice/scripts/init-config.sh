#!/bin/bash
# SiYuan 配置初始化脚本
# 自动创建配置文件并引导用户输入 API Token

set -e

# 检测操作系统
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CONFIG_DIR="$HOME/.config/scribe"
    CONFIG_FILE="$CONFIG_DIR/config.toml"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    CONFIG_DIR="$HOME/Library/Application Support/scribe"
    CONFIG_FILE="$CONFIG_DIR/config.toml"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

echo "📝 SiYuan 配置初始化"
echo "===================="
echo ""
echo "配置文件路径：$CONFIG_FILE"
echo ""

# 获取 API Token
read -p "请输入 SiYuan API Token (在 设置 -> 关于 中获取): " -s API_TOKEN
echo ""

# 获取 API URL（可选）
read -p "API URL (默认：http://127.0.0.1:6806): " API_URL
API_URL=${API_URL:-http://127.0.0.1:6806}

# 创建配置目录
mkdir -p "$CONFIG_DIR"

# 生成配置文件
cat > "$CONFIG_FILE" << EOF
# SiYuan 配置文件
# 生成时间：$(date '+%Y-%m-%d %H:%M:%S')

base_url = "$API_URL"
token = "$API_TOKEN"
timeout_secs = 30
max_retries = 3
retry_delay_ms = 100
EOF

# 设置文件权限（仅所有者可读写）
chmod 600 "$CONFIG_FILE"

echo ""
echo "✅ 配置文件已创建：$CONFIG_FILE"
echo ""
echo "测试配置："
echo "  cargo test --test integration -- --ignored"
echo ""
echo "📖 更多信息请查看：syservice/CONFIG.md"
