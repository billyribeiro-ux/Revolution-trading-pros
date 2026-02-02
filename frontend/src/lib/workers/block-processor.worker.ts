/**
 * Block Processor Web Worker
 * ═══════════════════════════════════════════════════════════════════════════
 * Offload heavy computations to background thread
 * Handles: validation, sanitization, word counts, processing
 */

import type { Block, BlockContent } from '$lib/components/cms/blocks/types';

// Message types
interface ProcessBlocksMessage {
	type: 'PROCESS_BLOCKS';
	blocks: Block[];
}

interface ValidateBlockMessage {
	type: 'VALIDATE_BLOCK';
	block: Block;
}

interface SanitizeContentMessage {
	type: 'SANITIZE_CONTENT';
	content: BlockContent;
}

interface CalculateStatsMessage {
	type: 'CALCULATE_STATS';
	blocks: Block[];
}

type WorkerMessage =
	| ProcessBlocksMessage
	| ValidateBlockMessage
	| SanitizeContentMessage
	| CalculateStatsMessage;

// Word count helper
function countWords(text: string | undefined): number {
	if (!text) return 0;
	return text.trim().split(/\s+/).filter(Boolean).length;
}

// Strip HTML tags
function stripHtml(html: string | undefined): string {
	if (!html) return '';
	return html.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim();
}

// Process multiple blocks
function processBlocks(blocks: Block[]): Block[] {
	return blocks.map((block) => {
		const textContent = block.content.text || stripHtml(block.content.html);
		const wordCount = countWords(textContent);
		const charCount = textContent.length;

		return {
			...block,
			metadata: {
				...block.metadata,
				processedAt: new Date().toISOString(),
				wordCount,
				charCount,
				readingTime: Math.ceil(wordCount / 200) // ~200 WPM
			}
		};
	});
}

// Validate block structure
function validateBlock(block: Block): { valid: boolean; errors: string[] } {
	const errors: string[] = [];

	if (!block.id) errors.push('Missing block ID');
	if (!block.type) errors.push('Missing block type');
	if (!block.content) errors.push('Missing block content');

	// Type-specific validation
	switch (block.type) {
		case 'image':
		case 'video':
		case 'audio':
			if (!block.content.mediaUrl) {
				errors.push(`Missing media URL for ${block.type} block`);
			}
			break;
		case 'heading':
			if (!block.settings?.level || block.settings.level < 1 || block.settings.level > 6) {
				errors.push('Invalid heading level (must be 1-6)');
			}
			break;
		case 'gallery':
			if (!block.content.galleryImages || block.content.galleryImages.length === 0) {
				errors.push('Gallery must have at least one image');
			}
			break;
	}

	return {
		valid: errors.length === 0,
		errors
	};
}

// Sanitize content
function sanitizeContent(content: BlockContent): BlockContent {
	const sanitized = { ...content };

	// Sanitize HTML content
	if (sanitized.html) {
		// Remove script tags
		sanitized.html = sanitized.html.replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '');
		// Remove event handlers
		sanitized.html = sanitized.html.replace(/\son\w+="[^"]*"/gi, '');
		sanitized.html = sanitized.html.replace(/\son\w+='[^']*'/gi, '');
	}

	// Validate URLs
	if (sanitized.mediaUrl) {
		try {
			const url = new URL(sanitized.mediaUrl);
			if (!['http:', 'https:', 'data:'].includes(url.protocol)) {
				sanitized.mediaUrl = '';
			}
		} catch {
			sanitized.mediaUrl = '';
		}
	}

	return sanitized;
}

// Calculate document statistics
function calculateStats(blocks: Block[]): {
	totalBlocks: number;
	totalWords: number;
	totalChars: number;
	readingTime: number;
	blockTypes: Record<string, number>;
} {
	const blockTypes: Record<string, number> = {};
	let totalWords = 0;
	let totalChars = 0;

	for (const block of blocks) {
		// Count block types
		blockTypes[block.type] = (blockTypes[block.type] || 0) + 1;

		// Count text
		const text = block.content.text || stripHtml(block.content.html);
		totalWords += countWords(text);
		totalChars += text.length;
	}

	return {
		totalBlocks: blocks.length,
		totalWords,
		totalChars,
		readingTime: Math.ceil(totalWords / 200),
		blockTypes
	};
}

// Listen for messages
self.onmessage = (e: MessageEvent<WorkerMessage>) => {
	const { type } = e.data;

	switch (type) {
		case 'PROCESS_BLOCKS': {
			const result = processBlocks(e.data.blocks);
			self.postMessage({ type: 'BLOCKS_PROCESSED', blocks: result });
			break;
		}

		case 'VALIDATE_BLOCK': {
			const result = validateBlock(e.data.block);
			self.postMessage({ type: 'BLOCK_VALIDATED', blockId: e.data.block.id, ...result });
			break;
		}

		case 'SANITIZE_CONTENT': {
			const result = sanitizeContent(e.data.content);
			self.postMessage({ type: 'CONTENT_SANITIZED', content: result });
			break;
		}

		case 'CALCULATE_STATS': {
			const result = calculateStats(e.data.blocks);
			self.postMessage({ type: 'STATS_CALCULATED', stats: result });
			break;
		}
	}
};

export {};
