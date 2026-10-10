/**
 * Shared sanitizer fixtures.
 *
 * The same cases run against the browser parser (jsdom, `engine.browser.test.ts`)
 * and the server parser (no DOM, `engine.server.test.ts`). Each case pins the
 * exact output, so the two suites together prove the server-rendered HTML is
 * byte-identical to what the browser computes for the same input.
 */

import { MODES, PROFILES, type HtmlSanitizePolicy } from '#lib/utils/html-sanitizer/policy.js';

export const POLICIES = {
	strict: PROFILES.strict,
	standard: PROFILES.standard,
	rich: PROFILES.rich,
	minimal: PROFILES.minimal,
	modeDefault: MODES.default,
	modeStrict: MODES.strict,
	modeCustom: MODES.custom,
	modeDefaultNoLinkHardening: { ...MODES.default, links: 'none' }
} as const satisfies Record<string, HtmlSanitizePolicy>;

export interface SanitizeCase {
	readonly name: string;
	readonly policy: keyof typeof POLICIES;
	readonly html: string;
	readonly expected: string;
}

export const CASES: readonly SanitizeCase[] = [
	{
		name: 'keeps formatting and escapes text and attribute values',
		policy: 'standard',
		html: '<p>Q&amp;A <strong>bold</strong> &lt;tag&gt; <a href="https://x.com/?a=1&b=2">x</a></p>',
		expected:
			'<p>Q&amp;A <strong>bold</strong> &lt;tag> <a href="https://x.com/?a=1&amp;b=2" rel="noopener noreferrer" target="_blank">x</a></p>'
	},
	{
		name: 'drops script-capable URL schemes however they are spelled',
		policy: 'standard',
		html: '<a href="javascript:alert(1)">a</a><a href="java&#x09;script:alert(1)">b</a><a href=" javascript:alert(1)">c</a><a href="JaVaScRiPt:alert(1)">d</a><a href="&#106;avascript:alert(1)">e</a><a href="vbscript:x">f</a><a href="data:text/html,x">g</a>',
		expected:
			'<a rel="noopener noreferrer">a</a><a rel="noopener noreferrer">b</a><a rel="noopener noreferrer">c</a><a rel="noopener noreferrer">d</a><a rel="noopener noreferrer">e</a><a rel="noopener noreferrer">f</a><a rel="noopener noreferrer">g</a>'
	},
	{
		name: 'keeps relative, fragment, mailto, tel and upper-case https URLs',
		policy: 'standard',
		html: '<a href="/p">1</a><a href="#top">2</a><a href="?q=1">3</a><a href="mailto:a@b.c">4</a><a href="tel:+1">5</a><a href="HTTPS://EXAMPLE.COM">6</a>',
		expected:
			'<a href="/p" rel="noopener noreferrer">1</a><a href="#top" rel="noopener noreferrer">2</a><a href="?q=1" rel="noopener noreferrer">3</a><a href="mailto:a@b.c" rel="noopener noreferrer">4</a><a href="tel:+1" rel="noopener noreferrer">5</a><a href="HTTPS://EXAMPLE.COM" rel="noopener noreferrer">6</a>'
	},
	{
		name: 'allows data: URLs only for image sources',
		policy: 'standard',
		html: '<img src="data:image/png;base64,iVBORw0KGgo=" alt="px"><img src="data:text/html,x"><a href="data:image/png;base64,AA">a</a>',
		expected:
			'<img src="data:image/png;base64,iVBORw0KGgo=" alt="px"><img><a rel="noopener noreferrer">a</a>'
	},
	{
		name: 'removes executable and foreign elements with their content, unwraps the rest',
		policy: 'standard',
		html: '<script>alert(1)</script><style>p{}</style><svg><a>s</a></svg><math><mi>m</mi></math><template><p>t</p></template><noscript>n</noscript><select><option>o</option></select><object data="x">obj</object><form><button>btn</button></form><p>kept</p>',
		expected: 'btn<p>kept</p>'
	},
	{
		name: 'strips event handlers and normalizes class lists',
		policy: 'standard',
		html: '<div onclick="x" onmouseover="y" class=" a  b ">d</div><img src="/i.png" onerror="alert(1)">',
		expected: '<div class="a b">d</div><img src="/i.png">'
	},
	{
		name: 'drops id values that would clobber document or form properties',
		policy: 'standard',
		html: '<p id="cookie">a</p><p id="getElementById">b</p><h2 id="getting-started">c</h2>',
		expected: '<p>a</p><p>b</p><h2 id="getting-started">c</h2>'
	},
	{
		name: 'keeps data-* and ARIA attributes, drops unlisted ones',
		policy: 'standard',
		html: '<ul><li data-x="1" aria-label="l" role="button" style="color:red">i</li></ul>',
		expected: '<ul><li data-x="1" aria-label="l">i</li></ul>'
	},
	{
		name: 'strict profile keeps basic formatting only',
		policy: 'strict',
		html: '<div class="c" id="i">a<span class="s">b</span><a href="https://x">l</a><img src="/i.png"></div>',
		expected: 'a<span class="s">b</span>l'
	},
	{
		name: 'minimal profile keeps inline emphasis only',
		policy: 'minimal',
		html: '<p class="c"><em>e</em> <b>b</b> <span>s</span></p>',
		expected: '<em>e</em> <b>b</b> s'
	},
	{
		name: 'rich profile keeps trusted https player embeds without fallback content',
		policy: 'rich',
		html: '<iframe src="https://www.youtube.com/embed/abc" allowfullscreen frameborder="0" srcdoc="<script>x</script>">fallback</iframe><iframe src="//player.vimeo.com/video/1"></iframe>',
		expected:
			'<iframe src="https://www.youtube.com/embed/abc" allowfullscreen frameborder="0"></iframe><iframe src="//player.vimeo.com/video/1"></iframe>'
	},
	{
		name: 'rich profile drops embeds from untrusted, look-alike or insecure origins',
		policy: 'rich',
		html: '<iframe src="https://evil.tld/?youtube.com"></iframe><iframe src="https://youtube.com.evil.tld/"></iframe><iframe src="http://www.youtube.com/embed/a"></iframe><iframe src="/embed"></iframe><iframe></iframe><p>p</p>',
		expected: '<p>p</p>'
	},
	{
		name: 'rich profile keeps media but checks every URL',
		policy: 'rich',
		html: '<video src="https://cdn.example/v.mp4" controls poster="javascript:alert(1)"><source src="https://cdn.example/v.webm"></video><audio src="data:audio/mp3;base64,AAAA" controls></audio>',
		expected:
			'<video src="https://cdn.example/v.mp4" controls><source src="https://cdn.example/v.webm"></video><audio src="data:audio/mp3;base64,AAAA" controls></audio>'
	},
	{
		name: 'tables keep their whole structure',
		policy: 'standard',
		html: '<table><caption>cap</caption><thead><tr><th>h</th></tr></thead><tfoot><tr><td>f</td></tr></tfoot><tbody><tr><td colspan="2">c</td></tr></tbody></table><td>loose</td>',
		expected:
			'<table><caption>cap</caption><thead><tr><th>h</th></tr></thead><tfoot><tr><td>f</td></tr></tfoot><tbody><tr><td>c</td></tr></tbody></table>loose'
	},
	{
		name: 'document mode keeps inline styles and secures target=_blank links',
		policy: 'modeDefault',
		html: '<p style="color: red">s</p><a href="https://x" target="_blank" rel="nofollow">x</a><a href="https://y">y</a>',
		expected:
			'<p style="color: red">s</p><a href="https://x" target="_blank" rel="nofollow noopener noreferrer">x</a><a href="https://y">y</a>'
	},
	{
		name: 'document mode without link hardening keeps rel as authored',
		policy: 'modeDefaultNoLinkHardening',
		html: '<a href="https://x" target="_blank" rel="nofollow">x</a>',
		expected: '<a href="https://x" target="_blank" rel="nofollow">x</a>'
	},
	{
		name: 'document strict mode drops every attribute',
		policy: 'modeStrict',
		html: '<p class="important" id="para">Text <sub>1</sub><a href="https://x">l</a></p>',
		expected: '<p>Text <sub>1</sub>l</p>'
	},
	{
		name: 'custom mode allows embeds from any https origin and validates srcset',
		policy: 'modeCustom',
		html: '<iframe src="https://any.example/e" sandbox="allow-scripts" allow="fullscreen">f</iframe><iframe src="javascript:alert(1)"></iframe><img src="/a.png" srcset="/a.png 1x, javascript:alert(1) 2x"><picture><source srcset="/a.webp 1x, /b.webp 2x" type="image/webp"><img src="/a.png"></picture>',
		expected:
			'<iframe src="https://any.example/e" sandbox="allow-scripts" allow="fullscreen"></iframe><iframe></iframe><img src="/a.png"><picture><source srcset="/a.webp 1x, /b.webp 2x" type="image/webp"><img src="/a.png"></picture>'
	},
	{
		name: 'drops comments, including malformed ones',
		policy: 'standard',
		html: '<!-- <img src=x onerror=alert(1)> --><p>c</p><!--><b>after</b>-->',
		expected: '<p>c</p><b>after</b>-->'
	},
	{
		name: 'mXSS: MathML/style namespace confusion',
		policy: 'rich',
		html: '<math><mtext><table><mglyph><style><img src=x onerror=alert(1)>',
		expected: ''
	},
	{
		name: 'mXSS: noscript parsed with scripting disabled, like the browser does',
		policy: 'standard',
		// With scripting enabled the title would end the noscript early and
		// surface an <img>; disabled, the whole payload stays an attribute.
		html: '<noscript><p title="</noscript><img src=x onerror=alert(1)>"></p></noscript><p>ok</p>',
		expected: '<p>ok</p>'
	},
	{
		name: 'mXSS: raw-text elements cannot smuggle markup',
		policy: 'standard',
		html: '<xmp><img src=x onerror=alert(1)></xmp><textarea><img title="</textarea><img src onerror=alert(1)>"></textarea>',
		expected: '&lt;img title="<img src="">">'
	},
	{
		name: 'empty and whitespace input',
		policy: 'standard',
		html: '   ',
		expected: '   '
	}
];

export interface TextCase {
	readonly name: string;
	readonly html: string;
	readonly expected: string;
}

export const TEXT_CASES: readonly TextCase[] = [
	{
		name: 'returns decoded plain text',
		html: '<p>Q&amp;A <b>bold</b> &lt;tag&gt; café</p>',
		expected: 'Q&A bold <tag> café'
	},
	{
		name: 'drops script and style bodies',
		html: 'Before<script>evil()</script><style>p{}</style>After',
		expected: 'BeforeAfter'
	}
];

/** Classic and mutation-XSS payloads; nothing executable may survive any policy. */
export const XSS_PAYLOADS: readonly string[] = [
	'<img src=x onerror=alert(1)>',
	'<svg onload=alert(1)>',
	'<svg><script>alert(1)</script></svg>',
	'<a href="javas&#x09;cript:alert(1)">x</a>',
	'<a href="&#14;javascript:alert(1)">x</a>',
	'<a href="java&#0000script:alert(1)">x</a>',
	'<body onload=alert(1)>',
	'<input onfocus=alert(1) autofocus>',
	'<object data="javascript:alert(1)"></object>',
	'<iframe src="javascript:alert(1)"></iframe>',
	'<math><mi xlink:href="javascript:alert(1)">x</mi></math>',
	'<form><math><mtext></form><form><mglyph><style></math><img src onerror=alert(1)>',
	'<svg></p><style><a id="</style><img src=1 onerror=alert(1)>">',
	'<noembed><img title="</noembed><img src onerror=alert(1)>"></noembed>',
	'<noframes><img title="</noframes><img src onerror=alert(1)>"></noframes>',
	'<title><img title="</title><img src onerror=alert(1)>"></title>',
	'<base href="javascript:alert(1)//"><a href="/x">x</a>',
	'<meta http-equiv="refresh" content="0;url=javascript:alert(1)">',
	'<a title="x&quot; onclick=&quot;alert(1)" href="#">q</a>',
	'<img src="x" alt="&quot;><script>alert(1)</script>">',
	'<frameset onload=alert(1)>',
	'<svg><a xlink:href="javascript:alert(1)"><text>t</text></a></svg>',
	'<!--><img src=x onerror=alert(1)>-->'
];
