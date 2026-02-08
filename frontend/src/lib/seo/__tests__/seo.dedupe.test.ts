/**
 * SEO Dedupe Engine Tests
 * ═══════════════════════════════════════════════════════════════════════════════
 */

import { describe, it, expect } from 'vitest';
import {
	dedupeHeadTags,
	dedupeAlternates,
	dedupeJsonLdNodes,
	findConflicts,
	titleKey,
	metaNameKey,
	metaPropertyKey,
	canonicalKey,
	alternateKey,
	jsonLdKey
} from '../dedupe';
import type { HeadTag } from '../dedupe';
import type { AlternateLink, JsonLdNode } from '../types';

describe('key generators', () => {
	it('titleKey returns singleton key', () => {
		expect(titleKey()).toBe('title::singleton');
	});

	it('metaNameKey lowercases name', () => {
		expect(metaNameKey('Description')).toBe('meta-name::description');
	});

	it('metaPropertyKey lowercases property', () => {
		expect(metaPropertyKey('og:Title')).toBe('meta-property::og:title');
	});

	it('canonicalKey returns singleton key', () => {
		expect(canonicalKey()).toBe('link-canonical::singleton');
	});

	it('alternateKey lowercases hreflang', () => {
		expect(alternateKey('EN-US')).toBe('link-alternate::en-us');
	});

	it('jsonLdKey uses @id when present', () => {
		const node: JsonLdNode = { '@type': 'Organization', '@id': '#org', name: 'Test', url: 'https://test.com' };
		expect(jsonLdKey(node)).toBe('jsonld::#org');
	});

	it('jsonLdKey uses stable hash when no @id', () => {
		const node: JsonLdNode = { '@type': 'Organization', name: 'Test', url: 'https://test.com' };
		const key = jsonLdKey(node);
		expect(key).toContain('jsonld::');
		expect(key).not.toBe('jsonld::');
	});
});

describe('dedupeHeadTags', () => {
	it('removes duplicate tags by key (last wins)', () => {
		const tags: HeadTag[] = [
			{ type: 'title', key: 'title::singleton', value: 'First' },
			{ type: 'meta-name', key: 'meta-name::description', value: 'Desc 1' },
			{ type: 'title', key: 'title::singleton', value: 'Second' }
		];
		const result = dedupeHeadTags(tags);
		expect(result).toHaveLength(2);
		expect(result.find((t) => t.type === 'title')?.value).toBe('Second');
	});

	it('preserves unique tags', () => {
		const tags: HeadTag[] = [
			{ type: 'meta-name', key: 'meta-name::description', value: 'Desc' },
			{ type: 'meta-name', key: 'meta-name::robots', value: 'index' },
			{ type: 'meta-property', key: 'meta-property::og:title', value: 'Title' }
		];
		const result = dedupeHeadTags(tags);
		expect(result).toHaveLength(3);
	});

	it('handles empty array', () => {
		expect(dedupeHeadTags([])).toEqual([]);
	});
});

describe('dedupeAlternates', () => {
	it('deduplicates by hreflang (last wins)', () => {
		const alts: AlternateLink[] = [
			{ hreflang: 'en', href: 'https://example.com/en' },
			{ hreflang: 'fr', href: 'https://example.com/fr' },
			{ hreflang: 'en', href: 'https://example.com/en-v2' }
		];
		const result = dedupeAlternates(alts);
		expect(result).toHaveLength(2);
		const en = result.find((a) => a.hreflang === 'en');
		expect(en?.href).toBe('https://example.com/en-v2');
	});

	it('case-insensitive hreflang matching', () => {
		const alts: AlternateLink[] = [
			{ hreflang: 'EN', href: 'https://example.com/en' },
			{ hreflang: 'en', href: 'https://example.com/en-v2' }
		];
		const result = dedupeAlternates(alts);
		expect(result).toHaveLength(1);
	});
});

describe('dedupeJsonLdNodes', () => {
	it('deduplicates by @id (last wins)', () => {
		const nodes: JsonLdNode[] = [
			{ '@type': 'Organization', '@id': '#org', name: 'Old', url: 'https://old.com' },
			{ '@type': 'Organization', '@id': '#org', name: 'New', url: 'https://new.com' }
		];
		const result = dedupeJsonLdNodes(nodes);
		expect(result).toHaveLength(1);
		expect((result[0] as Record<string, unknown>).name).toBe('New');
	});

	it('keeps nodes with different @ids', () => {
		const nodes: JsonLdNode[] = [
			{ '@type': 'Organization', '@id': '#org', name: 'Org', url: 'https://org.com' },
			{ '@type': 'WebSite', '@id': '#site', name: 'Site', url: 'https://site.com' }
		];
		const result = dedupeJsonLdNodes(nodes);
		expect(result).toHaveLength(2);
	});

	it('deduplicates by stable hash when no @id', () => {
		const nodes: JsonLdNode[] = [
			{ '@type': 'Organization', name: 'Same', url: 'https://same.com' },
			{ '@type': 'Organization', name: 'Same', url: 'https://same.com' }
		];
		const result = dedupeJsonLdNodes(nodes);
		expect(result).toHaveLength(1);
	});
});

describe('findConflicts', () => {
	it('returns empty array for no conflicts', () => {
		const tags: HeadTag[] = [
			{ type: 'title', key: 'title::singleton', value: 'Title' },
			{ type: 'meta-name', key: 'meta-name::description', value: 'Desc' }
		];
		expect(findConflicts(tags)).toEqual([]);
	});

	it('detects duplicate keys', () => {
		const tags: HeadTag[] = [
			{ type: 'title', key: 'title::singleton', value: 'First' },
			{ type: 'title', key: 'title::singleton', value: 'Second' }
		];
		const conflicts = findConflicts(tags);
		expect(conflicts).toHaveLength(1);
		expect(conflicts[0]).toContain('title::singleton');
	});

	it('detects multiple duplicates', () => {
		const tags: HeadTag[] = [
			{ type: 'title', key: 'title::singleton', value: 'A' },
			{ type: 'title', key: 'title::singleton', value: 'B' },
			{ type: 'meta-name', key: 'meta-name::description', value: 'C' },
			{ type: 'meta-name', key: 'meta-name::description', value: 'D' }
		];
		const conflicts = findConflicts(tags);
		expect(conflicts).toHaveLength(2);
	});
});
