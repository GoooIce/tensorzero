#!/bin/bash

echo "🔍 TensorZero 配置验证"
echo "======================"

# 检查配置文件
if [ -f "config/tensorzero.toml" ]; then
    echo "✅ 配置文件存在: config/tensorzero.toml"
    
    # 检查数据库配置 (现在通过环境变量)
    if grep -q "TENSORZERO_CLICKHOUSE_URL" docker-compose.yml; then
        echo "✅ 数据库配置: ClickHouse (通过环境变量)"
    else
        echo "⚠️  数据库配置: 未在Docker Compose中找到"
    fi
    
    # 检查模型配置
    model_count=$(grep -c "\[models\." config/tensorzero.toml)
    echo "📊 配置的模型数量: $model_count"
    
else
    echo "❌ 配置文件不存在: config/tensorzero.toml"
    exit 1
fi

# 检查Docker Compose
if [ -f "docker-compose.yml" ]; then
    echo "✅ Docker Compose 文件存在"
    
    # 检查服务
    if grep -q "clickhouse:" docker-compose.yml; then
        echo "✅ ClickHouse 服务已配置"
    fi
    
    if grep -q "gateway:" docker-compose.yml; then
        echo "✅ Gateway 服务已配置"
    fi
    
    if grep -q "ui:" docker-compose.yml; then
        echo "✅ UI 服务已配置"
    fi
else
    echo "❌ docker-compose.yml 不存在"
fi

echo ""
echo "🚀 准备启动测试..."
echo ""

# 环境变量检查
./config/env-check.sh 