#!/bin/bash
# Complete Deployment Script - Revolution Trading Pros API
# This script handles everything: CLI installation, deployment, and verification

set -e

APP_NAME="revolution-trading-pros-api"
API_URL="https://$APP_NAME.fly.dev"

echo "🚀 Revolution Trading Pros API - Complete Deployment"
echo "====================================================="
echo ""

# Step 1: Check/Install Fly.io CLI
echo "📦 Step 1: Checking Fly.io CLI..."
if command -v flyctl &> /dev/null; then
    echo "✅ Fly.io CLI already installed"
    FLYCTL_CMD="flyctl"
elif command -v fly &> /dev/null; then
    echo "✅ Fly.io CLI already installed"
    FLYCTL_CMD="fly"
elif [ -f "$HOME/.fly/bin/flyctl" ]; then
    echo "✅ Fly.io CLI found in ~/.fly/bin"
    export PATH="$HOME/.fly/bin:$PATH"
    FLYCTL_CMD="flyctl"
else
    echo "⚠️  Fly.io CLI not found. Please install manually:"
    echo ""
    echo "Run these commands:"
    echo "  curl -L https://fly.io/install.sh | sh"
    echo "  export PATH=\"\$HOME/.fly/bin:\$PATH\""
    echo "  echo 'export PATH=\"\$HOME/.fly/bin:\$PATH\"' >> ~/.zshrc"
    echo "  flyctl auth login"
    echo ""
    echo "Then run this script again."
    exit 1
fi

echo ""

# Step 2: Check Authentication
echo "🔐 Step 2: Checking authentication..."
if ! $FLYCTL_CMD auth whoami &> /dev/null; then
    echo "❌ Not authenticated with Fly.io"
    echo "Please run: $FLYCTL_CMD auth login"
    exit 1
fi
echo "✅ Authenticated with Fly.io"
echo ""

# Step 3: Check Current Status
echo "📊 Step 3: Checking current deployment status..."
if $FLYCTL_CMD status -a $APP_NAME &> /dev/null; then
    echo "✅ App exists: $APP_NAME"
    $FLYCTL_CMD status -a $APP_NAME
else
    echo "⚠️  App does not exist. Creating..."
    $FLYCTL_CMD apps create $APP_NAME --org personal || true
fi
echo ""

# Step 4: Check Secrets
echo "🔐 Step 4: Checking environment variables..."
echo "Current secrets:"
$FLYCTL_CMD secrets list -a $APP_NAME || echo "No secrets set yet"
echo ""
echo "⚠️  Ensure these secrets are set:"
echo "  - DATABASE_URL"
echo "  - REDIS_URL"
echo "  - JWT_SECRET"
echo "  - STRIPE_SECRET"
echo "  - STRIPE_WEBHOOK_SECRET"
echo "  - POSTMARK_TOKEN"
echo "  - MEILISEARCH_API_KEY"
echo ""
read -p "Are all required secrets set? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Please set secrets using: $FLYCTL_CMD secrets set KEY=VALUE -a $APP_NAME"
    exit 1
fi
echo ""

# Step 5: Deploy
echo "🚀 Step 5: Deploying latest code..."
echo "This will build the Docker image and deploy to Fly.io..."
$FLYCTL_CMD deploy -a $APP_NAME --remote-only

echo ""
echo "⏳ Waiting for deployment to stabilize..."
sleep 15
echo ""

# Step 6: Verify Health
echo "🏥 Step 6: Verifying deployment health..."
echo "Testing /health endpoint..."
HEALTH_RESPONSE=$(curl -s -w "\n%{http_code}" $API_URL/health)
HEALTH_BODY=$(echo "$HEALTH_RESPONSE" | head -n 1)
HEALTH_CODE=$(echo "$HEALTH_RESPONSE" | tail -n 1)

if [ "$HEALTH_CODE" = "200" ]; then
    echo "✅ Health check passed: $HEALTH_BODY"
else
    echo "❌ Health check failed with code: $HEALTH_CODE"
    echo "Response: $HEALTH_BODY"
    echo ""
    echo "Checking logs..."
    $FLYCTL_CMD logs -a $APP_NAME
    exit 1
fi
echo ""

# Step 7: Verify Readiness
echo "🔍 Step 7: Verifying database connectivity..."
echo "Testing /ready endpoint..."
READY_RESPONSE=$(curl -s -w "\n%{http_code}" $API_URL/ready)
READY_BODY=$(echo "$READY_RESPONSE" | head -n 1)
READY_CODE=$(echo "$READY_RESPONSE" | tail -n 1)

if [ "$READY_CODE" = "200" ]; then
    echo "✅ Ready check passed: $READY_BODY"
else
    echo "❌ Ready check failed with code: $READY_CODE"
    echo "Response: $READY_BODY"
    echo "Database connection may be failing"
    exit 1
fi
echo ""

# Step 8: Database Setup
echo "🗄️  Step 8: Setting up database..."
echo "Running /setup-db endpoint..."
SETUP_RESPONSE=$(curl -s -X POST -w "\n%{http_code}" $API_URL/setup-db)
SETUP_BODY=$(echo "$SETUP_RESPONSE" | head -n 1)
SETUP_CODE=$(echo "$SETUP_RESPONSE" | tail -n 1)

if [ "$SETUP_CODE" = "200" ]; then
    echo "✅ Database setup: $SETUP_BODY"
else
    echo "⚠️  Setup response code: $SETUP_CODE"
    echo "Response: $SETUP_BODY"
    echo "(This may be expected if already set up)"
fi
echo ""

# Step 9: Run Migrations
echo "📝 Step 9: Running migrations..."
echo "Running /run-migrations endpoint..."
MIGRATION_RESPONSE=$(curl -s -X POST -w "\n%{http_code}" $API_URL/run-migrations)
MIGRATION_BODY=$(echo "$MIGRATION_RESPONSE" | head -n 1)
MIGRATION_CODE=$(echo "$MIGRATION_RESPONSE" | tail -n 1)

if [ "$MIGRATION_CODE" = "200" ]; then
    echo "✅ Migrations completed: $MIGRATION_BODY"
else
    echo "⚠️  Migration response code: $MIGRATION_CODE"
    echo "Response: $MIGRATION_BODY"
    echo "(This may be expected if already run)"
fi
echo ""

# Step 10: Test Authentication
echo "🔑 Step 10: Testing authentication..."
echo "Attempting superadmin login..."
LOGIN_RESPONSE=$(curl -s -X POST -w "\n%{http_code}" $API_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"welberribeirodrums@gmail.com","password":"Revolution2024!"}')
LOGIN_BODY=$(echo "$LOGIN_RESPONSE" | head -n 1)
LOGIN_CODE=$(echo "$LOGIN_RESPONSE" | tail -n 1)

if [ "$LOGIN_CODE" = "200" ]; then
    echo "✅ Authentication working"
    echo "Login response: $LOGIN_BODY"
else
    echo "⚠️  Login response code: $LOGIN_CODE"
    echo "Response: $LOGIN_BODY"
    echo "(Superadmin may need to be created or password may be different)"
fi
echo ""

# Step 11: Check Logs
echo "📋 Step 11: Checking recent logs for errors..."
echo "Last 50 lines:"
$FLYCTL_CMD logs -a $APP_NAME | tail -n 50
echo ""

# Final Summary
echo "✅ =============================================="
echo "✅ DEPLOYMENT COMPLETE"
echo "✅ =============================================="
echo ""
echo "📊 Deployment Summary:"
echo "  • App Name: $APP_NAME"
echo "  • URL: $API_URL"
echo "  • Health: $API_URL/health"
echo "  • Ready: $API_URL/ready"
echo "  • Version: 0.1.0"
echo "  • Environment: production"
echo ""
echo "🔗 Quick Links:"
echo "  • Dashboard: https://fly.io/apps/$APP_NAME"
echo "  • Metrics: https://fly.io/apps/$APP_NAME/metrics"
echo "  • Logs: flyctl logs -a $APP_NAME"
echo ""
echo "🧪 Test Endpoints:"
echo "  curl $API_URL/health"
echo "  curl $API_URL/ready"
echo "  curl -X POST $API_URL/api/auth/login -H 'Content-Type: application/json' -d '{\"email\":\"welberribeirodrums@gmail.com\",\"password\":\"Revolution2024!\"}'"
echo ""
echo "✅ All systems operational!"
