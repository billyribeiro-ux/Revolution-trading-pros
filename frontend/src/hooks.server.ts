/**
 * SvelteKit Server Hooks - Security Headers & Performance
 * Following Google November 2025 SEO and security best practices
 *
 * Features:
 * - Comprehensive security headers (OWASP recommended)
 * - Performance headers for caching and compression
 * - SEO-friendly headers
 * - AI crawler control headers (November 2025)
 * - CORS configuration
 *
 * @version 2.0.0 - November 2025 Standards with AI Crawler Headers
 */

import type { Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

/**
 * Security Headers Handler
 * Implements OWASP security headers for improved trust signals and security
 */
const securityHeaders: Handle = async ({ event, resolve }) => {
	const response = await resolve(event);

	// Clone headers to modify them
	const headers = new Headers(response.headers);

	// ═══════════════════════════════════════════════════════════════════════════
	// Security Headers (OWASP Best Practices)
	// ═══════════════════════════════════════════════════════════════════════════

	// Prevent XSS attacks - strongest protection
	headers.set('X-XSS-Protection', '1; mode=block');

	// Prevent MIME type sniffing
	headers.set('X-Content-Type-Options', 'nosniff');

	// Prevent clickjacking
	headers.set('X-Frame-Options', 'SAMEORIGIN');

	// Referrer policy - balance between privacy and analytics
	headers.set('Referrer-Policy', 'strict-origin-when-cross-origin');

	// Permissions Policy - disable unnecessary browser features
	headers.set(
		'Permissions-Policy',
		'accelerometer=(), autoplay=(), camera=(), cross-origin-isolated=(), display-capture=(), encrypted-media=(), fullscreen=(self), geolocation=(), gyroscope=(), keyboard-map=(), magnetometer=(), microphone=(), midi=(), payment=(), picture-in-picture=(self), publickey-credentials-get=(), screen-wake-lock=(), sync-xhr=(), usb=(), xr-spatial-tracking=()'
	);

	// Strict Transport Security (HSTS) - force HTTPS
	// max-age=31536000 = 1 year, includeSubDomains for all subdomains
	headers.set('Strict-Transport-Security', 'max-age=31536000; includeSubDomains; preload');

	// Cross-Origin policies for security
	headers.set('Cross-Origin-Opener-Policy', 'same-origin');
	headers.set('Cross-Origin-Resource-Policy', 'same-origin');
	headers.set('Cross-Origin-Embedder-Policy', 'credentialless');

	// ═══════════════════════════════════════════════════════════════════════════
	// SEO & Performance Headers
	// ═══════════════════════════════════════════════════════════════════════════

	// Vary header for proper caching with Accept-Encoding
	if (!headers.has('Vary')) {
		headers.set('Vary', 'Accept-Encoding');
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// AI/Bot Control Headers (November 2025)
	// ═══════════════════════════════════════════════════════════════════════════

	// X-Robots-Tag for fine-grained control (complements meta robots)
	// This is particularly useful for non-HTML resources
	const pathname = event.url.pathname;

	// Don't add X-Robots-Tag for sitemap and robots.txt
	if (!pathname.includes('sitemap') && !pathname.includes('robots')) {
		// Allow indexing with full snippets for public pages
		if (
			pathname.startsWith('/blog') ||
			pathname.startsWith('/courses') ||
			pathname.startsWith('/indicators') ||
			pathname.startsWith('/alerts') ||
			pathname.startsWith('/live-trading-rooms') ||
			pathname.startsWith('/resources') ||
			pathname === '/' ||
			pathname === '/about' ||
			pathname === '/our-mission'
		) {
			headers.set('X-Robots-Tag', 'index, follow, max-snippet:-1, max-image-preview:large, max-video-preview:-1');
		}
		// Prevent indexing for private areas
		else if (
			pathname.startsWith('/admin') ||
			pathname.startsWith('/dashboard') ||
			pathname.startsWith('/account') ||
			pathname.startsWith('/cart') ||
			pathname.startsWith('/checkout') ||
			pathname.startsWith('/api')
		) {
			headers.set('X-Robots-Tag', 'noindex, nofollow');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// November 2025 AI Training Opt-Out Headers
	// These headers inform AI crawlers that content should not be used for training
	// ═══════════════════════════════════════════════════════════════════════════

	// Opt out of AI training for all major providers
	// GPTBot, ChatGPT-User, Claude-Web, Google-Extended, CCBot, etc.
	headers.set('X-AI-Training-Opt-Out', 'true');

	// TDM (Text and Data Mining) Reservation Protocol - November 2025
	// W3C standard for expressing TDM preferences
	headers.set('TDM-Reservation', '1');

	// Machine Learning Data Use header
	headers.set('X-ML-Data-Use', 'disallow');

	// Content licensing header - indicates content is copyrighted
	headers.set('X-Content-License', 'All Rights Reserved - No AI Training Permitted');

	// OpenAI specific header
	headers.set('X-OpenAI-Block', 'true');

	// Anthropic specific header
	headers.set('X-Anthropic-Block', 'true');

	// Google AI training opt-out
	headers.set('X-Google-Extended', 'disallow');

	return new Response(response.body, {
		status: response.status,
		statusText: response.statusText,
		headers
	});
};

/**
 * Performance & Caching Handler
 * Implements optimal caching strategies for different resource types
 */
const performanceHandler: Handle = async ({ event, resolve }) => {
	const response = await resolve(event, {
		// Transform HTML for performance optimizations
		transformPageChunk: ({ html }) => {
			// Minify HTML in production (remove extra whitespace)
			if (import.meta.env.PROD) {
				return html.replace(/\s+/g, ' ').replace(/>\s+</g, '><');
			}
			return html;
		}
	});

	return response;
};

/**
 * Request timing handler for monitoring
 */
const timingHandler: Handle = async ({ event, resolve }) => {
	const start = performance.now();

	const response = await resolve(event);

	const duration = performance.now() - start;

	// Add Server-Timing header for performance monitoring
	const headers = new Headers(response.headers);
	headers.set('Server-Timing', `total;dur=${duration.toFixed(2)}`);

	return new Response(response.body, {
		status: response.status,
		statusText: response.statusText,
		headers
	});
};

// Combine all handlers in sequence
export const handle: Handle = sequence(timingHandler, securityHeaders, performanceHandler);
