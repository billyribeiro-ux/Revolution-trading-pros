<script lang="ts">
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import { cardEntrance } from '../../utils/animations.js';
	import type { Snippet } from 'svelte';

	interface Props {
		class?: string;
		glow?: 'call' | 'put' | 'accent' | 'none';
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

	let cardEl: HTMLDivElement | undefined = $state();
	let gsapInstance: typeof import('gsap').default | undefined;
	let entranceTween: gsap.core.Tween | undefined;

	$effect(() => {
		if (animate && cardEl) {
			entranceTween?.kill();
			entranceTween = cardEntrance(cardEl, delay);
		}
	});

	let glowClass = $derived(
		glow === 'call'
			? 'glow-call'
			: glow === 'put'
				? 'glow-put'
				: glow === 'accent'
					? 'shadow-[0_0_20px_var(--calc-accent-glow)]'
					: ''
	);

	// --- Parallax tilt handlers ---
	function handlePointerMove(e: PointerEvent) {
		if (!cardEl) return;
		const rect = cardEl.getBoundingClientRect();
		const cx = rect.left + rect.width / 2;
		const cy = rect.top + rect.height / 2;
		const dx = (e.clientX - cx) / (rect.width / 2);
		const dy = (e.clientY - cy) / (rect.height / 2);
		const rotateY = dx * 2.5;
		const rotateX = -dy * 2.5;
		cardEl.style.transform = `perspective(800px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
	}

	function handlePointerLeave() {
		if (!cardEl) return;
		cardEl.style.transform = 'perspective(800px) rotateX(0deg) rotateY(0deg)';
	}

	// --- GSAP glow pulse on mount ---
	let glowTween: gsap.core.Tween | undefined;

	onMount(() => {
		if (!browser) return;
		(async () => {
			gsapInstance = (await import('gsap')).default;
			if (glow !== 'none' && cardEl && gsapInstance) {
				glowTween = gsapInstance.to(cardEl, {
					boxShadow:
						glow === 'call'
							? '0 0 30px rgba(34,197,94,0.25), 0 0 60px rgba(34,197,94,0.10)'
							: glow === 'put'
								? '0 0 30px rgba(239,68,68,0.25), 0 0 60px rgba(239,68,68,0.10)'
								: '0 0 30px rgba(99,102,241,0.25), 0 0 60px rgba(99,102,241,0.10)',
					duration: 1.2,
					repeat: -1,
					yoyo: true,
					ease: 'sine.inOut'
				});
			}
		})();

		return () => {
			glowTween?.kill();
			entranceTween?.kill();
		};
	});
</script>

<div
	bind:this={cardEl}
	class="glass-card glass-card--animated {glowClass} {className}"
	onpointermove={handlePointerMove}
	onpointerleave={handlePointerLeave}
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
	@property --border-angle {
		syntax: '<angle>';
		initial-value: 0deg;
		inherits: false;
	}

	.glass-card--animated {
		transition: transform 0.15s ease-out;
		--border-angle: 0deg;
	}

	.glass-card--animated:hover {
		border-image: conic-gradient(
				from var(--border-angle),
				rgba(99, 102, 241, 0.6),
				rgba(34, 197, 94, 0.4),
				rgba(239, 68, 68, 0.4),
				rgba(99, 102, 241, 0.6)
			)
			1;
		animation: rotate-border 3s linear infinite;
	}

	@keyframes rotate-border {
		to {
			--border-angle: 360deg;
		}
	}
</style>
