<script lang="ts">
	/**
	 * CoursesSection - Apple/Netflix Cinematic Design
	 * Upgraded with ICT9+ Layout, Motion, and Interaction Physics
	 */
	import { onMount, tick } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';

	// Tabler Icons (Preserving sub-path imports for tree-shaking)
	import IconSchool from '@tabler/icons-svelte-runes/icons/school';
	import IconChartCandle from '@tabler/icons-svelte-runes/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconBrain from '@tabler/icons-svelte-runes/icons/brain';
	import IconShield from '@tabler/icons-svelte-runes/icons/shield';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play-filled';
	import IconCertificate from '@tabler/icons-svelte-runes/icons/certificate';
	import IconFlame from '@tabler/icons-svelte-runes/icons/flame';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import type { Attachment } from 'svelte/attachments';

	// ============================================================================
	// COURSE DATA
	// ============================================================================
	const courses = [
		{
			id: 'day-trading',
			title: 'Day Trading Masterclass',
			subtitle: 'From Zero to Profitable',
			description:
				'Master institutional-grade execution, order flow analysis, and real-time risk management.',
			level: 'Intermediate',
			duration: '8 Weeks',
			students: '2,847',
			rating: 4.9,
			reviews: 423,
			price: '$497',
			originalPrice: '$997',
			icon: IconChartCandle,
			gradient: 'from-blue-600 via-blue-500 to-cyan-500',
			backgroundGradient:
				'linear-gradient(to bottom right, rgba(23, 37, 84, 0.5), rgba(8, 51, 68, 0.3))',
			href: '/courses/day-trading-masterclass',
			badge: 'Best Seller',
			badgeColor: 'bg-amber-500',
			features: ['Live Trading Sessions', 'Real-Time Analysis', 'Risk Management'],
			imagePattern:
				'radial-gradient(circle at 10% 20%, rgba(59, 130, 246, 0.1) 0%, transparent 20%)'
		},
		{
			id: 'swing-trading',
			title: 'Swing Trading Pro',
			subtitle: 'Capture Multi-Day Moves',
			description: 'Learn to identify high-probability setups and ride trends for maximum profit.',
			level: 'Beginner',
			duration: '6 Weeks',
			students: '3,421',
			rating: 4.8,
			reviews: 567,
			price: '$397',
			originalPrice: '$797',
			icon: IconChartLine,
			gradient: 'from-emerald-600 via-emerald-500 to-teal-500',
			backgroundGradient:
				'linear-gradient(to bottom right, rgba(2, 44, 34, 0.5), rgba(4, 47, 46, 0.3))',
			href: '/courses/swing-trading-pro',
			badge: 'Popular',
			badgeColor: 'bg-emerald-500',
			features: ['Weekly Setups', 'Trend Analysis', 'Exit Strategies'],
			imagePattern:
				'radial-gradient(circle at 90% 80%, rgba(16, 185, 129, 0.1) 0%, transparent 20%)'
		},
		{
			id: 'options',
			title: 'Options Mastery',
			subtitle: 'Unlock Leverage Safely',
			description: 'Comprehensive options training from Greeks to advanced multi-leg strategies.',
			level: 'Advanced',
			duration: '10 Weeks',
			students: '1,892',
			rating: 4.9,
			reviews: 312,
			price: '$597',
			originalPrice: '$1,197',
			icon: IconBrain,
			gradient: 'from-violet-600 via-purple-500 to-fuchsia-500',
			backgroundGradient:
				'linear-gradient(to bottom right, rgba(46, 16, 101, 0.5), rgba(74, 4, 78, 0.3))',
			href: '/courses/options-trading',
			badge: 'Advanced',
			badgeColor: 'bg-violet-500',
			features: ['Greeks Mastery', 'Spreads & Straddles', 'Volatility Trading'],
			imagePattern:
				'radial-gradient(circle at 50% 50%, rgba(139, 92, 246, 0.1) 0%, transparent 25%)'
		},
		{
			id: 'risk-management',
			title: 'Risk Management',
			subtitle: 'Protect Your Capital',
			description:
				'The foundation of profitable trading. Learn position sizing and drawdown control.',
			level: 'All Levels',
			duration: '4 Weeks',
			students: '4,156',
			rating: 4.9,
			reviews: 689,
			price: '$297',
			originalPrice: '$497',
			icon: IconShield,
			gradient: 'from-amber-600 via-orange-500 to-red-500',
			backgroundGradient:
				'linear-gradient(to bottom right, rgba(69, 26, 3, 0.5), rgba(69, 10, 10, 0.3))',
			href: '/courses/risk-management',
			badge: 'Essential',
			badgeColor: 'bg-red-500',
			features: ['Position Sizing', 'Drawdown Control', 'Psychology'],
			imagePattern:
				'radial-gradient(circle at 0% 100%, rgba(245, 158, 11, 0.1) 0%, transparent 20%)'
		}
	];

	// ============================================================================
	// STATE & REFS
	// ============================================================================
	let sectionRef = $state<HTMLElement | null>(null);
	let cardsRef = $state<HTMLElement | null>(null);
	// January 2026 motion: content is revealed lazily as the section scrolls into view.
	let isVisible = $state(false);
	interface ScrollTriggerLike {
		trigger?: Element;
		kill(): void;
	}
	interface ScrollTriggerStatic {
		getAll(): ScrollTriggerLike[];
		refresh(): void;
	}
	let scrollTriggerInstance: ScrollTriggerStatic | null = null;
	let prefersReducedMotion = $state(false);
	let visibilityObserver: IntersectionObserver | null = null;

	const captureSection: Attachment<HTMLElement> = (node) => {
		sectionRef = node;

		return () => {
			if (sectionRef === node) sectionRef = null;
		};
	};

	const captureCards: Attachment<HTMLElement> = (node) => {
		cardsRef = node;

		return () => {
			if (cardsRef === node) cardsRef = null;
		};
	};

	// January 2026 effect: card grid spotlight follows the cursor across the course cards.
	let mouseX = $state(0);
	let mouseY = $state(0);

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	onMount(() => {
		if (!browser) return;

		// Check for reduced motion preference
		prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

		// Trigger entrance animations when section scrolls into viewport
		queueMicrotask(() => {
			if (sectionRef) {
				const observer = new IntersectionObserver(
					(entries) => {
						if (entries[0]?.isIntersecting) {
							isVisible = true;
							observer.disconnect();
						}
					},
					{ threshold: 0.1, rootMargin: '50px' }
				);
				visibilityObserver = observer;
				visibilityObserver.observe(sectionRef);
			} else {
				isVisible = true;
			}
		});

		// Load GSAP asynchronously
		if (!prefersReducedMotion) {
			loadGSAP();
		}

		return () => {
			visibilityObserver?.disconnect();
			// ICT11+ Fix: Kill only ScrollTriggers scoped to this component
			// Previously used killAll() which destroyed ALL ScrollTriggers globally,
			// causing layout breaks when scrolling to bottom of page
			if (scrollTriggerInstance && (cardsRef || sectionRef)) {
				scrollTriggerInstance.getAll().forEach((st) => {
					if (
						st.trigger === cardsRef ||
						st.trigger === sectionRef ||
						cardsRef?.contains(st.trigger ?? null) ||
						sectionRef?.contains(st.trigger ?? null)
					) {
						st.kill();
					}
				});
			}
		};
	});

	function handleMouseMove(e: MouseEvent) {
		if (cardsRef) {
			const rect = cardsRef.getBoundingClientRect();
			mouseX = e.clientX - rect.left;
			mouseY = e.clientY - rect.top;
		}
	}

	async function loadGSAP() {
		try {
			const gsapModule = await import('gsap');
			const scrollTriggerModule = await import('gsap/ScrollTrigger');
			const gsap = gsapModule.gsap || gsapModule.default;
			const ScrollTrigger = scrollTriggerModule.ScrollTrigger || scrollTriggerModule.default;
			gsap.registerPlugin(ScrollTrigger);
			scrollTriggerInstance = ScrollTrigger;

			// Wait for cards to render
			await tick();

			// Animate cards on scroll with staggered parallax
			const cards = cardsRef?.querySelectorAll('.course-card');
			if (cards && cards.length > 0) {
				// Main Entrance
				gsap.fromTo(
					cards,
					{ y: 100, opacity: 0, scale: 0.95 },
					{
						y: 0,
						opacity: 1,
						scale: 1,
						duration: 1,
						stagger: 0.1,
						ease: 'power3.out', // More cinematic ease
						scrollTrigger: {
							trigger: cardsRef,
							start: 'top 85%',
							toggleActions: 'play none none reverse'
						}
					}
				);

				// fromTo applies the opacity:0 from-state immediately, and
				// ScrollTrigger computes start positions now — before the
				// hero/lazy images and web fonts have loaded. Without a
				// post-load refresh those positions are stale and the
				// trigger can never fire, leaving the cards permanently
				// invisible on a real (image-heavy) marketing page.
				ScrollTrigger.refresh();
				if (document.readyState !== 'complete') {
					window.addEventListener('load', () => scrollTriggerInstance?.refresh(), { once: true });
				}
			}
		} catch (e) {
			console.warn('[CoursesSection] GSAP failed to load:', e);
		}
	}

	// ============================================================================
	// TRANSITIONS
	// ============================================================================
	function slideUp(_node: Element, { delay = 0, duration = 800 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 40}px);`;
			}
		};
	}
</script>

<svelte:window onmousemove={handleMouseMove} />

<section
	{@attach captureSection}
	class="relative py-20 sm:py-32 overflow-hidden bg-[#050812] selection:bg-violet-500/30 selection:text-violet-200"
>
	<div class="absolute inset-0 pointer-events-none">
		<div
			class="absolute inset-0 opacity-[0.03] mix-blend-overlay"
			style="background-image: url('/textures/noise.svg');"
		></div>

		<div
			class="absolute top-[-10%] left-[-10%] w-[800px] h-[800px] bg-violet-600/10 rounded-full blur-[120px] mix-blend-screen animate-pulse-slow"
		></div>
		<div
			class="absolute bottom-[-10%] right-[-10%] w-[800px] h-[800px] bg-cyan-600/10 rounded-full blur-[120px] mix-blend-screen animate-pulse-slow delay-1000"
		></div>

		<div
			class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-size-[60px_60px] mask-[radial-gradient(ellipse_at_center,black_40%,transparent_80%)]"
		></div>
	</div>

	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 z-10">
		{#if isVisible}
			<div class="max-w-4xl mx-auto text-center mb-24" in:slideUp={{ delay: 0, duration: 1000 }}>
				<div
					class="inline-flex items-center gap-3 px-4 py-1.5 border border-violet-900/30 bg-violet-950/10 text-violet-500 text-[10px] font-bold tracking-[0.3em] uppercase mb-8 rounded-sm"
				>
					<IconSchool class="w-4 h-4" />
					Professional Education
				</div>

				<h2 class="text-5xl md:text-7xl font-serif text-white mb-8 tracking-tight">
					Trading <span class="text-slate-700">Curriculum.</span>
				</h2>

				<p class="text-lg text-slate-400 font-light leading-relaxed max-w-2xl mx-auto">
					We don't sell courses. We provide institutional-grade trading education. Verified by
					funded traders and prop firm graduates worldwide.
				</p>

				<div
					class="courses-stats-grid grid grid-cols-2 md:grid-cols-4 gap-6 max-w-4xl mx-auto mt-12 pt-8 border-t border-white/5"
					in:slideUp={{ delay: 200, duration: 800 }}
				>
					<div class="text-center group cursor-default">
						<div
							class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300"
						>
							12k+
						</div>
						<div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">
							Students
						</div>
					</div>
					<div class="text-center group cursor-default">
						<div
							class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300"
						>
							4.9
						</div>
						<div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">
							Rating
						</div>
					</div>
					<div class="text-center group cursor-default">
						<div
							class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300"
						>
							89%
						</div>
						<div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">
							Completion
						</div>
					</div>
					<div class="text-center group cursor-default">
						<div
							class="text-3xl font-bold text-white tabular-nums group-hover:scale-110 transition-transform duration-300"
						>
							24/7
						</div>
						<div class="text-xs font-medium text-zinc-500 uppercase tracking-wider mt-1">
							Support
						</div>
					</div>
				</div>
			</div>
		{/if}

		<div
			{@attach captureCards}
			class="courses-grid grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8 relative"
			style:--mouse-x={`${mouseX}px`}
			style:--mouse-y={`${mouseY}px`}
		>
			<div
				class="pointer-events-none absolute -inset-px opacity-0 md:opacity-100 transition-opacity duration-300 z-0 rounded-3xl"
				style="background: radial-gradient(800px circle at var(--mouse-x) var(--mouse-y), rgba(139, 92, 246, 0.08), transparent 40%);"
			></div>

			{#each courses as course (course.id)}
				<a
					href={course.href}
					class="course-card group relative rounded-4xl overflow-hidden bg-zinc-900/40 border border-white/5 hover:border-white/10 transition-all duration-500 hover:shadow-2xl hover:shadow-violet-900/20 active:scale-[0.99] z-10 isolate"
				>
					<div
						class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-700 ease-in-out"
						style:background={course.imagePattern}
						style:filter="blur(40px)"
					></div>

					<div
						class="absolute inset-0 opacity-50 group-hover:opacity-60 transition-opacity duration-500"
						style:background={course.backgroundGradient}
					></div>

					<div class="relative p-6 sm:p-8 lg:p-10 h-full flex flex-col">
						<div class="flex items-start justify-between gap-4 mb-6">
							<div class="flex items-center gap-4">
								<div
									class="w-14 h-14 rounded-2xl bg-white/5 border border-white/10 flex items-center justify-center shadow-lg backdrop-blur-sm group-hover:scale-110 group-hover:rotate-3 transition-all duration-500 ease-out"
								>
									{#if course.icon}
										{@const IconComponent = course.icon}
										<IconComponent
											class="w-7 h-7 text-white drop-shadow-[0_0_8px_rgba(255,255,255,0.3)]"
										/>
									{/if}
								</div>
								<div>
									<span
										class="inline-flex items-center gap-1.5 text-xs font-bold text-violet-300 uppercase tracking-wider mb-1"
									>
										{#if course.level === 'Advanced'}
											<IconFlame size={12} class="animate-pulse" />
										{:else if course.level === 'Intermediate'}
											<IconActivity size={12} />
										{/if}
										{course.level}
									</span>
									<h3
										class="text-xl sm:text-2xl font-bold text-white tracking-tight leading-none group-hover:text-violet-200 transition-colors"
									>
										{course.title}
									</h3>
								</div>
							</div>

							<span
								class="px-3 py-1 text-[10px] sm:text-xs font-bold rounded-full bg-white/10 text-white backdrop-blur-md border border-white/10 shadow-lg group-hover:bg-white/20 transition-colors"
							>
								{course.badge}
							</span>
						</div>

						<p
							class="text-base text-zinc-400 mb-8 leading-relaxed line-clamp-2 group-hover:text-zinc-300 transition-colors duration-300"
						>
							{course.description}
						</p>

						<div class="flex gap-2 mb-8 overflow-x-auto pb-2 -mx-2 px-2 scrollbar-hide snap-x">
							{#each course.features as feature (feature)}
								<span
									class="snap-start shrink-0 px-3 py-1.5 text-xs font-medium rounded-lg bg-white/5 text-zinc-300 border border-white/5 whitespace-nowrap group-hover:border-white/20 transition-colors"
								>
									{feature}
								</span>
							{/each}
						</div>

						<div class="grow"></div>

						<div
							class="pt-6 border-t border-white/5 flex flex-col sm:flex-row sm:items-end justify-between gap-4"
						>
							<div class="flex flex-col gap-2">
								<div class="flex items-center gap-4 text-xs sm:text-sm text-zinc-500 font-medium">
									<div class="flex items-center gap-1.5">
										<IconClock class="w-4 h-4 text-zinc-600" />
										<span>{course.duration}</span>
									</div>
									<div class="flex items-center gap-1.5">
										<IconUsers class="w-4 h-4 text-zinc-600" />
										<span>{course.students}</span>
									</div>
								</div>
								<div class="flex items-baseline gap-2 mt-1">
									<span class="text-2xl font-bold text-white tracking-tight tabular-nums"
										>{course.price}</span
									>
									<span
										class="text-sm text-zinc-600 line-through decoration-zinc-600/50 tabular-nums"
										>{course.originalPrice}</span
									>
								</div>
							</div>

							<div
								class="flex items-center justify-center gap-2 px-6 py-3 rounded-xl bg-white text-black font-bold text-sm shadow-xl shadow-white/5 group-hover:shadow-white/20 transform group-hover:-translate-y-1 transition-all duration-300"
							>
								<span>Start Learning</span>
								<IconArrowRight class="w-4 h-4" />
							</div>
						</div>
					</div>

					<div
						class="hidden sm:flex absolute inset-0 items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none z-20"
					>
						<div class="absolute inset-0 bg-black/40 backdrop-blur-[2px]"></div>
						<div
							class="relative w-20 h-20 rounded-full bg-white/10 backdrop-blur-xl flex items-center justify-center border border-white/20 shadow-2xl scale-50 group-hover:scale-100 transition-transform duration-500 cubic-bezier(0.34, 1.56, 0.64, 1)"
						>
							<IconPlayerPlay class="w-8 h-8 text-white ml-1 drop-shadow-lg" />
						</div>
					</div>
				</a>
			{/each}
		</div>

		{#if isVisible}
			<div class="text-center mt-20" in:slideUp={{ delay: 400, duration: 800 }}>
				<a
					href="/courses"
					class="group inline-flex items-center justify-center gap-3 px-8 py-4 rounded-full bg-linear-to-r from-violet-600 to-indigo-600 text-white font-semibold shadow-lg shadow-violet-500/25 hover:shadow-violet-500/50 active:scale-[0.98] hover:-translate-y-1 transition-all duration-300 relative overflow-hidden"
				>
					<span
						class="absolute inset-0 bg-linear-to-r from-transparent via-white/20 to-transparent -translate-x-full group-hover:translate-x-full transition-transform duration-700 ease-in-out"
					></span>
					<span class="text-base relative z-10">View Full Curriculum</span>
					<IconArrowRight
						class="w-5 h-5 group-hover:translate-x-1 transition-transform relative z-10"
					/>
				</a>

				<p
					class="mt-6 text-sm text-zinc-500 flex items-center justify-center gap-2 opacity-60 hover:opacity-100 transition-opacity"
				>
					<IconCertificate class="w-4 h-4 text-violet-500" />
					Official certification included with all pathways
				</p>
			</div>
		{/if}
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * COURSESSECTION - 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Utility for hiding scrollbars */
	.scrollbar-hide {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}
	.scrollbar-hide::-webkit-scrollbar {
		display: none;
	}

	/* Custom Pulse for background orbs */
	@keyframes pulse-slow {
		0%,
		100% {
			opacity: 0.5;
			transform: scale(1);
		}
		50% {
			opacity: 0.8;
			transform: scale(1.1);
		}
	}
	.animate-pulse-slow {
		animation: pulse-slow 8s ease-in-out infinite;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SAFE AREA INSETS - iOS/Android notch & gesture areas
	 * ═══════════════════════════════════════════════════════════════════════════ */
	section {
		padding-left: env(safe-area-inset-left);
		padding-right: env(safe-area-inset-right);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TOUCH TARGETS - Minimum 44x44px for all interactive elements
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (hover: none) and (pointer: coarse) {
		:global(.course-card),
		:global(a[href]) {
			min-height: 44px;
		}
	}

	/* Responsive overrides for nested Tailwind utilities. Component <style>
	   rules are unlayered (win over @layer utilities on layer-order); the
	   compound `.course-card .p-6` selector also outspecs single-class
	   utilities. Hardened 2026-04-25 per CSS_ISOLATION_PLAN. */
	@media (max-width: 359px) {
		section {
			padding-top: 3rem;
			padding-bottom: 3rem;
		}

		:global(.course-card) {
			border-radius: 1rem;
		}

		:global(.course-card .p-6),
		:global(.course-card .p-8),
		:global(.course-card .p-10) {
			padding: 1rem;
		}

		:global(.text-5xl),
		:global(.text-7xl) {
			font-size: 1.75rem;
		}
	}

	@media (min-width: 360px) and (max-width: 639px) {
		section {
			padding-top: 4rem;
			padding-bottom: 4rem;
		}

		:global(.course-card .p-6),
		:global(.course-card .p-8) {
			padding: 1.25rem;
		}

		.courses-stats-grid {
			grid-template-columns: 1fr;
		}

		.courses-stats-grid,
		.courses-grid {
			gap: 1rem;
		}

		:global(.text-5xl),
		:global(.text-7xl) {
			font-size: 2rem;
		}

		:global(.text-2xl) {
			font-size: 1.25rem;
		}
	}

	@media (min-width: 640px) and (max-width: 767px) {
		.courses-stats-grid {
			grid-template-columns: 1fr;
		}

		.courses-stats-grid,
		.courses-grid {
			gap: 1.25rem;
		}
	}

	@media (min-width: 768px) and (max-width: 1023px) {
		.courses-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DYNAMIC VIEWPORT HEIGHT - Use dvh for modern mobile browsers
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@supports (min-height: 100dvh) {
		section {
			min-height: auto;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * REDUCED MOTION - Accessibility
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.animate-pulse-slow {
			animation: none;
		}

		:global(.course-card) {
			transition: none;
		}
	}

	/* Landscape mobile — overrides for horizontal orientation. Component
	   `<style>` is unlayered, beats `@layer utilities` on layer-order. */
	@media (max-height: 500px) and (orientation: landscape) {
		section {
			padding-top: 2rem;
			padding-bottom: 2rem;
		}

		:global(.mb-24) {
			margin-bottom: 2rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * FEATURE TAGS - Horizontal scroll on mobile
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 767px) {
		:global(.snap-x) {
			scroll-snap-type: x mandatory;
			-webkit-overflow-scrolling: touch;
		}

		:global(.snap-start) {
			scroll-snap-align: start;
		}
	}
</style>
