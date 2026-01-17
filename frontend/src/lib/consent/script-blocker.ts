/**
 * Script Blocker Module
 *
 * Blocks third-party scripts until consent is given.
 * Works by:
 * 1. Converting script tags to data-script-blocked
 * 2. Intercepting dynamic script creation
 * 3. Releasing scripts when consent is granted
 *
 * @module consent/script-blocker
 * @version 1.0.0
 */

import { browser } from '$app/environment';
import type { ConsentCategory, ConsentState } from './types';

/**
 * Blocked script patterns by category.
 * Synced with consent-magic-pro v5.1.0 vendor list.
 */
const BLOCKED_SCRIPTS: Record<ConsentCategory, RegExp[]> = {
	necessary: [], // Never blocked

	analytics: [
		// Google Analytics
		/google-analytics\.com/i,
		/googletagmanager\.com/i,
		/analytics\.google\.com/i,
		/gtag\/js/i,
		// Other analytics
		/plausible\.io/i,
		/mixpanel\.com/i,
		/segment\.com/i,
		/amplitude\.com/i,
		/hotjar\.com/i,
		/fullstory\.com/i,
		/clarity\.ms/i,
		/heap-analytics/i,
		/matomo\./i,
		/piwik\./i,
		// Google Maps
		/maps\.googleapis\.com/i,
		/maps\.google\.com/i,
		// Google AdSense
		/googlesyndication\.com/i,
		/pagead2\.googlesyndication/i,
		// AddThis/ShareThis
		/addthis\.com/i,
		/sharethis\.com/i,
		// Instagram embeds
		/instagram\.com\/embed/i,
		/cdninstagram\.com/i
	],

	marketing: [
		// Meta/Facebook
		/facebook\.net/i,
		/facebook\.com\/tr/i,
		/fbevents\.js/i,
		/connect\.facebook/i,
		// Google Ads
		/doubleclick\.net/i,
		/googleadservices\.com/i,
		/google\.com\/pagead/i,
		/googleads\.g\.doubleclick/i,
		// LinkedIn
		/linkedin\.com\/insight/i,
		/snap\.licdn\.com/i,
		// Twitter/X
		/ads-twitter\.com/i,
		/static\.ads-twitter\.com/i,
		/analytics\.twitter\.com/i,
		// TikTok
		/tiktok\.com\/i18n/i,
		/analytics\.tiktok\.com/i,
		// Pinterest
		/pinterest\.com\/js/i,
		/pinimg\.com\/ct/i,
		// Bing/Microsoft UET
		/bing\.com\/bat/i,
		/bat\.bing\.com/i,
		// Reddit
		/redditstatic\.com\/ads/i,
		/events\.reddit\.com/i,
		// Criteo
		/criteo\.net/i,
		/criteo\.com/i,
		// Outbrain/Taboola
		/outbrain\.com/i,
		/taboola\.com/i,
		// HubSpot
		/hubspot\.com/i,
		/hs-scripts\.com/i,
		/hs-analytics\.net/i,
		// Soundcloud
		/soundcloud\.com\/player/i,
		// Slideshare
		/slideshare\.net/i
	],

	preferences: [
		// Chat widgets
		/intercom\.com/i,
		/crisp\.chat/i,
		/zendesk\.com/i,
		/drift\.com/i,
		/livechatinc\.com/i,
		/tawk\.to/i,
		// YouTube embeds
		/youtube\.com\/embed/i,
		/youtube-nocookie\.com\/embed/i,
		/ytimg\.com/i,
		// Vimeo embeds
		/player\.vimeo\.com/i,
		/vimeo\.com\/video/i,
		// Spotify embeds
		/open\.spotify\.com\/embed/i,
		// Google Fonts (optional - can be enabled/disabled)
		/fonts\.googleapis\.com/i,
		/fonts\.gstatic\.com/i
	]
};

/**
 * Extended blocking patterns for embedded content.
 * These map to the main categories but are tracked separately.
 */
export const EMBEDDED_CONTENT_PATTERNS: Record<
	string,
	{ pattern: RegExp; category: ConsentCategory; name: string }[]
> = {
	youtube: [
		{ pattern: /youtube\.com\/embed/i, category: 'preferences', name: 'YouTube' },
		{
			pattern: /youtube-nocookie\.com\/embed/i,
			category: 'preferences',
			name: 'YouTube (Privacy-Enhanced)'
		}
	],
	vimeo: [{ pattern: /player\.vimeo\.com/i, category: 'preferences', name: 'Vimeo' }],
	google_maps: [
		{ pattern: /google\.com\/maps\/embed/i, category: 'analytics', name: 'Google Maps' },
		{ pattern: /maps\.googleapis\.com/i, category: 'analytics', name: 'Google Maps API' }
	],
	soundcloud: [{ pattern: /soundcloud\.com\/player/i, category: 'marketing', name: 'SoundCloud' }],
	spotify: [{ pattern: /open\.spotify\.com\/embed/i, category: 'preferences', name: 'Spotify' }],
	instagram: [{ pattern: /instagram\.com\/embed/i, category: 'analytics', name: 'Instagram' }]
};

/**
 * Google Fonts blocking configuration.
 * Can be used to completely block Google Fonts requests.
 */
export const GOOGLE_FONTS_CONFIG = {
	patterns: [/fonts\.googleapis\.com/i, /fonts\.gstatic\.com/i],
	category: 'preferences' as ConsentCategory,
	description:
		'Google Fonts are loaded from Google servers. Blocking prevents external font requests.'
};

/**
 * Check if a URL is an embedded content type
 */
export function getEmbeddedContentType(
	url: string
): { type: string; name: string; category: ConsentCategory } | null {
	for (const [type, patterns] of Object.entries(EMBEDDED_CONTENT_PATTERNS)) {
		for (const { pattern, name, category } of patterns) {
			if (pattern.test(url)) {
				return { type, name, category };
			}
		}
	}
	return null;
}

/**
 * Check if a URL should be blocked as embedded content
 */
export function shouldBlockEmbed(url: string, consent: ConsentState): boolean {
	const embedInfo = getEmbeddedContentType(url);
	if (!embedInfo) return false;
	return !consent[embedInfo.category];
}

/**
 * Queue of blocked scripts to be executed when consent is granted
 */
interface BlockedScript {
	src?: string;
	innerHTML?: string;
	category: ConsentCategory;
	attributes: Record<string, string>;
}

let blockedScripts: BlockedScript[] = [];
let isBlocking = false;
let originalCreateElement: typeof document.createElement | null = null;

/**
 * Determine which category a script URL belongs to
 */
export function getScriptCategory(src: string): ConsentCategory | null {
	for (const [category, patterns] of Object.entries(BLOCKED_SCRIPTS)) {
		for (const pattern of patterns) {
			if (pattern.test(src)) {
				return category as ConsentCategory;
			}
		}
	}
	return null;
}

/**
 * Check if a script should be blocked based on consent state
 */
export function shouldBlockScript(src: string, consent: ConsentState): boolean {
	const category = getScriptCategory(src);
	if (!category) return false;
	return !consent[category];
}

/**
 * Start blocking scripts (call on page load before consent)
 */
export function startScriptBlocking(): void {
	if (!browser || isBlocking) return;

	isBlocking = true;
	blockedScripts = [];

	// Override document.createElement to intercept script creation
	originalCreateElement = document.createElement.bind(document);

	document.createElement = function <K extends keyof HTMLElementTagNameMap>(
		tagName: K,
		options?: ElementCreationOptions
	): HTMLElementTagNameMap[K] {
		const element = originalCreateElement!(tagName, options);

		if (tagName.toLowerCase() === 'script') {
			const script = element as HTMLScriptElement;

			// Intercept src setter
			const originalSrcDescriptor = Object.getOwnPropertyDescriptor(
				HTMLScriptElement.prototype,
				'src'
			);

			if (originalSrcDescriptor) {
				Object.defineProperty(script, 'src', {
					get() {
						return originalSrcDescriptor.get?.call(this);
					},
					set(value: string) {
						const category = getScriptCategory(value);
						if (category) {
							// Block this script
							console.debug(`[ScriptBlocker] Blocked: ${value} (${category})`);

							blockedScripts.push({
								src: value,
								category,
								attributes: {}
							});

							// Set a data attribute instead
							script.setAttribute('data-blocked-src', value);
							script.setAttribute('data-blocked-category', category);
							return;
						}

						// Allow the script
						originalSrcDescriptor.set?.call(this, value);
					},
					configurable: true
				});
			}
		}

		return element;
	};

	// Also scan and block existing scripts in the DOM
	scanAndBlockExistingScripts();

	console.debug('[ScriptBlocker] Script blocking enabled');
}

/**
 * Scan DOM for scripts that should be blocked
 */
function scanAndBlockExistingScripts(): void {
	const scripts = document.querySelectorAll('script[src]');

	scripts.forEach((script) => {
		const src = script.getAttribute('src');
		if (!src) return;

		const category = getScriptCategory(src);
		if (category) {
			// Replace with blocked version
			script.setAttribute('data-blocked-src', src);
			script.setAttribute('data-blocked-category', category);
			script.removeAttribute('src');

			console.debug(`[ScriptBlocker] Blocked existing script: ${src}`);
		}
	});
}

/**
 * Stop blocking and release blocked scripts for granted categories
 */
export function releaseBlockedScripts(consent: ConsentState): void {
	if (!browser) return;

	// Restore original createElement if we're done blocking
	if (originalCreateElement && consent.analytics && consent.marketing && consent.preferences) {
		document.createElement = originalCreateElement;
		originalCreateElement = null;
		isBlocking = false;
	}

	// Execute queued scripts that now have consent
	const toExecute = blockedScripts.filter((script) => consent[script.category]);
	blockedScripts = blockedScripts.filter((script) => !consent[script.category]);

	toExecute.forEach((scriptInfo) => {
		if (scriptInfo.src) {
			const script = document.createElement('script');
			script.src = scriptInfo.src;

			// Copy attributes
			Object.entries(scriptInfo.attributes).forEach(([key, value]) => {
				script.setAttribute(key, value);
			});

			document.head.appendChild(script);
			console.debug(`[ScriptBlocker] Released: ${scriptInfo.src}`);
		}
	});

	// Also release blocked scripts in the DOM
	const blockedDomScripts = document.querySelectorAll('[data-blocked-src]');
	blockedDomScripts.forEach((script) => {
		const src = script.getAttribute('data-blocked-src');
		const category = script.getAttribute('data-blocked-category') as ConsentCategory;

		if (src && category && consent[category]) {
			script.setAttribute('src', src);
			script.removeAttribute('data-blocked-src');
			script.removeAttribute('data-blocked-category');
			console.debug(`[ScriptBlocker] Released DOM script: ${src}`);
		}
	});
}

/**
 * Get list of currently blocked scripts
 */
export function getBlockedScripts(): BlockedScript[] {
	return [...blockedScripts];
}

/**
 * Check if script blocking is active
 */
export function isScriptBlockingActive(): boolean {
	return isBlocking;
}

/**
 * Generate inline script to block scripts before main JS loads
 * This should be injected into the HTML head
 */
export function generateBlockingScript(): string {
	const patterns = {
		analytics: BLOCKED_SCRIPTS.analytics.map((r) => r.source).join('|'),
		marketing: BLOCKED_SCRIPTS.marketing.map((r) => r.source).join('|'),
		preferences: BLOCKED_SCRIPTS.preferences.map((r) => r.source).join('|')
	};

	return `
(function() {
  var consent = null;
  try {
    var stored = localStorage.getItem('rtp_consent');
    if (stored) consent = JSON.parse(stored);
  } catch(e) {}

  if (consent && consent.hasInteracted) return; // Already has consent

  var patterns = ${JSON.stringify(patterns)};
  var origCreate = document.createElement.bind(document);

  document.createElement = function(tag, opts) {
    var el = origCreate(tag, opts);
    if (tag.toLowerCase() === 'script') {
      var origSrcSet = Object.getOwnPropertyDescriptor(HTMLScriptElement.prototype, 'src').set;
      Object.defineProperty(el, 'src', {
        set: function(v) {
          for (var cat in patterns) {
            if (new RegExp(patterns[cat], 'i').test(v)) {
              console.debug('[Blocker] Blocked:', v);
              el.setAttribute('data-blocked-src', v);
              el.setAttribute('data-blocked-category', cat);
              return;
            }
          }
          origSrcSet.call(this, v);
        },
        get: function() { return el.getAttribute('src') || ''; }
      });
    }
    return el;
  };
})();
`.trim();
}

/**
 * Generate noscript fallback message
 */
export function generateNoscriptMessage(): string {
	return `
<noscript>
  <style>
    .consent-noscript-warning {
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      background: #1a1a2e;
      color: white;
      padding: 1rem;
      text-align: center;
      z-index: 9999;
    }
  </style>
  <div class="consent-noscript-warning">
    JavaScript is required for cookie consent management. Some features may not work without JavaScript.
  </div>
</noscript>
`.trim();
}
