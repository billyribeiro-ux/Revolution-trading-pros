#!/bin/bash

set -e

echo "Creating Cloudflare API Token with full Pages permissions..."
echo ""

# SECURITY: Credentials must be set via environment variables
# Export these before running the script:
#   export CLOUDFLARE_EMAIL="your-email@example.com"
#   export CLOUDFLARE_API_KEY="your-global-api-key"
#   export CLOUDFLARE_ACCOUNT_ID="your-account-id"

if [ -z "$CLOUDFLARE_EMAIL" ] || [ -z "$CLOUDFLARE_API_KEY" ] || [ -z "$CLOUDFLARE_ACCOUNT_ID" ]; then
    echo "ERROR: Required environment variables are not set."
    echo "Please set the following environment variables:"
    echo "  CLOUDFLARE_EMAIL"
    echo "  CLOUDFLARE_API_KEY"
    echo "  CLOUDFLARE_ACCOUNT_ID"
    exit 1
fi

CLOUDFLARE_GLOBAL_API_KEY="$CLOUDFLARE_API_KEY"
ACCOUNT_ID="$CLOUDFLARE_ACCOUNT_ID"

# Create API token with all required permissions for Cloudflare Pages
echo "Creating new API token with comprehensive permissions..."

RESPONSE=$(curl -s -X POST "https://api.cloudflare.com/client/v4/user/tokens" \
    -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
    -H "X-Auth-Key: $CLOUDFLARE_GLOBAL_API_KEY" \
    -H "Content-Type: application/json" \
    --data "{
        \"name\": \"GitHub Actions - Pages Full Access $(date +%Y%m%d-%H%M%S)\",
        \"policies\": [
            {
                \"effect\": \"allow\",
                \"resources\": {
                    \"com.cloudflare.api.account.$ACCOUNT_ID\": \"*\"
                },
                \"permission_groups\": [
                    {
                        \"id\": \"c8fed203ed3043cba015a93ad1616f1f\",
                        \"name\": \"Cloudflare Pages\"
                    },
                    {
                        \"id\": \"e086da7e2179491d91ee5f35b3ca210a\",
                        \"name\": \"Workers Scripts\"
                    },
                    {
                        \"id\": \"b8f4f7c0f5e04e1f8e1e1e1e1e1e1e1e\",
                        \"name\": \"Account Settings\"
                    }
                ]
            },
            {
                \"effect\": \"allow\",
                \"resources\": {
                    \"com.cloudflare.api.user.$ACCOUNT_ID\": \"*\"
                },
                \"permission_groups\": [
                    {
                        \"id\": \"4755a26eedb94da69e1066d98aa820be\",
                        \"name\": \"User Details\"
                    }
                ]
            }
        ]
    }")

SUCCESS=$(echo "$RESPONSE" | jq -r '.success')

if [ "$SUCCESS" = "true" ]; then
    NEW_TOKEN=$(echo "$RESPONSE" | jq -r '.result.value')
    echo "✅ Token created successfully!"
    echo ""
    echo "Updating GitHub secrets..."
    
    echo "$NEW_TOKEN" | gh secret set CLOUDFLARE_API_TOKEN
    
    echo "✅ GitHub secret updated!"
    echo ""
    echo "Re-running failed workflow..."
    
    RUN_ID=$(gh run list --workflow=deploy-cloudflare.yml --limit 1 --json databaseId --jq '.[0].databaseId')
    
    if [ -n "$RUN_ID" ]; then
        gh run rerun $RUN_ID --failed
        echo "✅ Workflow re-run triggered!"
        echo ""
        echo "Watch progress: gh run watch $RUN_ID"
    fi
else
    echo "❌ Failed to create token"
    echo "Response: $RESPONSE"
    
    # Try simpler approach - just Cloudflare Pages with Edit permission
    echo ""
    echo "Trying with minimal permissions..."
    
    RESPONSE2=$(curl -s -X POST "https://api.cloudflare.com/client/v4/user/tokens" \
        -H "X-Auth-Email: $CLOUDFLARE_EMAIL" \
        -H "X-Auth-Key: $CLOUDFLARE_GLOBAL_API_KEY" \
        -H "Content-Type: application/json" \
        --data "{
            \"name\": \"GitHub Actions - Pages Minimal $(date +%Y%m%d-%H%M%S)\",
            \"policies\": [
                {
                    \"effect\": \"allow\",
                    \"resources\": {
                        \"com.cloudflare.api.account.$ACCOUNT_ID\": \"*\"
                    },
                    \"permission_groups\": [
                        {
                            \"id\": \"c8fed203ed3043cba015a93ad1616f1f\"
                        }
                    ]
                }
            ],
            \"condition\": {
                \"request.ip\": {
                    \"in\": []
                }
            }
        }")
    
    SUCCESS2=$(echo "$RESPONSE2" | jq -r '.success')
    
    if [ "$SUCCESS2" = "true" ]; then
        NEW_TOKEN=$(echo "$RESPONSE2" | jq -r '.result.value')
        echo "✅ Token created with minimal permissions!"
        echo ""
        echo "Updating GitHub secrets..."
        
        echo "$NEW_TOKEN" | gh secret set CLOUDFLARE_API_TOKEN
        
        echo "✅ GitHub secret updated!"
        echo ""
        echo "Re-running failed workflow..."
        
        RUN_ID=$(gh run list --workflow=deploy-cloudflare.yml --limit 1 --json databaseId --jq '.[0].databaseId')
        
        if [ -n "$RUN_ID" ]; then
            gh run rerun $RUN_ID --failed
            echo "✅ Workflow re-run triggered!"
        fi
    else
        echo "❌ Both attempts failed"
        echo "Response: $RESPONSE2"
        exit 1
    fi
fi
