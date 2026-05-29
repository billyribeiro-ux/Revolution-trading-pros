<script lang="ts">
	/**
	 * Live Trading Rooms — Rooms Grid
	 * Three-card grid (Day / Swing / Small Accounts).
	 * Pricing is per-room (data.rooms[*].price), distinct for each service.
	 * Extracted from /live-trading-rooms/+page.svelte (Cascade audit, May 2026).
	 */
	import { Spring } from 'svelte/motion';

	interface PricePlan {
		monthly: number;
		quarterly: number;
		annual: number;
	}

	interface TradingRoom {
		id: string;
		iconType: 'volatility' | 'trend' | 'growth';
		name: string;
		tagline: string;
		description: string;
		liveCount: number;
		features: string[];
		price: PricePlan;
		accent: 'cyan' | 'emerald' | 'amber';
		badge: string;
	}

	interface Props {
		rooms: TradingRoom[];
	}
	let { rooms }: Props = $props();

	/** Action: 3D Tilt Effect (mouse-tracking parallax) — Svelte 5 Spring class */
	function tilt(node: HTMLElement) {
		const x = new Spring(0, { stiffness: 0.05, damping: 0.25 });
		const y = new Spring(0, { stiffness: 0.05, damping: 0.25 });

		const stopEffect = $effect.root(() => {
			$effect(() => {
				node.style.setProperty('--rotX', `${x.current}deg`);
				node.style.setProperty('--rotY', `${y.current}deg`);
			});
		});

		function handleMove(e: MouseEvent) {
			const rect = node.getBoundingClientRect();
			x.target = ((e.clientY - rect.top - rect.height / 2) / rect.height) * -3;
			y.target = ((e.clientX - rect.left - rect.width / 2) / rect.width) * 3;
		}

		function handleLeave() {
			x.target = 0;
			y.target = 0;
		}

		node.addEventListener('mousemove', handleMove);
		node.addEventListener('mouseleave', handleLeave);

		return {
			destroy() {
				node.removeEventListener('mousemove', handleMove);
				node.removeEventListener('mouseleave', handleLeave);
				stopEffect();
			}
		};
	}
</script>

<div
	id="rooms-section"
	class="grid grid-cols-1 md:grid-cols-3 gap-6 lg:gap-8 perspective-container mb-32"
>
	{#each rooms as room (room.id)}
		<article use:tilt class="group relative h-full card-3d" role="region" aria-label={room.name}>
			<div
				class={`absolute -inset-px rounded-3xl bg-linear-to-b opacity-0 group-hover:opacity-100 transition-opacity duration-500 blur-sm 
                    ${room.accent === 'cyan' ? 'from-cyan-500/50' : ''}
                    ${room.accent === 'emerald' ? 'from-emerald-500/50' : ''}
                    ${room.accent === 'amber' ? 'from-amber-500/50' : ''}
                    to-transparent`}
			></div>
			<div
				class="relative h-full flex flex-col bg-[#0A0A0A] border border-white/5 hover:border-white/10 rounded-3xl p-1 shadow-2xl overflow-hidden transition-colors duration-300"
			>
				<div
					class="flex-1 flex flex-col p-6 lg:p-8 rounded-[20px] bg-linear-to-b from-white/2 to-transparent"
				>
					{#if room.badge}
						<div class="absolute top-6 right-6">
							<span
								class={`inline-block px-2 py-1 rounded text-[10px] font-bold uppercase tracking-wider border
                                    ${room.accent === 'cyan' ? 'text-cyan-300 bg-cyan-500/10 border-cyan-500/20' : ''}
                                    ${room.accent === 'emerald' ? 'text-emerald-300 bg-emerald-500/10 border-emerald-500/20' : ''}
                                    ${room.accent === 'amber' ? 'text-amber-300 bg-amber-500/10 border-amber-500/20' : ''}
                                `}>{room.badge}</span
							>
						</div>
					{/if}
					<div
						class={`w-14 h-14 rounded-xl flex items-center justify-center mb-6 transition-all duration-300 ease-out border border-white/5
                            ${room.accent === 'cyan' ? 'bg-cyan-500/5 text-cyan-400' : ''}
                            ${room.accent === 'emerald' ? 'bg-emerald-500/5 text-emerald-400' : ''}
                            ${room.accent === 'amber' ? 'bg-amber-500/5 text-amber-400' : ''}
                        `}
					>
						{#if room.iconType === 'volatility'}
							<svg
								class="w-8 h-8"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="1.5"
							>
								<path class="group-hover:animate-candle-1" d="M6 6v12" stroke-linecap="round" />
								<rect
									class="group-hover:animate-candle-body-1"
									x="4"
									y="9"
									width="4"
									height="6"
									fill="currentColor"
									fill-opacity="0.2"
								/>
								<path class="group-hover:animate-candle-2" d="M12 4v16" stroke-linecap="round" />
								<rect
									class="group-hover:animate-candle-body-2"
									x="10"
									y="7"
									width="4"
									height="10"
									fill="currentColor"
									fill-opacity="0.2"
								/>
								<path class="group-hover:animate-candle-3" d="M18 8v8" stroke-linecap="round" />
								<rect
									class="group-hover:animate-candle-body-3"
									x="16"
									y="10"
									width="4"
									height="4"
									fill="currentColor"
									fill-opacity="0.2"
								/>
							</svg>
						{:else if room.iconType === 'trend'}
							<svg
								class="w-8 h-8"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="1.5"
							>
								<path
									class="group-hover:animate-draw-line"
									stroke-dasharray="30"
									stroke-dashoffset="30"
									d="M3 12c0-3 3-6 6-6s6 3 6 6 3 6 6 6"
									stroke-linecap="round"
								/>
								<circle
									cx="21"
									cy="18"
									r="2"
									fill="currentColor"
									class="opacity-0 group-hover:opacity-100 transition-opacity delay-300"
								/>
							</svg>
						{:else if room.iconType === 'growth'}
							<svg
								class="w-8 h-8"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="1.5"
							>
								<path d="M3 20h18" stroke-linecap="round" />
								<path class="group-hover:animate-grow-1" d="M5 20v-4" stroke-linecap="round" />
								<path class="group-hover:animate-grow-2" d="M9 20v-8" stroke-linecap="round" />
								<path class="group-hover:animate-grow-3" d="M13 20v-12" stroke-linecap="round" />
								<path class="group-hover:animate-grow-4" d="M17 20v-16" stroke-linecap="round" />
								<path d="M17 4l2 2" stroke-linecap="round" />
							</svg>
						{/if}
					</div>
					<h2 class="text-2xl font-bold text-white mb-2">{room.name}</h2>
					<p class="text-sm font-medium text-zinc-400 mb-4">{room.tagline}</p>
					<div class="flex items-center gap-2 mb-6 text-xs font-mono text-zinc-500">
						<span class="relative flex h-2 w-2">
							<span
								class={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75
                                    ${room.accent === 'cyan' ? 'bg-cyan-400' : ''}
                                    ${room.accent === 'emerald' ? 'bg-emerald-400' : ''}
                                    ${room.accent === 'amber' ? 'bg-amber-400' : ''}
                                `}
							></span>
							<span
								class={`relative inline-flex rounded-full h-2 w-2 
                                    ${room.accent === 'cyan' ? 'bg-cyan-500' : ''}
                                    ${room.accent === 'emerald' ? 'bg-emerald-500' : ''}
                                    ${room.accent === 'amber' ? 'bg-amber-500' : ''}
                                `}
							></span>
						</span>
						{room.liveCount} Traders Online
					</div>
					<p class="text-sm leading-relaxed text-zinc-400 mb-8 border-t border-white/5 pt-6">
						{room.description}
					</p>
					<ul class="space-y-3 mb-8 flex-1">
						{#each room.features as feature (feature)}
							<li class="flex items-start gap-3 text-sm text-zinc-300">
								<svg
									class={`w-4 h-4 shrink-0 mt-[3px] 
                                        ${room.accent === 'cyan' ? 'text-cyan-500/80' : ''}
                                        ${room.accent === 'emerald' ? 'text-emerald-500/80' : ''}
                                        ${room.accent === 'amber' ? 'text-amber-500/80' : ''}
                                    `}
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2.5"
									stroke-linecap="round"
									stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg
								>
								{feature}
							</li>
						{/each}
					</ul>
					<div class="mt-auto">
						<div class="flex flex-col gap-1 mb-6">
							<div class="flex items-end gap-1">
								<span class="text-3xl font-bold text-white">${room.price.monthly}</span>
								<span class="text-zinc-500 text-sm mb-1">/ month</span>
							</div>
							<div class="flex items-center justify-between text-xs text-zinc-500 mt-2">
								<span>Annual: ${room.price.annual}/yr</span>
								<span class="text-emerald-400 font-medium">Save 36%</span>
							</div>
						</div>
						<a
							href="/live-trading-rooms/{room.id}"
							class={`group/btn relative w-full flex items-center justify-center gap-2 py-4 rounded-xl text-black font-bold text-sm transition-all duration-300 overflow-hidden
                                    ${room.accent === 'cyan' ? 'bg-white hover:bg-cyan-400' : ''}
                                    ${room.accent === 'emerald' ? 'bg-white hover:bg-emerald-400' : ''}
                                    ${room.accent === 'amber' ? 'bg-white hover:bg-amber-400' : ''}
                                    hover:shadow-[0_0_20px_rgba(255,255,255,0.3)]
                                `}
						>
							<span class="relative z-10">Access Room</span>
							<svg
								class="w-4 h-4 relative z-10 transition-transform group-hover/btn:translate-x-1"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"><path d="M5 12h14m-7-7l7 7-7 7" /></svg
							>
						</a>
					</div>
				</div>
			</div>
		</article>
	{/each}
</div>

<style>
	/* --- 3D Transform Container --- */
	.card-3d {
		transform-style: preserve-3d;
		transform: perspective(1000px) rotateX(var(--rotX, 0deg)) rotateY(var(--rotY, 0deg));
		will-change: transform;
	}
	.perspective-container {
		perspective: 2000px;
	}

	/* --- Draw-line keyframe used inside the trend room icon (Tailwind group-hover variant) --- */
	@keyframes draw-line {
		to {
			stroke-dashoffset: 0;
		}
	}
	:global(.animate-draw-line) {
		animation: draw-line 1s ease-out forwards;
	}

	/* --- Candle Animations (volatility room icon) --- */
	@keyframes candle-up {
		0%,
		100% {
			transform: scaleY(1);
		}
		50% {
			transform: scaleY(1.2);
		}
	}
	:global(.animate-candle-body-1) {
		transform-origin: bottom;
		animation: candle-up 2s infinite ease-in-out;
	}
	:global(.animate-candle-body-2) {
		transform-origin: bottom;
		animation: candle-up 3s infinite ease-in-out 0.5s;
	}
	:global(.animate-candle-body-3) {
		transform-origin: bottom;
		animation: candle-up 2.5s infinite ease-in-out 0.2s;
	}

	/* --- Growth Animations (growth room icon) --- */
	@keyframes grow-up {
		from {
			transform: scaleY(0);
		}
		to {
			transform: scaleY(1);
		}
	}
	:global(.animate-grow-1) {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out forwards;
	}
	:global(.animate-grow-2) {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out 0.1s forwards;
	}
	:global(.animate-grow-3) {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out 0.2s forwards;
	}
	:global(.animate-grow-4) {
		transform-origin: bottom;
		animation: grow-up 0.4s ease-out 0.3s forwards;
	}
</style>
