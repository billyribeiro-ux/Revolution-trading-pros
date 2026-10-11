<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import { on } from 'svelte/events';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { cardEntrance } from '../../utils/animations.js';

	type Glow = 'call' | 'put' | 'accent' | 'none';

	/** Other attributes (`data-*`, `aria-*`, …) are forwarded to the card's element. */
	interface Props extends Omit<HTMLAttributes<HTMLDivElement>, 'children'> {
		glow?: Glow;
		animate?: boolean;
		delay?: number;
		header?: Snippet;
		children: Snippet;
	}

	let {
		class: className,
		glow = 'none',
		animate = true,
		delay = 0,
		header,
		children,
		...rest
	}: Props = $props();

	/** The entrance tween. The tilt leaves `transform` alone while it plays. */
	let entrance: ReturnType<typeof cardEntrance> | undefined;

	/**
	 * Entrance on mount. The props are read untracked: a later change must neither replay
	 * the entrance nor kill it midway (which would strand the card half-faded).
	 */
	const enter: Attachment<HTMLDivElement> = (element) => {
		untrack(() => {
			if (animate && !prefersReducedMotion.current) entrance = cardEntrance(element, delay);
		});
		return () => entrance?.kill();
	};

	// ── Parallax tilt ────────────────────────────────────
	// Decorative and mouse-only. It must never move a control under the cursor:
	//  • the angle is capped so an edge lifts at most MAX_EDGE_LIFT_PX towards the viewer
	//    (a fixed 2.5° lifted the ends of the tall InputPanel ~20px and warped its sliders);
	//  • the tilt holds still while a button is pressed (a slider drag, a text selection)
	//    and while the pointer is over a control or a chart, and a drag that leaves the
	//    card keeps the tilt until the button is released.
	const MAX_TILT_DEG = 2.5;
	const MAX_EDGE_LIFT_PX = 10;
	const PERSPECTIVE_PX = 800;
	const HOLD_OVER =
		'a[href], button, input, select, textarea, label, [role="slider"], [contenteditable], canvas, svg';

	/** Largest angle (deg) that keeps an edge `halfSize` px from the centre within the lift cap. */
	function maxTilt(halfSize: number): number {
		return Math.min(MAX_TILT_DEG, (Math.atan(MAX_EDGE_LIFT_PX / halfSize) * 180) / Math.PI);
	}

	const clampUnit = (n: number) => Math.max(-1, Math.min(1, n));

	const parallaxTilt: Attachment<HTMLDivElement> = (element) => {
		if (prefersReducedMotion.current) return;

		let frame = 0;
		let pointerX = 0;
		let pointerY = 0;
		let flattenOnRelease = false;

		const holds = (target: EventTarget | null) => {
			const match = target instanceof Element ? target.closest(HOLD_OVER) : null;
			return match !== null && element.contains(match);
		};

		const apply = () => {
			frame = 0;
			if (entrance?.isActive()) return;
			// Layout size, not the (tilted) bounding box, sets the angle cap.
			const halfWidth = element.offsetWidth / 2;
			const halfHeight = element.offsetHeight / 2;
			if (halfWidth === 0 || halfHeight === 0) return;
			const rect = element.getBoundingClientRect();
			const dx = clampUnit((pointerX - (rect.left + rect.width / 2)) / halfWidth);
			const dy = clampUnit((pointerY - (rect.top + rect.height / 2)) / halfHeight);
			element.style.transform = `perspective(${PERSPECTIVE_PX}px) rotateX(${-dy * maxTilt(halfHeight)}deg) rotateY(${dx * maxTilt(halfWidth)}deg)`;
		};

		const flatten = () => {
			cancelAnimationFrame(frame);
			frame = 0;
			flattenOnRelease = false;
			if (!entrance?.isActive()) element.style.transform = '';
		};

		const stop = [
			on(element, 'pointermove', (event) => {
				if (event.pointerType !== 'mouse' || event.buttons !== 0 || holds(event.target)) return;
				pointerX = event.clientX;
				pointerY = event.clientY;
				frame ||= requestAnimationFrame(apply);
			}),
			on(element, 'pointerenter', () => (flattenOnRelease = false)),
			on(element, 'pointerleave', (event) => {
				if (event.buttons === 0) flatten();
				else flattenOnRelease = true;
			}),
			on(window, 'pointerup', () => {
				if (flattenOnRelease) flatten();
			})
		];

		return () => {
			for (const off of stop) off();
			flatten();
		};
	};
</script>

<div
	{...rest}
	{@attach enter}
	{@attach parallaxTilt}
	class={[
		'glass-card glass-card--interactive',
		{
			'glow-call glow-pulse-call': glow === 'call',
			'glow-put glow-pulse-put': glow === 'put',
			'shadow-[0_0_20px_var(--calc-accent-glow)] glow-pulse-accent': glow === 'accent'
		},
		className
	]}
>
	<span class="border-ring" aria-hidden="true"><span class="border-ring-spin"></span></span>
	{#if header}
		<div class="border-b border-[var(--calc-glass-border)] px-5 py-3">
			{@render header()}
		</div>
	{/if}
	<div class="p-5">
		{@render children()}
	</div>
</div>

<style>
	.glass-card--interactive {
		position: relative;
	}

	/* Rotating gradient border on hover: a 1px ring (masked, so it keeps the card's
	   rounded corners) over an oversized conic-gradient square that spins with a
	   transform. A transform animation runs on the compositor, so the card is not
	   repainted every frame — animating the gradient's angle (a custom property)
	   repainted the whole card, slider drags included. A conic gradient depends only
	   on the angle from its centre, so spinning the square looks the same as rotating
	   the gradient's start angle. */
	.border-ring {
		position: absolute;
		inset: -1px;
		padding: 1px;
		border-radius: inherit;
		overflow: clip;
		container-type: size;
		mask:
			linear-gradient(#000 0 0) content-box exclude,
			linear-gradient(#000 0 0);
		opacity: 0;
		transition: opacity 0.3s ease;
		pointer-events: none;
	}

	.border-ring-spin {
		position: absolute;
		top: 50%;
		left: 50%;
		/* √2 × the longer side covers the card's diagonal at every angle. */
		width: 142cqmax;
		aspect-ratio: 1;
		translate: -50% -50%;
		background: conic-gradient(
			rgba(99, 102, 241, 0.6),
			rgba(34, 197, 94, 0.4),
			rgba(239, 68, 68, 0.4),
			rgba(99, 102, 241, 0.6)
		);
	}

	@media (hover: hover) {
		.glass-card--interactive:hover > .border-ring {
			opacity: 1;
		}
	}

	@media (hover: hover) and (prefers-reduced-motion: no-preference) {
		.glass-card--interactive:hover .border-ring-spin {
			animation: border-spin 3s linear infinite;
		}
	}

	@keyframes border-spin {
		to {
			transform: rotate(1turn);
		}
	}

	/* Glow pulse: a decorative loop, so a CSS animation rather than a JS tween. It starts
	   from the static glow class's shadow and eases (sine in-out) to a wider glow and back. */
	@media (prefers-reduced-motion: no-preference) {
		.glow-pulse-call {
			animation: glow-pulse-call 1.2s cubic-bezier(0.37, 0, 0.63, 1) infinite alternate;
		}

		.glow-pulse-put {
			animation: glow-pulse-put 1.2s cubic-bezier(0.37, 0, 0.63, 1) infinite alternate;
		}

		.glow-pulse-accent {
			animation: glow-pulse-accent 1.2s cubic-bezier(0.37, 0, 0.63, 1) infinite alternate;
		}
	}

	@keyframes glow-pulse-call {
		from {
			box-shadow:
				0 0 20px var(--calc-call-glow),
				0 0 40px var(--calc-call-glow);
		}
		to {
			box-shadow:
				0 0 30px rgba(34, 197, 94, 0.25),
				0 0 60px rgba(34, 197, 94, 0.1);
		}
	}

	@keyframes glow-pulse-put {
		from {
			box-shadow:
				0 0 20px var(--calc-put-glow),
				0 0 40px var(--calc-put-glow);
		}
		to {
			box-shadow:
				0 0 30px rgba(239, 68, 68, 0.25),
				0 0 60px rgba(239, 68, 68, 0.1);
		}
	}

	@keyframes glow-pulse-accent {
		from {
			box-shadow: 0 0 20px var(--calc-accent-glow);
		}
		to {
			box-shadow:
				0 0 30px rgba(99, 102, 241, 0.25),
				0 0 60px rgba(99, 102, 241, 0.1);
		}
	}
</style>
