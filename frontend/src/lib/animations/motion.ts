import { prefersReducedMotion } from 'svelte/motion';
import { fade, fly, slide, scale, draw } from 'svelte/transition';

/** CSS media rules cannot disable Svelte's Web Animations transitions. */
export const motionFade: typeof fade = (node, options) =>
	prefersReducedMotion.current ? { duration: 0 } : fade(node, options);
export const motionFly: typeof fly = (node, options) =>
	prefersReducedMotion.current ? { duration: 0 } : fly(node, options);
export const motionSlide: typeof slide = (node, options) =>
	prefersReducedMotion.current ? { duration: 0 } : slide(node, options);
export const motionScale: typeof scale = (node, options) =>
	prefersReducedMotion.current ? { duration: 0 } : scale(node, options);
export const motionDraw: typeof draw = (node, options) =>
	prefersReducedMotion.current ? { duration: 0 } : draw(node, options);
