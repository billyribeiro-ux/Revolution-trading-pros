// ============================================================
// GSAP ANIMATION PRESETS & UTILITIES
// ============================================================
//
// Two rules hold for every preset here:
//
// 1. `fromTo` with explicit end values, never `from`. A `from()` tween reads its end
//    values from the element when it initialises, and a ScrollTrigger refresh that
//    re-initialises it while the element still sits at the hidden start state makes it
//    animate from hidden to hidden — the element stays invisible for good (a confirmed
//    bug in this calculator).
// 2. Reduced motion is honoured. `reducedMotion` defaults to the user's
//    `prefers-reduced-motion` setting (read when the preset is called); when it is on,
//    nothing moves — the tween lands on its end state at once, or (pulseHighlight) keeps
//    only its non-moving part.

import gsap from 'gsap';
import { prefersReducedMotion } from 'svelte/motion';

export interface MotionOptions {
	/** Skip motion. Defaults to the user's `prefers-reduced-motion` setting. */
	reducedMotion?: boolean;
}

/** Animate a number from the value currently shown in `element` to `targetValue`. */
export function animateNumber(
	element: HTMLElement,
	targetValue: number,
	options: {
		duration?: number;
		decimals?: number;
		prefix?: string;
		suffix?: string;
		onUpdate?: (value: number) => void;
	} & MotionOptions = {}
): gsap.core.Tween {
	const {
		duration = 0.6,
		decimals = 2,
		prefix = '',
		suffix = '',
		onUpdate,
		reducedMotion = prefersReducedMotion.current
	} = options;
	const shown = Number.parseFloat(element.textContent?.replace(/[^0-9.-]/g, '') ?? '');
	const counter = { value: Number.isFinite(shown) ? shown : 0 };

	return gsap.fromTo(
		counter,
		{ value: counter.value },
		{
			value: targetValue,
			duration: reducedMotion ? 0 : duration,
			ease: 'power2.out',
			onUpdate() {
				element.textContent = `${prefix}${counter.value.toFixed(decimals)}${suffix}`;
				onUpdate?.(counter.value);
			}
		}
	);
}

/** Where an entrance ends for the properties it usually starts from. */
const RESTING: Readonly<Record<string, number>> = {
	opacity: 1,
	autoAlpha: 1,
	x: 0,
	y: 0,
	z: 0,
	xPercent: 0,
	yPercent: 0,
	rotation: 0,
	rotate: 0,
	rotationX: 0,
	rotateX: 0,
	rotationY: 0,
	rotateY: 0,
	skewX: 0,
	skewY: 0,
	scale: 1,
	scaleX: 1,
	scaleY: 1
};

/**
 * Explicit end values for an entrance that starts at `from`: the resting value of each
 * known property, `to` where given, and otherwise each element's current value —
 * captured once, now, so a later re-initialisation cannot read the hidden start state
 * back as the end state.
 */
function entranceEnd(targets: Element[], from: gsap.TweenVars, to: gsap.TweenVars): gsap.TweenVars {
	const end: gsap.TweenVars = {};
	for (const key of Object.keys(from)) {
		if (Object.hasOwn(to, key)) continue;
		if (Object.hasOwn(RESTING, key)) {
			end[key] = RESTING[key];
		} else {
			const current = targets.map((target) => gsap.getProperty(target, key));
			end[key] = (index: number) => current[index];
		}
	}
	return { ...end, ...to };
}

/** Staggered entrance animation for a list of elements. */
export function staggerEntrance(
	elements: HTMLElement[] | NodeListOf<Element>,
	options: {
		/** Start values. */
		from?: gsap.TweenVars;
		/** End values for properties without an obvious resting value (see `entranceEnd`). */
		to?: gsap.TweenVars;
		duration?: number;
		stagger?: number;
		ease?: string;
	} & MotionOptions = {}
): gsap.core.Tween {
	const {
		from = { opacity: 0, y: 20, scale: 0.95 },
		to = {},
		duration = 0.5,
		stagger = 0.08,
		ease = 'power3.out',
		reducedMotion = prefersReducedMotion.current
	} = options;
	const targets = Array.from(elements);

	return gsap.fromTo(targets, from, {
		...entranceEnd(targets, from, to),
		duration: reducedMotion ? 0 : duration,
		stagger: reducedMotion ? 0 : stagger,
		ease
	});
}

/**
 * Pulse animation for value changes: a brief glow and lift, then back to rest. With
 * reduced motion the glow still pulses (it does not move) but the lift is left out.
 * The inline styles it sets are cleared at the end, so the element's own stylesheet
 * shadow and transform apply again.
 */
export function pulseHighlight(
	element: HTMLElement,
	color: string = '#6366f1',
	{ reducedMotion = prefersReducedMotion.current }: MotionOptions = {}
): gsap.core.Timeline {
	const rest: gsap.TweenVars = { boxShadow: '0 0 0px transparent' };
	const peak: gsap.TweenVars = { boxShadow: `0 0 20px ${color}44, 0 0 40px ${color}22` };
	if (!reducedMotion) {
		rest.scale = 1;
		peak.scale = 1.02;
	}
	const animated = reducedMotion ? 'boxShadow' : 'boxShadow,scale';

	return gsap
		.timeline()
		.fromTo(element, rest, { ...peak, duration: 0.15, ease: 'power2.out' })
		.fromTo(element, peak, {
			...rest,
			duration: 0.4,
			ease: 'power2.inOut',
			// Inside a timeline a fromTo renders its start state immediately; this second
			// step must not, or the peak would show before the first step has begun.
			immediateRender: false,
			clearProps: animated
		});
}

/** Card entrance on mount (fades and lifts in after `delay` seconds). */
export function cardEntrance(
	element: HTMLElement,
	delay: number = 0,
	{ reducedMotion = prefersReducedMotion.current }: MotionOptions = {}
): gsap.core.Tween {
	return gsap.fromTo(
		element,
		{ opacity: 0, y: 30, scale: 0.97 },
		{
			opacity: 1,
			y: 0,
			scale: 1,
			duration: reducedMotion ? 0 : 0.6,
			delay: reducedMotion ? 0 : delay,
			ease: 'power3.out'
		}
	);
}
