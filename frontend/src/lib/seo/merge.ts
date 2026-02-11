/**
 * SEO Plugin Layer - Merge Engine
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Deterministic deep merge for SEO inputs.
 * Input order: global defaults → layout overrides → page overrides.
 * Later overrides win. Explicit null removes inherited value.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since February 2026
 */

import type {
	SEOInput,
	SEODefaults,
	AlternateLink,
	JsonLdNode,
	OpenGraph,
	TwitterCard,
	RobotsDirectives,
	VerificationTags
} from './types';

/**
 * Stable hash for JSON-LD nodes without an @id.
 * Uses sorted JSON serialization for deterministic output.
 */
function stableJsonLdHash(node: JsonLdNode): string {
	if (node['@id']) return node['@id'];
	const sortedKeys = Object.keys(node).sort();
	const parts = sortedKeys.map(
		(k) => `${k}:${JSON.stringify((node as Record<string, unknown>)[k])}`
	);
	return parts.join('|');
}

/**
 * Merge two objects shallowly, respecting null-removal semantics.
 * - undefined in override = inherit from base
 * - null in override = explicitly remove (set to undefined in result)
 * - any other value = override wins
 */
function mergeShallow<T extends object>(
	base: T | undefined | null,
	override: Partial<{ [K in keyof T]: T[K] | null }> | undefined | null
): T | undefined {
	if (override === null) return undefined;
	if (override === undefined) return base ?? undefined;
	if (!base) return override as T;

	const result = { ...base };
	for (const key of Object.keys(override) as Array<keyof T>) {
		const val = (override as Record<string, unknown>)[key as string];
		if (val === null) {
			delete result[key];
		} else if (val !== undefined) {
			(result as Record<string, unknown>)[key as string] = val;
		}
	}
	return result;
}

/**
 * Deduplicate alternates by hreflang+href composite key.
 * Later entries win.
 */
function dedupeAlternates(alternates: AlternateLink[]): AlternateLink[] {
	const map = new Map<string, AlternateLink>();
	for (const alt of alternates) {
		map.set(`${alt.hreflang}::${alt.href}`, alt);
	}
	return Array.from(map.values());
}

/**
 * Deduplicate JSON-LD nodes by @id or stable hash.
 * Later entries win.
 */
function dedupeJsonLd(nodes: JsonLdNode[]): JsonLdNode[] {
	const map = new Map<string, JsonLdNode>();
	for (const node of nodes) {
		const key = stableJsonLdHash(node);
		map.set(key, node);
	}
	return Array.from(map.values());
}

/**
 * Convert SEODefaults into an SEOInput for merge compatibility.
 */
export function defaultsToInput(defaults: SEODefaults): SEOInput {
	return {
		title: defaults.defaultTitle,
		titleTemplate: defaults.titleTemplate,
		description: defaults.defaultDescription,
		canonical: undefined,
		robots: defaults.robots,
		og: {
			title: defaults.defaultTitle,
			description: defaults.defaultDescription,
			image: defaults.defaultImage,
			imageAlt: defaults.defaultImageAlt,
			type: 'website',
			siteName: defaults.siteName,
			locale: defaults.defaultLocale
		},
		twitter: {
			card: defaults.twitterCard,
			site: defaults.twitterSite,
			title: defaults.defaultTitle,
			description: defaults.defaultDescription,
			image: defaults.defaultImage,
			imageAlt: defaults.defaultImageAlt
		},
		alternates: [],
		verification: defaults.verification,
		jsonld: defaults.jsonld
	};
}

/**
 * Merge multiple SEOInput layers with deterministic precedence.
 * Later layers override earlier ones.
 */
export function mergeSEO(...layers: (SEOInput | undefined | null)[]): SEOInput {
	let result: SEOInput = {};

	for (const layer of layers) {
		if (!layer) continue;

		// Scalar fields: null removes, undefined inherits, value overrides
		if (layer.title !== undefined) {
			result.title = layer.title;
		}
		if (layer.titleTemplate !== undefined) {
			result.titleTemplate = layer.titleTemplate;
		}
		if (layer.description !== undefined) {
			result.description = layer.description;
		}
		if (layer.canonical !== undefined) {
			result.canonical = layer.canonical;
		}

		// Object fields: deep merge with null-removal
		result.robots = mergeShallow<RobotsDirectives>(result.robots ?? undefined, layer.robots) as
			| RobotsDirectives
			| undefined;

		result.og = mergeShallow<OpenGraph>(result.og ?? undefined, layer.og) as OpenGraph | undefined;

		result.twitter = mergeShallow<TwitterCard>(result.twitter ?? undefined, layer.twitter) as
			| TwitterCard
			| undefined;

		result.verification = mergeShallow<VerificationTags>(
			result.verification ?? undefined,
			layer.verification
		) as VerificationTags | undefined;

		// Array fields: combine then dedupe
		if (layer.alternates !== undefined) {
			if (layer.alternates === null) {
				result.alternates = null;
			} else {
				const base = result.alternates ?? [];
				result.alternates = dedupeAlternates([...base, ...layer.alternates]);
			}
		}

		if (layer.jsonld !== undefined) {
			if (layer.jsonld === null) {
				result.jsonld = null;
			} else {
				const base = result.jsonld ?? [];
				result.jsonld = dedupeJsonLd([...base, ...layer.jsonld]);
			}
		}
	}

	return result;
}
