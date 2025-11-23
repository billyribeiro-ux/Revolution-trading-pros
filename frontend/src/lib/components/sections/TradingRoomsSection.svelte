<script lang="ts">
	import { IconChartCandle, IconTrendingUp, IconShieldCheck } from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	const roomCards = [
		{
			title: 'Day Trading Room',
			description:
				'Active market sessions with real-time level identification, trade execution process, and risk management. Professional structure, zero chaos.',
			features: [
				'Live market analysis',
				'Clear entry/exit levels',
				'Risk-first execution',
				'Daily recap & review'
			],
			href: '/live-trading-rooms/day-trading',
			icon: IconChartCandle,
			gradient: 'from-purple-500 via-violet-500 to-indigo-500',
			accentColor: 'purple'
		},
		{
			title: 'Swing Trading Room',
			description:
				'Multi-day setups with institutional positioning analysis. Learn to hold winners and manage overnight risk professionally.',
			features: [
				'Multi-timeframe analysis',
				'Position sizing guidance',
				'Exit strategy development',
				'Weekly market review'
			],
			href: '/live-trading-rooms/swing-trading',
			icon: IconTrendingUp,
			gradient: 'from-violet-500 via-purple-500 to-fuchsia-500',
			accentColor: 'violet'
		},
		{
			title: 'Small Accounts Room',
			description:
				'Structured approach for accounts under $25K. Focus on risk management, consistency, and building capital systematically.',
			features: [
				'Capital preservation focus',
				'Realistic expectations',
				'Scaling strategies',
				'Community support'
			],
			href: '/live-trading-rooms/small-accounts',
			icon: IconShieldCheck,
			gradient: 'from-indigo-500 via-blue-500 to-purple-500',
			accentColor: 'indigo'
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
	class="trading-rooms-section relative py-24 px-4 sm:px-6 lg:px-8 overflow-hidden"
>
	<!-- Background with grid pattern -->
	<div class="absolute inset-0 bg-gradient-to-b from-indigo-950 via-purple-950 to-slate-950"></div>

	<!-- Animated grid -->
	<div class="absolute inset-0 bg-grid-pattern opacity-10"></div>

	<!-- Radial gradient overlay -->
	<div class="absolute inset-0 bg-radial-purple"></div>

	<div class="relative max-w-7xl mx-auto z-10">
		<div class="text-center max-w-3xl mx-auto mb-20">
			<div
				class="inline-block mb-4 px-4 py-2 bg-purple-500/10 border border-purple-500/30 rounded-full"
			>
				<span class="text-sm font-semibold text-purple-300">PROFESSIONAL SERVICES</span>
			</div>
			<h2
				class="text-4xl sm:text-5xl font-heading font-bold mb-6 bg-gradient-to-r from-purple-300 via-violet-300 to-indigo-300 bg-clip-text text-transparent"
			>
				Live Trading Rooms
			</h2>
			<p class="text-lg text-slate-300 leading-relaxed">
				Join structured trading sessions where process beats prediction. Learn real-time execution,
				risk management, and market analysis from experienced professionals.
			</p>
		</div>

		<div class="grid md:grid-cols-3 gap-8">
			{#each roomCards as room, i}
				<div
					class="trading-card group perspective-1000"
					class:animate-card-reveal={isVisible}
					style="animation-delay: {i * 200}ms"
				>
					<div
						class="card-inner relative bg-slate-900/60 backdrop-blur-xl border border-purple-500/20 rounded-3xl p-8 transform-gpu hover:scale-105 transition-all duration-500 hover:shadow-2xl hover:shadow-purple-500/20"
					>
						<!-- 3D Depth Layer -->
						<div
							class="absolute inset-0 bg-gradient-to-br {room.gradient} opacity-0 group-hover:opacity-10 blur-2xl transition-opacity duration-500 rounded-3xl"
						></div>

						<!-- Shine effect -->
						<div
							class="absolute inset-0 bg-gradient-to-tr from-transparent via-white/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-700 rounded-3xl"
						></div>

						<div class="relative z-10">
							<!-- Icon with 3D effect -->
							<div class="mb-6 relative">
								<div
									class="absolute inset-0 bg-gradient-to-r {room.gradient} blur-xl opacity-50 group-hover:opacity-100 transition-opacity duration-500"
								></div>
								<div
									class="relative w-20 h-20 rounded-2xl bg-gradient-to-br {room.gradient} p-0.5 transform group-hover:rotate-12 transition-transform duration-500"
								>
									<div
										class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center"
									>
										<svelte:component this={room.icon} size={40} class="text-white" />
									</div>
								</div>
							</div>

							<h3
								class="text-2xl font-heading font-bold mb-4 text-white group-hover:bg-gradient-to-r group-hover:{room.gradient} group-hover:bg-clip-text group-hover:text-transparent transition-all duration-300"
							>
								{room.title}
							</h3>
							<p
								class="text-slate-400 mb-6 leading-relaxed group-hover:text-slate-300 transition-colors duration-300"
							>
								{room.description}
							</p>

							<!-- Features list with staggered animation -->
							<ul class="space-y-3 mb-8">
								{#each room.features as feature, j}
									<li
										class="flex items-start gap-3 text-sm text-slate-300 transform translate-x-0 group-hover:translate-x-2 transition-transform duration-300"
										style="transition-delay: {j * 50}ms"
									>
										<div class="flex-shrink-0 mt-0.5">
											<div class="w-6 h-6 rounded-lg bg-gradient-to-br {room.gradient} p-0.5">
												<div
													class="w-full h-full bg-slate-900 rounded-lg flex items-center justify-center"
												>
													<svg class="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
														<path
															fill-rule="evenodd"
															d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
															clip-rule="evenodd"
														/>
													</svg>
												</div>
											</div>
										</div>
										<span class="leading-relaxed">{feature}</span>
									</li>
								{/each}
							</ul>

							<!-- CTA Button with gradient -->
							<a
								href={room.href}
								class="block w-full text-center px-6 py-4 bg-gradient-to-r {room.gradient} text-white font-heading font-bold rounded-xl hover:shadow-lg hover:shadow-purple-500/50 transform hover:-translate-y-1 transition-all duration-300 relative overflow-hidden group/button"
							>
								<span class="relative z-10">Learn More</span>
								<div
									class="absolute inset-0 bg-white/20 transform scale-x-0 group-hover/button:scale-x-100 transition-transform duration-500 origin-left"
								></div>
							</a>
						</div>

						<!-- Corner accents -->
						<div
							class="absolute top-0 right-0 w-20 h-20 bg-gradient-to-bl {room.gradient} opacity-10 rounded-tr-3xl"
						></div>
						<div
							class="absolute bottom-0 left-0 w-20 h-20 bg-gradient-to-tr {room.gradient} opacity-10 rounded-bl-3xl"
						></div>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	.trading-rooms-section {
		position: relative;
	}

	/* Grid pattern */
	.bg-grid-pattern {
		background-image:
			linear-gradient(rgba(139, 92, 246, 0.1) 1px, transparent 1px),
			linear-gradient(90deg, rgba(139, 92, 246, 0.1) 1px, transparent 1px);
		background-size: 50px 50px;
		animation: grid-move 20s linear infinite;
	}

	@keyframes grid-move {
		0% {
			transform: translate(0, 0);
		}
		100% {
			transform: translate(50px, 50px);
		}
	}

	/* Radial gradient */
	.bg-radial-purple {
		background: radial-gradient(circle at 50% 50%, rgba(139, 92, 246, 0.1) 0%, transparent 70%);
	}

	/* 3D Perspective */
	.perspective-1000 {
		perspective: 1000px;
	}

	/* Card reveal animation */
	@keyframes card-reveal {
		from {
			opacity: 0;
			transform: translateY(60px) rotateX(-15deg);
		}
		to {
			opacity: 1;
			transform: translateY(0) rotateX(0);
		}
	}

	.animate-card-reveal {
		animation: card-reveal 0.8s ease-out forwards;
		opacity: 0;
	}

	/* 3D Card Transform */
	.trading-card:hover .card-inner {
		transform: translateY(-10px) rotateY(5deg) rotateX(5deg);
	}

	/* Shine effect animation */
	@keyframes shine {
		0% {
			transform: translateX(-100%) rotate(45deg);
		}
		100% {
			transform: translateX(100%) rotate(45deg);
		}
	}

	.trading-card:hover .card-inner::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
		animation: shine 1.5s ease-in-out;
	}
</style>
