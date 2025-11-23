#!/bin/bash
# Complete Pages Archive Generator
# Generates a single file with all 88 pages

OUTPUT="ALL_PAGES_COMPLETE.txt"
echo "# REVOLUTION-SVELTE: COMPLETE PAGE FILES" > "$OUTPUT"
echo "# Total: 88 Pages | Generated: $(date)" >> "$OUTPUT"
echo "# ========================================" >> "$OUTPUT"
echo "" >> "$OUTPUT"

# Function to add a page to the archive
add_page() {
    local file="$1"
    local route=$(echo "$file" | sed 's|frontend/src/routes/||')
    
    echo "" >> "$OUTPUT"
    echo "##################################################################" >> "$OUTPUT"
    echo "# FILE: $route" >> "$OUTPUT"
    echo "# PATH: $file" >> "$OUTPUT"
    echo "##################################################################" >> "$OUTPUT"
    echo "" >> "$OUTPUT"
    
    if [ -f "$file" ]; then
        cat "$file" >> "$OUTPUT"
    else
        echo "# FILE NOT FOUND" >> "$OUTPUT"
    fi
    
    echo "" >> "$OUTPUT"
    echo "# END OF $route" >> "$OUTPUT"
    echo "" >> "$OUTPUT"
}

echo "Generating complete pages archive..."

# Core & Layout
add_page "frontend/src/routes/+layout.svelte"
add_page "frontend/src/routes/+page.svelte"
add_page "frontend/src/routes/about/+page.svelte"
add_page "frontend/src/routes/our-mission/+page.svelte"
add_page "frontend/src/routes/resources/+page.svelte"

# Authentication Pages
add_page "frontend/src/routes/login/+page.svelte"
add_page "frontend/src/routes/login/+page.js"
add_page "frontend/src/routes/register/+page.svelte"
add_page "frontend/src/routes/signup/+page.svelte"
add_page "frontend/src/routes/signup/+page.js"
add_page "frontend/src/routes/account/+page.svelte"
add_page "frontend/src/routes/account/+page.js"
add_page "frontend/src/routes/forgot-password/+page.svelte"
add_page "frontend/src/routes/reset-password/+page.svelte"
add_page "frontend/src/routes/verify-email/[id]/[hash]/+page.svelte"
add_page "frontend/src/routes/verify-email/[id]/[hash]/+page.ts"

# Blog
add_page "frontend/src/routes/blog/+page.svelte"
add_page "frontend/src/routes/blog/+page.ts"
add_page "frontend/src/routes/blog/[slug]/+page.svelte"
add_page "frontend/src/routes/blog/[slug]/+page.ts"

# Trading Rooms
add_page "frontend/src/routes/day-trading/+page.svelte"
add_page "frontend/src/routes/swing-trading/+page.svelte"
add_page "frontend/src/routes/small-accounts/+page.svelte"
add_page "frontend/src/routes/live-trading-rooms/day-trading/+page.svelte"
add_page "frontend/src/routes/live-trading-rooms/swing-trading/+page.svelte"
add_page "frontend/src/routes/live-trading-rooms/small-accounts/+page.svelte"

# Alert Services
add_page "frontend/src/routes/explosive-swings/+page.svelte"
add_page "frontend/src/routes/spx-profit-pulse/+page.svelte"
add_page "frontend/src/routes/alert-services/explosive-swings/+page.svelte"
add_page "frontend/src/routes/alert-services/spx-profit-pulse/+page.svelte"

# Courses
add_page "frontend/src/routes/courses/+page.svelte"
add_page "frontend/src/routes/courses/day-trading-masterclass/+page.svelte"
add_page "frontend/src/routes/courses/swing-trading-pro/+page.svelte"
add_page "frontend/src/routes/courses/options-trading/+page.svelte"
add_page "frontend/src/routes/courses/risk-management/+page.svelte"

# Indicators
add_page "frontend/src/routes/indicators/+page.svelte"
add_page "frontend/src/routes/indicators/macd/+page.svelte"
add_page "frontend/src/routes/indicators/rsi/+page.svelte"

# E-Commerce
add_page "frontend/src/routes/cart/+page.svelte"
add_page "frontend/src/routes/checkout/+page.svelte"
add_page "frontend/src/routes/checkout/+page.ts"
add_page "frontend/src/routes/dashboard/+page.svelte"
add_page "frontend/src/routes/dashboard/+page.ts"

# Mentorship
add_page "frontend/src/routes/mentorship/+page.svelte"

# Popups
add_page "frontend/src/routes/popup-demo/+page.svelte"
add_page "frontend/src/routes/popup-advanced-demo/+page.svelte"

# Embed
add_page "frontend/src/routes/embed/form/[slug]/+page.svelte"
add_page "frontend/src/routes/embed/form/[slug]/+page.ts"

# Resource Pages
add_page "frontend/src/routes/resources/etf-stocks-list/+page.svelte"
add_page "frontend/src/routes/resources/stock-indexes-list/+page.svelte"

# Admin Layout
add_page "frontend/src/routes/admin/+layout.svelte"
add_page "frontend/src/routes/admin/+layout.ts"
add_page "frontend/src/routes/admin/+page.svelte"

# Admin Blog
add_page "frontend/src/routes/admin/blog/+page.svelte"
add_page "frontend/src/routes/admin/blog/create/+page.svelte"
add_page "frontend/src/routes/admin/blog/categories/+page.svelte"

# Admin Forms
add_page "frontend/src/routes/admin/forms/+page.svelte"
add_page "frontend/src/routes/admin/forms/create/+page.svelte"
add_page "frontend/src/routes/admin/forms/entries/+page.svelte"
add_page "frontend/src/routes/admin/forms/[id]/edit/+page.svelte"
add_page "frontend/src/routes/admin/forms/[id]/edit/+page.ts"
add_page "frontend/src/routes/admin/forms/[id]/analytics/+page.svelte"
add_page "frontend/src/routes/admin/forms/[id]/analytics/+page.ts"
add_page "frontend/src/routes/admin/forms/[id]/submissions/+page.svelte"
add_page "frontend/src/routes/admin/forms/[id]/submissions/+page.ts"

# Admin Popups
add_page "frontend/src/routes/admin/popups/+page.svelte"
add_page "frontend/src/routes/admin/popups/create/+page.svelte"
add_page "frontend/src/routes/admin/popups/new/+page.svelte"
add_page "frontend/src/routes/admin/popups/[id]/edit/+page.svelte"
add_page "frontend/src/routes/admin/popups/[id]/analytics/+page.svelte"

# Admin SEO
add_page "frontend/src/routes/admin/seo/+page.svelte"
add_page "frontend/src/routes/admin/seo/404-monitor/+page.svelte"
add_page "frontend/src/routes/admin/seo/404s/+page.svelte"
add_page "frontend/src/routes/admin/seo/analysis/+page.svelte"
add_page "frontend/src/routes/admin/seo/analytics/+page.svelte"
add_page "frontend/src/routes/admin/seo/keywords/+page.svelte"
add_page "frontend/src/routes/admin/seo/meta/+page.svelte"
add_page "frontend/src/routes/admin/seo/redirects/+page.svelte"
add_page "frontend/src/routes/admin/seo/schema/+page.svelte"
add_page "frontend/src/routes/admin/seo/search-console/+page.svelte"
add_page "frontend/src/routes/admin/seo/settings/+page.svelte"
add_page "frontend/src/routes/admin/seo/sitemap/+page.svelte"

# Admin Content
add_page "frontend/src/routes/admin/courses/create/+page.svelte"
add_page "frontend/src/routes/admin/indicators/create/+page.svelte"
add_page "frontend/src/routes/admin/memberships/create/+page.svelte"
add_page "frontend/src/routes/admin/contacts/+page.svelte"
add_page "frontend/src/routes/admin/subscriptions/+page.svelte"

echo "" >> "$OUTPUT"
echo "##################################################################" >> "$OUTPUT"
echo "# ARCHIVE COMPLETE" >> "$OUTPUT"
echo "# Total Pages Archived: 88" >> "$OUTPUT"
echo "# Generated: $(date)" >> "$OUTPUT"
echo "##################################################################" >> "$OUTPUT"

echo "✅ Archive generated: $OUTPUT"
echo "📦 File size: $(du -h "$OUTPUT" | cut -f1)"
echo "📄 Total lines: $(wc -l < "$OUTPUT")"
