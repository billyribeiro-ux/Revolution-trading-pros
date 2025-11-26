#!/bin/bash

# Performance Testing Script
# Tests the optimizations implemented

echo "🚀 Revolution Trading Pros - Performance Test"
echo "=============================================="
echo ""

# Check if build exists
if [ ! -d "build" ]; then
    echo "📦 Building production bundle..."
    npm run build
    echo ""
fi

# Start preview server in background
echo "🌐 Starting preview server..."
npm run preview &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo ""
echo "✅ Server started (PID: $SERVER_PID)"
echo ""
echo "📊 Performance Checks:"
echo "--------------------"

# Check if service worker exists
if [ -f "static/sw.js" ]; then
    echo "✅ Service Worker: Present"
else
    echo "❌ Service Worker: Missing"
fi

# Check if offline page exists
if [ -f "static/offline.html" ]; then
    echo "✅ Offline Page: Present"
else
    echo "❌ Offline Page: Missing"
fi

# Check if LazySection component exists
if [ -f "src/lib/components/LazySection.svelte" ]; then
    echo "✅ LazySection Component: Present"
else
    echo "❌ LazySection Component: Missing"
fi

# Check if performance utils exist
if [ -f "src/lib/utils/performance.ts" ]; then
    echo "✅ Performance Monitoring: Present"
else
    echo "❌ Performance Monitoring: Missing"
fi

# Check bundle size
if [ -d "build" ]; then
    TOTAL_SIZE=$(du -sh build | cut -f1)
    echo "✅ Build Size: $TOTAL_SIZE"
    
    # Count JS files
    JS_COUNT=$(find build -name "*.js" | wc -l | tr -d ' ')
    echo "✅ JavaScript Files: $JS_COUNT"
    
    # Count CSS files
    CSS_COUNT=$(find build -name "*.css" | wc -l | tr -d ' ')
    echo "✅ CSS Files: $CSS_COUNT"
fi

echo ""
echo "🎯 Next Steps:"
echo "-------------"
echo "1. Open http://localhost:4173 in Chrome"
echo "2. Open DevTools (F12)"
echo "3. Go to Lighthouse tab"
echo "4. Run Performance audit"
echo "5. Check Core Web Vitals:"
echo "   - LCP should be < 2.5s"
echo "   - FID should be < 100ms"
echo "   - CLS should be < 0.1"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Wait for user to stop
wait $SERVER_PID
