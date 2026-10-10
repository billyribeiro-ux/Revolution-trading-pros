/**
 * HTML fragment parser — server build (SSR, prerender, Node, workerd).
 *
 * Selected by the `default` condition of `#lib/utils/html-sanitizer/parse-fragment.js`
 * in package.json `imports`; the browser build gets `parse-fragment.browser.ts`.
 *
 * parse5 is a WHATWG-conformant HTML parser (the one jsdom uses) with no DOM
 * or Node built-in dependencies, so it runs unchanged in Cloudflare's workerd.
 * It parses exactly what the browser build hands to `DOMParser` — the same
 * document prefix, with scripting disabled — so both builds produce the same
 * tree for the same input.
 */

import type { Root } from 'hast';
import { fromParse5 } from 'hast-util-from-parse5';
import { defaultTreeAdapter, parse, type DefaultTreeAdapterMap, type TreeAdapter } from 'parse5';
import { DOCUMENT_PREFIX } from '#lib/utils/html-sanitizer/policy.js';

type ParentNode = DefaultTreeAdapterMap['parentNode'];
type Element = DefaultTreeAdapterMap['element'];

/**
 * Chromium's and WebKit's `kMaximumHTMLParserDOMTreeDepth`. Their parsers
 * attach an element to its intended parent's parent once that parent sits this
 * deep, so the tree never grows past it. parse5 has no such cap, and the hast
 * conversion, sanitizer and serializer all recurse per level, so a few thousand
 * nested tags would exhaust the stack and fail the whole server render. The
 * tree adapter below applies the browsers' rule, keeping the server tree both
 * bounded and identical to the one `DOMParser` builds.
 */
const MAX_TREE_DEPTH = 512;

/** Whether `node` lies more than `MAX_TREE_DEPTH` levels below the document. */
function isTooDeep(node: ParentNode): boolean {
	let depth = 0;
	for (
		let current: ParentNode | null = node;
		current;
		current = defaultTreeAdapter.getParentNode(current)
	) {
		if (++depth > MAX_TREE_DEPTH + 1) return true;
	}
	return false;
}

function attachmentParent(parent: ParentNode): ParentNode {
	return isTooDeep(parent) ? (defaultTreeAdapter.getParentNode(parent) ?? parent) : parent;
}

const depthCappedTreeAdapter: TreeAdapter<DefaultTreeAdapterMap> = {
	...defaultTreeAdapter,
	// Elements (and comments) are re-parented; text is not — Chromium appends
	// text to the current node even past the limit, and text adds no depth.
	appendChild(parent, child) {
		defaultTreeAdapter.appendChild(attachmentParent(parent), child);
	}
};

function childElement(parent: ParentNode, tagName: string): Element {
	const found = parent.childNodes.find(
		(node): node is Element => 'tagName' in node && node.tagName === tagName
	);
	if (!found) throw new TypeError(`Parsed document has no <${tagName}> element`);
	return found;
}

export function parseHtmlFragment(html: string): Root {
	const document = parse(DOCUMENT_PREFIX + html, {
		scriptingEnabled: false,
		treeAdapter: depthCappedTreeAdapter
	});
	const body = fromParse5(childElement(childElement(document, 'html'), 'body'));
	if (body.type !== 'element') {
		throw new TypeError(`Expected the parsed <body> as a hast element, got "${body.type}"`);
	}
	return { type: 'root', children: body.children };
}
