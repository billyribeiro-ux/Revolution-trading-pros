#!/bin/bash
# Create R2 API Token via Cloudflare API
# 
# You need a Cloudflare API Token with "Account R2 Read & Write" permissions
# Get it from: https://dash.cloudflare.com/profile/api-tokens

echo "Creating R2 API token via Cloudflare API..."
echo ""
echo "You need a Cloudflare API Token first."
echo "Get one here: https://dash.cloudflare.com/profile/api-tokens"
echo ""
echo "Create a token with these permissions:"
echo "  - Account > R2 > Edit"
echo ""
read -p "Paste your Cloudflare API Token: " CF_API_TOKEN

if [ -z "$CF_API_TOKEN" ]; then
    echo "Error: No API token provided"
    exit 1
fi

echo ""
echo "Creating R2 credentials..."

RESPONSE=$(curl -s -X POST "https://api.cloudflare.com/client/v4/accounts/9c72eb0d1b0b7891aca6532fe709cacc/r2/credentials" \
  -H "Authorization: Bearer $CF_API_TOKEN" \
  -H "Content-Type: application/json" \
  --data '{
    "name": "revolution-api-2026",
    "permissions": {
      "buckets": {
        "revolution-trading-media": {
          "read": true,
          "write": true
        }
      }
    }
  }')

echo ""
echo "Response:"
echo "$RESPONSE" | jq '.'

# Extract credentials if successful
if echo "$RESPONSE" | jq -e '.success == true' > /dev/null 2>&1; then
    echo ""
    echo "✅ SUCCESS! Here are your credentials:"
    echo ""
    echo "Access Key ID:"
    echo "$RESPONSE" | jq -r '.result.accessKeyId'
    echo ""
    echo "Secret Access Key:"
    echo "$RESPONSE" | jq -r '.result.secretAccessKey'
    echo ""
    echo "Copy these values and update api/.env"
else
    echo ""
    echo "❌ Failed to create token. Check the error above."
fi
