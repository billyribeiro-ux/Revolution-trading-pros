<script lang="ts">
	import gsap from 'gsap';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import { cardEntrance } from '../../utils/animations.js';
	import type { Snippet } from 'svelte';

	type Glow = 'call' | 'put' | 'accent' | 'none';

	interface Props {
		class?: string;
		glow?: Glow;
		animate?: boolean;
		delay?: number;
		header?: Snippet;
		children: Snippet;
	}

	let {
		class: className = '',
		glow = 'none',
		animate = true,
		delay = 0,
		header,
		children
	}: Props = $props();

	let entranceTween: gsap.core.Tween | undefined;

	let glowClass = $derived(
		glow === 'call'
			? 'glow-call'
			: glow === 'put'
				? 'glow-put'
				: glow === 'accent'
					? 'shadow-[0_0_20px_var(--calc-accent-glow)]'
					: ''
	);

	const setupCard: Attachment<HTMLDivElement> = (element) => {
		entranceTween = animate ? cardEntrance(element, delay) : undefined;
		return () => {
			entranceTween?.kill();
			entranceTween = undefined;
		};
	};

	// ── Glow pulse ──────────────────────────────────────
	const GLOW_PULSE: Record<Exclude<Glow, 'none'>, string> = {
		call: '0 0 30px rgba(34, 197, 94, 0.25), 0 0 60px rgba(34, 197, 94, 0.10)',
		put: '0 0 30px rgba(239, 68, 68, 0.25), 0 0 60px rgba(239, 68, 68, 0.10)',
		accent: '0 0 30px rgba(99, 102, 241, 0.25), 0 0 60px rgba(99, 102, 241, 0.10)'
	};

	const glowPulse: Attachment<HTMLDivElement> = (element) => {
		if (glow === 'none' || prefersReducedMotion.current) return;
		const tween = gsap.to(element, {
			boxShadow: GLOW_PULSE[glow],
			duration: 1.2,
			repeat: -1,
			yoyo: true,
			ease: 'sine.inOut'
		});
		return () => {
			tween.kill();
			gsap.set(element, { clearProps: 'boxShadow' });
		};
	};

	// ── Parallax tilt (decorative; mouse only, never while the entrance tween owns transform) ──
	const MAX_TILT_DEG = 2.5;

	const parallaxTilt: Attachment<HTMLDivElement> = (element) => {
		if (prefersReducedMotion.current) return;

		const onPointerMove = (event: PointerEvent) => {
			if (event.pointerType !== 'mouse' || entranceTween?.isActive()) return;
			const rect = element.getBoundingClientRect();
			const dx = (event.clientX - (rect.left + rect.width / 2)) / (rect.width / 2);
			const dy = (event.clientY - (rect.top + rect.height / 2)) / (rect.height / 2);
			element.style.transform = `perspective(800px) rotateX(${-dy * MAX_TILT_DEG}deg) rotateY(${dx * MAX_TILT_DEG}deg)`;
		};
		const onPointerLeave = () => {
			if (!entranceTween?.isActive()) element.style.transform = '';
		};

		element.addEventListener('pointermove', onPointerMove);
		element.addEventListener('pointerleave', onPointerLeave);
		return () => {
			element.removeEventListener('pointermove', onPointerMove);
			element.removeEventListener('pointerleave', onPointerLeave);
			onPointerLeave();
		};
	};
</script>

<div
	{@attach setupCard}
	{@attach glowPulse}
	{@attach parallaxTilt}
	class={['glass-card', 'glass-card--interactive', glowClass, className]}
>
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

	/* Rotating gradient border on hover. A masked pseudo-element instead of
	   border-image, which would square off the card's rounded corners. */
	.glass-card--interactive::before {
		content: '';
		position: absolute;
		inset: -1px;
		padding: 1px;
		border-radius: inherit;
		background: conic-gradient(
			from var(--calc-border-angle),
			rgba(99, 102, 241, 0.6),
			rgba(34, 197, 94, 0.4),
			rgba(239, 68, 68, 0.4),
			rgba(99, 102, 241, 0.6)
		);
		mask:
			linear-gradient(#000 0 0) content-box exclude,
			linear-gradient(#000 0 0);
		opacity: 0;
		transition: opacity 0.3s ease;
		pointer-events: none;
	}

	@media (hover: hover) {
		.glass-card--interactive:hover::before {
			opacity: 1;
			animation: calc-border-rotate 3s linear infinite;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.glass-card--interactive:hover::before {
			animation: none;
		}
	}
</style>
