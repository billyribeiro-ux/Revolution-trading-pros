/**
 * Canonical JSON-LD Serializer Tests
 * ===========================================================================
 *
 * Guards the stored-XSS class fixed in audit FULL_REPO_AUDIT_2026-05-17
 * §P0-6: a CMS/user-controlled schema field must never be able to break out
 * of the host `<script type="application/ld+json">` element.
 *
 * U+2028 / U+2029 are referenced via String.fromCharCode, never as literal
 * source characters (a literal one is itself a JS line terminator).
 */

import { describe, it, expect } from 'vitest';
import { serializeJsonLd } from '../serializeJsonLd';

const U2028 = String.fromCharCode(0x2028);
const U2029 = String.fromCharCode(0x2029);

describe('serializeJsonLd', () => {
	it('round-trips a valid JSON-LD node losslessly', () => {
		const node = {
			'@context': 'https://schema.org',
			'@type': 'Article',
			headline: 'A perfectly normal headline',
			author: { '@type': 'Person', name: 'Jane Doe' },
			tags: ['alpha', 'beta'],
			wordCount: 1234
		};
		const out = serializeJsonLd(node);
		expect(JSON.parse(out)).toEqual(node);
	});

	it('escapes a </script><script> breakout payload so no live script remains', () => {
		const payload = '</script><script>alert(1)</script>';
		const node = { '@type': 'Article', headline: payload };
		const out = serializeJsonLd(node);

		// No raw HTML structural characters survive.
		expect(out).not.toContain('<');
		expect(out).not.toContain('>');
		expect(out.toLowerCase()).not.toContain('</script>');
		expect(out.toLowerCase()).not.toContain('<script');

		// ...but the value is preserved exactly after JSON parse.
		expect(JSON.parse(out).headline).toBe(payload);
	});

	it('escapes raw < > & characters while preserving the value', () => {
		const value = 'a < b && c > d';
		const node = { '@type': 'FAQPage', question: value };
		const out = serializeJsonLd(node);

		expect(out).not.toContain('<');
		expect(out).not.toContain('>');
		expect(out).not.toContain('&');
		expect(out).toContain('\\u003C');
		expect(out).toContain('\\u003E');
		expect(out).toContain('\\u0026');

		expect(JSON.parse(out).question).toBe(value);
	});

	it('escapes literal U+2028 / U+2029 separators while preserving the value', () => {
		const value = `line${U2028}sep${U2029}para`;
		const node = { '@type': 'VideoObject', name: value };
		const out = serializeJsonLd(node);

		expect(out).not.toContain(U2028);
		expect(out).not.toContain(U2029);
		expect(out).toContain('\\u2028');
		expect(out).toContain('\\u2029');

		expect(JSON.parse(out).name).toBe(value);
	});

	it('handles a combined hostile payload (all vectors at once)', () => {
		const hostile = `</script><script>alert(1)</script> & <img src=x> ${U2028}${U2029}`;
		const node = {
			'@type': 'BreadcrumbList',
			itemListElement: [{ '@type': 'ListItem', position: 1, name: hostile }]
		};
		const out = serializeJsonLd(node);

		expect(out).not.toContain('<');
		expect(out).not.toContain('>');
		expect(out).not.toContain('&');
		expect(out).not.toContain(U2028);
		expect(out).not.toContain(U2029);
		expect(out.toLowerCase()).not.toContain('</script>');

		expect(JSON.parse(out)).toEqual(node);
	});

	it('produces deterministic output regardless of key insertion order', () => {
		const a = { '@context': 'https://schema.org', '@type': 'WebSite', name: 'X', url: 'u' };
		const b = { url: 'u', name: 'X', '@type': 'WebSite', '@context': 'https://schema.org' };
		expect(serializeJsonLd(a)).toBe(serializeJsonLd(b));
	});

	it('orders nested object keys deterministically too', () => {
		const a = { outer: { z: 1, a: 2, m: { y: 3, x: 4 } } };
		const b = { outer: { m: { x: 4, y: 3 }, a: 2, z: 1 } };
		expect(serializeJsonLd(a)).toBe(serializeJsonLd(b));
	});

	it('preserves array order (semantically meaningful for itemListElement)', () => {
		const node = {
			'@type': 'ItemList',
			itemListElement: [
				{ position: 1, name: 'first' },
				{ position: 2, name: 'second' },
				{ position: 3, name: 'third' }
			]
		};
		const parsed = JSON.parse(serializeJsonLd(node)) as typeof node;
		expect(parsed.itemListElement.map((i) => i.position)).toEqual([1, 2, 3]);
	});

	it('drops undefined object members like JSON.stringify', () => {
		const node = { '@type': 'Article', headline: 'h', author: undefined };
		const out = serializeJsonLd(node);
		expect(out).toBe('{"@type":"Article","headline":"h"}');
		expect(JSON.parse(out)).toEqual({ '@type': 'Article', headline: 'h' });
	});

	it('serializes an array of nodes safely', () => {
		const nodes = [
			{ '@type': 'WebSite', name: 'a</script>' },
			{ '@type': 'Organization', name: 'b<script>' }
		];
		const out = serializeJsonLd(nodes);
		expect(out).not.toContain('<');
		expect(out).not.toContain('>');
		expect(JSON.parse(out)).toEqual(nodes);
	});
});
