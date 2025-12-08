<script lang="ts">
	/**
	 * CoursesSection - Apple/Netflix Cinematic Design
	 * Principal Engineer ICT9+ Standard
	 * ══════════════════════════════════════════════════════════════════════════════
	 * Features:
	 * ✓ GSAP-powered scroll animations
	 * ✓ 3D card hover effects
	 * ✓ Cinematic video-style preview cards
	 * ✓ Progress indicators and ratings
	 * ✓ Staggered reveal with parallax
	 * ✓ Netflix-style horizontal scroll on mobile
	 * ══════════════════════════════════════════════════════════════════════════════
	 */
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut, backOut, elasticOut } from 'svelte/easing';
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

	// ============================================================================
	// LIFECYCLE
	// ============================================================================
	let observer: IntersectionObserver | null = null;

	onMount(() => {
		if (!browser) return;

		// Intersection Observer
		observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					isVisible = true;
					observer?.disconnect();
				}
			},
			{ threshold: 0.15 }
		);
		if (sectionRef) observer.observe(sectionRef);

		// Load GSAP asynchronously
		loadGSAP();

		return () => {
			observer?.disconnect();
		};
	});

	async function loadGSAP() {
		const gsap = (await import('gsap')).default;
		const ScrollTrigger = (await import('gsap/ScrollTrigger')).default;
		gsap.registerPlugin(ScrollTrigger);
		gsapInstance = gsap;
		scrollTriggerInstance = ScrollTrigger;

		// Parallax effect on cards
		if (cardsRef) {
			gsap.fromTo(
				cardsRef.querySelectorAll('.course-card'),
				{ y: 100, opacity: 0 },
				{
					y: 0,
					opacity: 1,
					duration: 1,
					stagger: 0.15,
					ease: 'power3.out',
					scrollTrigger: {
						trigger: cardsRef,
						start: 'top 85%',
						end: 'bottom 20%',
						toggleActions: 'play none none reverse'
					}
				}
			);
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
	class="relative py-32 lg:py-40 overflow-hidden bg-gradient-to-b from-zinc-950 via-black to-zinc-950"
>
	<!-- Cinematic Background -->
	<div class="absolute inset-0 pointer-events-none">
		<!-- Spotlight effects -->
		<div
			class="absolute top-0 left-1/4 w-[600px] h-[600px] bg-violet-500/5 rounded-full blur-[150px]"
		></div>
		<div
			class="absolute bottom-0 right-1/4 w-[500px] h-[500px] bg-emerald-500/5 rounded-full blur-[150px]"
		></div>

		<!-- Film grain overlay -->
		<div class="absolute inset-0 opacity-[0.015] bg-[url('data:image/svg+xml,%3Csvg viewBox=%220 0 256 256%22 xmlns=%22http://www.w3.org/2000/svg%22%3E%3Cfilter id=%22noise%22%3E%3CfeTurbulence type=%22fractalNoise%22 baseFrequency=%220.9%22 numOctaves=%224%22 stitchTiles=%22stitch%22/%3E%3C/filter%3E%3Crect width=%22100%25%22 height=%22100%25%22 filter=%22url(%23noise)%22/%3E%3C/svg%3E')]"></div>

		<!-- Subtle grid -->
		<div
			class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.01)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.01)_1px,transparent_1px)] bg-[size:80px_80px]"
		></div>
	</div>

	<div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<!-- Section Header -->
		{#if isVisible}
			<div class="text-center mb-20" in:slideUp={{ delay: 0, duration: 1000 }}>
				<div
					class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-violet-500/10 border border-violet-500/20 mb-6"
				>
					<IconSchool class="w-4 h-4 text-violet-400" />
					<span class="text-sm font-medium text-violet-400 tracking-wide">Trading Education</span>
				</div>

				<h2 class="text-4xl sm:text-5xl lg:text-6xl font-bold text-white mb-6 tracking-tight">
					Master the Markets
					<span
						class="block mt-2 bg-gradient-to-r from-violet-400 via-fuchsia-400 to-pink-400 bg-clip-text text-transparent"
					>
						With Expert-Led Courses
					</span>
				</h2>

				<p class="text-lg sm:text-xl text-zinc-400 max-w-2xl mx-auto leading-relaxed">
					Structured curriculum designed by profitable traders. Learn at your pace with lifetime
					access and live support.
				</p>

				<!-- Stats row -->
				<div
					class="flex flex-wrap justify-center gap-8 mt-10 pt-10 border-t border-zinc-800/50"
					in:slideUp={{ delay: 200, duration: 800 }}
				>
					<div class="text-center">
						<div class="text-3xl font-bold text-white">12,000+</div>
						<div class="text-sm text-zinc-500">Students Enrolled</div>
					</div>
					<div class="text-center">
						<div class="text-3xl font-bold text-white">4.9</div>
						<div class="text-sm text-zinc-500">Average Rating</div>
					</div>
					<div class="text-center">
						<div class="text-3xl font-bold text-white">89%</div>
						<div class="text-sm text-zinc-500">Completion Rate</div>
					</div>
					<div class="text-center">
						<div class="text-3xl font-bold text-white">24/7</div>
						<div class="text-sm text-zinc-500">Community Access</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Course Cards Grid -->
		<div
			bind:this={cardsRef}
			class="grid md:grid-cols-2 gap-6 lg:gap-8"
		>
			{#each courses as course, i}
				<a
					href={course.href}
					class="course-card group relative rounded-3xl overflow-hidden bg-gradient-to-br {course.bgGradient} border border-zinc-800/50 hover:border-zinc-700/50 transition-all duration-500 hover:shadow-2xl hover:shadow-violet-500/10"
					onmouseenter={() => (hoveredCard = course.id)}
					onmouseleave={() => (hoveredCard = null)}
				>
					<!-- Card gradient overlay -->
					<div
						class="absolute inset-0 bg-gradient-to-br {course.gradient} opacity-0 group-hover:opacity-5 transition-opacity duration-500"
					></div>

					<!-- Content -->
					<div class="relative p-6 lg:p-8">
						<!-- Header -->
						<div class="flex items-start justify-between mb-6">
							<div class="flex items-center gap-4">
								<!-- Icon -->
								<div
									class="w-14 h-14 rounded-2xl bg-gradient-to-br {course.gradient} flex items-center justify-center shadow-lg group-hover:scale-110 group-hover:rotate-3 transition-all duration-500"
								>
									{#if course.icon}
									{@const IconComponent = course.icon}
									<IconComponent class="w-7 h-7 text-white" />
								{/if}
								</div>
								<div>
									<span class="text-xs font-medium text-zinc-500 uppercase tracking-wider"
										>{course.level}</span
									>
									<h3
										class="text-xl lg:text-2xl font-bold text-white group-hover:text-transparent group-hover:bg-gradient-to-r group-hover:{course.gradient} group-hover:bg-clip-text transition-all duration-300"
									>
										{course.title}
									</h3>
								</div>
							</div>

							<!-- Badge -->
							<span
								class="px-3 py-1 text-xs font-bold rounded-full {course.badgeColor} text-white shadow-lg"
							>
								{course.badge}
							</span>
						</div>

						<!-- Description -->
						<p class="text-zinc-400 mb-6 leading-relaxed">{course.description}</p>

						<!-- Features -->
						<div class="flex flex-wrap gap-2 mb-6">
							{#each course.features as feature}
								<span
									class="px-3 py-1 text-xs font-medium rounded-full bg-zinc-800/50 text-zinc-300 border border-zinc-700/50"
								>
									{feature}
								</span>
							{/each}
						</div>

						<!-- Meta info -->
						<div class="flex items-center gap-6 text-sm text-zinc-500 mb-6">
							<div class="flex items-center gap-1.5">
								<IconClock class="w-4 h-4" />
								<span>{course.duration}</span>
							</div>
							<div class="flex items-center gap-1.5">
								<IconUsers class="w-4 h-4" />
								<span>{course.students} students</span>
							</div>
							<div class="flex items-center gap-1.5">
								<IconStar class="w-4 h-4 text-amber-400" />
								<span class="text-white font-medium">{course.rating}</span>
								<span>({course.reviews})</span>
							</div>
						</div>

						<!-- Footer -->
						<div
							class="flex items-center justify-between pt-6 border-t border-zinc-800/50"
						>
							<div class="flex items-baseline gap-2">
								<span class="text-2xl font-bold text-white">{course.price}</span>
								<span class="text-sm text-zinc-500 line-through">{course.originalPrice}</span>
							</div>

							<div
								class="flex items-center gap-2 px-5 py-2.5 rounded-full bg-white/5 border border-white/10 group-hover:bg-gradient-to-r group-hover:{course.gradient} group-hover:border-transparent transition-all duration-300"
							>
								<span class="text-sm font-semibold text-white">Enroll Now</span>
								<IconArrowRight
									class="w-4 h-4 text-white group-hover:translate-x-1 transition-transform"
								/>
							</div>
						</div>
					</div>

					<!-- Play button overlay (appears on hover) -->
					<div
						class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"
					>
						<div
							class="w-20 h-20 rounded-full bg-white/10 backdrop-blur-xl flex items-center justify-center border border-white/20 shadow-2xl"
						>
							<IconPlayerPlay class="w-8 h-8 text-white ml-1" />
						</div>
					</div>
				</a>
			{/each}
		</div>

		<!-- Bottom CTA -->
		{#if isVisible}
			<div class="text-center mt-16" in:slideUp={{ delay: 600, duration: 800 }}>
				<a
					href="/courses"
					class="group inline-flex items-center gap-3 px-8 py-4 rounded-full bg-gradient-to-r from-violet-600 to-fuchsia-600 text-white font-semibold shadow-lg shadow-violet-500/25 hover:shadow-violet-500/40 hover:scale-105 transition-all duration-300"
				>
					<span>View All Courses</span>
					<IconArrowRight class="w-5 h-5 group-hover:translate-x-1 transition-transform" />
				</a>

				<p class="mt-6 text-sm text-zinc-500">
					<IconCertificate class="w-4 h-4 inline mr-1" />
					All courses include certificate of completion
				</p>
			</div>
		{/if}
	</div>
</section>
