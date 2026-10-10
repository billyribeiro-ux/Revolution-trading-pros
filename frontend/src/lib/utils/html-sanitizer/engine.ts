/**
 * Isomorphic HTML sanitizer.
 * ═══════════════════════════════════════════════════════════════════════════
 * parse → allowlist → harden → serialize, all on a hast tree:
 *
 * 1. Parse with a WHATWG-conformant parser, scripting disabled — the browser's
 *    own parser (`DOMParser`) on the client, parse5 on the server, selected at
 *    build time through the package.json `imports` conditions so neither
 *    bundle carries the other's parser. Nothing executes or loads.
 * 2. Allowlist elements and attributes with `hast-util-sanitize`. Disallowed
 *    elements are unwrapped; script/style/foreign/raw-text elements are dropped
 *    with their content.
 * 3. Harden what survived: URL schemes, DOM-clobbering `id`/`name` values,
 *    link `rel`/`target`, and iframe origins.
 * 4. Serialize with `hast-util-to-html`, which quotes every attribute and
 *    escapes text, so the output re-parses to the same tree.
 *
 * Because every step after parsing is the same code on both sides, a page's
 * server-rendered `{@html}` content is byte-identical to what the client
 * computes during hydration — for any input both parsers build the same tree
 * from. They do for all content markup. The known exception is `<select>`:
 * Chromium already implements the relaxed select parsing that parse5 8 does
 * not, which can change where a malformed select ends. Both outputs are still
 * sanitized; the worst case is a dev-mode `hydration_html_changed` warning.
 * `pnpm test:sanitizer:browser` re-checks parity against real Chromium.
 *
 * This replaces `isomorphic-dompurify`, whose browser build — the one
 * Cloudflare's workerd resolves — throws at import time without a DOM and took
 * every server-rendered route down with it.
 */

import type { Element, ElementContent, Properties, Root, RootContent } from 'hast';
import { defaultSchema, sanitize, type Schema } from 'hast-util-sanitize';
import { toHtml } from 'hast-util-to-html';
import { find, html as htmlSpace } from 'property-information';
import { parseHtmlFragment } from '#lib/utils/html-sanitizer/parse-fragment.js';
import { CLOBBER_NAMES } from '#lib/utils/html-sanitizer/clobber-names.generated.js';
import {
	DROP_WITH_CONTENT,
	TEXT_ONLY,
	URL_ATTRIBUTES,
	type HtmlSanitizePolicy
} from '#lib/utils/html-sanitizer/policy.js';

/**
 * `aria-*` attributes are allowed under every policy (DOMPurify's
 * `ALLOW_ARIA_ATTR` default). Only attributes defined by WAI-ARIA are known to
 * the parser's property table; anything else is not an ARIA attribute.
 */
const ARIA_PROPERTIES: readonly string[] = Object.values(htmlSpace.property)
	.filter((info) => info.attribute.startsWith('aria-'))
	.map((info) => info.property);

/** hast property names of the URL-valued attributes. */
const URL_PROPERTIES: ReadonlySet<string> = new Set(
	[...URL_ATTRIBUTES].map((attribute) => find(htmlSpace, attribute).property)
);

/** `data:` URLs are allowed in `src` only for media of the element's own kind. */
const DATA_URL_MEDIA: Readonly<Record<string, readonly string[]>> = {
	img: ['image/'],
	source: ['image/', 'video/', 'audio/'],
	video: ['video/'],
	audio: ['audio/']
};

const SCHEME = /^([a-z][a-z\d+.-]*):/i;

const schemaCache = new WeakMap<HtmlSanitizePolicy, Schema>();

function schemaFor(policy: HtmlSanitizePolicy): Schema {
	const cached = schemaCache.get(policy);
	if (cached) return cached;

	const allowed = [
		...policy.attributes.map((attribute) => find(htmlSpace, attribute).property),
		...ARIA_PROPERTIES,
		...(policy.allowDataAttributes ? ['data*'] : [])
	];
	const schema: Schema = {
		tagNames: [...policy.elements],
		attributes: { '*': allowed },
		// Table parts only count inside a table.
		ancestors: defaultSchema.ancestors ?? {},
		strip: [...DROP_WITH_CONTENT],
		// URL schemes, clobbering and required attributes are enforced by
		// `harden` below, with browser-equivalent URL normalization.
		protocols: {},
		clobber: [],
		clobberPrefix: '',
		required: {},
		allowComments: false,
		allowDoctypes: false
	};
	schemaCache.set(policy, schema);
	return schema;
}

/**
 * Normalizes a URL the way the WHATWG URL parser does before reading its
 * scheme: leading/trailing C0 controls and spaces are trimmed and ASCII tabs
 * and newlines are removed anywhere (`java\tscript:` is `javascript:`).
 */
function normalizeUrl(value: string): string {
	let start = 0;
	let end = value.length;
	while (start < end && value.charCodeAt(start) <= 0x20) start++;
	while (end > start && value.charCodeAt(end - 1) <= 0x20) end--;
	let result = '';
	for (let index = start; index < end; index++) {
		const code = value.charCodeAt(index);
		if (code !== 0x09 && code !== 0x0a && code !== 0x0d) result += value[index];
	}
	return result;
}

/** The lowercased scheme of a URL, or `null` for a relative URL. */
function urlScheme(url: string): string | null {
	const match = SCHEME.exec(url);
	return match ? match[1].toLowerCase() : null;
}

/**
 * Whether a bare URL (one going straight into an `href` or `src`, not through
 * the HTML sanitizer) is relative or uses one of `schemes`. The scheme is read
 * after WHATWG normalization, so `java\tscript:` and ` JAVASCRIPT:` are seen
 * for what they are.
 */
export function isSafeUrl(value: string, schemes: readonly string[]): boolean {
	const scheme = urlScheme(normalizeUrl(value));
	return scheme === null || schemes.includes(scheme);
}

function isAllowedUrl(
	value: string,
	tagName: string,
	property: string,
	policy: HtmlSanitizePolicy
): boolean {
	const url = normalizeUrl(value);
	const scheme = urlScheme(url);
	if (scheme === null) return true; // relative URL
	if (policy.urlSchemes.includes(scheme)) return true;
	if (scheme === 'data' && property === 'src') {
		const media = DATA_URL_MEDIA[tagName];
		const mime = url.slice('data:'.length).toLowerCase();
		return media !== undefined && media.some((prefix) => mime.startsWith(prefix));
	}
	return false;
}

/** Every candidate URL of a `srcset` must pass; otherwise the attribute goes. */
function isAllowedSrcset(value: string, tagName: string, policy: HtmlSanitizePolicy): boolean {
	return value
		.split(',')
		.map((candidate) => candidate.trim().split(/\s+/)[0] ?? '')
		.every((url) => url === '' || isAllowedUrl(url, tagName, 'src', policy));
}

function isTrustedEmbed(src: unknown, hosts: readonly string[]): boolean {
	if (typeof src !== 'string') return false;
	let url: URL;
	try {
		// A protocol-relative `//host/path` resolves to https; a relative path
		// resolves to a host that is never trusted.
		url = new URL(normalizeUrl(src), 'https://relative.invalid');
	} catch {
		return false;
	}
	return url.protocol === 'https:' && hosts.includes(url.hostname);
}

function stringValue(value: Properties[string]): string {
	return Array.isArray(value) ? value.join(' ') : String(value);
}

function tokens(value: Properties[string]): string[] {
	if (Array.isArray(value)) return value.map(String);
	return typeof value === 'string' ? value.split(/\s+/).filter(Boolean) : [];
}

function hardenProperties(element: Element, policy: HtmlSanitizePolicy): void {
	const { properties, tagName } = element;

	for (const key of Object.keys(properties)) {
		const value = properties[key];
		if (value === undefined || value === null) continue;

		if (URL_PROPERTIES.has(key)) {
			const allowed =
				key === 'srcSet'
					? isAllowedSrcset(stringValue(value), tagName, policy)
					: isAllowedUrl(stringValue(value), tagName, key, policy);
			if (!allowed) delete properties[key];
		} else if ((key === 'id' || key === 'name') && CLOBBER_NAMES.has(stringValue(value))) {
			delete properties[key];
		}
	}

	if (tagName !== 'a') return;

	if (policy.links === 'external-new-tab') {
		properties.rel = ['noopener', 'noreferrer'];
		const href = properties.href;
		if (typeof href === 'string' && href.startsWith('http')) properties.target = '_blank';
	} else if (policy.links === 'secure-blank-targets' && properties.target === '_blank') {
		const rel = tokens(properties.rel);
		for (const required of ['noopener', 'noreferrer']) {
			if (!rel.includes(required)) rel.push(required);
		}
		properties.rel = rel;
	}
}

/** Returns the hardened children, dropping untrusted iframes. */
function hardenChildren<T extends RootContent | ElementContent>(
	children: T[],
	policy: HtmlSanitizePolicy
): T[] {
	const result: T[] = [];
	for (const child of children) {
		if (child.type !== 'element') {
			result.push(child);
			continue;
		}
		if (child.tagName === 'iframe') {
			if (
				policy.iframeHosts !== null &&
				!isTrustedEmbed(child.properties.src, policy.iframeHosts)
			) {
				continue;
			}
			// An iframe's children are unparsed fallback text, never content.
			child.children = [];
		}
		hardenProperties(child, policy);
		child.children = hardenChildren(child.children, policy);
		result.push(child);
	}
	return result;
}

function toRoot(tree: ReturnType<typeof sanitize>): Root {
	if (tree.type === 'root') return tree;
	// `sanitize` unwraps a single surviving child into the result itself.
	return { type: 'root', children: [tree] };
}

/**
 * The tree walks recurse once per nesting level. The server parser caps depth
 * the way Chromium's does, but a browser whose parser does not could hand over
 * a tree deep enough to exhaust the stack. Fail closed — render nothing — rather
 * than throw: a throw during SSR is a 500 for the whole page.
 */
function failClosedOnStackOverflow(run: () => string): string {
	try {
		return run();
	} catch (error) {
		if (error instanceof RangeError) return '';
		throw error;
	}
}

/** Sanitizes an HTML fragment under `policy` and returns safe HTML. */
export function sanitizeWithPolicy(html: string, policy: HtmlSanitizePolicy): string {
	if (!html) return '';
	return failClosedOnStackOverflow(() => {
		const tree = toRoot(sanitize(parseHtmlFragment(html), schemaFor(policy)));
		tree.children = hardenChildren(tree.children, policy);
		return toHtml(tree, { characterReferences: { useNamedReferences: true } });
	});
}

/**
 * Returns the text content of an HTML fragment: every element is unwrapped and
 * script, style and other non-content elements are dropped with their content.
 * The result is plain text, not HTML — interpolate it (`{text}`); never pass it
 * to `{@html}`.
 */
export function extractText(html: string): string {
	if (!html) return '';
	return failClosedOnStackOverflow(() => {
		const tree = toRoot(sanitize(parseHtmlFragment(html), schemaFor(TEXT_ONLY)));
		return tree.children.map((child) => (child.type === 'text' ? child.value : '')).join('');
	});
}
