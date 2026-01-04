/**
 * Theme Store - Apple ICT9+ Design System
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Centralized theme management with:
 * - Dark/Light mode toggle
 * - System preference detection
 * - Persistent storage
 * - Smooth transitions
 */

import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export type Theme = 'light' | 'dark' | 'auto';

function createThemeStore() {
	// ICT 11+ Apple Principal Engineer: Light theme default for better UX
	const defaultTheme: Theme = 'light';

	// Get initial theme from localStorage or default
	const initialTheme = browser
		? (localStorage.getItem('theme') as Theme) || defaultTheme
		: defaultTheme;

	const { subscribe, set, update } = writable<Theme>(initialTheme);

	// Apply theme to document
	function applyTheme(theme: Theme) {
		if (!browser) return;

		const root = document.documentElement;
		const body = document.body;

		// Remove existing classes
		root.classList.remove('dark', 'light');
		body.classList.remove('dark', 'light');

		let effectiveTheme: 'dark' | 'light';

		if (theme === 'auto') {
			effectiveTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
		} else {
			effectiveTheme = theme;
		}

		// Apply theme classes
		root.classList.add(effectiveTheme);
		body.classList.add(effectiveTheme);

		// Set data attribute for CSS selectors
		root.dataset['theme'] = effectiveTheme;

		// Update meta theme-color for mobile browsers - Apple-style colors
		const metaThemeColor = document.querySelector('meta[name="theme-color"]');
		if (metaThemeColor) {
			metaThemeColor.setAttribute('content', effectiveTheme === 'dark' ? '#0f172a' : '#f5f5f7');
		}
	}

	// Initialize theme
	if (browser) {
		applyTheme(initialTheme);

		// Listen for system theme changes
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		mediaQuery.addEventListener('change', () => {
			const currentTheme = localStorage.getItem('theme') as Theme;
			if (currentTheme === 'auto') {
				applyTheme('auto');
			}
		});
	}

	return {
		subscribe,

		/**
		 * Set theme and persist to localStorage
		 */
		setTheme: (theme: Theme) => {
			if (browser) {
				localStorage.setItem('theme', theme);
				applyTheme(theme);
			}
			set(theme);
		},

		/**
		 * Toggle between dark and light
		 */
		toggle: () => {
			update((current) => {
				const newTheme: Theme = current === 'light' ? 'dark' : 'light';
				if (browser) {
					localStorage.setItem('theme', newTheme);
					applyTheme(newTheme);
				}
				return newTheme;
			});
		},

		/**
		 * Cycle through all themes: dark -> light -> auto
		 */
		cycle: () => {
			update((current) => {
				const order: Theme[] = ['dark', 'light', 'auto'];
				const currentIndex = order.indexOf(current);
				const next = order[(currentIndex + 1) % order.length];
				if (browser && next) {
					localStorage.setItem('theme', next);
					applyTheme(next);
				}
				return next ?? 'dark';
			});
		},

		/**
		 * Initialize theme (call on app mount)
		 */
		init: () => {
			if (browser) {
				const theme = (localStorage.getItem('theme') as Theme) || defaultTheme;
				applyTheme(theme);
			}
		}
	};
}

export const themeStore = createThemeStore();

/**
 * Derived store for effective theme (resolves 'auto' to actual value)
 */
export const effectiveTheme = derived(themeStore, ($theme) => {
	if ($theme === 'auto' && browser) {
		return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
	}
	return $theme === 'auto' ? 'dark' : $theme;
});

/**
 * Check if dark mode is active
 */
export const isDark = derived(effectiveTheme, ($effectiveTheme) => $effectiveTheme === 'dark');
