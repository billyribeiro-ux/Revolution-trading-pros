/**
 * HTML sanitization policies.
 * ═══════════════════════════════════════════════════════════════════════════
 * A policy is pure data: which elements and attributes survive, which URL
 * schemes a URL-valued attribute may use, and how links and iframes are
 * hardened afterwards. The engine (`./engine.ts`) enforces it identically on
 * the server and in the browser, so the HTML a page is server-rendered with is
 * byte-for-byte the HTML the client computes — `{@html}` hydrates without a
 * mismatch.
 *
 * Names are HTML attribute names (`class`, `colspan`, `allowfullscreen`), not
 * DOM/hast property names; the engine maps them.
 */

/**
 * Both parsers parse a whole document — this prefix followed by the fragment —
 * and sanitize the `<body>`'s children. The explicit `<body>` puts the fragment
 * in body context from its first character, so leading `<style>`, `<meta>` or
 * `<noscript>` are not hoisted into `<head>` and silently dropped from view.
 */
export const DOCUMENT_PREFIX = '<!DOCTYPE html><html><head></head><body>';

/** How `<a>` elements are hardened after sanitizing. */
export type LinkHardening =
	/**
	 * Every link gets `rel="noopener noreferrer"`; links whose `href` starts with
	 * `http` also get `target="_blank"`.
	 */
	| 'external-new-tab'
	/** Links with `target="_blank"` get `noopener` and `noreferrer` added to their `rel`. */
	| 'secure-blank-targets'
	/** Links are left as authored (after the allowlist). */
	| 'none';

export interface HtmlSanitizePolicy {
	/** Elements that are kept. Any other element is unwrapped (its children are kept)
	 * unless it is in `DROP_WITH_CONTENT`, in which case it is removed with its content. */
	readonly elements: readonly string[];
	/** Attributes allowed on every kept element. */
	readonly attributes: readonly string[];
	/** Schemes a URL-valued attribute may use (lowercase, without `:`). Scheme-less
	 * (relative) URLs are always allowed. */
	readonly urlSchemes: readonly string[];
	/** Whether `data-*` attributes are kept. */
	readonly allowDataAttributes: boolean;
	readonly links: LinkHardening;
	/**
	 * Hosts an `<iframe src>` may point at (exact hostname match over `https:`).
	 * `null` allows any host whose URL passes `urlSchemes`. Only relevant when
	 * `iframe` is in `elements`.
	 */
	readonly iframeHosts: readonly string[] | null;
}

/**
 * Elements removed together with everything inside them when they are not
 * allowed — never unwrapped. DOMPurify's default `FORBID_CONTENTS` (script and
 * style bodies, foreign SVG/MathML content, raw-text and template-like
 * elements, media fallback content), plus:
 * - plugin elements (`object`, `embed`, `applet`), whose content is fallback;
 * - `select`, whose content model browsers are relaxing ahead of parse5
 *   (Chromium already parses arbitrary elements inside `<option>`), so the
 *   server and browser parsers can disagree about what is inside it.
 */
export const DROP_WITH_CONTENT: readonly string[] = [
	'annotation-xml',
	'applet',
	'audio',
	'colgroup',
	'desc',
	'embed',
	'foreignObject',
	'foreignobject',
	'head',
	'iframe',
	'math',
	'mi',
	'mn',
	'mo',
	'ms',
	'mtext',
	'noembed',
	'noframes',
	'noscript',
	'object',
	'plaintext',
	'script',
	'select',
	'style',
	'svg',
	'template',
	'thead',
	'title',
	'video',
	'xmp'
];

/** Attributes whose value is a URL (or, for `srcset`, a list of URLs). */
export const URL_ATTRIBUTES: ReadonlySet<string> = new Set([
	'href',
	'src',
	'cite',
	'poster',
	'srcset'
]);

/** Hosts trusted for embedded players in the `rich` profile. */
export const TRUSTED_EMBED_HOSTS: readonly string[] = [
	'youtube.com',
	'www.youtube.com',
	'youtube-nocookie.com',
	'www.youtube-nocookie.com',
	'player.vimeo.com',
	'vimeo.com'
];

const BASIC_SCHEMES = ['http', 'https', 'mailto', 'tel'] as const;

/** Schemes accepted by `sanitizeHTML` in `#lib/utils/sanitization.js`. */
const DOCUMENT_SCHEMES = [
	'http',
	'https',
	'ftp',
	'ftps',
	'mailto',
	'tel',
	'callto',
	'sms',
	'cid',
	'xmpp'
] as const;

/** DOMPurify's default `ALLOWED_URI_REGEXP` schemes. */
const EXTENDED_SCHEMES = [
	'http',
	'https',
	'ftp',
	'ftps',
	'mailto',
	'tel',
	'callto',
	'sms',
	'cid',
	'xmpp',
	'matrix'
] as const;

const TEXT_FORMATTING = ['b', 'i', 'em', 'strong', 'u', 's', 'br', 'p', 'span'] as const;

/**
 * A policy that allows `table` allows its whole structure. Unwrapping a
 * disallowed `caption` or `tfoot` would leave its content directly inside the
 * table, where the browser re-parses it out in front of the table ("foster
 * parenting") — the output would no longer describe the DOM it produces.
 */
const TABLE_ELEMENTS = [
	'table',
	'caption',
	'colgroup',
	'col',
	'thead',
	'tbody',
	'tfoot',
	'tr',
	'th',
	'td'
] as const;

const CONTENT_ELEMENTS = [
	...TEXT_FORMATTING,
	'div',
	'h1',
	'h2',
	'h3',
	'h4',
	'h5',
	'h6',
	'ul',
	'ol',
	'li',
	'blockquote',
	'pre',
	'code',
	'a',
	'img',
	'figure',
	'figcaption',
	...TABLE_ELEMENTS
] as const;

const CONTENT_ATTRIBUTES = [
	'class',
	'id',
	'href',
	'target',
	'rel',
	'src',
	'alt',
	'title',
	'width',
	'height',
	'loading'
] as const;

// ─────────────────────────────────────────────────────────────────────────────
// Profiles used by `#lib/utils/sanitize.js`
// ─────────────────────────────────────────────────────────────────────────────

export const PROFILES = {
	/** Basic formatting only — no links, no images. */
	strict: {
		elements: TEXT_FORMATTING,
		attributes: ['class'],
		urlSchemes: EXTENDED_SCHEMES,
		allowDataAttributes: true,
		links: 'external-new-tab',
		iframeHosts: TRUSTED_EMBED_HOSTS
	},
	/** Common content elements for user-facing copy (popups, overlays, tables). */
	standard: {
		elements: CONTENT_ELEMENTS,
		attributes: CONTENT_ATTRIBUTES,
		urlSchemes: EXTENDED_SCHEMES,
		allowDataAttributes: true,
		links: 'external-new-tab',
		iframeHosts: TRUSTED_EMBED_HOSTS
	},
	/** Admin-authored long-form content: adds media and trusted player embeds. */
	rich: {
		elements: [...CONTENT_ELEMENTS, 'video', 'source', 'audio', 'iframe'],
		attributes: [
			...CONTENT_ATTRIBUTES,
			'controls',
			'autoplay',
			'muted',
			'loop',
			'poster',
			'frameborder',
			'allowfullscreen',
			'allow'
		],
		urlSchemes: BASIC_SCHEMES,
		allowDataAttributes: true,
		links: 'external-new-tab',
		iframeHosts: TRUSTED_EMBED_HOSTS
	},
	/** Inline emphasis only. */
	minimal: {
		elements: ['b', 'i', 'em', 'strong', 'br'],
		attributes: [],
		urlSchemes: EXTENDED_SCHEMES,
		allowDataAttributes: true,
		links: 'external-new-tab',
		iframeHosts: TRUSTED_EMBED_HOSTS
	}
} as const satisfies Record<string, HtmlSanitizePolicy>;

// ─────────────────────────────────────────────────────────────────────────────
// Modes used by `#lib/utils/sanitization.js`
// ─────────────────────────────────────────────────────────────────────────────

const DOCUMENT_ELEMENTS = [
	'p',
	'br',
	'strong',
	'em',
	'u',
	's',
	'a',
	'ul',
	'ol',
	'li',
	'h1',
	'h2',
	'h3',
	'h4',
	'h5',
	'h6',
	'blockquote',
	'code',
	'pre',
	'div',
	'span',
	'img',
	...TABLE_ELEMENTS,
	'figure',
	'figcaption',
	'hr',
	'sub',
	'sup',
	'mark',
	'abbr',
	'cite',
	'del',
	'ins',
	'kbd',
	'samp',
	'var',
	'details',
	'summary'
] as const;

const DOCUMENT_ATTRIBUTES = [
	'href',
	'title',
	'target',
	'rel',
	'class',
	'id',
	'src',
	'alt',
	'width',
	'height',
	'style',
	'colspan',
	'rowspan',
	'scope',
	'datetime',
	'cite',
	'lang',
	'dir',
	'open'
] as const;

export const MODES = {
	default: {
		elements: DOCUMENT_ELEMENTS,
		attributes: DOCUMENT_ATTRIBUTES,
		urlSchemes: DOCUMENT_SCHEMES,
		allowDataAttributes: false,
		links: 'secure-blank-targets',
		iframeHosts: null
	},
	strict: {
		elements: ['p', 'br', 'strong', 'em', 'u', 's', 'sub', 'sup'],
		attributes: [],
		urlSchemes: DOCUMENT_SCHEMES,
		allowDataAttributes: false,
		links: 'secure-blank-targets',
		iframeHosts: null
	},
	/** Admin-authored raw HTML blocks: adds media and embeds from any https host. */
	custom: {
		elements: [
			...DOCUMENT_ELEMENTS,
			'iframe',
			'video',
			'audio',
			'source',
			'track',
			'picture',
			'canvas'
		],
		attributes: [
			...DOCUMENT_ATTRIBUTES,
			'frameborder',
			'allowfullscreen',
			'controls',
			'autoplay',
			'muted',
			'loop',
			'playsinline',
			'poster',
			'preload',
			'srcset',
			'sizes',
			'media',
			'type',
			'kind',
			'srclang',
			'label',
			'default',
			'loading',
			'decoding',
			'crossorigin',
			'referrerpolicy',
			'sandbox',
			'allow'
		],
		urlSchemes: DOCUMENT_SCHEMES,
		allowDataAttributes: false,
		links: 'secure-blank-targets',
		iframeHosts: null
	}
} as const satisfies Record<string, HtmlSanitizePolicy>;

/** Text extraction: every element is unwrapped, `DROP_WITH_CONTENT` is removed. */
export const TEXT_ONLY: HtmlSanitizePolicy = {
	elements: [],
	attributes: [],
	urlSchemes: [],
	allowDataAttributes: false,
	links: 'secure-blank-targets',
	iframeHosts: null
};
