/**
 * blocks utility — Unit Tests (R26-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/blocks.ts` is the CMS block runtime. The most fragile and
 * highest-blast-radius piece is the internal `getDefaultContent(type)`
 * lookup invoked from `createBlock()` — that table seeds every block the
 * editor inserts. The R25-A audit caught two HIGH-IMPACT mismatches where
 * the defaults table used schema-style field names (`symbol`, `direction`,
 * `entry`, `stopLoss`, `targetPrice`, `confidence`) but the SVELTE
 * COMPONENTS that render `priceAlert` / `tradingIdea` blocks actually
 * read prefixed names (`alertSymbol`, `tradeDirection`, …).
 *
 * Result: new blocks rendered with the in-component hardcoded fallbacks
 * (SPY/480/475/470 for priceAlert; AAPL/185/180/195/205/75 for tradingIdea),
 * never with the schema defaults. The fix (commit 634f46eca) realigned the
 * defaults to the component-read field names — but there is NO test pin.
 *
 * These tests freeze the contract:
 *   1. `priceAlert` defaults use the `alert*` prefix.
 *   2. `tradingIdea` defaults use the `trade*` prefix and `tradeConfidence`
 *      is a NUMBER (the old `'medium'` string-vs-number mismatch).
 *   3. `embed` defaults use `embedType: 'custom'` (not the invalid `'iframe'`
 *      from before LB-R25-1).
 *   4. `columns` defaults use `width: '50%'` STRING (not bare number 50,
 *      which produced unit-less CSS — LB-R25-2).
 *   5. Falsy lookups (`unknownType`) return the empty-object fallback.
 *
 * Plus the rest of the block utility surface (cloneBlock, validateBlock,
 * flattenBlocks, findBlockById, countBlocksByType, calculateReadingTime,
 * extractHeadings, serialize/deserialize round-trip).
 */

import { describe, expect, it } from 'vitest';
import {
	createBlock,
	cloneBlock,
	validateBlock,
	flattenBlocks,
	findBlockById,
	countBlocksByType,
	calculateReadingTime,
	extractHeadings,
	serializeBlocks,
	deserializeBlocks,
	generateBlockId
} from '../blocks';

// ═══════════════════════════════════════════════════════════════════════════
// generateBlockId
// ═══════════════════════════════════════════════════════════════════════════

describe('generateBlockId', () => {
	it('returns a string-shaped id with the documented prefix', () => {
		const id = generateBlockId();
		// Format: `block_${Date.now()}_${random}`
		expect(typeof id).toBe('string');
		expect(id).toMatch(/^block_\d+_[a-z0-9]+$/);
	});

	it('produces unique ids across rapid successive calls', () => {
		const ids = new Set<string>();
		for (let i = 0; i < 50; i++) {
			ids.add(generateBlockId());
		}
		// 50 calls -> 50 distinct ids (collision would mean ID reuse)
		expect(ids.size).toBe(50);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// createBlock — default content table
//
// These assertions are the regression test for the R25-A audit findings.
// Each one pins a contract `getDefaultContent` MUST honour to keep new
// blocks rendering with the documented defaults instead of the in-
// component hardcoded fallbacks.
// ═══════════════════════════════════════════════════════════════════════════

describe('createBlock — paragraph / heading defaults (sanity baseline)', () => {
	it('paragraph: { text: "" }', () => {
		const b = createBlock('paragraph');
		expect(b.type).toBe('paragraph');
		expect(b.content).toEqual({ text: '' });
	});

	it('heading: { text: "", level: 2 }', () => {
		const b = createBlock('heading');
		expect(b.content).toEqual({ text: '', level: 2 });
	});
});

describe('createBlock — priceAlert defaults pin (R25-A LB-R25-3)', () => {
	// CONTRACT (defended by this test):
	//   PriceAlertBlock.svelte reads these keys, in this exact spelling:
	//     content.alertSymbol, content.alertDirection, content.alertTarget,
	//     content.alertEntry, content.alertStop, content.alertNote
	//   Any drift (e.g. back to `symbol`/`direction`/`targetPrice`) will
	//   mean new alert blocks render with the in-component fallback
	//   ('SPY'/480/475/470/'above'/'') instead of the schema defaults.
	it('uses the `alert*` prefixed field names matching PriceAlertBlock reads', () => {
		const b = createBlock('priceAlert');
		expect(b.content).toEqual({
			alertSymbol: '',
			alertTarget: 0,
			alertEntry: 0,
			alertStop: 0,
			alertDirection: 'above',
			alertNote: ''
		});
	});

	it('does NOT use the old un-prefixed field names (regression guard)', () => {
		const b = createBlock('priceAlert');
		// These four names were the bug: defaults set them, component
		// never read them. Asserting absence locks the fix in place.
		expect(b.content).not.toHaveProperty('symbol');
		expect(b.content).not.toHaveProperty('direction');
		expect(b.content).not.toHaveProperty('targetPrice');
		expect(b.content).not.toHaveProperty('entry');
		expect(b.content).not.toHaveProperty('stopLoss');
	});

	it('alertDirection default is the literal "above" (component branches on this)', () => {
		const b = createBlock('priceAlert');
		expect(b.content.alertDirection).toBe('above');
	});
});

describe('createBlock — tradingIdea defaults pin (R25-A LB-R25-4)', () => {
	// CONTRACT (defended by this test):
	//   TradingIdeaBlock.svelte reads these keys, in this exact spelling:
	//     content.tradeSymbol, content.tradeDirection, content.tradeEntry,
	//     content.tradeStop, content.tradeTarget1, content.tradeTarget2,
	//     content.tradeConfidence, content.tradeThesis, content.tradeTimeframe.
	//   `tradeConfidence` is also pinned as a NUMBER (the previous
	//   `confidence: 'medium'` was a string vs the declared `number` type).
	it('uses the `trade*` prefixed field names matching TradingIdeaBlock reads', () => {
		const b = createBlock('tradingIdea');
		expect(b.content).toEqual({
			tradeSymbol: '',
			tradeDirection: 'long',
			tradeEntry: 0,
			tradeStop: 0,
			tradeTarget1: 0,
			tradeTarget2: 0,
			tradeConfidence: 50,
			tradeThesis: '',
			tradeTimeframe: ''
		});
	});

	it('tradeConfidence default is a NUMBER (not the string "medium")', () => {
		// LB-R25-4 sub-finding: the prior default was the string 'medium'
		// in a field declared as `number`. Pin both type AND value.
		const b = createBlock('tradingIdea');
		expect(typeof b.content.tradeConfidence).toBe('number');
		expect(b.content.tradeConfidence).toBe(50);
	});

	it('does NOT carry the legacy un-prefixed schema field names', () => {
		const b = createBlock('tradingIdea');
		expect(b.content).not.toHaveProperty('symbol');
		expect(b.content).not.toHaveProperty('direction');
		expect(b.content).not.toHaveProperty('entry');
		expect(b.content).not.toHaveProperty('stopLoss');
		expect(b.content).not.toHaveProperty('takeProfit');
		expect(b.content).not.toHaveProperty('confidence');
	});
});

describe('createBlock — embed defaults pin (R25-A LB-R25-1)', () => {
	it('embedType default is "custom" (the only `BlockContent.embedType` valid generic value)', () => {
		// LB-R25-1: 'iframe' is not a member of BlockContent.embedType. The
		// type narrows to `youtube|vimeo|twitter|instagram|tiktok|soundcloud|
		// spotify|custom`. 'custom' is the documented generic-iframe choice.
		const b = createBlock('embed');
		expect(b.content).toEqual({ embedUrl: '', embedType: 'custom' });
	});

	it('does NOT default to the invalid "iframe" embedType', () => {
		const b = createBlock('embed');
		expect(b.content.embedType).not.toBe('iframe');
	});
});

describe('createBlock — columns defaults pin (R26-A LB-R26-7)', () => {
	it('emits the canonical { columnCount, columnLayout, children } shape ColumnsBlock.svelte reads', () => {
		// LB-R25-2 (historical): the legacy `{ columns: [{blocks, width}] }`
		// shape stored width as the bare number `50`, which would have emitted
		// `style="width:50"` with no unit had it ever reached a renderer.
		//
		// LB-R26-7 (current): `ColumnsBlock.svelte` (src/lib/components/cms/
		// blocks/layout/ColumnsBlock.svelte) reads `columnCount, columnLayout,
		// children` — NOT the legacy nested-columns shape. The R26-A fix in
		// commit 711d0e8d7 replaced the orphan default with the canonical
		// fields the component actually destructures. Column widths are now
		// derived from the `columnLayout` preset ('50/50', '33/33/33', etc.),
		// not stored per-column. Pin the new shape so a regression to the
		// orphan default (which silently rendered as an empty placeholder)
		// is caught.
		const b = createBlock('columns');
		expect(b.content).toEqual({ columnCount: 2, columnLayout: '50/50', children: [] });
		expect(b.content.columnCount).toBe(2);
		expect(b.content.columnLayout).toBe('50/50');
		expect(Array.isArray(b.content.children)).toBe(true);
		expect(b.content.children).toEqual([]);
	});
});

describe('createBlock — fallback for unknown block types', () => {
	it('returns an empty content object when the type is missing from the table', () => {
		// The implementation falls back to `{}` via `defaults[type] || {}`.
		// Cast through unknown for the runtime-only test (the function
		// signature would normally reject this at compile time).
		const b = createBlock('made-up-type' as unknown as 'paragraph');
		expect(b.content).toEqual({});
	});
});

describe('createBlock — base shape', () => {
	it('returns a Block with id/type/content/settings/metadata fields', () => {
		const b = createBlock('paragraph');
		expect(b).toMatchObject({
			id: expect.any(String),
			type: 'paragraph',
			content: expect.any(Object),
			settings: expect.any(Object),
			metadata: expect.any(Object)
		});
		expect(b.metadata.version).toBe(1);
		expect(typeof b.metadata.createdAt).toBe('number');
		expect(typeof b.metadata.updatedAt).toBe('number');
	});

	it('honours `overrides.content` instead of the defaults table', () => {
		const b = createBlock('paragraph', { content: { text: 'override' }, settings: {} });
		expect(b.content).toEqual({ text: 'override' });
	});

	it('honours `overrides.settings` when provided', () => {
		const b = createBlock('heading', {
			content: { text: 'h', level: 1 },
			settings: { level: 1, anchor: 'top' }
		});
		expect(b.settings).toEqual({ level: 1, anchor: 'top' });
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// cloneBlock
// ═══════════════════════════════════════════════════════════════════════════

describe('cloneBlock', () => {
	it('produces a deep copy with a new id and reset metadata', () => {
		const original = createBlock('paragraph', {
			content: { text: 'hello' },
			settings: { textAlign: 'center' }
		});
		const cloned = cloneBlock(original);
		expect(cloned.id).not.toBe(original.id);
		expect(cloned.content).toEqual(original.content);
		expect(cloned.settings).toEqual(original.settings);
		expect(cloned.metadata.version).toBe(1);
	});

	it('does not share content references with the source (true deep clone)', () => {
		const original = createBlock('list', {
			content: { listItems: ['a', 'b'], listType: 'bullet' },
			settings: {}
		});
		const cloned = cloneBlock(original);
		(cloned.content.listItems as string[]).push('c');
		expect((original.content.listItems as string[]).length).toBe(2);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// validateBlock
// ═══════════════════════════════════════════════════════════════════════════

describe('validateBlock', () => {
	it('returns true for a freshly-created block', () => {
		const b = createBlock('paragraph');
		expect(validateBlock(b)).toBe(true);
	});

	it('returns false for null / undefined', () => {
		expect(validateBlock(null)).toBe(false);
		expect(validateBlock(undefined)).toBe(false);
	});

	it('returns false for non-object primitives', () => {
		expect(validateBlock('not-a-block')).toBe(false);
		expect(validateBlock(42)).toBe(false);
		expect(validateBlock(true)).toBe(false);
	});

	it('returns false when any required field is missing or wrong-typed', () => {
		expect(validateBlock({ id: 'x', type: 'p', content: {}, settings: {} })).toBe(false); // no metadata
		expect(validateBlock({ id: 1, type: 'p', content: {}, settings: {}, metadata: {} })).toBe(
			false
		); // id not string
		expect(
			validateBlock({ id: 'x', type: 'p', content: 'not-object', settings: {}, metadata: {} })
		).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// flattenBlocks / findBlockById / countBlocksByType
// ═══════════════════════════════════════════════════════════════════════════

describe('flattenBlocks', () => {
	it('returns the same array when there are no nested containers', () => {
		const a = createBlock('paragraph');
		const b = createBlock('heading');
		const flat = flattenBlocks([a, b]);
		expect(flat).toHaveLength(2);
		expect(flat[0].id).toBe(a.id);
		expect(flat[1].id).toBe(b.id);
	});

	it('descends into `columns` content (each column.blocks is inlined)', () => {
		const inner = createBlock('paragraph', { content: { text: 'inside-column' }, settings: {} });
		const cols = createBlock('columns', {
			content: { columns: [{ blocks: [inner], width: '50%' }] },
			settings: {}
		});
		const flat = flattenBlocks([cols]);
		// Columns block itself + inner paragraph
		expect(flat).toHaveLength(2);
		expect(flat.find((b) => b.id === inner.id)).toBeDefined();
	});

	it('descends into `group` content (group.blocks is inlined)', () => {
		const inner = createBlock('paragraph');
		const group = createBlock('group', { content: { blocks: [inner] }, settings: {} });
		const flat = flattenBlocks([group]);
		expect(flat).toHaveLength(2);
	});
});

describe('findBlockById', () => {
	it('finds a block at the root level', () => {
		const a = createBlock('paragraph');
		const found = findBlockById([a], a.id);
		expect(found?.id).toBe(a.id);
	});

	it('finds a block nested inside a columns container', () => {
		const inner = createBlock('paragraph');
		const cols = createBlock('columns', {
			content: { columns: [{ blocks: [inner], width: '50%' }] },
			settings: {}
		});
		const found = findBlockById([cols], inner.id);
		expect(found?.id).toBe(inner.id);
	});

	it('returns null when the id is not present', () => {
		const a = createBlock('paragraph');
		expect(findBlockById([a], 'non-existent-id')).toBeNull();
	});
});

describe('countBlocksByType', () => {
	it('counts root + nested blocks by type', () => {
		const inner = createBlock('paragraph');
		const cols = createBlock('columns', {
			content: { columns: [{ blocks: [inner], width: '50%' }] },
			settings: {}
		});
		const para = createBlock('paragraph');
		const counts = countBlocksByType([cols, para]);
		expect(counts.paragraph).toBe(2); // root para + nested para
		expect(counts.columns).toBe(1);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// calculateReadingTime
// ═══════════════════════════════════════════════════════════════════════════

describe('calculateReadingTime', () => {
	it('rounds up to at least 1 minute when there is any content', () => {
		const para = createBlock('paragraph', {
			content: { text: 'hello world this is a test' },
			settings: {}
		});
		// 6 words / 200 wpm = 0.03 min -> ceil = 1.
		expect(calculateReadingTime([para])).toBe(1);
	});

	it('treats image/gallery/video blocks as 50 words each for time budget', () => {
		// 50 words at 200 wpm = 0.25 min -> ceil = 1 minute.
		const img = createBlock('image');
		expect(calculateReadingTime([img])).toBe(1);
	});

	it('returns 0 when there is no text and no media-equivalent blocks', () => {
		// A divider has no text and is not in the image/gallery/video set.
		const divider = createBlock('divider');
		expect(calculateReadingTime([divider])).toBe(0);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// extractHeadings
// ═══════════════════════════════════════════════════════════════════════════

describe('extractHeadings', () => {
	it('returns one entry per heading block with generated anchor', () => {
		const h = createBlock('heading', {
			content: { text: 'Hello World', level: 2 },
			settings: { level: 2 }
		});
		const result = extractHeadings([h]);
		expect(result).toHaveLength(1);
		expect(result[0].text).toBe('Hello World');
		expect(result[0].level).toBe(2);
		expect(result[0].anchor).toBe('hello-world');
	});

	it('uses settings.anchor when provided instead of the generated one', () => {
		const h = createBlock('heading', {
			content: { text: 'Foo Bar', level: 2 },
			settings: { level: 2, anchor: 'custom-anchor' }
		});
		const result = extractHeadings([h]);
		expect(result[0].anchor).toBe('custom-anchor');
	});

	it('skips heading blocks with empty text', () => {
		const h = createBlock('heading');
		const result = extractHeadings([h]);
		expect(result).toHaveLength(0);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// serializeBlocks / deserializeBlocks
// ═══════════════════════════════════════════════════════════════════════════

describe('serializeBlocks / deserializeBlocks round-trip', () => {
	it('produces valid JSON that round-trips back to equivalent blocks', () => {
		const original = [createBlock('paragraph'), createBlock('heading')];
		const json = serializeBlocks(original);
		expect(() => JSON.parse(json)).not.toThrow();
		const restored = deserializeBlocks(json);
		expect(restored).toHaveLength(2);
		expect(restored[0].id).toBe(original[0].id);
		expect(restored[1].type).toBe('heading');
	});

	it('deserializeBlocks returns [] when JSON is malformed (no throw)', () => {
		expect(deserializeBlocks('not-json')).toEqual([]);
		expect(deserializeBlocks('{"not":"array"}')).toEqual([]);
	});

	it('deserializeBlocks drops invalid items but keeps valid ones', () => {
		const good = createBlock('paragraph');
		const json = JSON.stringify([good, { id: 'x' /* missing fields */ }]);
		const restored = deserializeBlocks(json);
		expect(restored).toHaveLength(1);
		expect(restored[0].id).toBe(good.id);
	});
});
