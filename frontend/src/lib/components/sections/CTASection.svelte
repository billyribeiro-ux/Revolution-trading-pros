<script lang="ts">
	import { IconRocket, IconArrowRight, IconStar } from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let sectionRef: HTMLElement;
	let isVisible = false;

	onMount(() => {
		if (!browser) return;

		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						isVisible = true;
					}
				});
			},
			{ threshold: 0.1 }
		);

		if (sectionRef) {
			observer.observe(sectionRef);
		}

		return () => observer.disconnect();
	});
</script>

<section
	bind:this={sectionRef}
	class="cta-section relative py-32 px-4 sm:px-6 lg:px-8 overflow-hidden"
>
	<!-- Vibrant gradient background -->
	<div class="absolute inset-0 bg-gradient-to-br from-yellow-950 via-orange-950 to-red-950"></div>

	<!-- Animated rays -->
	<div class="absolute inset-0 rays"></div>

	<!-- Energetic particles -->
	<div class="absolute inset-0">
		{#each Array(20) as _, i}
			<div
				class="energy-particle"
				style="
          left: {Math.random() * 100}%;
          top: {Math.random() * 100}%;
          animation-delay: {Math.random() * 5}s;
          animation-duration: {3 + Math.random() * 4}s;
        "
			></div>
		{/each}
	</div>

	<div class="relative max-w-5xl mx-auto z-10">
		<div class="text-center">
			<!-- Urgency badge -->
			<div
				class="inline-flex items-center gap-2 mb-8 px-6 py-3 bg-gradient-to-r from-yellow-500/20 to-orange-500/20 border-2 border-yellow-500/50 rounded-full backdrop-blur-sm animate-pulse-border"
				class:animate-badge-reveal={isVisible}
			>
				<IconStar size={20} class="text-yellow-400 animate-spin-slow" />
				<span class="text-sm font-bold tracking-widest text-yellow-300"
					>JOIN THOUSANDS OF DISCIPLINED TRADERS</span
				>
				<IconStar size={20} class="text-yellow-400 animate-spin-slow-reverse" />
			</div>

			<!-- Main heading with dramatic reveal -->
			<h2
				class="text-5xl sm:text-6xl lg:text-7xl font-heading font-black mb-8"
				class:animate-title-explode={isVisible}
			>
				<span
					class="block bg-gradient-to-r from-yellow-300 via-orange-300 to-red-300 bg-clip-text text-transparent"
				>
					Ready to Trade
				</span>
				<span
					class="block bg-gradient-to-r from-orange-300 via-yellow-300 to-yellow-400 bg-clip-text text-transparent"
				>
					with Discipline?
				</span>
			</h2>

			<!-- Supporting text -->
			<p
				class="text-xl sm:text-2xl text-slate-200 mb-12 leading-relaxed max-w-3xl mx-auto font-semibold"
				class:animate-text-fade-in={isVisible}
			>
				Join thousands of traders who've chosen the risk-first approach.
				<span class="block mt-2 text-yellow-300">Start your journey today.</span>
			</p>

			<!-- Social proof badges -->
			<div class="flex flex-wrap justify-center gap-6 mb-12">
				{#each [{ icon: '👥', count: '5,000+', label: 'Active Members' }, { icon: '📊', count: '10,000+', label: 'Trades Analyzed' }, { icon: '⭐', count: '4.9/5', label: 'Member Rating' }] as stat, i}
					<div
						class="bg-slate-900/60 backdrop-blur-xl border border-yellow-500/30 rounded-2xl px-6 py-4 transform hover:scale-110 hover:border-yellow-500/60 transition-all duration-300"
						class:animate-stat-pop={isVisible}
						style="animation-delay: {i * 100}ms"
					>
						<div class="text-3xl mb-1">{stat.icon}</div>
						<div class="text-2xl font-bold text-yellow-300">{stat.count}</div>
						<div class="text-xs text-slate-400">{stat.label}</div>
					</div>
				{/each}
			</div>

			<!-- MASSIVE CTA Button -->
			<div class="relative inline-block" class:animate-cta-reveal={isVisible}>
				<!-- Glowing aura -->
				<div
					class="absolute -inset-4 bg-gradient-to-r from-yellow-400 via-orange-500 to-red-500 rounded-3xl blur-2xl opacity-60 animate-pulse-glow-intense"
				></div>

				<a
					href="/signup"
					class="relative block px-16 py-8 bg-gradient-to-r from-yellow-400 via-orange-500 to-red-500 text-slate-900 text-2xl font-heading font-black rounded-2xl hover:shadow-[0_0_80px_rgba(251,191,36,0.8)] transform hover:scale-110 hover:-translate-y-2 transition-all duration-300 overflow-hidden group"
				>
					<span class="relative z-10 flex items-center justify-center gap-4">
						<IconRocket size={32} class="animate-bounce-subtle" />
						<span>Create Your Free Account</span>
						<IconArrowRight
							size={32}
							class="transform group-hover:translate-x-3 transition-transform duration-300"
						/>
					</span>

					<!-- Shimmer effect -->
					<div
						class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent transform -skew-x-12 -translate-x-full group-hover:translate-x-full transition-transform duration-1000"
					></div>
				</a>
			</div>

			<!-- Risk-free guarantee -->
			<p class="mt-10 text-sm text-slate-400">
				<span class="inline-flex items-center gap-2">
					<svg class="w-5 h-5 text-emerald-400" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
							clip-rule="evenodd"
						/>
					</svg>
					<span>No credit card required</span>
				</span>
				<span class="mx-3">•</span>
				<span class="inline-flex items-center gap-2">
					<svg class="w-5 h-5 text-emerald-400" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
							clip-rule="evenodd"
						/>
					</svg>
					<span>Cancel anytime</span>
				</span>
				<span class="mx-3">•</span>
				<span class="inline-flex items-center gap-2">
					<svg class="w-5 h-5 text-emerald-400" fill="currentColor" viewBox="0 0 20 20">
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
							clip-rule="evenodd"
						/>
					</svg>
					<span>Instant access</span>
				</span>
			</p>
		</div>
	</div>
</section>

<style>
	.cta-section {
		position: relative;
	}

	/* Animated rays */
	.rays {
		background: repeating-conic-gradient(
			from 0deg,
			transparent 0deg 2deg,
			rgba(251, 191, 36, 0.05) 2deg 4deg
		);
		animation: rotate-rays 30s linear infinite;
	}

	@keyframes rotate-rays {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* Energy particles */
	.energy-particle {
		position: absolute;
		width: 4px;
		height: 4px;
		background: radial-gradient(circle, #fbbf24, transparent);
		border-radius: 50%;
		animation: float-energy 5s ease-in-out infinite;
	}

	@keyframes float-energy {
		0%,
		100% {
			transform: translateY(0) scale(1);
			opacity: 0;
		}
		50% {
			transform: translateY(-100px) scale(1.5);
			opacity: 1;
		}
	}

	/* Pulse border */
	@keyframes pulse-border {
		0%,
		100% {
			box-shadow: 0 0 20px rgba(251, 191, 36, 0.5);
		}
		50% {
			box-shadow: 0 0 40px rgba(251, 191, 36, 0.8);
		}
	}

	.animate-pulse-border {
		animation: pulse-border 2s ease-in-out infinite;
	}

	/* Intense glow pulse */
	@keyframes pulse-glow-intense {
		0%,
		100% {
			opacity: 0.4;
			transform: scale(1);
		}
		50% {
			opacity: 0.8;
			transform: scale(1.05);
		}
	}

	.animate-pulse-glow-intense {
		animation: pulse-glow-intense 2s ease-in-out infinite;
	}

	/* Spin animations */
	@keyframes spin-slow {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes spin-slow-reverse {
		from {
			transform: rotate(360deg);
		}
		to {
			transform: rotate(0deg);
		}
	}

	:global(.animate-spin-slow) {
		animation: spin-slow 8s linear infinite;
	}

	:global(.animate-spin-slow-reverse) {
		animation: spin-slow-reverse 8s linear infinite;
	}

	/* Bounce subtle */
	@keyframes bounce-subtle {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-10px);
		}
	}

	:global(.animate-bounce-subtle) {
		animation: bounce-subtle 2s ease-in-out infinite;
	}

	/* Reveal animations */
	@keyframes badge-reveal {
		from {
			opacity: 0;
			transform: translateY(-20px) scale(0.8);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	@keyframes title-explode {
		from {
			opacity: 0;
			transform: scale(0.8);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}

	@keyframes text-fade-in {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes stat-pop {
		from {
			opacity: 0;
			transform: scale(0) rotate(-180deg);
		}
		to {
			opacity: 1;
			transform: scale(1) rotate(0deg);
		}
	}

	@keyframes cta-reveal {
		from {
			opacity: 0;
			transform: translateY(40px) scale(0.9);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.animate-badge-reveal {
		animation: badge-reveal 0.6s ease-out forwards;
		opacity: 0;
	}

	.animate-title-explode {
		animation: title-explode 0.8s cubic-bezier(0.34, 1.56, 0.64, 1) 0.2s forwards;
		opacity: 0;
	}

	.animate-text-fade-in {
		animation: text-fade-in 0.8s ease-out 0.4s forwards;
		opacity: 0;
	}

	.animate-stat-pop {
		animation: stat-pop 0.6s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
		opacity: 0;
	}

	.animate-cta-reveal {
		animation: cta-reveal 0.8s ease-out 0.8s forwards;
		opacity: 0;
	}
</style>
