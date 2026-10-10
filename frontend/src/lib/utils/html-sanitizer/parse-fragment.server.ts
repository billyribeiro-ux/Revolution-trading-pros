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
import { parse, type DefaultTreeAdapterMap } from 'parse5';
import { DOCUMENT_PREFIX } from '#lib/utils/html-sanitizer/policy.js';

type ParentNode = DefaultTreeAdapterMap['parentNode'];
type Element = DefaultTreeAdapterMap['element'];

function childElement(parent: ParentNode, tagName: string): Element {
	const found = parent.childNodes.find(
		(node): node is Element => 'tagName' in node && node.tagName === tagName
	);
	if (!found) throw new TypeError(`Parsed document has no <${tagName}> element`);
	return found;
}

export function parseHtmlFragment(html: string): Root {
	const document = parse(DOCUMENT_PREFIX + html, { scriptingEnabled: false });
	const body = fromParse5(childElement(childElement(document, 'html'), 'body'));
	if (body.type !== 'element') {
		throw new TypeError(`Expected the parsed <body> as a hast element, got "${body.type}"`);
	}
	return { type: 'root', children: body.children };
}
