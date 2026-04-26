# Block Renderer Architecture

## Overview

The Block Renderer system is built using a modular, component-based architecture that prioritizes:

1. **Type Safety** - Comprehensive TypeScript types throughout
2. **Performance** - Optimized rendering with memoization and virtual scrolling
3. **Maintainability** - Clear separation of concerns and single responsibility
4. **Testability** - Isolated components with dependency injection
5. **Accessibility** - WCAG 2.1 AA compliant with ARIA support

## Directory Structure

```
lib/components/cms/blocks/
├── BlockRenderer.svelte          # Main orchestrator (100 lines)
├── types.ts                       # Shared TypeScript types
├── content/                       # Content blocks
│   ├── ParagraphBlock.svelte
│   ├── HeadingBlock.svelte
│   ├── QuoteBlock.svelte
│   ├── CodeBlock.svelte
│   └── ListBlock.svelte
├── media/                         # Media blocks
│   ├── ImageBlock.svelte
│   ├── VideoBlock.svelte
│   ├── AudioBlock.svelte
│   └── GalleryBlock.svelte
├── interactive/                   # Interactive blocks
│   ├── AccordionBlock.svelte
│   ├── TabsBlock.svelte
│   └── ToggleBlock.svelte
├── trading/                       # Trading-specific blocks
│   ├── TickerBlock.svelte
│   ├── PriceAlertBlock.svelte
│   └── TradingIdeaBlock.svelte
├── ai/                           # AI-powered blocks
│   ├── AIGeneratedBlock.svelte
│   ├── AISummaryBlock.svelte
│   └── AITranslationBlock.svelte
├── hooks/                        # Reusable composition hooks
│   ├── useMediaControls.svelte.ts
│   ├── useAIGeneration.svelte.ts
│   └── useBlockValidation.svelte.ts
└── __tests__/                    # Test suites
    ├── BlockRenderer.test.ts
    ├── useMediaControls.test.ts
    └── accessibility.test.ts
```

## Core Principles

### 1. Single Responsibility

Each block component handles ONE block type only:

```svelte
<!-- ParagraphBlock.svelte - Does ONE thing well -->
<script lang="ts">
	// Only paragraph-specific logic
</script>
```

### 2. Centralized State Management

All state lives in `BlockStateManager`:

```typescript
// ONE source of truth for ALL block state
const stateManager = getBlockStateManager();
let audioState = $derived(stateManager.getAudioState(blockId));
```

### 3. Composition Over Inheritance

Reusable logic via hooks:

```typescript
// Composable media controls
const controls = useMediaControls({ blockId, type: 'audio' });
```

### 4. Props Flow (Not Destructuring)

Maintain reactivity:

```typescript
// CORRECT - Reactive
const props: Props = $props();

// Access via props.block, props.isEditing
```

## Data Flow

```
User Action
    ↓
Event Handler (in block component)
    ↓
State Manager Update
    ↓
Derived State Recomputes
    ↓
Component Re-renders
    ↓
onUpdate Callback (propagates to parent)
```

## Performance Strategies

### 1. Memoization

```typescript
// Expensive computations cached
let blockStyles = $derived.by(() => {
	// Only recomputes when block.settings changes
	return computeStyles(block.settings);
});
```

### 2. Code Splitting

```typescript
// Lazy load heavy blocks
const HeavyBlock = lazy(() => import('./heavy/HeavyBlock.svelte'));
```

### 3. Virtual Scrolling

For lists with 100+ items, use virtual scrolling to render only visible items.

## Security

### HTML Sanitization

ALL user-generated HTML is sanitized:

```typescript
import { sanitizeHTML } from '$lib/utils/sanitization';

let safe = sanitizeHTML(userInput, { mode: 'strict' });
```

### XSS Prevention

- No `innerHTML` without sanitization
- All `{@html}` tags use sanitized content
- CSP headers configured

## Testing Strategy

### Unit Tests (80%+ coverage)

```bash
npm run test:unit
```

### E2E Tests

```bash
npm run test:e2e
```

### Accessibility Tests

```bash
npm run test:a11y
```

## Migration Guide

### From Old BlockRenderer

1. Install new dependencies:

```bash
npm install isomorphic-dompurify
```

2. Update imports:

```typescript
// OLD
import BlockRenderer from './BlockRenderer.svelte';

// NEW - same import, new implementation
import BlockRenderer from './BlockRenderer.svelte';
```

3. Wrap app with state provider:

```svelte
<script>
	import { BlockStateManager, setBlockStateManager } from '$lib/stores/blockState.svelte';

	const stateManager = new BlockStateManager();
	setBlockStateManager(stateManager);
</script>
```

4. Update tests to use new patterns

## Contributing

1. Create feature branch
2. Write tests FIRST
3. Implement feature
4. Ensure 80%+ coverage
5. Pass all CI checks
6. Submit PR

## Performance Benchmarks

Target metrics:

- Initial render: <16ms (60fps)
- Update render: <8ms
- Memory usage: <50MB for 100 blocks
- Bundle size: <50KB gzipped per block type

## Resources

- [Svelte 5 Runes Documentation](https://svelte-5-preview.vercel.app/docs/runes)
- [SvelteKit 2.x Guide](https://kit.svelte.dev/docs)
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html)
