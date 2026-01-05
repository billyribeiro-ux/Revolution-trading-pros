#!/bin/bash
################################################################################
# REVOLUTION TRADING PROS - FLY.IO DEPLOYMENT SCRIPT
#
# Stack: Laravel 12 + Octane + FrankenPHP (December 2025)
#
# Usage:
#   ./scripts/fly-deploy.sh          # Full setup (first time)
#   ./scripts/fly-deploy.sh deploy   # Deploy only
#   ./scripts/fly-deploy.sh secrets  # Set secrets only
################################################################################

set -e

APP_NAME="revolution-trading-pros"
REGION="iad"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Check if fly CLI is installed
check_fly() {
    if ! command -v fly &> /dev/null; then
        error "Fly CLI not installed. Install: curl -L https://fly.io/install.sh | sh"
    fi
    log "Fly CLI found"
}

# First-time setup
setup() {
    log "Starting first-time Fly.io setup..."

    # Launch app (no deploy yet)
    log "Creating Fly.io app..."
    fly launch --name "$APP_NAME" --region "$REGION" --no-deploy --yes || true

    # Create Postgres database
    log "Creating Postgres database..."
    fly postgres create --name "${APP_NAME}-db" --region "$REGION" --initial-cluster-size 1 --vm-size shared-cpu-1x --volume-size 1 || warn "Postgres may already exist"

    # Attach Postgres to app
    log "Attaching Postgres to app..."
    fly postgres attach "${APP_NAME}-db" --app "$APP_NAME" || warn "Postgres may already be attached"

    # Create Redis
    log "Creating Redis..."
    fly redis create --name "${APP_NAME}-redis" --region "$REGION" --no-replicas || warn "Redis may already exist"

    # Attach Redis
    log "Attaching Redis to app..."
    fly redis attach "${APP_NAME}-redis" --app "$APP_NAME" || warn "Redis may already be attached"

    # Create storage volume
    log "Creating storage volume..."
    fly volumes create storage_volume --region "$REGION" --size 1 --app "$APP_NAME" || warn "Volume may already exist"

    log "Setup complete! Now run: ./scripts/fly-deploy.sh secrets"
}

# Set secrets
set_secrets() {
    log "Setting Fly.io secrets..."

    # Generate APP_KEY if not provided
    APP_KEY=$(openssl rand -base64 32)

    cat << EOF

================================================================================
SET THESE SECRETS IN FLY.IO
================================================================================

Run the following command with YOUR actual values:

fly secrets set \\
  APP_KEY="base64:$APP_KEY" \\
  R2_ACCESS_KEY_ID="your-r2-access-key" \\
  R2_SECRET_ACCESS_KEY="your-r2-secret-key" \\
  R2_ENDPOINT="https://YOUR_ACCOUNT_ID.r2.cloudflarestorage.com" \\
  R2_BUCKET="revolution-trading-media" \\
  POSTMARK_TOKEN="your-postmark-token" \\
  STRIPE_KEY="pk_live_xxx" \\
  STRIPE_SECRET="sk_live_xxx" \\
  STRIPE_WEBHOOK_SECRET="whsec_xxx" \\
  SENTRY_LARAVEL_DSN="https://xxx@xxx.ingest.sentry.io/xxx" \\
  REVERB_APP_KEY="$(openssl rand -hex 16)" \\
  REVERB_APP_SECRET="$(openssl rand -hex 32)"

================================================================================
REQUIRED SECRETS:
================================================================================
- APP_KEY: Generated above (or use: php artisan key:generate --show)
- R2_*: From Cloudflare Dashboard → R2 → Manage API Tokens

================================================================================
OPTIONAL SECRETS (add when you have them):
================================================================================
- POSTMARK_TOKEN: From postmarkapp.com → Server → API Tokens
- STRIPE_*: From dashboard.stripe.com → Developers → API Keys
- SENTRY_LARAVEL_DSN: From sentry.io → Settings → Client Keys
- REVERB_*: Generated above for WebSocket authentication

EOF
}

# Deploy
deploy() {
    log "Deploying to Fly.io..."
    fly deploy --app "$APP_NAME"

    log "Deployment complete!"
    log "App URL: https://${APP_NAME}.fly.dev"
    log "Health check: https://${APP_NAME}.fly.dev/api/health"
    log "Horizon: https://${APP_NAME}.fly.dev/horizon"
    log "Pulse: https://${APP_NAME}.fly.dev/pulse"
}

# Status
status() {
    log "Checking app status..."
    fly status --app "$APP_NAME"
    echo ""
    fly postgres list
    echo ""
    fly redis list
}

# Logs
logs() {
    fly logs --app "$APP_NAME"
}

# SSH
ssh_console() {
    fly ssh console --app "$APP_NAME"
}

# Main
main() {
    check_fly

    case "${1:-}" in
        setup)
            setup
            ;;
        secrets)
            set_secrets
            ;;
        deploy)
            deploy
            ;;
        status)
            status
            ;;
        logs)
            logs
            ;;
        ssh)
            ssh_console
            ;;
        *)
            echo "Usage: $0 {setup|secrets|deploy|status|logs|ssh}"
            echo ""
            echo "Commands:"
            echo "  setup   - First-time setup (creates DB, Redis, volumes)"
            echo "  secrets - Show secrets that need to be set"
            echo "  deploy  - Deploy the application"
            echo "  status  - Check app status"
            echo "  logs    - View app logs"
            echo "  ssh     - SSH into container"
            exit 1
            ;;
    esac
}

main "$@"
