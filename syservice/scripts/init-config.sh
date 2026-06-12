#!/bin/bash
# SiYuan 配置初始化脚本
# 自动创建配置文件并引导用户输入（token 仅写入本地，不会进入 git）

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

echo "SiYuan 配置初始化"
echo "===================="
echo ""
echo "配置文件路径：$CONFIG_FILE"
echo ""

read -p "请输入 SiYuan API Token (设置 -> 关于): " -s API_TOKEN
echo ""

read -p "API URL (默认：http://127.0.0.1:6806): " API_URL
API_URL=${API_URL:-http://127.0.0.1:6806}

read -p "workspace_dir (思源 data 目录，可留空稍后编辑): " WORKSPACE_DIR

mkdir -p "$CONFIG_DIR"

if [[ -n "$WORKSPACE_DIR" ]]; then
    WORKSPACE_LINE="workspace_dir = \"${WORKSPACE_DIR}\""
else
    WORKSPACE_LINE="# workspace_dir = \"/path/to/SiYuanKnowledgeBase/data\""
fi

cat > "$CONFIG_FILE" << EOF
# SiYuan 配置文件
# 生成时间：$(date '+%Y-%m-%d %H:%M:%S')

base_url = "$API_URL"
token = "$API_TOKEN"
timeout_secs = 30
$WORKSPACE_LINE
EOF

chmod 600 "$CONFIG_FILE"

echo ""
echo "配置文件已创建：$CONFIG_FILE"
echo ""
echo "测试：cargo test -p syservice --test integration -- --ignored"
echo "文档：syservice/CONFIG.md"
