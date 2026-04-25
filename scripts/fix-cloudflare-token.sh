#!/bin/bash

echo "=============================================="
echo "Cloudflare API Token Setup for GitHub Actions"
echo "=============================================="
echo ""
echo "1. In the Cloudflare dashboard (now open), click 'Create Token'"
echo ""
echo "2. Use 'Create Custom Token' with these settings:"
echo "   - Token name: GitHub Actions - Pages Deploy"
echo "   - Permissions:"
echo "     * Account → Cloudflare Pages → Edit"
echo "   - Account Resources:"
echo "     * Include → Your Account"
echo ""
echo "3. Click 'Continue to summary' then 'Create Token'"
echo ""
echo "4. Copy the token and paste it below when prompted"
echo ""
read -p "Press Enter when you have the token ready..."
echo ""
read -sp "Paste your new Cloudflare API token: " NEW_TOKEN
echo ""
echo ""

if [ -z "$NEW_TOKEN" ]; then
    echo "❌ No token provided. Exiting."
    exit 1
fi

echo "Updating GitHub secret CLOUDFLARE_API_TOKEN..."
echo "$NEW_TOKEN" | gh secret set CLOUDFLARE_API_TOKEN

if [ $? -eq 0 ]; then
    echo "✅ GitHub secret updated successfully!"
    echo ""
    echo "Now re-running the failed GitHub Actions workflow..."
    
    # Get the latest workflow run ID
    RUN_ID=$(gh run list --workflow=deploy-cloudflare.yml --limit 1 --json databaseId --jq '.[0].databaseId')
    
    if [ -n "$RUN_ID" ]; then
        gh run rerun $RUN_ID --failed
        echo "✅ Workflow re-run triggered!"
        echo ""
        echo "View status: gh run watch $RUN_ID"
    else
        echo "⚠️  Could not find workflow run to re-run. Trigger manually with:"
        echo "   gh workflow run deploy-cloudflare.yml"
    fi
else
    echo "❌ Failed to update GitHub secret"
    exit 1
fi
