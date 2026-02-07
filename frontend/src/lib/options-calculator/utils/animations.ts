// ============================================================
// GSAP ANIMATION PRESETS & UTILITIES
// ============================================================

import gsap from 'gsap';

/** Animate a number from current to target value */
export function animateNumber(
	element: HTMLElement,
	targetValue: number,
	options: {
		duration?: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		onUpdate?: (value: number) => void;
	} = {}
): gsap.core.Tween {
	const { duration = 0.6, decimals = 2, prefix = '', suffix = '', onUpdate } = options;
	const obj = { value: parseFloat(element.textContent?.replace(/[^0-9.-]/g, '') || '0') };

	return gsap.to(obj, {
		value: targetValue,
		duration,
		ease: 'power2.out',
		onUpdate() {
			const formatted = obj.value.toFixed(decimals);
			element.textContent = `${prefix}${formatted}${suffix}`;
			onUpdate?.(obj.value);
		},
	});
}

/** Staggered entrance animation for a list of elements */
export function staggerEntrance(
	elements: HTMLElement[] | NodeListOf<Element>,
	options: {
		from?: gsap.TweenVars;
		duration?: number;
		stagger?: number;
		ease?: string;
	} = {}
): gsap.core.Tween {
	const {
		from = { opacity: 0, y: 20, scale: 0.95 },
		duration = 0.5,
		stagger = 0.08,
		ease = 'power3.out',
	} = options;

	return gsap.from(elements, {
		...from,
		duration,
		stagger,
		ease,
	});
}

/** Pulse animation for value changes */
export function pulseHighlight(
	element: HTMLElement,
	color: string = '#6366f1'
): gsap.core.Timeline {
	const tl = gsap.timeline();
	tl.to(element, {
		boxShadow: `0 0 20px ${color}44, 0 0 40px ${color}22`,
		scale: 1.02,
		duration: 0.15,
		ease: 'power2.out',
	});
	tl.to(element, {
		boxShadow: '0 0 0px transparent',
		scale: 1,
		duration: 0.4,
		ease: 'power2.inOut',
	});
	return tl;
}

/** Card entrance on mount */
export function cardEntrance(element: HTMLElement, delay: number = 0): gsap.core.Tween {
	return gsap.from(element, {
		opacity: 0,
		y: 30,
		scale: 0.97,
		duration: 0.6,
		delay,
		ease: 'power3.out',
	});
}
