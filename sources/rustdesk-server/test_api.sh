#!/bin/bash
# RustDesk HTTP API 测试脚本

echo "=========================================="
echo "  RustDesk HTTP API 测试"
echo "=========================================="
echo ""

# 测试健康检查
echo "1. 测试健康检查端点..."
echo "   GET http://localhost:21119/api/health"
echo ""
curl -s http://localhost:21119/api/health | jq . 2>/dev/null || curl -s http://localhost:21119/api/health
echo ""
echo ""

# 测试在线客户端列表
echo "2. 测试在线客户端列表..."
echo "   GET http://localhost:21119/api/online-peers"
echo ""
curl -s http://localhost:21119/api/online-peers | jq . 2>/dev/null || curl -s http://localhost:21119/api/online-peers
echo ""
echo ""

echo "=========================================="
echo "  测试完成"
echo "=========================================="
