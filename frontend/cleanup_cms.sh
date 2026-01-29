#!/bin/bash
# Comprehensive CMS cleanup script

echo "=========================================="
echo "CMS Cleanup - Removing all CMS references"
echo "=========================================="
echo ""

# Remove CMS-related store files
echo "🗑️  Removing CMS stores..."
rm -f src/lib/stores/comments.svelte.ts
rm -f src/lib/stores/autosave.svelte.ts
rm -f src/lib/stores/editor.svelte.ts

# Remove CMS-related utility files
echo "🗑️  Removing CMS utilities..."
rm -f src/lib/page-builder/registry.ts
rm -f src/lib/page-builder/types.ts
rm -rf src/lib/page-builder

# Remove CMS routes (already done but double-check)
echo "🗑️  Removing CMS routes..."
rm -rf src/routes/admin/cms
rm -rf src/routes/admin/cms-v2

# Remove CMS components (already done but double-check)
echo "🗑️  Removing CMS components..."
rm -rf src/lib/components/cms
rm -rf src/lib/components/cms-v2

# Remove CMS API files (already done but double-check)
echo "🗑️  Removing CMS API files..."
rm -f src/lib/api/cms-v2.ts

echo ""
echo "✅ CMS files removed"
echo ""
echo "📝 Files that need manual editing:"
echo "   - src/routes/admin/+page.svelte (remove CMS navigation)"
echo "   - src/routes/admin/consent/settings/+page.svelte"
echo "   - src/routes/admin/schedules/+page.svelte"
echo "   - src/routes/atom.xml/+server.ts"
echo "   - src/routes/feed.xml/+server.ts"
echo "   - src/lib/utils/keyboard-shortcuts.ts"
