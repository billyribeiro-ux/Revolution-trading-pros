/**
 * SEO Plugin Layer - Dedupe Engine
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Unique-key strategy for all head tags to prevent duplicate/conflicting output.
 *
 * Key strategy:
 * - title → single (last wins)
 * - meta[name] → unique by name attribute
 * - meta[property] → unique by property attribute
 * - link[rel=canonical] → single (last wins)
 * - link[rel=alternate][hreflang] → unique by rel+hreflang
 * - JSON-LD → unique by @id or stable hash
 *
 * Latest source wins (page > layout > defaults).
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type { AlternateLink, JsonLdNode } from './types';
import { jsonLdHash } from './jsonld';

// ═══════════════════════════════════════════════════════════════════════════════
// TAG REPRESENTATION
// ═══════════════════════════════════════════════════════════════════════════════

export type HeadTagType =
	| 'title'
	| 'meta-name'
	| 'meta-property'
	| 'link-canonical'
	| 'link-alternate'
	| 'jsonld';

export interface HeadTag {
	type: HeadTagType;
	key: string;
	value: string;
	attributes?: Record<string, string>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// KEY GENERATORS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Generate a unique key for a title tag.
 */
export function titleKey(): string {
	return 'title::singleton';
}

/**
 * Generate a unique key for a meta[name] tag.
 */
export function metaNameKey(name: string): string {
	return `meta-name::${name.toLowerCase()}`;
}

/**
 * Generate a unique key for a meta[property] tag (OG, etc.).
 */
export function metaPropertyKey(property: string): string {
	return `meta-property::${property.toLowerCase()}`;
}

/**
 * Generate a unique key for a canonical link.
 */
export function canonicalKey(): string {
	return 'link-canonical::singleton';
}

/**
 * Generate a unique key for an alternate link by hreflang.
 */
export function alternateKey(hreflang: string): string {
	return `link-alternate::${hreflang.toLowerCase()}`;
}

/**
 * Generate a unique key for a JSON-LD node.
 */
export function jsonLdKey(node: JsonLdNode): string {
	return `jsonld::${jsonLdHash(node)}`;
}

// ═══════════════════════════════════════════════════════════════════════════════
// DEDUPLICATION
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Deduplicate an array of HeadTag objects.
 * Later entries win (last-write-wins semantics).
 */
export function dedupeHeadTags(tags: HeadTag[]): HeadTag[] {
	const map = new Map<string, HeadTag>();
	for (const tag of tags) {
		map.set(tag.key, tag);
	}
	return Array.from(map.values());
}

/**
 * Deduplicate alternates by hreflang.
 * Later entries win.
 */
export function dedupeAlternates(alternates: AlternateLink[]): AlternateLink[] {
	const map = new Map<string, AlternateLink>();
	for (const alt of alternates) {
		map.set(alt.hreflang.toLowerCase(), alt);
	}
	return Array.from(map.values());
}

/**
 * Deduplicate JSON-LD nodes by @id or stable hash.
 * Later entries win.
 */
export function dedupeJsonLdNodes(nodes: JsonLdNode[]): JsonLdNode[] {
	const map = new Map<string, JsonLdNode>();
	for (const node of nodes) {
		map.set(jsonLdHash(node), node);
	}
	return Array.from(map.values());
}

/**
 * Validate that a resolved SEO payload has no duplicate keys.
 * Returns an array of conflict descriptions (empty = no conflicts).
 */
export function findConflicts(tags: HeadTag[]): string[] {
	const seen = new Map<string, number>();
	const conflicts: string[] = [];

	for (const tag of tags) {
		const count = (seen.get(tag.key) ?? 0) + 1;
		seen.set(tag.key, count);
		if (count > 1) {
			conflicts.push(`Duplicate tag: ${tag.key} (occurrence #${count})`);
		}
	}

	return conflicts;
}
