#!/bin/bash
# Quick Deployment Script for Revolution Trading Pros API
# Run this after installing Fly.io CLI

set -e

APP_NAME="revolution-trading-pros-api"

echo "🚀 Revolution Trading Pros API - Quick Deploy"
echo "=============================================="
echo ""

# Check if flyctl is installed
if ! command -v flyctl &> /dev/null; then
    echo "❌ Fly.io CLI not found. Installing..."
    curl -L https://fly.io/install.sh | sh
    export PATH="$HOME/.fly/bin:$PATH"
    echo "✅ Fly.io CLI installed"
    echo ""
    echo "⚠️  Please run: export PATH=\"\$HOME/.fly/bin:\$PATH\""
    echo "⚠️  Then run this script again"
    exit 1
fi

# Check if authenticated
if ! flyctl auth whoami &> /dev/null; then
    echo "🔐 Not authenticated. Please login..."
    flyctl auth login
fi

echo "📊 Checking current deployment status..."
flyctl status -a $APP_NAME || echo "App may not exist yet"
echo ""

echo "🔨 Building and deploying..."
flyctl deploy -a $APP_NAME --remote-only

echo ""
echo "✅ Deployment initiated!"
echo ""
echo "🔍 Checking health..."
sleep 10

# Test health endpoint
echo "Testing /health endpoint..."
HEALTH_RESPONSE=$(curl -s https://$APP_NAME.fly.dev/health || echo "failed")
echo "Response: $HEALTH_RESPONSE"
echo ""

# Test ready endpoint
echo "Testing /ready endpoint..."
READY_RESPONSE=$(curl -s https://$APP_NAME.fly.dev/ready || echo "failed")
echo "Response: $READY_RESPONSE"
echo ""

if [[ $HEALTH_RESPONSE == *"healthy"* ]]; then
    echo "✅ API is healthy and running!"
    echo ""
    echo "🌐 API URL: https://$APP_NAME.fly.dev"
    echo "🏥 Health: https://$APP_NAME.fly.dev/health"
    echo "📊 Ready: https://$APP_NAME.fly.dev/ready"
    echo ""
    echo "📋 Next steps:"
    echo "1. Run database setup: curl -X POST https://$APP_NAME.fly.dev/setup-db"
    echo "2. Run migrations: curl -X POST https://$APP_NAME.fly.dev/run-migrations"
    echo "3. Test login: curl -X POST https://$APP_NAME.fly.dev/api/auth/login -H 'Content-Type: application/json' -d '{\"email\":\"welberribeirodrums@gmail.com\",\"password\":\"Revolution2024!\"}'"
else
    echo "⚠️  Health check failed. Checking logs..."
    flyctl logs -a $APP_NAME
    echo ""
    echo "❌ Deployment may have issues. Please check logs above."
    exit 1
fi
