/**
 * string utilities — Unit Tests (R24-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * The string module is a low-level utility used widely (slug generation
 * for URLs, excerpts in SEO/blog blocks, kebab/camel case conversions
 * in API payload normalizers). The helpers look trivial but their
 * contracts have subtle edge cases — especially `slugify` (URL safety,
 * Unicode behaviour) and `truncate` (suffix-length accounting).
 *
 * What we test:
 *   - truncate: exact-length boundary, suffix is INCLUDED in length budget.
 *   - slugify: lowercase + strip non-word, collapse runs, trim hyphens.
 *   - stripHtml + excerpt: HTML tags removed before truncation.
 *   - camelToKebab / kebabToCamel: round-trip parity.
 *   - randomString: returns the requested length from the allowed alphabet.
 */

import { describe, it, expect } from 'vitest';
import {
	truncate,
	slugify,
	capitalize,
	titleCase,
	camelToKebab,
	kebabToCamel,
	stripHtml,
	excerpt,
	wordCount,
	randomString
} from '../string';

// ═══════════════════════════════════════════════════════════════════════════
// truncate
// ═══════════════════════════════════════════════════════════════════════════

describe('truncate', () => {
	it('returns the string unchanged when shorter than the length budget', () => {
		expect(truncate('hello', 10)).toBe('hello');
	});

	it('returns the string unchanged when exactly at the length budget', () => {
		expect(truncate('hello', 5)).toBe('hello');
	});

	it('truncates with the default `...` suffix INSIDE the length budget', () => {
		// Result must be <= length. "hello world" (11) -> length 8 means
		// 5 chars from src + 3 chars suffix = 8 total.
		const result = truncate('hello world', 8);
		expect(result).toBe('hello...');
		expect(result.length).toBe(8);
	});

	it('accepts a custom suffix and still respects the length budget', () => {
		expect(truncate('hello world', 7, '…')).toBe('hello …');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// slugify
// ═══════════════════════════════════════════════════════════════════════════

describe('slugify', () => {
	it('lowercases and joins words with single hyphens', () => {
		expect(slugify('Hello World')).toBe('hello-world');
	});

	it('strips punctuation', () => {
		expect(slugify("It's a test!")).toBe('its-a-test');
	});

	it('collapses runs of whitespace, underscores, and hyphens', () => {
		expect(slugify('foo   bar___baz---qux')).toBe('foo-bar-baz-qux');
	});

	it('trims leading and trailing hyphens', () => {
		expect(slugify('  --hello--  ')).toBe('hello');
	});

	it('returns an empty string for input with only punctuation (NEGATIVE)', () => {
		expect(slugify('!!!@@@###')).toBe('');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// capitalize / titleCase
// ═══════════════════════════════════════════════════════════════════════════

describe('capitalize', () => {
	it('uppercases the first letter and preserves the rest', () => {
		expect(capitalize('hello')).toBe('Hello');
	});

	it('leaves an already-capitalized string unchanged', () => {
		expect(capitalize('Hello')).toBe('Hello');
	});

	it('handles an empty string without throwing', () => {
		expect(capitalize('')).toBe('');
	});
});

describe('titleCase', () => {
	it('capitalizes each word in a sentence', () => {
		expect(titleCase('hello world foo')).toBe('Hello World Foo');
	});

	it('lowercases mid-word characters that were uppercase', () => {
		// Documented behavior: titleCase lowercases first, then capitalizes
		// each word's leading char — so "HELLO" → "Hello".
		expect(titleCase('HELLO WORLD')).toBe('Hello World');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// camelToKebab / kebabToCamel
// ═══════════════════════════════════════════════════════════════════════════

describe('camelToKebab', () => {
	it('inserts a hyphen before each upper-case run', () => {
		expect(camelToKebab('camelCaseString')).toBe('camel-case-string');
	});

	it('handles trailing numbers', () => {
		expect(camelToKebab('item1Name')).toBe('item1-name');
	});
});

describe('kebabToCamel', () => {
	it('removes hyphens and uppercases the next letter', () => {
		expect(kebabToCamel('kebab-case-string')).toBe('kebabCaseString');
	});

	it('round-trips a camelCase string through both helpers', () => {
		const camel = 'fooBarBaz';
		expect(kebabToCamel(camelToKebab(camel))).toBe(camel);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// stripHtml / excerpt
// ═══════════════════════════════════════════════════════════════════════════

describe('stripHtml', () => {
	it('removes simple tags', () => {
		expect(stripHtml('<p>Hello</p>')).toBe('Hello');
	});

	it('removes nested tags', () => {
		expect(stripHtml('<div><span>Hi</span><b>!</b></div>')).toBe('Hi!');
	});

	it('leaves text without tags unchanged', () => {
		expect(stripHtml('plain text')).toBe('plain text');
	});
});

describe('excerpt', () => {
	it('strips HTML then truncates to the requested length', () => {
		const html = '<p>Lorem ipsum dolor sit amet</p>';
		// `excerpt` strips first (yielding "Lorem ipsum dolor sit amet"),
		// then truncates to length 15 by taking 12 chars + the 3-char "..."
		// suffix. The 12th char of the stripped text is a space, so the
		// observed output retains it.
		expect(excerpt(html, 15)).toBe('Lorem ipsum ...');
		expect(excerpt(html, 15).length).toBe(15);
	});

	it('uses 150 as the default length budget', () => {
		const long = 'a'.repeat(200);
		expect(excerpt(long).length).toBeLessThanOrEqual(150);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// wordCount / randomString
// ═══════════════════════════════════════════════════════════════════════════

describe('wordCount', () => {
	it('counts words separated by single spaces', () => {
		expect(wordCount('hello world foo')).toBe(3);
	});

	it('collapses runs of whitespace', () => {
		expect(wordCount('hello   world')).toBe(2);
	});

	it('returns 0 for an empty or whitespace-only string', () => {
		expect(wordCount('')).toBe(0);
		expect(wordCount('   ')).toBe(0);
	});
});

describe('randomString', () => {
	it('returns the requested length', () => {
		expect(randomString(10).length).toBe(10);
		expect(randomString(32).length).toBe(32);
	});

	it('returns 10 chars by default', () => {
		expect(randomString().length).toBe(10);
	});

	it('returns only characters from the allowed alphabet', () => {
		const alphabet = /^[A-Za-z0-9]+$/;
		expect(alphabet.test(randomString(50))).toBe(true);
	});
});
