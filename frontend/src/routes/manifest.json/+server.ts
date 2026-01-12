/**
 * Manifest.json Server Route
 * ICT 7 FIX: Explicitly serve manifest.json to prevent 404 errors
 * This ensures the PWA manifest is always available regardless of static file serving
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const manifest = {
	name: 'Revolution Trading Pros',
	short_name: 'RevTradingPros',
	description: 'Master the markets with institutional-grade trading tools, live trading rooms, and professional trading education.',
	start_url: '/',
	scope: '/',
	display: 'standalone',
	orientation: 'portrait-primary',
	background_color: '#0a0a0a',
	theme_color: '#1a1a1a',
	lang: 'en-US',
	dir: 'ltr',
	categories: ['finance', 'education', 'business'],
	icons: [
		{
			src: '/favicon.png',
			sizes: '128x128',
			type: 'image/png',
			purpose: 'any'
		},
		{
			src: '/favicon.png',
			sizes: '128x128',
			type: 'image/png',
			purpose: 'maskable'
		}
	],
	screenshots: [
		{
			src: '/revolution-trading-pros.png',
			sizes: '3163x835',
			type: 'image/png',
			form_factor: 'wide',
			label: 'Revolution Trading Pros Dashboard'
		}
	],
	shortcuts: [
		{
			name: 'Live Trading Rooms',
			short_name: 'Trading Rooms',
			description: 'Join live trading sessions with professional traders',
			url: '/live-trading-rooms',
			icons: [{ src: '/favicon.png', sizes: '128x128' }]
		},
		{
			name: 'Trading Courses',
			short_name: 'Courses',
			description: 'Access professional trading education',
			url: '/courses',
			icons: [{ src: '/favicon.png', sizes: '128x128' }]
		},
		{
			name: 'Trading Alerts',
			short_name: 'Alerts',
			description: 'Get real-time trading signals',
			url: '/alerts',
			icons: [{ src: '/favicon.png', sizes: '128x128' }]
		},
		{
			name: 'Trading Blog',
			short_name: 'Blog',
			description: 'Read latest trading insights',
			url: '/blog',
			icons: [{ src: '/favicon.png', sizes: '128x128' }]
		}
	],
	related_applications: [],
	prefer_related_applications: false,
	handle_links: 'preferred',
	launch_handler: {
		client_mode: ['navigate-existing', 'auto']
	},
	edge_side_panel: {
		preferred_width: 400
	},
	share_target: {
		action: '/share',
		method: 'POST',
		enctype: 'multipart/form-data',
		params: {
			title: 'title',
			text: 'text',
			url: 'url'
		}
	},
	protocol_handlers: [
		{
			protocol: 'web+trading',
			url: '/protocol?url=%s'
		}
	],
	serviceworker: {
		src: '/sw.js',
		scope: '/',
		type: 'classic',
		update_via_cache: 'none'
	}
};

export const GET: RequestHandler = async () => {
	return json(manifest, {
		headers: {
			'Content-Type': 'application/manifest+json',
			'Cache-Control': 'public, max-age=86400'
		}
	});
};
