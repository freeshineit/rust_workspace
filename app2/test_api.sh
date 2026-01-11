#!/bin/bash

# App2 API 测试脚本

BASE_URL="http://localhost:3000"

echo "🧪 测试 App2 Server API"
echo "========================"
echo ""

# 测试根路径
echo "1️⃣ 测试欢迎页面 (GET /)"
echo "---"
curl -s "$BASE_URL/" | head -n 20
echo ""
echo ""

# 测试健康检查
echo "2️⃣ 测试健康检查 (GET /health)"
echo "---"
curl -s "$BASE_URL/health" | jq '.'
echo ""
echo ""

# 测试 Hello API
echo "3️⃣ 测试 Hello API (GET /api/hello)"
echo "---"
curl -s "$BASE_URL/api/hello" | jq '.'
echo ""
echo ""

# 测试 Echo API
echo "4️⃣ 测试 Echo API (POST /api/echo)"
echo "---"
curl -s -X POST "$BASE_URL/api/echo" \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello from test script!"}' | jq '.'
echo ""
echo ""

# 测试 404
echo "5️⃣ 测试 404 (GET /nonexistent)"
echo "---"
curl -s "$BASE_URL/nonexistent" | jq '.'
echo ""
echo ""

echo "✅ 测试完成！"
