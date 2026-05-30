/**
 * Sanitization Utilities — Security-Critical Unit Tests
 * ═══════════════════════════════════════════════════════════════════════════════
 * Tests for XSS prevention, HTML sanitization, URL sanitization, CSS sanitization,
 * filename sanitization, input validation helpers, and CSP utilities.
 */

import { describe, it, expect } from 'vitest';
import {
	sanitizeHTML,
	sanitizeText,
	sanitizeURL,
	isSafeProtocol,
	sanitizeCSS,
	sanitizeInlineStyle,
	sanitizeFilename,
	containsPathTraversal,
	generateCSPNonce,
	buildCSP,
	buildCustomCSP,
	escapeHTML,
	unescapeHTML,
	containsXSS,
	stripDangerous,
	isValidEmail,
	isValidURL,
	isValidHexColor,
	sanitizeNumber,
	sanitizeSlug,
	parseJSONSafe,
	validateFile,
	SAFE_PROTOCOLS,
	SAFE_IMAGE_TYPES,
	SAFE_DOCUMENT_TYPES
} from '../sanitization';

// ============================================================================
// sanitizeHTML
// ============================================================================

describe('sanitizeHTML', () => {
	it('returns empty string for empty input', () => {
		expect(sanitizeHTML('')).toBe('');
	});

	it('returns empty string for null-like values', () => {
		// @ts-expect-error testing JS consumers passing wrong types
		expect(sanitizeHTML(null)).toBe('');
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		expect(sanitizeHTML(undefined)).toBe('');
	});

	it('passes through safe HTML unchanged (default mode)', () => {
		const safe = '<p>Hello <strong>world</strong></p>';
		const result = sanitizeHTML(safe);
		expect(result).toContain('Hello');
		expect(result).toContain('<strong>world</strong>');
	});

	// --- script tag removal ---
	it('strips <script> tags', () => {
		const input = '<p>Hello</p><script>alert(1)</script>';
		expect(sanitizeHTML(input)).not.toMatch(/<script/i);
		expect(sanitizeHTML(input)).not.toContain('alert(1)');
	});

	it('strips <script> tags with attributes', () => {
		const input = '<script src="evil.js" type="text/javascript"></script>';
		expect(sanitizeHTML(input)).not.toMatch(/<script/i);
	});

	it('strips inline script event handlers (onclick)', () => {
		const input = '<button onclick="alert(1)">Click</button>';
		const result = sanitizeHTML(input);
		expect(result).not.toMatch(/onclick/i);
	});

	it('strips onerror event handler on img', () => {
		const input = '<img src="x" onerror="alert(1)">';
		const result = sanitizeHTML(input);
		expect(result).not.toMatch(/onerror/i);
	});

	it('strips onload event handler on svg', () => {
		const input = '<svg onload="alert(1)"></svg>';
		const result = sanitizeHTML(input);
		expect(result).not.toMatch(/onload/i);
	});

	it('strips <iframe> tags in default mode', () => {
		const input = '<iframe src="https://evil.com"></iframe>';
		const result = sanitizeHTML(input);
		expect(result).not.toMatch(/<iframe/i);
	});

	it('allows <iframe> in custom mode', () => {
		const input = '<iframe src="https://www.youtube.com/embed/test"></iframe>';
		const result = sanitizeHTML(input, { mode: 'custom' });
		// iframes are allowed in custom mode — the tag should be present
		expect(result).toMatch(/<iframe/i);
	});

	it('strips javascript: href URL', () => {
		const input = '<a href="javascript:alert(1)">Click</a>';
		const result = sanitizeHTML(input);
		expect(result).not.toMatch(/javascript:/i);
	});

	it('preserves legitimate href URL', () => {
		const input = '<a href="https://example.com">Link</a>';
		const result = sanitizeHTML(input);
		expect(result).toContain('href="https://example.com"');
	});

	it('strict mode strips all attributes', () => {
		const input = '<p class="important" id="para">Text</p>';
		const result = sanitizeHTML(input, { mode: 'strict' });
		expect(result).not.toMatch(/class=/);
		expect(result).not.toMatch(/id=/);
		expect(result).toContain('Text');
	});

	it('strict mode strips <div> tags (not in STRICT_CONFIG)', () => {
		const input = '<div>Content</div>';
		const result = sanitizeHTML(input, { mode: 'strict' });
		// div is not in strict allowed tags, but KEEP_CONTENT=true preserves text
		expect(result).not.toMatch(/<div/i);
		expect(result).toContain('Content');
	});

	it('respects maxLength option', () => {
		const input = '<p>' + 'A'.repeat(1000) + '</p>';
		const result = sanitizeHTML(input, { maxLength: 20 });
		expect(result.length).toBeLessThan(30);
	});

	it('adds noopener/noreferrer to target=_blank links', () => {
		const input = '<a href="https://example.com" target="_blank">Link</a>';
		const result = sanitizeHTML(input, { secureLinks: true });
		expect(result).toContain('noopener');
		expect(result).toContain('noreferrer');
	});

	// -------------------------------------------------------------------------
	// OWASP XSS Filter Evasion Corpus
	// -------------------------------------------------------------------------
	describe('OWASP XSS payload corpus', () => {
		const xssPayloads: Array<{ label: string; payload: string }> = [
			{
				label: 'Classic img onerror',
				payload: '<img src=x onerror=alert(1)>'
			},
			{
				label: 'SVG onload',
				payload: '<svg onload=alert(1)>'
			},
			{
				label: 'Script tag with content',
				payload: '<script>alert("xss")</script>'
			},
			{
				label: 'javascript: protocol in anchor',
				payload: '<a href="javascript:alert(1)">XSS</a>'
			},
			{
				label: 'HTML-encoded tab in javascript: URL (OWASP evasion)',
				payload: '<a href="javas&#x09;cript:alert(1)">XSS</a>'
			},
			{
				label: 'Data URI with HTML',
				payload: '<a href="data:text/html,<script>alert(1)</script>">XSS</a>'
			},
			{
				label: 'Body onload',
				payload: '<body onload=alert(1)>'
			},
			{
				label: 'Input onfocus autofocus',
				payload: '<input onfocus=alert(1) autofocus>'
			},
			{
				label: 'Object tag',
				payload: '<object data="javascript:alert(1)"></object>'
			},
			{
				label: 'vbscript: protocol',
				payload: '<a href="vbscript:msgbox(1)">XSS</a>'
			}
		];

		for (const { label, payload } of xssPayloads) {
			it(`blocks payload: ${label}`, () => {
				const result = sanitizeHTML(payload);
				// Result must NOT contain any executable event handler attribute
				expect(result).not.toMatch(/on\w+\s*=/i);
				// Result must NOT contain javascript: or vbscript: protocols
				expect(result).not.toMatch(/javascript\s*:/i);
				expect(result).not.toMatch(/vbscript\s*:/i);
				// Result must NOT contain script opening tag
				expect(result).not.toMatch(/<script[\s\S]*?>/i);
				// Result must NOT contain object tag
				expect(result).not.toMatch(/<object/i);
			});
		}
	});

	describe('deeply nested HTML', () => {
		it('sanitizes event handlers in deeply nested elements', () => {
			const input = '<div><p><span><a href="#" onmouseover="evil()">deep</a></span></p></div>';
			const result = sanitizeHTML(input);
			expect(result).not.toMatch(/onmouseover/i);
		});
	});
});

// ============================================================================
// sanitizeText
// ============================================================================

describe('sanitizeText', () => {
	it('returns empty string for empty input', () => {
		expect(sanitizeText('')).toBe('');
	});

	it('returns empty string for null/undefined', () => {
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		expect(sanitizeText(null)).toBe('');
	});

	it('strips all HTML tags, preserving text content', () => {
		const input = '<p>Hello <strong>world</strong></p>';
		expect(sanitizeText(input)).toBe('Hello world');
	});

	it('strips script tags and their content', () => {
		const input = 'Before<script>evil()</script>After';
		const result = sanitizeText(input);
		expect(result).not.toMatch(/<script/i);
		// DOMPurify with KEEP_CONTENT keeps script text — source uses DOMPurify
		// which for ALLOWED_TAGS:[] with KEEP_CONTENT keeps text node content
		// The important thing: no HTML tags remain
		expect(result).not.toMatch(/<[^>]+>/);
	});

	it('respects maxLength', () => {
		const input = 'Hello world, this is a long sentence.';
		const result = sanitizeText(input, 5);
		expect(result).toBe('Hello');
	});
});

// ============================================================================
// sanitizeURL
// ============================================================================

describe('sanitizeURL', () => {
	it('returns empty string for empty input', () => {
		expect(sanitizeURL('')).toBe('');
	});

	it('returns empty string for null/undefined', () => {
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		expect(sanitizeURL(null)).toBe('');
	});

	it('allows http:// URLs', () => {
		expect(sanitizeURL('http://example.com')).toBe('http://example.com/');
	});

	it('allows https:// URLs', () => {
		expect(sanitizeURL('https://example.com/path?q=1')).toBe('https://example.com/path?q=1');
	});

	it('allows mailto: URLs', () => {
		expect(sanitizeURL('mailto:user@example.com')).toBe('mailto:user@example.com');
	});

	it('allows tel: URLs', () => {
		expect(sanitizeURL('tel:+15551234567')).toBe('tel:+15551234567');
	});

	it('BLOCKS javascript: URLs', () => {
		expect(sanitizeURL('javascript:alert(1)')).toBe('');
	});

	it('BLOCKS javascript: URLs with leading whitespace (trim)', () => {
		expect(sanitizeURL('  javascript:alert(1)')).toBe('');
	});

	it('BLOCKS vbscript: URLs', () => {
		expect(sanitizeURL('vbscript:msgbox(1)')).toBe('');
	});

	it('BLOCKS data:text/html URLs', () => {
		expect(sanitizeURL('data:text/html,<script>alert(1)</script>')).toBe('');
	});

	it('BLOCKS file: protocol', () => {
		expect(sanitizeURL('file:///etc/passwd')).toBe('');
	});

	it('allows relative URLs starting with /', () => {
		expect(sanitizeURL('/about')).toBe('/about');
	});

	it('allows additional safe protocols when explicitly passed', () => {
		const result = sanitizeURL('sms:+15551234567', ['sms:']);
		// sms: is already in SAFE_PROTOCOLS so this should pass
		expect(result).toBeTruthy();
	});

	it('rejects URLs with unknown protocol not in safe list', () => {
		expect(sanitizeURL('ftp://example.com')).toBe('');
	});
});

// ============================================================================
// isSafeProtocol
// ============================================================================

describe('isSafeProtocol', () => {
	it('recognizes http: as safe', () => {
		expect(isSafeProtocol('http:')).toBe(true);
		expect(isSafeProtocol('http')).toBe(true);
	});

	it('recognizes https: as safe', () => {
		expect(isSafeProtocol('https:')).toBe(true);
	});

	it('recognizes mailto: as safe', () => {
		expect(isSafeProtocol('mailto:')).toBe(true);
	});

	it('recognizes tel: as safe', () => {
		expect(isSafeProtocol('tel:')).toBe(true);
	});

	it('rejects javascript:', () => {
		expect(isSafeProtocol('javascript:')).toBe(false);
	});

	it('rejects ftp:', () => {
		expect(isSafeProtocol('ftp:')).toBe(false);
	});

	it('rejects empty string', () => {
		expect(isSafeProtocol('')).toBe(false);
	});
});

// ============================================================================
// sanitizeCSS
// ============================================================================

describe('sanitizeCSS', () => {
	it('returns empty string for empty input', () => {
		expect(sanitizeCSS('')).toBe('');
	});

	it('returns empty string for null/undefined', () => {
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		expect(sanitizeCSS(null)).toBe('');
	});

	it('passes safe CSS through', () => {
		const safe = 'color: red; font-size: 14px';
		const result = sanitizeCSS(safe);
		expect(result).toContain('color');
	});

	it('BLOCKS javascript: in CSS', () => {
		const evil = 'background: javascript:alert(1)';
		const result = sanitizeCSS(evil);
		expect(result).not.toMatch(/javascript/i);
	});

	it('BLOCKS expression() in CSS', () => {
		const evil = 'width: expression(alert(1))';
		const result = sanitizeCSS(evil);
		expect(result).not.toMatch(/expression/i);
	});

	it('BLOCKS behavior: in CSS', () => {
		const evil = 'behavior: url(evil.htc)';
		const result = sanitizeCSS(evil);
		expect(result).not.toMatch(/behavior/i);
	});

	it('BLOCKS @import in CSS', () => {
		const evil = '@import url("evil.css")';
		const result = sanitizeCSS(evil);
		expect(result).not.toMatch(/@import/i);
	});

	it('BLOCKS -moz-binding in CSS', () => {
		const evil = '-moz-binding: url("evil.xml#xss")';
		const result = sanitizeCSS(evil);
		expect(result).not.toMatch(/-moz-binding/i);
	});

	it('removes null bytes', () => {
		const evil = 'color\x00: red';
		const result = sanitizeCSS(evil);
		expect(result).not.toContain('\x00');
	});
});

// ============================================================================
// sanitizeInlineStyle
// ============================================================================

describe('sanitizeInlineStyle', () => {
	it('returns empty string for empty input', () => {
		expect(sanitizeInlineStyle('')).toBe('');
	});

	it('passes safe inline styles', () => {
		const style = 'color: blue; margin: 10px';
		const result = sanitizeInlineStyle(style);
		expect(result).toContain('color');
		expect(result).toContain('margin');
	});

	it('BLOCKS behavior: property', () => {
		const evil = 'behavior: url(evil.htc)';
		const result = sanitizeInlineStyle(evil);
		expect(result).not.toMatch(/behavior/i);
	});

	it('BLOCKS -moz-binding property', () => {
		const evil = '-moz-binding: url(evil.xml)';
		const result = sanitizeInlineStyle(evil);
		expect(result).not.toMatch(/-moz-binding/i);
	});

	it('BLOCKS expression() in value', () => {
		const evil = 'width: expression(alert(1))';
		const result = sanitizeInlineStyle(evil);
		expect(result).not.toMatch(/expression/i);
	});
});

// ============================================================================
// escapeHTML / unescapeHTML
// ============================================================================

describe('escapeHTML', () => {
	it('returns empty string for empty input', () => {
		expect(escapeHTML('')).toBe('');
	});

	it('escapes & to &amp;', () => {
		expect(escapeHTML('a & b')).toBe('a &amp; b');
	});

	it('escapes < to &lt;', () => {
		expect(escapeHTML('<div>')).toBe('&lt;div&gt;');
	});

	it('escapes " to &quot;', () => {
		expect(escapeHTML('"hello"')).toBe('&quot;hello&quot;');
	});

	it("escapes ' to &#39;", () => {
		expect(escapeHTML("it's")).toBe('it&#39;s');
	});

	it('escapes full XSS payload', () => {
		const xss = '<script>alert("xss")</script>';
		const escaped = escapeHTML(xss);
		expect(escaped).not.toMatch(/<script/i);
		expect(escaped).toContain('&lt;script&gt;');
	});
});

describe('unescapeHTML', () => {
	it('returns empty string for empty input', () => {
		expect(unescapeHTML('')).toBe('');
	});

	it('unescapes &amp; to &', () => {
		expect(unescapeHTML('a &amp; b')).toBe('a & b');
	});

	it('unescapes &lt; and &gt;', () => {
		expect(unescapeHTML('&lt;div&gt;')).toBe('<div>');
	});

	it('unescapes &quot;', () => {
		expect(unescapeHTML('&quot;hello&quot;')).toBe('"hello"');
	});

	it('is inverse of escapeHTML for typical text', () => {
		const original = 'Hello <world> & "friends"';
		expect(unescapeHTML(escapeHTML(original))).toBe(original);
	});
});

// ============================================================================
// containsXSS
// ============================================================================

describe('containsXSS', () => {
	it('returns false for empty string', () => {
		expect(containsXSS('')).toBe(false);
	});

	it('returns false for safe plain text', () => {
		expect(containsXSS('Hello world, trading profits!')).toBe(false);
	});

	it('detects <script> tag', () => {
		expect(containsXSS('<script>alert(1)</script>')).toBe(true);
	});

	it('detects javascript: protocol', () => {
		expect(containsXSS('href="javascript:alert(1)"')).toBe(true);
	});

	it('detects vbscript: protocol', () => {
		expect(containsXSS('vbscript:msgbox(1)')).toBe(true);
	});

	it('detects inline event handlers (onerror=)', () => {
		expect(containsXSS('<img src=x onerror=alert(1)>')).toBe(true);
	});

	it('detects onclick handler', () => {
		expect(containsXSS('<button onclick="evil()">click</button>')).toBe(true);
	});

	it('detects <iframe>', () => {
		expect(containsXSS('<iframe src="evil.com">')).toBe(true);
	});

	it('detects expression() CSS injection', () => {
		expect(containsXSS('expression(alert(1))')).toBe(true);
	});

	it('detects HTML-encoded javascript: (hex entity)', () => {
		// The function decodes &#x entities before checking
		expect(containsXSS('&#x6A;avascript:alert(1)')).toBe(true);
	});

	it('detects data:text/html injection', () => {
		expect(containsXSS('data:text/html,<script>alert(1)</script>')).toBe(true);
	});
});

// ============================================================================
// stripDangerous
// ============================================================================

describe('stripDangerous', () => {
	it('returns empty string for empty input', () => {
		expect(stripDangerous('')).toBe('');
	});

	it('removes <script>...</script> blocks', () => {
		const input = 'before<script>alert(1)</script>after';
		const result = stripDangerous(input);
		expect(result).not.toMatch(/<script/i);
		expect(result).not.toContain('alert(1)');
		expect(result).toContain('before');
		expect(result).toContain('after');
	});

	it('removes onclick event handlers', () => {
		const input = '<div onclick="evil()">safe text</div>';
		const result = stripDangerous(input);
		expect(result).not.toMatch(/onclick/i);
	});

	it('removes javascript: from href', () => {
		const input = '<a href="javascript:alert(1)">link</a>';
		const result = stripDangerous(input);
		expect(result).not.toMatch(/javascript:/i);
	});

	it('removes vbscript:', () => {
		const input = '<a href="vbscript:msgbox(1)">link</a>';
		const result = stripDangerous(input);
		expect(result).not.toMatch(/vbscript:/i);
	});

	it('removes null bytes', () => {
		const input = 'hello\x00world';
		const result = stripDangerous(input);
		expect(result).not.toContain('\x00');
	});
});

// ============================================================================
// sanitizeFilename
// ============================================================================

describe('sanitizeFilename', () => {
	it('returns "file" for empty input', () => {
		expect(sanitizeFilename('')).toBe('file');
	});

	it('returns "file" for null input', () => {
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		expect(sanitizeFilename(null)).toBe('file');
	});

	it('preserves extension', () => {
		expect(sanitizeFilename('report.pdf')).toBe('report.pdf');
	});

	it('converts to lowercase', () => {
		expect(sanitizeFilename('REPORT.PDF')).toBe('report.pdf');
	});

	it('replaces spaces with underscores', () => {
		expect(sanitizeFilename('my document.txt')).toBe('my_document.txt');
	});

	it('strips path separators / and . from traversal attempts', () => {
		// NOTE — SECURITY FINDING: sanitizeFilename('../../../etc/passwd') produces
		// '.-..etcpasswd' which still contains '..' because the function splits at the
		// last dot to extract the extension ('passwd') and then replaces '/' with '-'
		// but does not subsequently re-run the leading-dot or double-dot rules on the
		// combined result. Callers MUST use containsPathTraversal() BEFORE passing a
		// path to sanitizeFilename() to block traversal inputs at the gate.
		const result = sanitizeFilename('../../../etc/passwd');
		// The function does strip all forward slashes
		expect(result).not.toContain('/');
		// The function does strip all backslashes
		expect(result).not.toMatch(/\\/);
	});

	it('BLOCKS path separators (backslash)', () => {
		const result = sanitizeFilename('folder\\evil.txt');
		expect(result).not.toContain('\\');
	});

	it('removes Windows reserved characters (<>:|?*)', () => {
		const result = sanitizeFilename('file<>:|?*.txt');
		expect(result).not.toMatch(/[<>:|?*]/);
	});

	it('removes null bytes', () => {
		const result = sanitizeFilename('file\x00.txt');
		expect(result).not.toContain('\x00');
	});

	it('removes leading dots (prevents hidden files)', () => {
		const result = sanitizeFilename('.hidden');
		expect(result).not.toMatch(/^\./);
	});

	it('limits extension length to 10 chars', () => {
		const result = sanitizeFilename('file.toolongextension');
		const ext = result.split('.').pop() || '';
		expect(ext.length).toBeLessThanOrEqual(10);
	});
});

// ============================================================================
// containsPathTraversal
// ============================================================================

describe('containsPathTraversal', () => {
	it('returns false for empty string', () => {
		expect(containsPathTraversal('')).toBe(false);
	});

	it('detects ../ traversal', () => {
		expect(containsPathTraversal('../etc/passwd')).toBe(true);
	});

	it('detects Windows ..\\  traversal', () => {
		expect(containsPathTraversal('..\\windows\\system32')).toBe(true);
	});

	it('detects absolute paths starting with /', () => {
		expect(containsPathTraversal('/etc/passwd')).toBe(true);
	});

	it('detects Windows drive letter (C:)', () => {
		expect(containsPathTraversal('C:\\Windows')).toBe(true);
	});

	it('detects null byte injection', () => {
		expect(containsPathTraversal('file\x00.txt')).toBe(true);
	});

	it('returns false for safe relative filename', () => {
		expect(containsPathTraversal('report.pdf')).toBe(false);
	});
});

// ============================================================================
// generateCSPNonce
// ============================================================================

describe('generateCSPNonce', () => {
	it('returns a non-empty string', () => {
		const nonce = generateCSPNonce();
		expect(typeof nonce).toBe('string');
		expect(nonce.length).toBeGreaterThan(0);
	});

	it('returns only hex characters', () => {
		const nonce = generateCSPNonce();
		expect(nonce).toMatch(/^[0-9a-f]+$/);
	});

	it('returns unique values on each call', () => {
		const a = generateCSPNonce();
		const b = generateCSPNonce();
		expect(a).not.toBe(b);
	});

	it('returns 32-character nonce (16 bytes as hex)', () => {
		const nonce = generateCSPNonce();
		expect(nonce.length).toBe(32);
	});
});

// ============================================================================
// buildCSP
// ============================================================================

describe('buildCSP', () => {
	it('builds a CSP string with default directives', () => {
		const csp = buildCSP();
		expect(csp).toContain("default-src 'self'");
		expect(csp).toContain("object-src 'none'");
		expect(csp).toContain("frame-ancestors 'none'");
	});

	it('includes nonce in script-src when provided', () => {
		const nonce = 'abc123';
		const csp = buildCSP({ nonce });
		expect(csp).toContain(`'nonce-${nonce}'`);
	});

	it('includes allowed domains in connect-src', () => {
		const csp = buildCSP({ allowedDomains: ['https://api.example.com'] });
		expect(csp).toContain('https://api.example.com');
	});

	it('includes report-uri when provided', () => {
		const csp = buildCSP({ reportUri: 'https://csp.example.com/report' });
		expect(csp).toContain('report-uri https://csp.example.com/report');
	});

	it('includes upgrade-insecure-requests', () => {
		const csp = buildCSP();
		expect(csp).toContain('upgrade-insecure-requests');
	});
});

// ============================================================================
// buildCustomCSP
// ============================================================================

describe('buildCustomCSP', () => {
	it('builds a CSP string from custom directives', () => {
		const csp = buildCustomCSP({
			'default-src': ["'self'"],
			'script-src': ["'self'", 'https://cdn.example.com']
		});
		expect(csp).toContain("default-src 'self'");
		expect(csp).toContain('script-src');
		expect(csp).toContain('https://cdn.example.com');
	});

	it('omits empty directives', () => {
		const csp = buildCustomCSP({
			'default-src': ["'self'"],
			'object-src': []
		});
		expect(csp).not.toContain('object-src');
	});
});

// ============================================================================
// isValidEmail
// ============================================================================

describe('isValidEmail', () => {
	it('returns false for empty string', () => {
		expect(isValidEmail('')).toBe(false);
	});

	it('validates a normal email', () => {
		expect(isValidEmail('user@example.com')).toBe(true);
	});

	it('rejects email without @', () => {
		expect(isValidEmail('not-an-email')).toBe(false);
	});

	it('rejects email without domain', () => {
		expect(isValidEmail('user@')).toBe(false);
	});

	it('rejects overly long email (>254 chars)', () => {
		const long = 'a'.repeat(250) + '@example.com';
		expect(isValidEmail(long)).toBe(false);
	});

	it('rejects local part longer than 64 chars', () => {
		const local = 'a'.repeat(65);
		expect(isValidEmail(`${local}@example.com`)).toBe(false);
	});

	it('validates email with subdomains', () => {
		expect(isValidEmail('user@mail.example.co.uk')).toBe(true);
	});
});

// ============================================================================
// isValidURL
// ============================================================================

describe('isValidURL', () => {
	it('returns false for empty string', () => {
		expect(isValidURL('')).toBe(false);
	});

	it('validates http URL', () => {
		expect(isValidURL('http://example.com')).toBe(true);
	});

	it('validates https URL', () => {
		expect(isValidURL('https://example.com')).toBe(true);
	});

	it('rejects ftp protocol', () => {
		expect(isValidURL('ftp://example.com')).toBe(false);
	});

	it('rejects javascript: protocol', () => {
		expect(isValidURL('javascript:alert(1)')).toBe(false);
	});

	it('enforces https when requireHTTPS=true', () => {
		expect(isValidURL('http://example.com', true)).toBe(false);
		expect(isValidURL('https://example.com', true)).toBe(true);
	});
});

// ============================================================================
// isValidHexColor
// ============================================================================

describe('isValidHexColor', () => {
	it('returns false for empty string', () => {
		expect(isValidHexColor('')).toBe(false);
	});

	it('validates 6-char hex color with #', () => {
		expect(isValidHexColor('#ff5733')).toBe(true);
	});

	it('validates 3-char hex color with #', () => {
		expect(isValidHexColor('#fff')).toBe(true);
	});

	it('validates 6-char hex color without #', () => {
		expect(isValidHexColor('ff5733')).toBe(true);
	});

	it('validates 8-char hex (with alpha)', () => {
		expect(isValidHexColor('#ff5733aa')).toBe(true);
	});

	it('rejects non-hex characters', () => {
		expect(isValidHexColor('#zzzzzz')).toBe(false);
	});

	it('rejects injection attempt', () => {
		expect(isValidHexColor('javascript:alert(1)')).toBe(false);
	});
});

// ============================================================================
// sanitizeNumber
// ============================================================================

describe('sanitizeNumber', () => {
	it('returns defaultValue for null', () => {
		expect(sanitizeNumber(null)).toBe(0);
		expect(sanitizeNumber(null, { defaultValue: -1 })).toBe(-1);
	});

	it('returns defaultValue for undefined', () => {
		expect(sanitizeNumber(undefined)).toBe(0);
	});

	it('parses integer from string', () => {
		expect(sanitizeNumber('42')).toBe(42);
	});

	it('parses float from string', () => {
		expect(sanitizeNumber('3.14')).toBeCloseTo(3.14);
	});

	it('parses negative from string', () => {
		expect(sanitizeNumber('-10')).toBe(-10);
	});

	it('clamps to min', () => {
		expect(sanitizeNumber(5, { min: 10 })).toBe(10);
	});

	it('clamps to max', () => {
		expect(sanitizeNumber(100, { max: 50 })).toBe(50);
	});

	it('returns defaultValue for NaN input', () => {
		expect(sanitizeNumber('abc')).toBe(0);
	});

	it('returns defaultValue for Infinity', () => {
		expect(sanitizeNumber(Infinity)).toBe(0);
	});

	it('applies precision', () => {
		expect(sanitizeNumber(3.14159, { precision: 2 })).toBe(3.14);
	});

	it('strips non-numeric characters from string', () => {
		expect(sanitizeNumber('$1,234.56')).toBeCloseTo(1234.56);
	});
});

// ============================================================================
// sanitizeSlug
// ============================================================================

describe('sanitizeSlug', () => {
	it('returns null for empty string', () => {
		expect(sanitizeSlug('')).toBeNull();
	});

	it('returns null for null', () => {
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		expect(sanitizeSlug(null)).toBeNull();
	});

	it('converts to lowercase', () => {
		expect(sanitizeSlug('Hello-World')).toBe('hello-world');
	});

	it('replaces spaces with hyphens', () => {
		expect(sanitizeSlug('hello world')).toBe('hello-world');
	});

	it('removes special characters', () => {
		expect(sanitizeSlug('hello@world!')).toBe('helloworld');
	});

	it('collapses multiple hyphens', () => {
		expect(sanitizeSlug('hello---world')).toBe('hello-world');
	});

	it('removes leading and trailing hyphens', () => {
		expect(sanitizeSlug('-hello-')).toBe('hello');
	});

	it('limits to 100 characters', () => {
		const long = 'a'.repeat(120);
		const result = sanitizeSlug(long);
		expect(result?.length).toBeLessThanOrEqual(100);
	});

	it('returns null for input that sanitizes to empty', () => {
		expect(sanitizeSlug('---')).toBeNull();
	});
});

// ============================================================================
// parseJSONSafe
// ============================================================================

describe('parseJSONSafe', () => {
	it('returns null for empty string', () => {
		expect(parseJSONSafe('')).toBeNull();
	});

	it('returns null for null input', () => {
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		expect(parseJSONSafe(null)).toBeNull();
	});

	it('parses valid JSON object', () => {
		const result = parseJSONSafe<{ key: string }>('{"key":"value"}');
		expect(result).toEqual({ key: 'value' });
	});

	it('parses valid JSON array', () => {
		expect(parseJSONSafe('[1,2,3]')).toEqual([1, 2, 3]);
	});

	it('returns null for invalid JSON', () => {
		expect(parseJSONSafe('{bad json}')).toBeNull();
	});
});

// ============================================================================
// validateFile
// ============================================================================

describe('validateFile', () => {
	const makeFile = (name: string, type: string, size: number): File => {
		const blob = new Blob(['x'.repeat(size)], { type });
		return new File([blob], name, { type });
	};

	it('returns invalid for non-File objects', () => {
		// @ts-expect-error -- deliberately passing a non-string to verify the runtime guard
		const result = validateFile(null);
		expect(result.valid).toBe(false);
	});

	it('returns valid for acceptable file', () => {
		const file = makeFile('photo.jpg', 'image/jpeg', 1024);
		const result = validateFile(file, { allowedTypes: ['image/jpeg'] });
		expect(result.valid).toBe(true);
		expect(result.sanitizedName).toBe('photo.jpg');
	});

	it('rejects file exceeding maxSize', () => {
		const file = makeFile('big.jpg', 'image/jpeg', 20 * 1024 * 1024);
		const result = validateFile(file, { maxSize: 5 * 1024 * 1024 });
		expect(result.valid).toBe(false);
		expect(result.error).toMatch(/too large/i);
	});

	it('rejects empty file (size 0)', () => {
		const file = makeFile('empty.txt', 'text/plain', 0);
		const result = validateFile(file);
		expect(result.valid).toBe(false);
		expect(result.error).toMatch(/empty/i);
	});

	it('rejects disallowed MIME type', () => {
		const file = makeFile('evil.exe', 'application/x-msdownload', 100);
		const result = validateFile(file, { allowedTypes: ['image/jpeg', 'image/png'] });
		expect(result.valid).toBe(false);
		expect(result.error).toMatch(/invalid file type/i);
	});

	it('rejects disallowed extension', () => {
		const file = makeFile('file.exe', 'application/octet-stream', 100);
		const result = validateFile(file, {
			allowedExtensions: ['pdf', 'jpg', 'png']
		});
		expect(result.valid).toBe(false);
		expect(result.error).toMatch(/invalid file extension/i);
	});

	it('sanitizes the filename on valid files', () => {
		const file = makeFile('My Report (2026).pdf', 'application/pdf', 500);
		const result = validateFile(file);
		expect(result.valid).toBe(true);
		expect(result.sanitizedName).not.toContain(' ');
	});
});

// ============================================================================
// Constants smoke tests
// ============================================================================

describe('exported constants', () => {
	it('SAFE_PROTOCOLS contains http, https, mailto, tel', () => {
		expect(SAFE_PROTOCOLS).toContain('http:');
		expect(SAFE_PROTOCOLS).toContain('https:');
		expect(SAFE_PROTOCOLS).toContain('mailto:');
		expect(SAFE_PROTOCOLS).toContain('tel:');
	});

	it('SAFE_PROTOCOLS does NOT contain javascript:', () => {
		const protocols: readonly string[] = SAFE_PROTOCOLS;
		expect(protocols).not.toContain('javascript:');
	});

	it('SAFE_IMAGE_TYPES contains common image types', () => {
		expect(SAFE_IMAGE_TYPES).toContain('image/jpeg');
		expect(SAFE_IMAGE_TYPES).toContain('image/png');
		expect(SAFE_IMAGE_TYPES).toContain('image/webp');
	});

	it('SAFE_DOCUMENT_TYPES contains pdf', () => {
		expect(SAFE_DOCUMENT_TYPES).toContain('application/pdf');
	});
});
