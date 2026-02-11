# Component Guide

## BlockRenderer

Master component that renders all block types.

### Props

```typescript
interface Props {
	block: Block; // Block data
	isEditing?: boolean; // Edit mode
	isSelected?: boolean; // Selection state
	onUpdate?: (updates: Partial<Block>) => void;
	onError?: (error: Error) => void;
}
```

### Usage

```svelte
<script>
	import BlockRenderer from '$lib/components/cms/blocks/BlockRenderer.svelte';

	let block = {
		id: '1',
		type: 'paragraph',
		content: { text: 'Hello world' },
		settings: {},
		metadata: {}
	};
</script>

<BlockRenderer
	{block}
	isEditing={true}
	onUpdate={(updates) => {
		block = { ...block, ...updates };
	}}
/>
```

## Block State Manager

Centralized state management for all blocks.

### Methods

```typescript
// Initialize
const stateManager = new BlockStateManager();
setBlockStateManager(stateManager);

// Get manager
const manager = getBlockStateManager();

// Accordion
manager.toggleAccordionItem(blockId, itemId, allowMultiple);
manager.getAccordionState(blockId);

// Tabs
manager.setActiveTab(blockId, tabId);
manager.getActiveTab(blockId, defaultTabId);

// Lightbox
manager.openLightbox(blockId, index, total);
manager.closeLightbox();
manager.navigateLightbox(direction);

// Audio/Video
manager.setAudioState(blockId, { playing: true });
manager.getAudioState(blockId);

// Cleanup
manager.cleanup(blockId);
manager.cleanupAll();
```

## Hooks

### useMediaControls

Media player control hook.

```typescript
const controls = useMediaControls({
	blockId,
	mediaElement: audioElement,
	onEnded: () => console.log('Playback ended'),
	onError: (error) => console.error(error)
});

// Usage
controls.togglePlay();
controls.setVolume(0.5);
controls.seek(30);
```

### useAIGeneration

AI content generation hook.

```typescript
const ai = useAIGeneration({
	blockId,
	type: 'generate',
	onSuccess: (output) => console.log(output),
	onError: (error) => console.error(error)
});

// Usage
await ai.generate('Write about trading');
```

## Utilities

### Sanitization

```typescript
import { sanitizeHTML, sanitizeURL, validateFile } from '$lib/utils/sanitization';

// HTML sanitization
const clean = sanitizeHTML(dirty, { mode: 'strict' });

// URL validation
const safeUrl = sanitizeURL(userUrl);

// File validation
const validation = validateFile(file, {
	maxSize: 10 * 1024 * 1024,
	allowedTypes: ['image/jpeg']
});
```

### Block Utilities

```typescript
import { createBlock, cloneBlock, flattenBlocks } from '$lib/utils/blocks';

// Create new block
const block = createBlock('paragraph');

// Clone existing
const copy = cloneBlock(existingBlock);

// Flatten nested structure
const allBlocks = flattenBlocks(blocks);
```
