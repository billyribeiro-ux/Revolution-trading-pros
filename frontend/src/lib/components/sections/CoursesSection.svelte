<script lang="ts">
	/**
	 * CoursesSection - Apple/Netflix Cinematic Design
	 * Mobile-First + GSAP Animations Fixed
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Features:
	 * ✓ Mobile-first responsive design
	 * ✓ GSAP ScrollTrigger with proper initialization
	 * ✓ Touch-friendly card interactions
	 * ✓ Reduced motion support
	 * ✓ Horizontal scroll on mobile
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy, tick } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut, backOut } from 'svelte/easing';
	import IconSchool from '@tabler/icons-svelte/icons/school';
	import IconChartCandle from '@tabler/icons-svelte/icons/chart-candle';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconBrain from '@tabler/icons-svelte/icons/brain';
	import IconShield from '@tabler/icons-svelte/icons/shield';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconStar from '@tabler/icons-svelte/icons/star-filled';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play-filled';
	import IconCertificate from '@tabler/icons-svelte/icons/certificate';
	import IconFlame from '@tabler/icons-svelte/icons/flame';

	// ============================================================================
	// COURSE DATA
	// ============================================================================
	const courses = [
		{
			id: 'day-trading',
			title: 'Day Trading Masterclass',
			subtitle: 'From Zero to Profitable',
			description: 'Master institutional-grade execution, order flow analysis, and real-time risk management.',
			level: 'Intermediate',
			duration: '8 Weeks',
			students: '2,847',
			rating: 4.9,
			reviews: 423,
			price: '$497',
			originalPrice: '$997',
			icon: IconChartCandle,
			gradient: 'from-blue-600 via-blue-500 to-cyan-500',
			bgGradient: 'from-blue-950/50 to-cyan-950/30',
			href: '/courses/day-trading-masterclass',
			badge: 'Best Seller',
			badgeColor: 'bg-amber-500',
			features: ['Live Trading Sessions', 'Real-Time Analysis', 'Risk Management']
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
			bgGradient: 'from-emerald-950/50 to-teal-950/30',
			href: '/courses/swing-trading-pro',
			badge: 'Popular',
			badgeColor: 'bg-emerald-500',
			features: ['Weekly Setups', 'Trend Analysis', 'Exit Strategies']
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
			bgGradient: 'from-violet-950/50 to-fuchsia-950/30',
			href: '/courses/options-trading',
			badge: 'Advanced',
			badgeColor: 'bg-violet-500',
			features: ['Greeks Mastery', 'Spreads & Straddles', 'Volatility Trading']
		},
		{
			id: 'risk-management',
			title: 'Risk Management',
			subtitle: 'Protect Your Capital',
			description: 'The foundation of profitable trading. Learn position sizing and drawdown control.',
			level: 'All Levels',
			duration: '4 Weeks',
			students: '4,156',
			rating: 4.9,
			reviews: 689,
			price: '$297',
			originalPrice: '$497',
			icon: IconShield,
			gradient: 'from-amber-600 via-orange-500 to-red-500',
			bgGradient: 'from-amber-950/50 to-red-950/30',
			href: '/courses/risk-management',
			badge: 'Essential',
			badgeColor: 'bg-red-500',
			features: ['Position Sizing', 'Drawdown Control', 'Psychology']
		}
	];

	// ============================================================================
	// STATE & REFS
	// ============================================================================
	let sectionRef = $state<HTMLElement | null>(null);
	let cardsRef = $state<HTMLElement | null>(null);
	let isVisible = $state(false);
	let hoveredCard = $state<string | null>(null);
	let gsapInstance: any = null;
	let scrollTriggerInstance: any = null;
	let prefersReducedMotion = $state(false);

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	let observer: IntersectionObserver | null = null;

	onMount(() => {
		if (!browser) return;

		// Check for reduced motion preference
		prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

		// Intersection Observer - lower threshold for mobile
		observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					isVisible = true;
					observer?.disconnect();
				}
			},
			{ threshold: 0.1, rootMargin: '50px' }
		);
		if (sectionRef) observer.observe(sectionRef);

		// Load GSAP asynchronously
		if (!prefersReducedMotion) {
			loadGSAP();
		}

		return () => {
			observer?.disconnect();
		};
	});

	async function loadGSAP() {
		try {
			const gsap = (await import('gsap')).default;
			const ScrollTrigger = (await import('gsap/ScrollTrigger')).default;
			gsap.registerPlugin(ScrollTrigger);
			gsapInstance = gsap;
			scrollTriggerInstance = ScrollTrigger;

			// Wait for cards to render
			await tick();

			// Animate cards on scroll
			const cards = cardsRef?.querySelectorAll('.course-card');
			if (cards && cards.length > 0) {
				gsap.fromTo(
					cards,
					{ y: 60, opacity: 0 },
					{
						y: 0,
						opacity: 1,
						duration: 0.8,
						stagger: 0.12,
						ease: 'power2.out',
						scrollTrigger: {
							trigger: cardsRef,
							start: 'top 80%',
							toggleActions: 'play none none reverse'
						}
					}
				);
			}
		} catch (e) {
			console.debug('[CoursesSection] GSAP not available:', e);
		}
	}

	onDestroy(() => {
		if (scrollTriggerInstance) scrollTriggerInstance.killAll();
	});

	// ============================================================================
	// TRANSITIONS
	// ============================================================================
	function slideUp(node: Element, { delay = 0, duration = 800 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 50}px);`;
			}
		};
	}

	function scaleIn(node: Element, { delay = 0, duration = 600 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = backOut(t);
				return `opacity: ${t}; transform: scale(${0.9 + eased * 0.1});`;
			}
		};
	}

	// ============================================================================
	// HELPERS
	// ============================================================================
	function renderStars(rating: number) {
		return Array(5)
			.fill(0)
			.map((_, i) => i < Math.floor(rating));
	}
</script>

<section
	bind:this={sectionRef}
	class="relative py-16 sm:py-24 lg:py-32 overflow-hidden bg-gradient-to-b from-zinc-950 via-black to-zinc-950"
>
	<!-- Cinematic Background -->
	<div class="absolute inset-0 pointer-events-none">
		<!-- Spotlight effects - smaller on mobile -->
		<div
			class="absolute top-0 left-1/4 w-[300px] sm:w-[600px] h-[300px] sm:h-[600px] bg-violet-500/5 rounded-full blur-[100px] sm:blur-[150px]"
		></div>
		<div
			class="absolute bottom-0 right-1/4 w-[250px] sm:w-[500px] h-[250px] sm:h-[500px] bg-emerald-500/5 rounded-full blur-[100px] sm:blur-[150px]"
		></div>

		<!-- Subtle grid - larger on mobile for performance -->
		<div
			class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.01)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.01)_1px,transparent_1px)] bg-[size:40px_40px] sm:bg-[size:80px_80px]"
		></div>
	</div>

	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<!-- Section Header -->
		{#if isVisible}
			<div class="text-center mb-10 sm:mb-16 lg:mb-20" in:slideUp={{ delay: 0, duration: prefersReducedMotion ? 0 : 800 }}>
				<div
					class="inline-flex items-center gap-2 px-3 sm:px-4 py-1.5 sm:py-2 rounded-full bg-violet-500/10 border border-violet-500/20 mb-4 sm:mb-6"
				>
					<IconSchool class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-violet-400" />
					<span class="text-xs sm:text-sm font-medium text-violet-400 tracking-wide">Trading Education</span>
				</div>

				<h2 class="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold text-white mb-4 sm:mb-6 tracking-tight">
					Master the Markets
					<span
						class="block mt-1 sm:mt-2 bg-gradient-to-r from-violet-400 via-fuchsia-400 to-pink-400 bg-clip-text text-transparent"
					>
						With Expert-Led Courses
					</span>
				</h2>

				<p class="text-base sm:text-lg lg:text-xl text-zinc-400 max-w-2xl mx-auto leading-relaxed px-4 sm:px-0">
					Structured curriculum designed by profitable traders. Learn at your pace with lifetime
					access and live support.
				</p>

				<!-- Stats row - 2x2 grid on mobile -->
				<div
					class="grid grid-cols-2 sm:flex sm:flex-wrap justify-center gap-4 sm:gap-8 mt-8 sm:mt-10 pt-8 sm:pt-10 border-t border-zinc-800/50"
					in:slideUp={{ delay: prefersReducedMotion ? 0 : 200, duration: prefersReducedMotion ? 0 : 600 }}
				>
					<div class="text-center">
						<div class="text-2xl sm:text-3xl font-bold text-white">12,000+</div>
						<div class="text-xs sm:text-sm text-zinc-500">Students Enrolled</div>
					</div>
					<div class="text-center">
						<div class="text-2xl sm:text-3xl font-bold text-white">4.9</div>
						<div class="text-xs sm:text-sm text-zinc-500">Average Rating</div>
					</div>
					<div class="text-center">
						<div class="text-2xl sm:text-3xl font-bold text-white">89%</div>
						<div class="text-xs sm:text-sm text-zinc-500">Completion Rate</div>
					</div>
					<div class="text-center">
						<div class="text-2xl sm:text-3xl font-bold text-white">24/7</div>
						<div class="text-xs sm:text-sm text-zinc-500">Community Access</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Course Cards Grid - Single column on mobile -->
		<div
			bind:this={cardsRef}
			class="grid grid-cols-1 md:grid-cols-2 gap-4 sm:gap-6 lg:gap-8"
		>
			{#each courses as course, i}
				<a
					href={course.href}
					class="course-card group relative rounded-2xl sm:rounded-3xl overflow-hidden bg-gradient-to-br {course.bgGradient} border border-zinc-800/50 hover:border-zinc-700/50 transition-all duration-300 hover:shadow-2xl hover:shadow-violet-500/10 active:scale-[0.98]"
					onmouseenter={() => (hoveredCard = course.id)}
					onmouseleave={() => (hoveredCard = null)}
					ontouchstart={() => (hoveredCard = course.id)}
				>
					<!-- Card gradient overlay -->
					<div
						class="absolute inset-0 bg-gradient-to-br {course.gradient} opacity-0 group-hover:opacity-5 transition-opacity duration-300"
					></div>

					<!-- Content -->
					<div class="relative p-4 sm:p-6 lg:p-8">
						<!-- Header -->
						<div class="flex items-start justify-between gap-3 mb-4 sm:mb-6">
							<div class="flex items-center gap-3 sm:gap-4">
								<!-- Icon -->
								<div
									class="w-11 h-11 sm:w-14 sm:h-14 rounded-xl sm:rounded-2xl bg-gradient-to-br {course.gradient} flex items-center justify-center shadow-lg group-hover:scale-110 group-hover:rotate-3 transition-all duration-300"
								>
									{#if course.icon}
										{@const IconComponent = course.icon}
										<IconComponent class="w-5 h-5 sm:w-7 sm:h-7 text-white" />
									{/if}
								</div>
								<div class="min-w-0">
									<span class="text-[10px] sm:text-xs font-medium text-zinc-500 uppercase tracking-wider"
										>{course.level}</span
									>
									<h3
										class="text-lg sm:text-xl lg:text-2xl font-bold text-white truncate"
									>
										{course.title}
									</h3>
								</div>
							</div>

							<!-- Badge -->
							<span
								class="flex-shrink-0 px-2 sm:px-3 py-0.5 sm:py-1 text-[10px] sm:text-xs font-bold rounded-full {course.badgeColor} text-white shadow-lg"
							>
								{course.badge}
							</span>
						</div>

						<!-- Description -->
						<p class="text-sm sm:text-base text-zinc-400 mb-4 sm:mb-6 leading-relaxed line-clamp-2 sm:line-clamp-none">{course.description}</p>

						<!-- Features - Horizontal scroll on mobile -->
						<div class="flex gap-2 mb-4 sm:mb-6 overflow-x-auto pb-1 -mx-1 px-1 scrollbar-hide">
							{#each course.features as feature}
								<span
									class="flex-shrink-0 px-2.5 sm:px-3 py-1 text-[10px] sm:text-xs font-medium rounded-full bg-zinc-800/50 text-zinc-300 border border-zinc-700/50"
								>
									{feature}
								</span>
							{/each}
						</div>

						<!-- Meta info - Stack on very small screens -->
						<div class="flex flex-wrap items-center gap-3 sm:gap-6 text-xs sm:text-sm text-zinc-500 mb-4 sm:mb-6">
							<div class="flex items-center gap-1 sm:gap-1.5">
								<IconClock class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
								<span>{course.duration}</span>
							</div>
							<div class="flex items-center gap-1 sm:gap-1.5">
								<IconUsers class="w-3.5 h-3.5 sm:w-4 sm:h-4" />
								<span>{course.students}</span>
							</div>
							<div class="flex items-center gap-1 sm:gap-1.5">
								<IconStar class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-amber-400" />
								<span class="text-white font-medium">{course.rating}</span>
								<span class="hidden sm:inline">({course.reviews})</span>
							</div>
						</div>

						<!-- Footer -->
						<div
							class="flex items-center justify-between pt-4 sm:pt-6 border-t border-zinc-800/50"
						>
							<div class="flex items-baseline gap-1.5 sm:gap-2">
								<span class="text-xl sm:text-2xl font-bold text-white">{course.price}</span>
								<span class="text-xs sm:text-sm text-zinc-500 line-through">{course.originalPrice}</span>
							</div>

							<div
								class="flex items-center gap-1.5 sm:gap-2 px-3 sm:px-5 py-2 sm:py-2.5 rounded-full bg-white/5 border border-white/10 group-hover:bg-white/10 transition-all duration-300"
							>
								<span class="text-xs sm:text-sm font-semibold text-white">Enroll</span>
								<IconArrowRight
									class="w-3.5 h-3.5 sm:w-4 sm:h-4 text-white group-hover:translate-x-1 transition-transform"
								/>
							</div>
						</div>
					</div>

					<!-- Play button overlay (hidden on mobile for better touch UX) -->
					<div
						class="hidden sm:flex absolute inset-0 items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"
					>
						<div
							class="w-16 sm:w-20 h-16 sm:h-20 rounded-full bg-white/10 backdrop-blur-xl flex items-center justify-center border border-white/20 shadow-2xl"
						>
							<IconPlayerPlay class="w-6 h-6 sm:w-8 sm:h-8 text-white ml-1" />
						</div>
					</div>
				</a>
			{/each}
		</div>

		<!-- Bottom CTA -->
		{#if isVisible}
			<div class="text-center mt-10 sm:mt-16" in:slideUp={{ delay: prefersReducedMotion ? 0 : 400, duration: prefersReducedMotion ? 0 : 600 }}>
				<a
					href="/courses"
					class="group inline-flex items-center justify-center gap-2 sm:gap-3 w-full sm:w-auto px-6 sm:px-8 py-3.5 sm:py-4 rounded-full bg-gradient-to-r from-violet-600 to-fuchsia-600 text-white font-semibold shadow-lg shadow-violet-500/25 hover:shadow-violet-500/40 active:scale-[0.98] hover:scale-105 transition-all duration-300"
				>
					<span class="text-sm sm:text-base">View All Courses</span>
					<IconArrowRight class="w-4 h-4 sm:w-5 sm:h-5 group-hover:translate-x-1 transition-transform" />
				</a>

				<p class="mt-4 sm:mt-6 text-xs sm:text-sm text-zinc-500">
					<IconCertificate class="w-3.5 h-3.5 sm:w-4 sm:h-4 inline mr-1" />
					All courses include certificate of completion
				</p>
			</div>
		{/if}
	</div>
</section>

<style>
	/* Hide scrollbar for horizontal scroll on mobile */
	.scrollbar-hide {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}
	.scrollbar-hide::-webkit-scrollbar {
		display: none;
	}
</style>
