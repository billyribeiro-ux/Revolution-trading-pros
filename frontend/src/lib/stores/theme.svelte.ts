/**
 * Theme Store - Apple ICT9+ Design System (Svelte 5)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Centralized theme management with:
 * - Dark/Light mode toggle
 * - System preference detection
 * - Persistent storage
 * - Smooth transitions
 *
 * @version 2.0.0 - Svelte 5 Runes (January 2026)
 */

/// <reference lib="dom" />
/// <reference lib="dom.iterable" />

import { browser } from '$app/environment';

export type Theme = 'light' | 'dark' | 'auto';

// ═══════════════════════════════════════════════════════════════════════════════
// Svelte 5 Reactive State
// ═══════════════════════════════════════════════════════════════════════════════

const defaultTheme: Theme = 'light';

// Get initial theme from localStorage or default
const getInitialTheme = (): Theme => {
	if (!browser) return defaultTheme;
	return (localStorage.getItem('theme') as Theme) || defaultTheme;
};

let currentTheme = $state<Theme>(getInitialTheme());

// Derived state for effective theme (resolves 'auto' to actual value)
const effectiveThemeValue = $derived.by(() => {
	if (currentTheme === 'auto' && browser && typeof window !== 'undefined') {
		return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
	}
	return currentTheme === 'auto' ? 'dark' : currentTheme;
});

// Derived state for dark mode check
const isDarkValue = $derived(effectiveThemeValue === 'dark');

// ═══════════════════════════════════════════════════════════════════════════════
// Theme Application
// ═══════════════════════════════════════════════════════════════════════════════

function applyTheme(theme: Theme) {
	if (!browser || typeof document === 'undefined') return;

	const root = document.documentElement;
	const body = document.body;

	// Remove existing classes
	root.classList.remove('dark', 'light');
	body.classList.remove('dark', 'light');

	let effectiveTheme: 'dark' | 'light';

	if (theme === 'auto') {
		effectiveTheme =
			typeof window !== 'undefined' && window.matchMedia('(prefers-color-scheme: dark)').matches
				? 'dark'
				: 'light';
	} else {
		effectiveTheme = theme;
	}

	// Apply theme classes
	root.classList.add(effectiveTheme);
	body.classList.add(effectiveTheme);

	// Set data attribute for CSS selectors
	root.dataset['theme'] = effectiveTheme;

	// Update meta theme-color for mobile browsers - Apple-style colors
	const metaThemeColor =
		typeof document !== 'undefined' ? document.querySelector('meta[name="theme-color"]') : null;
	if (metaThemeColor) {
		metaThemeColor.setAttribute('content', effectiveTheme === 'dark' ? '#0f172a' : '#f5f5f7');
	}
}

// Initialize theme on load
if (browser) {
	applyTheme(currentTheme);

	// Listen for system theme changes
	if (typeof window !== 'undefined') {
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		mediaQuery.addEventListener('change', () => {
			if (currentTheme === 'auto') {
				applyTheme('auto');
			}
		});
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Store Actions
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Set theme and persist to localStorage
 */
function setTheme(theme: Theme) {
	if (browser) {
		localStorage.setItem('theme', theme);
		applyTheme(theme);
	}
	currentTheme = theme;
}

/**
 * Toggle between dark and light
 */
function toggle() {
	const newTheme: Theme = currentTheme === 'light' ? 'dark' : 'light';
	setTheme(newTheme);
}

/**
 * Cycle through all themes: dark -> light -> auto
 */
function cycle() {
	const order: Theme[] = ['dark', 'light', 'auto'];
	const currentIndex = order.indexOf(currentTheme);
	const next = order[(currentIndex + 1) % order.length] ?? 'dark';
	setTheme(next);
}

/**
 * Initialize theme (call on app mount)
 */
function init() {
	if (browser) {
		const theme = (localStorage.getItem('theme') as Theme) || defaultTheme;
		applyTheme(theme);
		currentTheme = theme;
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exported Store (Svelte 5 Pattern)
// ═══════════════════════════════════════════════════════════════════════════════

export const themeStore = {
	get current() {
		return currentTheme;
	},
	get effectiveTheme() {
		return effectiveThemeValue;
	},
	get isDark() {
		return isDarkValue;
	},
	setTheme,
	toggle,
	cycle,
	init
};

// Legacy exports for backwards compatibility
export const effectiveTheme = {
	get value() {
		return effectiveThemeValue;
	}
};

export const isDark = {
	get value() {
		return isDarkValue;
	}
};
