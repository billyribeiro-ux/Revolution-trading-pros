/**
 * Canonical JSON-LD Serializer - single source of truth
 * ===========================================================================
 *
 * WHY THIS MODULE EXISTS
 * ----------------------
 * JSON-LD structured data is emitted into the document head as a raw
 * `<script type="application/ld+json">...</script>` element (via Svelte
 * `{@html}`). That is the *correct* transport for JSON-LD - it must be a
 * literal script element, not escaped HTML text, or crawlers will not parse
 * it.
 *
 * The danger is that `JSON.stringify` does NOT escape characters that are
 * structural in HTML. A schema field sourced from the CMS / user content
 * (post headline, FAQ question, author name, breadcrumb name, video name,
 * etc.) that contains the byte sequence `</script>` will terminate the
 * `<script>` element early, and any markup that follows becomes live DOM.
 * That is the confirmed stored-XSS class (audit FULL_REPO_AUDIT_2026-05-17
 * section P0-6).
 *
 * `<`, `>`, `&` are likewise meaningful to the HTML tokenizer, and the
 * Unicode line separator (code point 0x2028) / paragraph separator
 * (code point 0x2029) are valid in JSON strings but are illegal *unescaped*
 * in a JavaScript / `<script>` context in older engines and break some
 * JSON-LD consumers.
 *
 * THE FIX
 * -------
 * Escape, at the JSON *string* level, every character that is structural to
 * the surrounding HTML `<script>` element, using the corresponding JSON
 * unicode escape (`\uXXXX`). Because these are ordinary JSON string escapes,
 * the output:
 *
 *   1. is still 100% valid JSON - `JSON.parse()` round-trips to the exact
 *      original value;
 *   2. is still valid JSON-LD - schema.org consumers `JSON.parse` the
 *      script body, so the escaped form is semantically identical;
 *   3. can NEVER break out of the `<script>` element, because the literal
 *      characters never appear unescaped in the output.
 *
 * Escaped characters and the JSON escape each is replaced with:
 *   `<`           -> backslash-u003C  (prevents `</script>` / `<!--` /
 *                    `<script>` breakout)
 *   `>`           -> backslash-u003E  (defense in depth; closes the
 *                    `</...>` vector)
 *   `&`           -> backslash-u0026  (prevents HTML entity /
 *                    `&lt;script&gt;` tricks)
 *   0x2028 (LS)   -> backslash-u2028  (line separator: illegal unescaped
 *                    in a script element)
 *   0x2029 (PS)   -> backslash-u2029  (paragraph separator: illegal
 *                    unescaped in a script element)
 *
 * NOTE: This source file references U+2028 / U+2029 only via their code
 * points (0x2028 / 0x2029) and `String.fromCharCode`, never as literal
 * characters - a literal U+2028 / U+2029 is itself a line terminator in
 * ECMAScript source and would corrupt this file.
 *
 * DETERMINISM
 * -----------
 * `SEOHead.svelte` previously carried its own `stableStringify` so that the
 * server-rendered and client-rendered JSON-LD bytes matched (avoiding a
 * hydration mismatch). That requirement is folded in here: object keys are
 * emitted in a stable (lexicographically sorted) order regardless of
 * insertion order, and `undefined` members are dropped (matching
 * `JSON.stringify` semantics). This is now the ONLY implementation - every
 * call site delegates to it, so there is exactly one place that can ever be
 * wrong, and it is covered by tests.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since May 2026
 */

const LINE_SEPARATOR = String.fromCharCode(0x2028);
const PARAGRAPH_SEPARATOR = String.fromCharCode(0x2029);

/**
 * Characters that are structural to the HTML `<script>` element (or illegal
 * unescaped inside one) mapped to their JSON `\uXXXX` escape. Using the JSON
 * unicode escape keeps the result valid JSON while making breakout
 * impossible.
 */
const HTML_UNSAFE_IN_SCRIPT: Readonly<Record<string, string>> = {
	'<': '\\u003C',
	'>': '\\u003E',
	'&': '\\u0026',
	[LINE_SEPARATOR]: '\\u2028',
	[PARAGRAPH_SEPARATOR]: '\\u2029'
};

const HTML_UNSAFE_PATTERN = new RegExp('[<>&\\u2028\\u2029]', 'g');

/**
 * Escape an already-serialized JSON string so it is safe to embed verbatim
 * inside an HTML `<script>` element. Operates on the JSON text, so the
 * substitutions land inside JSON string literals as valid `\uXXXX` escapes
 * and the document still `JSON.parse`s to the identical value.
 */
function escapeForScriptContext(json: string): string {
	return json.replace(HTML_UNSAFE_PATTERN, (ch) => HTML_UNSAFE_IN_SCRIPT[ch] ?? ch);
}

/**
 * Deterministically serialize a value to JSON text.
 *
 * Object keys are emitted in lexicographic order so the output bytes are
 * stable across object construction / key-insertion order - required so
 * server and client render byte-identical JSON-LD and Svelte does not flag a
 * hydration mismatch. `undefined` object members are omitted, matching
 * `JSON.stringify` behaviour. Arrays preserve order (array order is
 * semantically meaningful, e.g. `itemListElement` / `position`).
 */
function stableJsonStringify(value: unknown): string {
	if (value === null || typeof value !== 'object') {
		return JSON.stringify(value) ?? 'null';
	}
	if (Array.isArray(value)) {
		return '[' + value.map((v) => stableJsonStringify(v === undefined ? null : v)).join(',') + ']';
	}
	const record = value as Record<string, unknown>;
	const keys = Object.keys(record).sort();
	const pairs: string[] = [];
	for (const key of keys) {
		const member = record[key];
		if (member === undefined) continue;
		pairs.push(`${JSON.stringify(key)}:${stableJsonStringify(member)}`);
	}
	return '{' + pairs.join(',') + '}';
}

/**
 * Serialize a JSON-LD object (or array of objects) to a string that is SAFE
 * to embed verbatim inside an HTML `<script type="application/ld+json">`
 * element.
 *
 * This is the single canonical serializer. Every JSON-LD emission site MUST
 * route through it - do not hand-roll `JSON.stringify` into a script tag.
 *
 * Guarantees:
 *  - The result never contains an unescaped `<`, `>`, `&`, U+2028 or
 *    U+2029, so it cannot terminate or escape the host `<script>` element.
 *  - The result is valid JSON and valid JSON-LD: `JSON.parse(result)`
 *    yields a value deep-equal to the input.
 *  - Output is deterministic: object key order is stable regardless of
 *    insertion order, so SSR and CSR produce identical bytes.
 *
 * @param node A JSON-LD node, array of nodes, or plain object/graph.
 * @returns Injection-safe JSON text for use inside a `<script>` element.
 */
export function serializeJsonLd(
	node: Record<string, unknown> | Array<Record<string, unknown>> | unknown
): string {
	return escapeForScriptContext(stableJsonStringify(node));
}
