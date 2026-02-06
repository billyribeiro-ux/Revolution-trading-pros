/**
 * Block Utility Functions
 * ═══════════════════════════════════════════════════════════════════════════
 * Helper functions for block manipulation and validation
 */

import type { Block, BlockType, CreateBlockPayload } from '$lib/components/cms/blocks/types';
import { toBlockId, type BlockId } from '$lib/stores/blockState.svelte';

/**
 * Generate a unique block ID
 */
export function generateBlockId(): BlockId {
	return toBlockId(`block_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`);
}

/**
 * Create a new block with defaults
 */
export function createBlock(type: BlockType, overrides?: Partial<CreateBlockPayload>): Block {
	const now = Date.now();

	const defaultContent = getDefaultContent(type);

	return {
		id: generateBlockId(),
		type,
		content: overrides?.content || defaultContent,
		settings: overrides?.settings || {},
		metadata: {
			createdAt: now,
			updatedAt: now,
			version: 1
		}
	};
}

/**
 * Get default content for block type
 */
function getDefaultContent(type: BlockType): Record<string, any> {
	const defaults: Record<BlockType, Record<string, any>> = {
		// Content
		paragraph: { text: '' },
		heading: { text: '', level: 2 },
		quote: { text: '', title: '' },
		pullquote: { text: '', title: '' },
		code: { code: '', language: 'javascript' },
		list: { listItems: [''], listType: 'bullet' },
		checklist: { items: [{ id: '1', text: '', checked: false }] },
		preformatted: { text: '' },

		// Media
		image: { mediaUrl: '', mediaAlt: '', mediaCaption: '' },
		video: { embedUrl: '', mediaCaption: '' },
		audio: { mediaUrl: '', mediaCaption: '' },
		gallery: { galleryImages: [] },
		file: { mediaUrl: '', title: '' },
		embed: { embedUrl: '', embedType: 'iframe' },
		gif: { mediaUrl: '', mediaAlt: '' },

		// Interactive
		accordion: {
			accordionItems: [
				{ id: '1', title: 'Item 1', content: 'Content 1' },
				{ id: '2', title: 'Item 2', content: 'Content 2' }
			]
		},
		tabs: {
			tabs: [
				{ id: '1', label: 'Tab 1', content: 'Content 1' },
				{ id: '2', label: 'Tab 2', content: 'Content 2' }
			]
		},
		toggle: { title: 'Toggle Title', content: 'Toggle content' },
		toc: { title: 'Table of Contents' },

		// Layout
		columns: { columns: [{ blocks: [], width: 50 }] },
		group: { blocks: [] },
		row: { blocks: [] },
		divider: {},

		// Trading
		ticker: { symbol: 'SPY', exchange: 'NYSE' },
		chart: { symbol: 'SPY', chartType: 'candlestick' },
		priceAlert: { symbol: '', targetPrice: 0, direction: 'above' },
		tradingIdea: {
			symbol: '',
			direction: 'long',
			entry: 0,
			stopLoss: 0,
			takeProfit: 0,
			confidence: 'medium'
		},

		// AI
		aiGenerated: { prompt: '' },
		aiSummary: { sourceText: '' },
		aiTranslation: { sourceText: '', targetLanguage: 'es' },

		// Advanced
		card: { title: '', description: '', imageUrl: '' },
		testimonial: { text: '', authorName: '', authorTitle: '', rating: 5 },
		cta: {
			ctaHeading: 'Ready to get started?',
			ctaDescription: 'Join thousands of traders already using our platform.',
			ctaPrimaryButton: { text: 'Get Started', url: '#' }
		},
		countdown: { title: 'Event Countdown' },
		socialShare: {},
		author: { authorName: '', authorTitle: '', authorBio: '' },
		relatedPosts: { title: 'Related Posts' },
		newsletter: {
			newsletterPlaceholder: 'Enter your email',
			newsletterButtonText: 'Subscribe'
		},
		separator: {},
		spacer: {},
		html: { html: '' },
		button: { buttonText: 'Click me', buttonUrl: '#' },
		callout: { title: '', description: '' },
		riskDisclaimer: {
			text: 'Trading involves substantial risk of loss and is not suitable for all investors.'
		},
		buttons: { buttons: [{ text: 'Button', url: '#' }] },
		shortcode: { code: '' },
		reusable: { referenceId: '' }
	};

	return defaults[type] || {};
}

/**
 * Clone a block with new ID
 */
export function cloneBlock(block: Block): Block {
	const cloned = structuredClone(block);
	cloned.id = generateBlockId();
	cloned.metadata.createdAt = Date.now();
	cloned.metadata.updatedAt = Date.now();
	cloned.metadata.version = 1;

	return cloned;
}

/**
 * Validate block structure
 */
export function validateBlock(block: unknown): block is Block {
	if (!block || typeof block !== 'object') return false;

	const b = block as any;

	return (
		typeof b.id === 'string' &&
		typeof b.type === 'string' &&
		typeof b.content === 'object' &&
		typeof b.settings === 'object' &&
		typeof b.metadata === 'object'
	);
}

/**
 * Extract all blocks from nested structures (columns, groups)
 */
export function flattenBlocks(blocks: Block[]): Block[] {
	const flattened: Block[] = [];

	for (const block of blocks) {
		flattened.push(block);

		// Handle nested blocks
		if (block.type === 'columns' && block.content.columns) {
			for (const column of block.content.columns) {
				if (column.blocks) {
					flattened.push(...flattenBlocks(column.blocks));
				}
			}
		} else if (block.type === 'group' && block.content.blocks) {
			flattened.push(...flattenBlocks(block.content.blocks));
		}
	}

	return flattened;
}

/**
 * Find block by ID in nested structure
 */
export function findBlockById(blocks: Block[], id: string): Block | null {
	for (const block of blocks) {
		if (block.id === id) return block;

		// Search in nested blocks
		if (block.type === 'columns' && block.content.columns) {
			for (const column of block.content.columns) {
				if (column.blocks) {
					const found = findBlockById(column.blocks, id);
					if (found) return found;
				}
			}
		} else if (block.type === 'group' && block.content.blocks) {
			const found = findBlockById(block.content.blocks, id);
			if (found) return found;
		}
	}

	return null;
}

/**
 * Count blocks by type
 */
export function countBlocksByType(blocks: Block[]): Record<string, number> {
	const counts: Record<string, number> = {};
	const allBlocks = flattenBlocks(blocks);

	for (const block of allBlocks) {
		counts[block.type] = (counts[block.type] || 0) + 1;
	}

	return counts;
}

/**
 * Calculate reading time based on blocks
 */
export function calculateReadingTime(blocks: Block[]): number {
	const WORDS_PER_MINUTE = 200;
	let wordCount = 0;

	const allBlocks = flattenBlocks(blocks);

	for (const block of allBlocks) {
		// Count words in text content
		if (block.content.text) {
			wordCount += countWords(block.content.text);
		}
		if (block.content.description) {
			wordCount += countWords(block.content.description);
		}
		if (block.content.title) {
			wordCount += countWords(block.content.title);
		}

		// Add time for media
		if (['image', 'gallery', 'video'].includes(block.type)) {
			wordCount += 50; // Equivalent words for media consumption
		}
	}

	return Math.ceil(wordCount / WORDS_PER_MINUTE);
}

/**
 * Count words in text
 */
function countWords(text: string): number {
	return text.trim().split(/\s+/).filter(Boolean).length;
}

/**
 * Extract headings for TOC
 */
export function extractHeadings(
	blocks: Block[]
): Array<{ id: string; level: number; text: string; anchor: string }> {
	const headings: Array<{ id: string; level: number; text: string; anchor: string }> = [];
	const allBlocks = flattenBlocks(blocks);

	for (const block of allBlocks) {
		if (block.type === 'heading' && block.content.text) {
			headings.push({
				id: block.id,
				level: block.settings.level || 2,
				text: block.content.text,
				anchor: block.settings.anchor || generateAnchor(block.content.text)
			});
		}
	}

	return headings;
}

/**
 * Generate URL-safe anchor from text
 */
function generateAnchor(text: string): string {
	return text
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, '-')
		.replace(/(^-|-$)/g, '');
}

/**
 * Serialize blocks to JSON
 */
export function serializeBlocks(blocks: Block[]): string {
	return JSON.stringify(blocks, null, 2);
}

/**
 * Deserialize blocks from JSON
 */
export function deserializeBlocks(json: string): Block[] {
	try {
		const parsed = JSON.parse(json);
		if (!Array.isArray(parsed)) return [];

		return parsed.filter(validateBlock);
	} catch {
		return [];
	}
}
