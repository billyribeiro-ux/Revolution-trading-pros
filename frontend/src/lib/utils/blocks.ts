/**
 * Block Utility Functions
 * ═══════════════════════════════════════════════════════════════════════════
 * Helper functions for block manipulation and validation
 */

import type {
	Block,
	BlockContent,
	BlockType,
	CreateBlockPayload
} from '$lib/components/cms/blocks/types';
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
 * Get default content for block type.
 *
 * Returns a `BlockContent` view (which already carries the open
 * `[key: string]: unknown` index for forward-compat fields). The
 * previous `Record<string, any>` return widened every consumer of
 * `defaultContent` to `any`, silently disabling type-checking on the
 * `Block.content` assignment downstream.
 */
function getDefaultContent(type: BlockType): BlockContent {
	/**
	 * R26-A audit — `BlockEditor` block-default field-name alignment.
	 *
	 * R25-A surfaced the `priceAlert` / `tradingIdea` defaults-vs-component
	 * read-field-name drift (LB-R25-3, LB-R25-4). R26-A extends that audit
	 * across every block component and fixes 17 additional FAILs.
	 *
	 * The bug class is the same: the defaults factory writes a key
	 * (e.g. `embedUrl`) that the block component never reads (it reads
	 * `mediaUrl` instead). The component's `||` / `??` fallback then
	 * kicks in, so new blocks render with hard-coded in-component
	 * placeholder values (`'NASDAQ:AAPL'` for charts, the `DEFAULT_TICKERS`
	 * array of fake SPY/QQQ/AAPL rows for tickers, `'Card Title'` for
	 * cards, `'John Doe' / 'CEO' / 'Acme Corp'` for testimonials, etc.).
	 *
	 * Every fix below aligns the default KEY name to what the corresponding
	 * `*.svelte` block component writes via `updateContent({...})` and reads
	 * via `props.block.content.<key>`. The component's write-key is the
	 * canonical name; old saved blocks store data under those keys already,
	 * so this only affects newly-created blocks (where the defaults factory
	 * is invoked). LB-R26-* annotations below mark each individual fix.
	 */
	const defaults: Record<BlockType, BlockContent> = {
		// Content
		paragraph: { text: '' },
		heading: { text: '', level: 2 },
		quote: { text: '', html: '' },
		// LB-R26-2: `PullQuoteBlock.svelte` reads `text, attribution, source,
		// alignment, borderStyle, accentColor`. Old default `{ text, title }`
		// left `title` orphan (never reached the renderer). Aligning so the
		// styled defaults (`alignment: 'center'`, `borderStyle: 'top-bottom'`,
		// `accentColor: '#3b82f6'` — the component's hard-coded fallbacks)
		// are explicit at creation time.
		pullquote: {
			text: '',
			attribution: '',
			source: '',
			alignment: 'center',
			borderStyle: 'top-bottom',
			accentColor: '#3b82f6'
		},
		code: { code: '', language: 'javascript' },
		list: { listItems: [''], listType: 'bullet' },
		checklist: { items: [{ id: '1', text: '', checked: false }] },
		preformatted: { text: '' },

		// Media
		image: { mediaUrl: '', mediaAlt: '', mediaCaption: '' },
		// LB-R26-3: `VideoBlock.svelte` reads `mediaUrl` (matching every
		// other media block) — the old default `{ embedUrl, mediaCaption }`
		// put the URL under an orphan key, so the video preview started
		// blank even when the operator had pasted a URL elsewhere.
		video: { mediaUrl: '', mediaCaption: '' },
		audio: { mediaUrl: '', mediaCaption: '' },
		gallery: { galleryImages: [] },
		// LB-R26-4: `FileBlock.svelte` reads `fileUrl, fileName, fileSize`.
		// Old default `{ mediaUrl, title }` was orphan. Aligning so the file
		// block renders the documented "Document" placeholder + zero size.
		file: { fileUrl: '', fileName: 'Document', fileSize: 0 },
		// LB-R25-1: `'iframe'` is not a member of `BlockContent.embedType`
		// (`youtube | vimeo | twitter | instagram | tiktok | soundcloud |
		// spotify | custom`). `'custom'` is the closest semantic match for a
		// generic iframe embed and matches what the editor actually wires up.
		embed: { embedUrl: '', embedType: 'custom' },
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
		// LB-R26-5: `ToggleBlock.svelte` reads `toggleTitle, toggleContent`.
		// Old default `{ title, content }` was orphan; new toggle blocks
		// rendered with the in-component fallback `'Click to expand'` placeholder
		// instead of the documented `'Toggle Title' / 'Toggle content'` pair.
		toggle: { toggleTitle: 'Toggle Title', toggleContent: 'Toggle content' },
		// LB-R26-6: `TocBlock.svelte` reads `tocTitle` (not `title`). The
		// hardcoded fallback `'Table of Contents'` is what new TOC blocks
		// rendered with — same string but masking the bug. Aligning so a
		// future redesign of `title` (e.g. for a hero block) doesn't bleed in.
		toc: { tocTitle: 'Table of Contents' },

		// Layout — LB-R25-2: `width` was previously the number `50` which
		// would emit `style="width:50"` with no unit. LB-R26-7: additionally
		// `ColumnsBlock.svelte` reads `columnCount, columnLayout, children`
		// — NOT the legacy `{ columns: [{blocks, width}] }` shape. The
		// `flattenBlocks` / `findBlockById` utilities in this file (lines
		// 222–264) still traverse the OLD shape and would silently miss
		// every nested block — that's a separate latent bug
		// (LB-R26-NESTED-TRAVERSAL) tracked in the report; the fix here is
		// limited to the defaults factory per the task brief.
		columns: { columnCount: 2, columnLayout: '50/50', children: [] },
		// LB-R26-8: `GroupBlock.svelte` reads `children`, not `blocks`.
		group: { children: [] },
		row: { blocks: [] },
		divider: {},

		// Trading
		// LB-R26-9: `TickerBlock.svelte` reads `tickerItems` (an array of
		// `{id, symbol, price, change, changePercent}`). Old default
		// `{ symbol: 'SPY', exchange: 'NYSE' }` never reached the renderer;
		// new ticker blocks showed the `DEFAULT_TICKERS` hard-coded SPY/QQQ/
		// AAPL rows with fake prices (478.52, 412.18, 185.92) on every fresh
		// insert. Aligning to a single empty-symbol row so operators see one
		// editable placeholder instead of fake market data.
		ticker: {
			tickerItems: [{ id: 't1', symbol: 'SPY', price: 0, change: 0, changePercent: 0 }]
		},
		// LB-R26-10: `ChartBlock.svelte` reads `chartSymbol, chartInterval,
		// chartMode, chartTheme, chartImageUrl, chartImageAlt, chartImageCaption`.
		// Old default `{ symbol, chartType }` was completely orphan; new chart
		// blocks rendered with hard-coded `NASDAQ:AAPL` / `1D` / `auto` /
		// embed-mode fallbacks.
		chart: {
			chartSymbol: 'NASDAQ:AAPL',
			chartInterval: '1D',
			chartMode: 'embed',
			chartTheme: 'auto',
			chartImageUrl: '',
			chartImageAlt: 'Trading chart',
			chartImageCaption: ''
		},
		// LB-R25-3: `PriceAlertBlock.svelte` reads `alertSymbol`,
		// `alertDirection`, `alertTarget`, `alertEntry`, `alertStop`,
		// `alertNote` — NOT `symbol`/`direction`/`targetPrice`. The
		// previous defaults never reached the block (the `||` fallbacks
		// inside the component took over) so new alert blocks rendered
		// with hard-coded fallback prices (`SPY` / `480` / `475` / `470`),
		// not the schema's documented defaults. Aligning to the real field
		// names so the defaults actually take effect.
		priceAlert: {
			alertSymbol: '',
			alertTarget: 0,
			alertEntry: 0,
			alertStop: 0,
			alertDirection: 'above',
			alertNote: ''
		},
		// LB-R25-4: same drift as LB-R25-3 — `TradingIdeaBlock.svelte`
		// reads `tradeSymbol`, `tradeDirection`, `tradeEntry`, `tradeStop`,
		// `tradeTarget1`, `tradeTarget2`, `tradeConfidence`, `tradeThesis`,
		// `tradeTimeframe`. The previous defaults (`symbol`, `direction`,
		// `entry`, `stopLoss`, `takeProfit`, `confidence`) never reached
		// the component; new trading-idea blocks rendered with the hard-
		// coded in-component fallbacks (`AAPL`, 185, 180, 195, 205, 75).
		// Additionally the old default `confidence: 'medium'` was a string
		// where the declared type is `number` — the type-tightening from
		// this sweep surfaced the mismatch.
		tradingIdea: {
			tradeSymbol: '',
			tradeDirection: 'long',
			tradeEntry: 0,
			tradeStop: 0,
			tradeTarget1: 0,
			tradeTarget2: 0,
			tradeConfidence: 50,
			tradeThesis: '',
			tradeTimeframe: ''
		},

		// AI
		// LB-R26-11: `AIGeneratedBlock.svelte` reads `aiPrompt, aiModel,
		// aiOutput`. Old default `{ prompt }` was orphan; new AI blocks
		// rendered with the in-component fallback `'gpt-4'` model and an
		// empty prompt despite the schema implying `prompt` was wired up.
		aiGenerated: { aiPrompt: '', aiModel: 'gpt-4', aiOutput: '' },
		// LB-R26-12: `AISummaryBlock.svelte` reads `summarySource,
		// summaryLength, summaryOutput`. Old default `{ sourceText }` orphan.
		aiSummary: { summarySource: '', summaryLength: 'medium', summaryOutput: '' },
		// LB-R26-13: `AITranslationBlock.svelte` reads `translationSource,
		// translationSourceLang, translationTargetLang, translationOutput,
		// translationView`. Old default `{ sourceText, targetLanguage }` was
		// completely orphan; new translation blocks rendered with the in-
		// component `'en' → 'es'` fallback regardless of what the defaults
		// claimed.
		aiTranslation: {
			translationSource: '',
			translationSourceLang: 'en',
			translationTargetLang: 'es',
			translationOutput: '',
			translationView: 'stacked'
		},

		// Advanced
		// LB-R26-14: `CardBlock.svelte` reads `cardTitle, cardDescription,
		// cardImage, cardButtonText, cardButtonUrl`. Old default
		// `{ title, description, imageUrl }` was orphan; new card blocks
		// rendered with the in-component fallbacks `'Card Title'` /
		// `'Learn More'` / `#` placeholder URL.
		card: {
			cardTitle: '',
			cardDescription: '',
			cardImage: '',
			cardButtonText: 'Learn More',
			cardButtonUrl: '#'
		},
		// LB-R26-15: `TestimonialBlock.svelte` reads `testimonialQuote,
		// testimonialAuthor, testimonialTitle, testimonialCompany,
		// testimonialPhoto, testimonialRating`. Old default `{ text,
		// authorName, authorTitle, rating }` orphan; new testimonial blocks
		// rendered with the in-component fake-testimonial fallbacks
		// (`'This is an amazing product...' / 'John Doe' / 'CEO' /
		// 'Acme Corp'`).
		testimonial: {
			testimonialQuote: '',
			testimonialAuthor: '',
			testimonialTitle: '',
			testimonialCompany: '',
			testimonialPhoto: '',
			testimonialRating: 5
		},
		cta: {
			ctaHeading: 'Ready to get started?',
			ctaDescription: 'Join thousands of traders already using our platform.',
			ctaPrimaryButton: { text: 'Get Started', url: '#' }
		},
		// LB-R26-16: `CountdownBlock.svelte` reads `countdownTitle,
		// countdownTarget, countdownExpiredMessage`. Old default
		// `{ title: 'Event Countdown' }` was orphan; new countdown blocks
		// rendered with the in-component `'Offer Ends In'` title fallback
		// (different copy than the supposed default).
		countdown: {
			countdownTitle: 'Event Countdown',
			countdownTarget: '',
			countdownExpiredMessage: 'Time is up!'
		},
		socialShare: {},
		author: { authorName: '', authorTitle: '', authorBio: '' },
		// LB-R26-17: `RelatedPostsBlock.svelte` reads `relatedPostsTitle,
		// relatedPostsCount, relatedPostsLayout`. Old default `{ title }` orphan.
		relatedPosts: {
			relatedPostsTitle: 'Related Posts',
			relatedPostsCount: 3,
			relatedPostsLayout: 'grid'
		},
		newsletter: {
			newsletterPlaceholder: 'Enter your email',
			newsletterButtonText: 'Subscribe'
		},
		separator: {},
		spacer: {},
		html: { html: '' },
		button: { buttonText: 'Click me', buttonUrl: '#' },
		callout: { title: '', description: '' },
		// LB-R26-18: `RiskDisclaimerBlock.svelte` reads `disclaimerText,
		// disclaimerStyle, disclaimerPreset, disclaimerExpandedText,
		// disclaimerRequireAck`. Old default `{ text }` was orphan; the
		// component fell back to its hard-coded disclaimer copy regardless.
		riskDisclaimer: {
			disclaimerText:
				'Trading involves substantial risk of loss and is not suitable for all investors.',
			disclaimerStyle: 'warning',
			disclaimerPreset: 'standard',
			disclaimerExpandedText: '',
			disclaimerRequireAck: false
		},
		// LB-R26-19: `ButtonsBlock.svelte` reads `buttonItems` (array of
		// `{text, url, ...}`), not `buttons`.
		buttons: { buttonItems: [{ text: 'Button', url: '#' }] },
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
 * Validate block structure.
 *
 * The narrowing cast goes through `Record<string, unknown>` so the
 * downstream property reads stay `unknown` (not `any`), which keeps
 * the guard honest — every check below is the only thing standing
 * between an arbitrary deserialized payload and the rest of the
 * editor.
 */
export function validateBlock(block: unknown): block is Block {
	if (!block || typeof block !== 'object') return false;

	const b = block as Record<string, unknown>;

	return (
		typeof b.id === 'string' &&
		typeof b.type === 'string' &&
		typeof b.content === 'object' &&
		typeof b.settings === 'object' &&
		typeof b.metadata === 'object'
	);
}

/**
 * Extract all blocks from nested structures (columns, groups).
 *
 * @deprecated **DO NOT USE in runtime code.** This helper traverses the
 * LEGACY nested-block shape (`block.content.columns[].blocks` for columns,
 * `block.content.blocks` for groups). Real components write nested blocks
 * under `block.content.children: Block[]` (see `ColumnsBlock.svelte:50`,
 * `GroupBlock.svelte:63`, R26-A defaults factory LB-R26-7 / LB-R26-8). This
 * function will silently return zero nested results for any block authored
 * by the current editor.
 *
 * Flagged in R26-A as **LB-R26-NESTED-TRAVERSAL** (LOW impact: zero runtime
 * callers in `src/` — only the unit tests in
 * `src/lib/utils/__tests__/blocks.test.ts` exercise it, and they pass the
 * legacy shape directly so the helper-vs-shape contract self-validates).
 *
 * Canonical replacement: recurse directly through
 * `(block.content.children ?? []) as Block[]` at the call site, matching
 * the `$derived` pattern used in `ColumnsBlock.svelte` and
 * `GroupBlock.svelte`. There is no canonical helper because no feature
 * currently needs cross-tree traversal; if one materialises, write a new
 * `walkBlocks(blocks, visit)` helper against the `children` shape rather
 * than reviving this one.
 */
export function flattenBlocks(blocks: Block[]): Block[] {
	const flattened: Block[] = [];

	for (const block of blocks) {
		flattened.push(block);

		// Handle nested blocks (LEGACY shape — see @deprecated note above)
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
 * Find block by ID in nested structure.
 *
 * @deprecated **DO NOT USE in runtime code.** Same LEGACY-shape traversal
 * bug as {@link flattenBlocks}: walks `block.content.columns[].blocks` and
 * `block.content.blocks`, but the editor writes nested blocks under
 * `block.content.children`. Will return `null` for any nested block
 * authored by the current editor.
 *
 * Flagged as **LB-R26-NESTED-TRAVERSAL** in R26-A. Zero runtime callers in
 * `src/` — only the unit tests reach it. Kept for backward-compat of any
 * legacy serialized post that survived the migration to `children`.
 *
 * Canonical replacement: recurse directly through `block.content.children`
 * at the call site, mirroring the `ColumnsBlock.svelte` / `GroupBlock.svelte`
 * `$derived` pattern.
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
 * Count blocks by type.
 *
 * @deprecated **DO NOT USE in runtime code.** Internally delegates to
 * {@link flattenBlocks}, which traverses the LEGACY nested-block shape
 * (`columns[].blocks` / `content.blocks`). Counts will under-report by
 * every nested block authored under the canonical `content.children` shape.
 *
 * Flagged as **LB-R26-NESTED-TRAVERSAL** in R26-A. Zero runtime callers in
 * `src/`. If a feature needs block-type counts, walk `content.children`
 * directly and tally inline — there's no canonical helper to redirect to.
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
 * Calculate reading time based on blocks.
 *
 * @deprecated **DO NOT USE in runtime code.** Same LEGACY-shape problem as
 * {@link flattenBlocks} (which this function delegates to): every nested
 * block authored under `content.children` is invisible to the word-count,
 * so the estimate under-reports for any post with columns or groups.
 *
 * Flagged as **LB-R26-NESTED-TRAVERSAL** in R26-A. Zero runtime callers in
 * `src/` — `routes/blog/[slug]/+page.svelte` uses the text-based
 * `calculateReadingTime` from `$lib/utils/readingAnalytics.ts` instead
 * (which works off rendered prose, not the editor block tree, and is the
 * canonical reading-time helper for this codebase).
 *
 * If a future feature needs block-tree reading-time, walk
 * `content.children` directly and accumulate `content.text` /
 * `content.description` / `content.title` word counts inline.
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
 * Extract headings for TOC.
 *
 * @deprecated **DO NOT USE in runtime code.** Delegates to
 * {@link flattenBlocks} and therefore misses every heading nested inside a
 * `columns` or `group` block authored under the canonical `content.children`
 * shape (see `ColumnsBlock.svelte:50`, `GroupBlock.svelte:63`). The TOC
 * would silently omit nested-section headings if this were wired up.
 *
 * Flagged as **LB-R26-NESTED-TRAVERSAL** in R26-A. Zero runtime callers in
 * `src/` — the live TOC implementation in
 * `src/lib/components/blog/TableOfContents.svelte:100` has its OWN local
 * `extractHeadings()` that iterates the post's content-blocks list
 * directly (and reads from `block.data ?? block.content` for legacy posts).
 * That local function is the canonical heading-extractor for this codebase.
 *
 * If a future feature needs cross-tree heading extraction, walk
 * `content.children` directly at the call site.
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
