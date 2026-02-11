# Troubleshooting Guide

## Common Issues

### Build Errors

#### "Cannot find module '@sveltejs/kit'"

**Cause:** Dependencies not installed

**Solution:**

```bash
rm -rf node_modules package-lock.json
npm install
```

#### "Type error in blockState.svelte.ts"

**Cause:** TypeScript strict mode violations

**Solution:** Check that all types are properly defined. Run:

```bash
npm run check
```

### Runtime Errors

#### "getBlockStateManager() called before initialization"

**Cause:** State manager not initialized in layout

**Solution:** Ensure `+layout.svelte` initializes the state manager:

```svelte
<script>
	import { setBlockStateManager, BlockStateManager } from '$lib/stores/blockState.svelte';

	const stateManager = new BlockStateManager();
	setBlockStateManager(stateManager);
</script>
```

#### "Block not rendering"

**Cause:** Block type not registered in componentMap

**Solution:** Add block to `BlockRenderer.svelte`:

```typescript
import MyBlock from './custom/MyBlock.svelte';

const componentMap = {
	// ... existing mappings
	myBlock: MyBlock
};
```

### Performance Issues

#### "Slow rendering with many blocks"

**Solution:** Enable virtual scrolling:

```typescript
import { calculateVirtualScrollIndices } from '$lib/utils/performance';

// Use in your renderer
```

#### "Large bundle size"

**Solution:** Check bundle analyzer:

```bash
npm run analyze
```

Remove unused dependencies and ensure code splitting is working.

### Database Issues

#### "Prisma Client not generated"

**Solution:**

```bash
npx prisma generate
```

#### "Migration failed"

**Solution:**

```bash
npx prisma migrate reset
npx prisma migrate dev
```

### API Issues

#### "AI generation fails"

**Cause:** Missing or invalid API key

**Solution:** Check `.env` file:

```bash
VITE_ANTHROPIC_API_KEY=sk-ant-...
```

#### "Upload fails"

**Cause:** Missing upload handler or storage not configured

**Solution:** Implement upload handler in `src/routes/api/cms/upload/image/+server.ts`

### Testing Issues

#### "E2E tests fail"

**Solution:** Ensure dev server is running:

```bash
# Terminal 1
npm run dev

# Terminal 2
npm run test:e2e
```

#### "Tests timeout"

**Solution:** Increase timeout in `playwright.config.ts`:

```typescript
timeout: 60000; // 60 seconds
```

## Debug Mode

Enable debug logging:

```bash
DEBUG=* npm run dev
```

## Getting Help

1. Check [Issues](https://github.com/yourusername/repo/issues)
2. Search [Discussions](https://github.com/yourusername/repo/discussions)
3. Email: support@revolutiontradingpros.com
