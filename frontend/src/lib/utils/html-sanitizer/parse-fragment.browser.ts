/**
 * HTML fragment parser — browser build.
 *
 * Selected by the `browser` condition of `#lib/utils/html-sanitizer/parse-fragment.js`
 * in package.json `imports`; every other build gets `parse-fragment.server.ts`.
 *
 * Uses the browser's own HTML parser through `DOMParser`, as DOMPurify does: the
 * document it creates has no browsing context, so nothing in it runs or loads,
 * and the specification fixes its scripting flag to disabled in every engine
 * (unlike `innerHTML`, where it depends on the context element). The server
 * build parses the same prefixed document with parse5. Keeping parse5 out of
 * the client bundle is the point of the split — it would add ~45 KB gzipped.
 */

import type { Root } from 'hast';
import { fromDom } from 'hast-util-from-dom';
import { DOCUMENT_PREFIX } from '#lib/utils/html-sanitizer/policy.js';

export function parseHtmlFragment(html: string): Root {
	const { body } = new DOMParser().parseFromString(DOCUMENT_PREFIX + html, 'text/html');
	const tree = fromDom(body);
	if (tree.type !== 'element') {
		throw new TypeError(`Expected the parsed <body> as a hast element, got "${tree.type}"`);
	}
	return { type: 'root', children: tree.children };
}
