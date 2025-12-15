#!/bin/bash
#
# Revolution Trading Pros - Rust API Deployment Script
# Usage: ./scripts/deploy.sh [command]
#
# Commands:
#   setup    - Initial setup (create app, database)
#   deploy   - Deploy the application
#   secrets  - Set secrets from .env file
#   logs     - View application logs
#   ssh      - SSH into running machine
#

set -e

APP_NAME="revolution-trading-pros-api"

case "${1:-deploy}" in
    setup)
        echo "🚀 Setting up Fly.io application..."

        # Create app
        fly apps create $APP_NAME --org personal || true

        echo "✅ App created: $APP_NAME"
        echo ""
        echo "📋 Next steps:"
        echo "1. Set up Neon PostgreSQL: https://console.neon.tech"
        echo "2. Set up Upstash Redis: https://console.upstash.com"
        echo "3. Run: ./scripts/deploy.sh secrets"
        echo "4. Run: ./scripts/deploy.sh deploy"
        ;;

    secrets)
        echo "🔐 Setting secrets..."

        if [ ! -f .env ]; then
            echo "❌ .env file not found. Copy .env.example to .env and fill in values."
            exit 1
        fi

        # Read .env and set secrets
        while IFS='=' read -r key value; do
            # Skip comments and empty lines
            [[ $key =~ ^#.*$ ]] && continue
            [[ -z $key ]] && continue

            # Remove quotes from value
            value="${value%\"}"
            value="${value#\"}"

            echo "Setting $key..."
            fly secrets set "$key=$value" -a $APP_NAME
        done < .env

        echo "✅ Secrets set"
        ;;

    deploy)
        echo "🚀 Deploying to Fly.io..."
        fly deploy -a $APP_NAME
        echo "✅ Deployed!"
        echo ""
        echo "🌐 Your API is live at: https://$APP_NAME.fly.dev"
        echo "🏥 Health check: https://$APP_NAME.fly.dev/health"
        ;;

    logs)
        echo "📋 Viewing logs..."
        fly logs -a $APP_NAME
        ;;

    ssh)
        echo "🔌 Connecting via SSH..."
        fly ssh console -a $APP_NAME
        ;;

    status)
        echo "📊 Application status..."
        fly status -a $APP_NAME
        ;;

    *)
        echo "Usage: ./scripts/deploy.sh [setup|secrets|deploy|logs|ssh|status]"
        exit 1
        ;;
esac
