<script lang="ts">
	import { IconSchool, IconSparkles } from '@tabler/icons-svelte';
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
	class="mentorship-section relative py-32 px-4 sm:px-6 lg:px-8 overflow-hidden"
>
	<!-- Luxury gradient background -->
	<div class="absolute inset-0 bg-gradient-to-br from-emerald-950 via-slate-950 to-teal-950"></div>

	<!-- Animated spotlight -->
	<div class="absolute inset-0 spotlight"></div>

	<!-- Floating golden particles -->
	<div class="absolute inset-0">
		<div class="gold-particle" style="top: 20%; left: 15%; animation-delay: 0s;"></div>
		<div class="gold-particle" style="top: 60%; right: 20%; animation-delay: 2s;"></div>
		<div class="gold-particle" style="bottom: 25%; left: 10%; animation-delay: 4s;"></div>
		<div class="gold-particle" style="top: 40%; right: 10%; animation-delay: 6s;"></div>
	</div>

	<div class="relative max-w-5xl mx-auto z-10">
		<div class="text-center">
			<!-- Elite badge -->
			<div
				class="inline-flex items-center gap-2 mb-6 px-6 py-3 bg-gradient-to-r from-emerald-500/10 to-teal-500/10 border border-emerald-500/30 rounded-full backdrop-blur-sm animate-fade-in"
			>
				<IconSparkles size={20} class="text-emerald-400 animate-sparkle" />
				<span class="text-sm font-bold tracking-wider text-emerald-300">ELITE PROGRAM</span>
				<IconSparkles size={20} class="text-emerald-400 animate-sparkle-delayed" />
			</div>

			<!-- Floating icon -->
			<div class="relative inline-block mb-8" class:animate-icon-reveal={isVisible}>
				<div
					class="absolute inset-0 bg-gradient-to-r from-emerald-400 to-teal-400 blur-3xl opacity-50 animate-pulse-glow"
				></div>
				<div
					class="relative w-24 h-24 mx-auto rounded-3xl bg-gradient-to-br from-emerald-400 via-teal-400 to-cyan-400 p-1 transform hover:scale-110 hover:rotate-12 transition-all duration-500"
				>
					<div class="w-full h-full bg-slate-900 rounded-3xl flex items-center justify-center">
						<IconSchool size={48} class="text-emerald-300" />
					</div>
				</div>
			</div>

			<h2
				class="text-5xl sm:text-6xl font-heading font-bold mb-8 bg-gradient-to-r from-emerald-300 via-teal-300 to-cyan-300 bg-clip-text text-transparent"
				class:animate-title-reveal={isVisible}
			>
				1-on-1 Mentorship
			</h2>

			<p
				class="text-xl text-slate-300 mb-12 leading-relaxed max-w-3xl mx-auto"
				class:animate-text-reveal={isVisible}
			>
				Work directly with experienced traders in private mentorship sessions. Custom curriculum,
				personalized feedback, and accountability to accelerate your development.
				<span class="block mt-4 text-emerald-400 font-semibold">Limited availability.</span>
			</p>

			<!-- Exclusive features grid -->
			<div class="grid sm:grid-cols-3 gap-6 mb-12 max-w-4xl mx-auto">
				{#each [{ label: 'Private Sessions', icon: '👤' }, { label: 'Custom Curriculum', icon: '📚' }, { label: 'Direct Feedback', icon: '💬' }] as feature, i}
					<div
						class="relative bg-slate-900/40 backdrop-blur-xl border border-emerald-500/20 rounded-2xl p-6 hover:border-emerald-500/40 transform hover:scale-105 transition-all duration-500"
						class:animate-feature-reveal={isVisible}
						style="animation-delay: {i * 150}ms"
					>
						<div class="text-4xl mb-3">{feature.icon}</div>
						<div class="text-sm font-semibold text-emerald-300">{feature.label}</div>
					</div>
				{/each}
			</div>

			<!-- Premium CTA -->
			<div class="relative inline-block">
				<div
					class="absolute -inset-1 bg-gradient-to-r from-emerald-400 to-teal-400 rounded-2xl blur-xl opacity-60 group-hover:opacity-100 transition-opacity duration-500"
				></div>
				<a
					href="/mentorship"
					class="relative block px-12 py-6 bg-gradient-to-r from-emerald-500 to-teal-500 text-white text-lg font-heading font-bold rounded-2xl hover:shadow-2xl hover:shadow-emerald-500/50 transform hover:scale-105 hover:-translate-y-1 transition-all duration-300 overflow-hidden group"
				>
					<span class="relative z-10 flex items-center gap-3">
						<IconSparkles size={24} class="animate-sparkle" />
						<span>Apply for Mentorship</span>
						<IconSparkles size={24} class="animate-sparkle-delayed" />
					</span>
					<div
						class="absolute inset-0 bg-white/20 transform scale-x-0 group-hover:scale-x-100 transition-transform duration-700 origin-left"
					></div>
				</a>
			</div>

			<!-- Exclusivity note -->
			<p class="mt-8 text-sm text-slate-500 italic">
				Application required • Personalized review • Investment starting at $2,500/month
			</p>
		</div>
	</div>
</section>

<style>
	.mentorship-section {
		position: relative;
	}

	/* Animated spotlight effect */
	.spotlight {
		background: radial-gradient(
			circle at 50% 50%,
			rgba(16, 185, 129, 0.15) 0%,
			rgba(20, 184, 166, 0.1) 25%,
			transparent 60%
		);
		animation: spotlight-pulse 8s ease-in-out infinite;
	}

	@keyframes spotlight-pulse {
		0%,
		100% {
			transform: scale(1);
			opacity: 0.5;
		}
		50% {
			transform: scale(1.2);
			opacity: 0.8;
		}
	}

	/* Golden floating particles */
	.gold-particle {
		position: absolute;
		width: 8px;
		height: 8px;
		background: radial-gradient(circle, #10b981, transparent);
		border-radius: 50%;
		animation: float-gold 6s ease-in-out infinite;
	}

	@keyframes float-gold {
		0%,
		100% {
			transform: translateY(0) scale(1);
			opacity: 0.3;
		}
		50% {
			transform: translateY(-40px) scale(1.5);
			opacity: 1;
		}
	}

	/* Sparkle animation */
	@keyframes sparkle {
		0%,
		100% {
			opacity: 1;
			transform: scale(1) rotate(0deg);
		}
		50% {
			opacity: 0.5;
			transform: scale(1.2) rotate(180deg);
		}
	}

	:global(.animate-sparkle) {
		animation: sparkle 2s ease-in-out infinite;
	}

	:global(.animate-sparkle-delayed) {
		animation: sparkle 2s ease-in-out infinite;
		animation-delay: 1s;
	}

	/* Pulse glow */
	@keyframes pulse-glow {
		0%,
		100% {
			opacity: 0.3;
			transform: scale(1);
		}
		50% {
			opacity: 0.6;
			transform: scale(1.1);
		}
	}

	.animate-pulse-glow {
		animation: pulse-glow 3s ease-in-out infinite;
	}

	/* Reveal animations */
	@keyframes icon-reveal {
		from {
			opacity: 0;
			transform: translateY(-30px) scale(0.8);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	@keyframes title-reveal {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes text-reveal {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes feature-reveal {
		from {
			opacity: 0;
			transform: translateY(30px) scale(0.9);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.animate-fade-in {
		animation: title-reveal 0.6s ease-out;
	}

	.animate-icon-reveal {
		animation: icon-reveal 0.8s ease-out forwards;
		opacity: 0;
	}

	.animate-title-reveal {
		animation: title-reveal 0.8s ease-out 0.2s forwards;
		opacity: 0;
	}

	.animate-text-reveal {
		animation: text-reveal 0.8s ease-out 0.4s forwards;
		opacity: 0;
	}

	.animate-feature-reveal {
		animation: feature-reveal 0.6s ease-out forwards;
		opacity: 0;
	}
</style>
