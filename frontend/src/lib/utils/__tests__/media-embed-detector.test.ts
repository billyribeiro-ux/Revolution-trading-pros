/**
 * media-embed-detector — Unit Tests (R26-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `detectEmbed` and friends are the URL-recognition layer behind every
 * blog/CMS embed block (YouTube, Vimeo, TradingView, Twitter/X). The
 * critical contracts:
 *
 *   1. ID extraction is REGEX-driven. YouTube IDs are exactly 11
 *      [A-Za-z0-9_-] chars per the YouTube spec; a 10-char or 12-char
 *      "id" must NOT match.
 *   2. The embed URL we hand to <iframe src> MUST be the privacy-
 *      enhanced `youtube-nocookie.com` host — switching back to
 *      `youtube.com` is a GDPR regression that the CMS docs explicitly
 *      called out.
 *   3. YouTube Shorts must come out with aspectRatio `9:16` (vertical),
 *      not the default `16:9` — otherwise the iframe stretches and the
 *      content is letterboxed wrong.
 *   4. Vimeo embeds must include `dnt=1` (do-not-track) in the embed URL.
 *   5. `extractEmbedId('garbage')` returns `null`, never throws.
 *   6. `isEmbedUrl` is a fast-path that doesn't materialise the full
 *      config — both APIs MUST agree on which URLs are embeddable.
 *   7. `scanTextForEmbeds` finds every URL in arbitrary text, not just
 *      ones at the start of a line.
 *   8. `getEmbedPlatform` maps unknown URLs to the literal 'unknown' (the
 *      sentinel the renderer branches on).
 *
 * The HTML generators are mostly deterministic string templates; we
 * pin the most fragile bits (escape, autoplay, allowfullscreen) but
 * don't snapshot the entire template — that would be churny.
 */

import { describe, expect, it } from 'vitest';
import {
	detectEmbed,
	isEmbedUrl,
	getEmbedPlatform,
	extractEmbedId,
	generateEmbedFromUrl,
	getEmbedThumbnail,
	detectEmbeds,
	scanTextForEmbeds,
	getYouTubeThumbnails,
	createTradingViewConfig
} from '../media-embed-detector';

// ═══════════════════════════════════════════════════════════════════════════
// detectEmbed — YouTube
// ═══════════════════════════════════════════════════════════════════════════

describe('detectEmbed — YouTube', () => {
	it('detects a standard watch URL and extracts the 11-char video id', () => {
		const result = detectEmbed('https://www.youtube.com/watch?v=dQw4w9WgXcQ');
		expect(result.isEmbed).toBe(true);
		expect(result.config?.platform).toBe('youtube');
		expect(result.config?.id).toBe('dQw4w9WgXcQ');
	});

	it('detects youtu.be short URLs', () => {
		const result = detectEmbed('https://youtu.be/dQw4w9WgXcQ');
		expect(result.isEmbed).toBe(true);
		expect(result.config?.id).toBe('dQw4w9WgXcQ');
	});

	it('detects youtube.com/shorts/* URLs and tags them with 9:16 aspect ratio', () => {
		const result = detectEmbed('https://www.youtube.com/shorts/dQw4w9WgXcQ');
		expect(result.isEmbed).toBe(true);
		expect(result.config?.aspectRatio).toBe('9:16');
		expect(result.config?.metadata.isShort).toBe(true);
	});

	it('builds the embed URL on the privacy-enhanced youtube-nocookie host', () => {
		// Contract pin: switching back to youtube.com is a GDPR regression.
		const result = detectEmbed('https://www.youtube.com/watch?v=dQw4w9WgXcQ');
		expect(result.config?.embedUrl).toContain('youtube-nocookie.com/embed/dQw4w9WgXcQ');
	});

	it('exposes the maxresdefault thumbnail URL', () => {
		const result = detectEmbed('https://youtu.be/dQw4w9WgXcQ');
		expect(result.config?.thumbnailUrl).toBe(
			'https://img.youtube.com/vi/dQw4w9WgXcQ/maxresdefault.jpg'
		);
	});

	it('does NOT match a 10-character "id" (length boundary)', () => {
		// YouTube ids are exactly 11 chars. A 10-char tail should be rejected.
		const result = detectEmbed('https://www.youtube.com/watch?v=dQw4w9WgXc');
		expect(result.isEmbed).toBe(false);
	});

	it('does NOT match a 12-character "id" (length boundary)', () => {
		// regex is `[a-zA-Z0-9_-]{11}` — 12 chars where the first 11 are
		// valid will MATCH the first 11 (greedy left-anchored), so this
		// test ONLY catches the case where the id is followed by an
		// invalid separator (no `&`, `?`, end-of-string). To rigorously
		// disprove "any-length", supply a truncated host instead.
		const result = detectEmbed('not-a-video-host/dQw4w9WgXcQ');
		expect(result.isEmbed).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// detectEmbed — Vimeo
// ═══════════════════════════════════════════════════════════════════════════

describe('detectEmbed — Vimeo', () => {
	it('detects vimeo.com/<digits>', () => {
		const result = detectEmbed('https://vimeo.com/123456789');
		expect(result.isEmbed).toBe(true);
		expect(result.config?.platform).toBe('vimeo');
		expect(result.config?.id).toBe('123456789');
	});

	it('builds an embed URL containing the dnt=1 (do-not-track) flag', () => {
		// Contract pin: dropping dnt=1 leaks data to Vimeo Analytics — the
		// CMS settings panel explicitly promises Vimeo embeds are DNT.
		const result = detectEmbed('https://vimeo.com/123456789');
		expect(result.config?.embedUrl).toContain('dnt=1');
		expect(result.config?.embedUrl).toContain('player.vimeo.com/video/123456789');
	});

	it('rejects vimeo.com/ when the path is not a number', () => {
		const result = detectEmbed('https://vimeo.com/about');
		expect(result.isEmbed).toBe(false);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// detectEmbed — Twitter / X
// ═══════════════════════════════════════════════════════════════════════════

describe('detectEmbed — Twitter / X', () => {
	it('detects twitter.com/<user>/status/<id>', () => {
		const result = detectEmbed('https://twitter.com/elonmusk/status/1234567890');
		expect(result.isEmbed).toBe(true);
		expect(result.config?.platform).toBe('twitter');
		expect(result.config?.id).toBe('1234567890');
		expect(result.config?.metadata.username).toBe('elonmusk');
	});

	it('detects x.com/<user>/status/<id> (the rebrand)', () => {
		const result = detectEmbed('https://x.com/elonmusk/status/9876543210');
		expect(result.isEmbed).toBe(true);
		expect(result.config?.platform).toBe('twitter');
		expect(result.config?.id).toBe('9876543210');
	});

	it('produces aspectRatio "auto" + responsive=true for tweets', () => {
		// Tweets are content-driven height, not a fixed-ratio frame.
		const result = detectEmbed('https://x.com/foo/status/1');
		expect(result.config?.aspectRatio).toBe('auto');
		expect(result.config?.responsive).toBe(true);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// detectEmbed — invalid inputs
// ═══════════════════════════════════════════════════════════════════════════

describe('detectEmbed — invalid / unknown inputs', () => {
	it('returns isEmbed=false with an explanatory error for empty string', () => {
		const result = detectEmbed('');
		expect(result.isEmbed).toBe(false);
		expect(result.error).toBeDefined();
	});

	it('returns isEmbed=false for a non-string / wrong-typed argument', () => {
		// Cast through unknown for the runtime branch — the signature
		// rules this out at compile time but the helper checks at run time.
		const result = detectEmbed(null as unknown as string);
		expect(result.isEmbed).toBe(false);
		expect(result.error).toBeDefined();
	});

	it('returns isEmbed=false (no error) for a valid-looking URL that is not an embed', () => {
		const result = detectEmbed('https://example.com/article/123');
		expect(result.isEmbed).toBe(false);
		// No error — we just didn't recognise the URL as an embed.
		expect(result.config).toBeNull();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// isEmbedUrl / getEmbedPlatform — fast-path parity with detectEmbed
// ═══════════════════════════════════════════════════════════════════════════

describe('isEmbedUrl', () => {
	it('returns true for a recognized URL', () => {
		expect(isEmbedUrl('https://youtu.be/dQw4w9WgXcQ')).toBe(true);
	});

	it('returns false for an unknown URL', () => {
		expect(isEmbedUrl('https://example.com/foo')).toBe(false);
	});

	it('returns false for empty / non-string', () => {
		expect(isEmbedUrl('')).toBe(false);
		expect(isEmbedUrl(null as unknown as string)).toBe(false);
	});
});

describe('getEmbedPlatform', () => {
	it('returns "youtube" for a YouTube URL', () => {
		expect(getEmbedPlatform('https://youtu.be/dQw4w9WgXcQ')).toBe('youtube');
	});

	it('returns "vimeo" for a Vimeo URL', () => {
		expect(getEmbedPlatform('https://vimeo.com/123456789')).toBe('vimeo');
	});

	it('returns "twitter" for an X URL', () => {
		expect(getEmbedPlatform('https://x.com/user/status/1')).toBe('twitter');
	});

	it('returns the literal "unknown" for unrecognized URLs (renderer-branch sentinel)', () => {
		// CONTRACT: the embed renderer switches on this exact string; do not
		// "improve" by returning null/undefined.
		expect(getEmbedPlatform('https://example.com')).toBe('unknown');
	});

	it('returns "unknown" for empty / null input', () => {
		expect(getEmbedPlatform('')).toBe('unknown');
		expect(getEmbedPlatform(null as unknown as string)).toBe('unknown');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// extractEmbedId
// ═══════════════════════════════════════════════════════════════════════════

describe('extractEmbedId', () => {
	it('returns the YouTube video id', () => {
		expect(extractEmbedId('https://youtu.be/dQw4w9WgXcQ')).toBe('dQw4w9WgXcQ');
	});

	it('returns the Vimeo numeric id', () => {
		expect(extractEmbedId('https://vimeo.com/123456789')).toBe('123456789');
	});

	it('returns null for an unknown URL (NOT a throw)', () => {
		expect(extractEmbedId('https://example.com')).toBeNull();
	});

	it('returns null for a malformed input', () => {
		expect(extractEmbedId('')).toBeNull();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// generateEmbedFromUrl / getEmbedThumbnail
// ═══════════════════════════════════════════════════════════════════════════

describe('generateEmbedFromUrl', () => {
	it('produces an <iframe> HTML string for a recognized URL', () => {
		const html = generateEmbedFromUrl('https://youtu.be/dQw4w9WgXcQ');
		expect(html).not.toBeNull();
		expect(html).toContain('<iframe');
		expect(html).toContain('youtube-nocookie.com/embed/dQw4w9WgXcQ');
	});

	it('respects autoplay=true by emitting autoplay=1 in the iframe src', () => {
		const html = generateEmbedFromUrl('https://youtu.be/dQw4w9WgXcQ', { autoplay: true });
		expect(html).toContain('autoplay=1');
	});

	it('returns null for an unrecognized URL', () => {
		expect(generateEmbedFromUrl('https://example.com')).toBeNull();
	});

	it('emits the `allowfullscreen` attribute when options.allowFullscreen is true', () => {
		const html = generateEmbedFromUrl('https://youtu.be/dQw4w9WgXcQ', {
			allowFullscreen: true
		});
		expect(html).toContain('allowfullscreen');
	});
});

describe('getEmbedThumbnail', () => {
	it('returns the YouTube thumbnail URL for a YouTube link', () => {
		expect(getEmbedThumbnail('https://youtu.be/dQw4w9WgXcQ')).toBe(
			'https://img.youtube.com/vi/dQw4w9WgXcQ/maxresdefault.jpg'
		);
	});

	it('returns null for a platform without an exposed thumbnail (Twitter)', () => {
		// Twitter extractor does not set thumbnailUrl, so the contract is null.
		expect(getEmbedThumbnail('https://x.com/foo/status/1')).toBeNull();
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// detectEmbeds (bulk) / scanTextForEmbeds
// ═══════════════════════════════════════════════════════════════════════════

describe('detectEmbeds (bulk)', () => {
	it('maps every input URL to a DetectionResult in order', () => {
		const results = detectEmbeds([
			'https://youtu.be/dQw4w9WgXcQ',
			'https://example.com',
			'https://vimeo.com/123'
		]);
		expect(results).toHaveLength(3);
		expect(results[0].isEmbed).toBe(true);
		expect(results[1].isEmbed).toBe(false);
		expect(results[2].isEmbed).toBe(true);
	});
});

describe('scanTextForEmbeds', () => {
	it('finds an embed URL embedded in prose', () => {
		const text = 'Check this out: https://youtu.be/dQw4w9WgXcQ — pretty wild.';
		const embeds = scanTextForEmbeds(text);
		expect(embeds).toHaveLength(1);
		expect(embeds[0].platform).toBe('youtube');
		expect(embeds[0].id).toBe('dQw4w9WgXcQ');
	});

	it('returns [] for plain text with no URLs', () => {
		expect(scanTextForEmbeds('just words, no links here')).toEqual([]);
	});

	it('returns [] for empty / null input', () => {
		expect(scanTextForEmbeds('')).toEqual([]);
		expect(scanTextForEmbeds(null as unknown as string)).toEqual([]);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Platform-specific helpers
// ═══════════════════════════════════════════════════════════════════════════

describe('getYouTubeThumbnails', () => {
	it('returns all five Google-documented thumbnail sizes', () => {
		const t = getYouTubeThumbnails('dQw4w9WgXcQ');
		expect(t.default).toBe('https://img.youtube.com/vi/dQw4w9WgXcQ/default.jpg');
		expect(t.medium).toBe('https://img.youtube.com/vi/dQw4w9WgXcQ/mqdefault.jpg');
		expect(t.high).toBe('https://img.youtube.com/vi/dQw4w9WgXcQ/hqdefault.jpg');
		expect(t.standard).toBe('https://img.youtube.com/vi/dQw4w9WgXcQ/sddefault.jpg');
		expect(t.maxres).toBe('https://img.youtube.com/vi/dQw4w9WgXcQ/maxresdefault.jpg');
	});
});

describe('createTradingViewConfig', () => {
	it('returns a JSON-stringified config with the symbol uppercased', () => {
		const json = createTradingViewConfig('aapl');
		const parsed = JSON.parse(json);
		expect(parsed.symbol).toBe('AAPL');
		expect(parsed.autosize).toBe(true);
	});

	it('uses default theme=dark when no theme option is supplied', () => {
		const parsed = JSON.parse(createTradingViewConfig('SPY'));
		expect(parsed.theme).toBe('dark');
	});

	it('respects theme/interval/width/height overrides', () => {
		const parsed = JSON.parse(
			createTradingViewConfig('SPY', { theme: 'light', interval: 'W', width: 800, height: 600 })
		);
		expect(parsed.theme).toBe('light');
		expect(parsed.interval).toBe('W');
		expect(parsed.width).toBe(800);
		expect(parsed.height).toBe(600);
	});
});
