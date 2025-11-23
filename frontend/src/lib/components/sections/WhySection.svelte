<script lang="ts">
	import { IconUsers, IconShieldCheck, IconTrendingUp } from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	const features = [
		{
			title: 'Structured Learning',
			description:
				'No random callouts. Learn proven frameworks in live rooms and comprehensive courses designed for real skill development.',
			icon: IconUsers,
			gradient: 'from-cyan-400 to-blue-500'
		},
		{
			title: 'Risk-First Philosophy',
			description:
				'Every alert, every trade, every lesson emphasizes risk management and position sizing. We teach survival before growth.',
			icon: IconShieldCheck,
			gradient: 'from-emerald-400 to-teal-500'
		},
		{
			title: 'Professional Tools',
			description:
				'Access institutional-grade indicators and analysis tools built by traders, for traders. No toy indicators here.',
			icon: IconTrendingUp,
			gradient: 'from-indigo-400 to-purple-500'
		}
	];

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
	class="why-section relative py-24 px-4 sm:px-6 lg:px-8 overflow-hidden"
>
	<!-- Animated Background -->
	<div class="absolute inset-0 bg-gradient-to-br from-slate-900 via-blue-950 to-slate-900"></div>

	<!-- Floating Orbs -->
	<div
		class="absolute top-20 left-10 w-72 h-72 bg-cyan-500/10 rounded-full blur-3xl animate-float"
	></div>
	<div
		class="absolute bottom-20 right-10 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl animate-float-delayed"
	></div>

	<div class="relative max-w-7xl mx-auto z-10">
		<div class="text-center max-w-3xl mx-auto mb-20">
			<h2
				class="text-4xl sm:text-5xl font-heading font-bold mb-6 bg-gradient-to-r from-cyan-300 via-blue-300 to-indigo-300 bg-clip-text text-transparent animate-fade-in"
			>
				Why Revolution Trading Pros?
			</h2>
			<p class="text-lg text-slate-300 leading-relaxed">
				We built Revolution Trading Pros because the industry needed an alternative to hype-driven
				education. Our ecosystem combines live trading rooms, precision alerts, structured courses,
				and professional indicators—all aligned around one philosophy: <span
					class="text-white font-semibold bg-gradient-to-r from-cyan-400 to-blue-400 bg-clip-text text-transparent"
					>disciplined, risk-first execution</span
				>.
			</p>
		</div>

		<div class="grid md:grid-cols-3 gap-8">
			{#each features as feature, i}
				<div
					class="feature-card group relative bg-slate-900/40 backdrop-blur-xl border border-slate-700/50 rounded-3xl p-8 hover:border-cyan-500/50 transition-all duration-500"
					class:animate-slide-up={isVisible}
					style="animation-delay: {i * 150}ms"
				>
					<!-- Glass effect overlay -->
					<div
						class="absolute inset-0 bg-gradient-to-br from-white/5 to-transparent rounded-3xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"
					></div>

					<!-- Animated gradient border on hover -->
					<div
						class="absolute inset-0 rounded-3xl bg-gradient-to-r {feature.gradient} opacity-0 group-hover:opacity-20 blur-xl transition-opacity duration-500"
					></div>

					<div class="relative z-10">
						<!-- Floating Icon -->
						<div
							class="mb-6 transform group-hover:scale-110 group-hover:-translate-y-2 transition-transform duration-500"
						>
							<div class="w-16 h-16 rounded-2xl bg-gradient-to-br {feature.gradient} p-0.5">
								<div
									class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center"
								>
									<svelte:component
										this={feature.icon}
										size={32}
										class="text-white animate-icon-float"
									/>
								</div>
							</div>
						</div>

						<h3
							class="text-2xl font-heading font-bold mb-4 text-white group-hover:text-cyan-300 transition-colors duration-300"
						>
							{feature.title}
						</h3>
						<p
							class="text-slate-400 leading-relaxed group-hover:text-slate-300 transition-colors duration-300"
						>
							{feature.description}
						</p>
					</div>

					<!-- Bottom accent line -->
					<div
						class="absolute bottom-0 left-0 right-0 h-1 bg-gradient-to-r {feature.gradient} transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 rounded-b-3xl"
					></div>
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	.why-section {
		position: relative;
	}

	/* Floating animations */
	@keyframes float {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(-20px) scale(1.05);
		}
	}

	@keyframes float-delayed {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(30px) scale(0.95);
		}
	}

	.animate-float {
		animation: float 8s ease-in-out infinite;
	}

	.animate-float-delayed {
		animation: float-delayed 10s ease-in-out infinite;
	}

	/* Icon floating */
	@keyframes icon-float {
		0%,
		100% {
			transform: translateY(0);
		}
		50% {
			transform: translateY(-8px);
		}
	}

	.animate-icon-float {
		animation: icon-float 3s ease-in-out infinite;
	}

	/* Fade in animation */
	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(-20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.animate-fade-in {
		animation: fade-in 0.8s ease-out;
	}

	/* Slide up animation */
	@keyframes slide-up {
		from {
			opacity: 0;
			transform: translateY(40px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.animate-slide-up {
		animation: slide-up 0.6s ease-out forwards;
		opacity: 0;
	}

	/* Card glow effect */
	.feature-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 1px;
		background: linear-gradient(90deg, transparent, rgba(6, 182, 212, 0.5), transparent);
		opacity: 0;
		transition: opacity 0.5s;
	}

	.feature-card:hover::before {
		opacity: 1;
	}
</style>
