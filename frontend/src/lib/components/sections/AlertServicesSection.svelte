<script lang="ts">
	import { IconBellRinging } from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	const alertServices = [
		{
			title: 'SPX Profit Pulse',
			description:
				'Context-rich SPX 0DTE alerts with complete trade structure. Entry levels, invalidation points, size guidance, and risk parameters—never lottery tickets.',
			features: [
				'Full trade context',
				'Risk-defined setups',
				'Real-time updates',
				'Performance tracking'
			],
			href: '/alert-services/spx-profit-pulse',
			price: 'From $97/mo',
			gradient: 'from-amber-400 via-yellow-500 to-orange-500'
		},
		{
			title: 'Explosive Swings',
			description:
				'High-probability swing setups in stocks and ETFs. Multi-day holds with institutional edge and clear risk management.',
			features: [
				'Multi-day setups',
				'Stock & ETF focus',
				'Position management',
				'Entry/exit guidance'
			],
			href: '/alert-services/explosive-swings',
			price: 'From $127/mo',
			gradient: 'from-orange-400 via-amber-500 to-yellow-500'
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
	class="alert-services-section relative py-24 px-4 sm:px-6 lg:px-8 overflow-hidden"
>
	<!-- Dark background with warmth -->
	<div
		class="absolute inset-0 bg-gradient-to-br from-slate-950 via-orange-950/20 to-slate-950"
	></div>

	<!-- Animated particles -->
	<div class="absolute inset-0">
		<div class="particle particle-1"></div>
		<div class="particle particle-2"></div>
		<div class="particle particle-3"></div>
	</div>

	<div class="relative max-w-7xl mx-auto z-10">
		<div class="text-center max-w-3xl mx-auto mb-20">
			<div
				class="inline-flex items-center gap-2 mb-4 px-4 py-2 bg-amber-500/10 border border-amber-500/30 rounded-full"
			>
				<IconBellRinging size={16} class="text-amber-400 animate-ring" />
				<span class="text-sm font-semibold text-amber-300">PREMIUM OFFERINGS</span>
			</div>
			<h2
				class="text-4xl sm:text-5xl font-heading font-bold mb-6 bg-gradient-to-r from-amber-300 via-yellow-300 to-orange-300 bg-clip-text text-transparent"
			>
				Alert Services
			</h2>
			<p class="text-lg text-slate-300 leading-relaxed">
				Receive fully contextualized trade alerts with complete risk parameters. No lottery tickets,
				no spam—just professional setups with clear structure.
			</p>
		</div>

		<div class="grid md:grid-cols-2 gap-10 max-w-6xl mx-auto">
			{#each alertServices as alert, i}
				<div
					class="alert-card group relative"
					class:animate-slide-in-left={isVisible && i === 0}
					class:animate-slide-in-right={isVisible && i === 1}
				>
					<!-- Glowing border effect -->
					<div
						class="absolute -inset-0.5 bg-gradient-to-r {alert.gradient} rounded-3xl opacity-20 group-hover:opacity-40 blur-lg transition-opacity duration-500"
					></div>

					<div
						class="relative bg-slate-900/90 backdrop-blur-xl border border-amber-500/20 rounded-3xl p-10 transform hover:scale-105 transition-all duration-500"
					>
						<!-- Premium badge -->
						<div class="flex items-start justify-between mb-6">
							<div class="relative">
								<!-- Pulsing bell icon -->
								<div
									class="absolute inset-0 bg-gradient-to-r {alert.gradient} blur-xl opacity-50 animate-pulse-slow"
								></div>
								<div
									class="relative w-16 h-16 rounded-2xl bg-gradient-to-br {alert.gradient} p-0.5 transform group-hover:rotate-12 group-hover:scale-110 transition-transform duration-500"
								>
									<div
										class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center"
									>
										<IconBellRinging size={32} class="text-white animate-ring-hover" />
									</div>
								</div>
							</div>

							<!-- Price badge with shine -->
							<div
								class="relative px-4 py-2 bg-gradient-to-r {alert.gradient} rounded-full overflow-hidden group/badge"
							>
								<div
									class="absolute inset-0 bg-white/20 transform -skew-x-12 -translate-x-full group-hover/badge:translate-x-full transition-transform duration-1000"
								></div>
								<span class="relative text-sm font-bold text-slate-900">
									{alert.price}
								</span>
							</div>
						</div>

						<h3
							class="text-3xl font-heading font-bold mb-4 text-white group-hover:text-amber-300 transition-colors duration-300"
						>
							{alert.title}
						</h3>
						<p
							class="text-slate-400 mb-8 leading-relaxed group-hover:text-slate-300 transition-colors duration-300"
						>
							{alert.description}
						</p>

						<!-- Features with golden checkmarks -->
						<ul class="space-y-4 mb-10">
							{#each alert.features as feature, j}
								<li
									class="flex items-start gap-3 text-sm text-slate-300 transform group-hover:translate-x-2 transition-transform duration-300"
									style="transition-delay: {j * 75}ms"
								>
									<div class="flex-shrink-0 mt-0.5">
										<div
											class="w-6 h-6 rounded-lg bg-gradient-to-br {alert.gradient} flex items-center justify-center shadow-lg shadow-amber-500/50"
										>
											<svg
												class="w-3.5 h-3.5 text-slate-900 font-bold"
												fill="currentColor"
												viewBox="0 0 20 20"
											>
												<path
													fill-rule="evenodd"
													d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
													clip-rule="evenodd"
												/>
											</svg>
										</div>
									</div>
									<span class="leading-relaxed">{feature}</span>
								</li>
							{/each}
						</ul>

						<!-- Premium CTA button -->
						<a
							href={alert.href}
							class="block relative px-8 py-5 bg-gradient-to-r {alert.gradient} text-slate-900 text-center font-heading font-bold rounded-xl overflow-hidden group/cta shadow-xl shadow-amber-500/20 hover:shadow-2xl hover:shadow-amber-500/40 transition-all duration-300"
						>
							<span class="relative z-10 flex items-center justify-center gap-2">
								<span>View Details</span>
								<svg
									class="w-5 h-5 transform group-hover/cta:translate-x-2 transition-transform duration-300"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M13 7l5 5m0 0l-5 5m5-5H6"
									/>
								</svg>
							</span>
							<div
								class="absolute inset-0 bg-white/20 transform scale-x-0 group-hover/cta:scale-x-100 transition-transform duration-500 origin-left"
							></div>
						</a>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	.alert-services-section {
		position: relative;
	}

	/* Floating particles */
	.particle {
		position: absolute;
		border-radius: 50%;
		background: radial-gradient(circle, rgba(251, 191, 36, 0.15), transparent);
		animation: float-particle 15s ease-in-out infinite;
	}

	.particle-1 {
		width: 300px;
		height: 300px;
		top: 10%;
		left: 10%;
		animation-delay: 0s;
	}

	.particle-2 {
		width: 200px;
		height: 200px;
		top: 60%;
		right: 15%;
		animation-delay: 5s;
	}

	.particle-3 {
		width: 250px;
		height: 250px;
		bottom: 20%;
		left: 50%;
		animation-delay: 10s;
	}

	@keyframes float-particle {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
			opacity: 0.3;
		}
		50% {
			transform: translate(30px, -30px) scale(1.1);
			opacity: 0.5;
		}
	}

	/* Bell ringing animation */
	@keyframes ring {
		0%,
		100% {
			transform: rotate(0deg);
		}
		10%,
		30% {
			transform: rotate(-10deg);
		}
		20%,
		40% {
			transform: rotate(10deg);
		}
	}

	.animate-ring {
		animation: ring 2s ease-in-out infinite;
	}

	.group:hover .animate-ring-hover {
		animation: ring 0.5s ease-in-out;
	}

	/* Pulse animation */
	@keyframes pulse-slow {
		0%,
		100% {
			opacity: 0.3;
			transform: scale(1);
		}
		50% {
			opacity: 0.6;
			transform: scale(1.05);
		}
	}

	.animate-pulse-slow {
		animation: pulse-slow 3s ease-in-out infinite;
	}

	/* Slide animations */
	@keyframes slide-in-left {
		from {
			opacity: 0;
			transform: translateX(-100px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	@keyframes slide-in-right {
		from {
			opacity: 0;
			transform: translateX(100px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	.animate-slide-in-left {
		animation: slide-in-left 0.8s ease-out forwards;
		opacity: 0;
	}

	.animate-slide-in-right {
		animation: slide-in-right 0.8s ease-out forwards;
		opacity: 0;
	}
</style>
