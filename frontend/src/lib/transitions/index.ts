/**
 * Centralized Transitions Library
 * ═══════════════════════════════════════════════════════════════════════════
 * Svelte 5 / SvelteKit Best Practices (Jan 2026)
 * Apple Principal Engineer ICT Level 7 Standard
 *
 * DRY Fix: Consolidates duplicate transition functions from 8+ section files
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */

import { cubicOut, backOut, quintOut } from 'svelte/easing';
import type { TransitionConfig } from 'svelte/transition';

// ═══════════════════════════════════════════════════════════════════════════
// CORE TRANSITIONS
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Heavy, premium-feeling slide transition
 * Used by: TradingRoomsSection, AlertServicesSection, MentorshipSection, etc.
 */
export function heavySlide(
	_node: Element,
	{ delay = 0, duration = 1000, y = 20 }: { delay?: number; duration?: number; y?: number } = {}
): TransitionConfig {
	return {
		delay,
		duration,
		css: (t: number) => {
			const eased = cubicOut(t);
			return `opacity: ${eased}; transform: translateY(${(1 - eased) * y}px);`;
		}
	};
}

/**
 * Fade transition with optional direction
 * Apple-style smooth fade for hero content
 */
export function fade(
	_node: Element,
	{ delay = 0, duration = 400 }: { delay?: number; duration?: number } = {}
): TransitionConfig {
	return {
		delay,
		duration,
		css: (t: number) => `opacity: ${cubicOut(t)};`
	};
}

/**
 * Scale transition with bounce
 * Used for cards, buttons, interactive elements
 */
export function scaleIn(
	_node: Element,
	{ delay = 0, duration = 500 }: { delay?: number; duration?: number } = {}
): TransitionConfig {
	return {
		delay,
		duration,
		css: (t: number) => {
			const eased = backOut(t);
			return `opacity: ${cubicOut(t)}; transform: scale(${0.9 + eased * 0.1});`;
		}
	};
}

/**
 * Slide from left
 * Used for side panels, navigation elements
 */
export function slideLeft(
	_node: Element,
	{ delay = 0, duration = 600, x = 30 }: { delay?: number; duration?: number; x?: number } = {}
): TransitionConfig {
	return {
		delay,
		duration,
		css: (t: number) => {
			const eased = quintOut(t);
			return `opacity: ${eased}; transform: translateX(${(1 - eased) * x}px);`;
		}
	};
}

/**
 * Slide from right
 * Used for modals, sidebars
 */
export function slideRight(
	_node: Element,
	{ delay = 0, duration = 600, x = -30 }: { delay?: number; duration?: number; x?: number } = {}
): TransitionConfig {
	return {
		delay,
		duration,
		css: (t: number) => {
			const eased = quintOut(t);
			return `opacity: ${eased}; transform: translateX(${(1 - eased) * x}px);`;
		}
	};
}

/**
 * Staggered children animation helper
 * Returns delay based on index for staggered reveals
 */
export function staggerDelay(index: number, baseDelay = 0, stagger = 100): number {
	return baseDelay + index * stagger;
}

// ═══════════════════════════════════════════════════════════════════════════
// INTERSECTION OBSERVER UTILITIES
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Create a visibility observer for scroll-triggered animations
 * Svelte 5 pattern: Use with $state and onMount
 *
 * @example
 * let isVisible = $state(false);
 * let containerRef = $state<HTMLElement | null>(null);
 *
 * onMount(() => {
 *   const cleanup = createVisibilityObserver(containerRef, (visible) => {
 *     isVisible = visible;
 *   });
 *   return cleanup;
 * });
 */
export function createVisibilityObserver(
	element: HTMLElement | null,
	callback: (isVisible: boolean) => void,
	options: { threshold?: number; rootMargin?: string; once?: boolean } = {}
): () => void {
	const { threshold = 0.1, rootMargin = '50px', once = true } = options;

	if (!element || typeof window === 'undefined') {
		// SSR or no element - trigger immediately
		callback(true);
		return () => {};
	}

	const observer = new IntersectionObserver(
		(entries) => {
			if (entries[0]?.isIntersecting) {
				callback(true);
				if (once) {
					observer.disconnect();
				}
			} else if (!once) {
				callback(false);
			}
		},
		{ threshold, rootMargin }
	);

	// Use queueMicrotask to ensure bind:this has completed
	queueMicrotask(() => {
		if (element) {
			observer.observe(element);
		}
	});

	return () => observer.disconnect();
}

// ═══════════════════════════════════════════════════════════════════════════
// SVELTE 5 ACTION FOR SCROLL REVEAL
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Svelte action for scroll-triggered animations
 * Alternative to IntersectionObserver + $state pattern
 *
 * @example
 * <div use:scrollReveal={{ type: 'fade-up', delay: 200 }}>Content</div>
 */
export function scrollReveal(
	node: HTMLElement,
	params: {
		type?: 'fade' | 'fade-up' | 'fade-down' | 'fade-left' | 'fade-right' | 'scale';
		delay?: number;
		duration?: number;
		threshold?: number;
		once?: boolean;
	} = {}
): { destroy: () => void } {
	const {
		type = 'fade-up',
		delay = 0,
		duration = 600,
		threshold = 0.15,
		once = true
	} = params;

	// Check reduced motion preference
	const prefersReducedMotion =
		typeof window !== 'undefined' &&
		window.matchMedia('(prefers-reduced-motion: reduce)').matches;

	if (prefersReducedMotion) {
		// No animations for reduced motion
		return { destroy: () => {} };
	}

	// Initial transform based on type
	const transforms: Record<string, string> = {
		fade: '',
		'fade-up': 'translateY(30px)',
		'fade-down': 'translateY(-30px)',
		'fade-left': 'translateX(30px)',
		'fade-right': 'translateX(-30px)',
		scale: 'scale(0.95)'
	};

	// Set initial hidden state
	node.style.opacity = '0';
	node.style.transform = transforms[type] || '';
	node.style.transition = `opacity ${duration}ms cubic-bezier(0.25, 0.46, 0.45, 0.94) ${delay}ms, transform ${duration}ms cubic-bezier(0.25, 0.46, 0.45, 0.94) ${delay}ms`;

	const observer = new IntersectionObserver(
		(entries) => {
			if (entries[0]?.isIntersecting) {
				node.style.opacity = '1';
				node.style.transform = 'translateY(0) translateX(0) scale(1)';

				if (once) {
					observer.disconnect();
				}
			} else if (!once) {
				node.style.opacity = '0';
				node.style.transform = transforms[type] || '';
			}
		},
		{ threshold, rootMargin: '0px 0px -50px 0px' }
	);

	observer.observe(node);

	return {
		destroy() {
			observer.disconnect();
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// CSS CLASS-BASED REVEAL SYSTEM
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Initialize reveal animations for elements with .reveal class
 * Used with CSS classes defined in app.css
 *
 * @example
 * onMount(() => initRevealObserver());
 */
export function initRevealObserver(): () => void {
	if (typeof window === 'undefined') return () => {};

	const observer = new IntersectionObserver(
		(entries) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					entry.target.classList.add('is-visible');
					observer.unobserve(entry.target);
				}
			});
		},
		{ threshold: 0.1, rootMargin: '0px 0px -50px 0px' }
	);

	// Observe all elements with .reveal class
	document.querySelectorAll('.reveal').forEach((el) => observer.observe(el));

	return () => observer.disconnect();
}
