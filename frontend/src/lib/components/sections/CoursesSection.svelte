<script lang="ts">
	import { logger } from '$lib/utils/logger';
	/**
	 * CoursesSection - Apple/Netflix Cinematic Design
	 * Upgraded with ICT9+ Layout, Motion, and Interaction Physics
	 */
	import { onMount, tick } from 'svelte';
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import {
		Icon,
		IconActivity,
		IconArrowRight,
		IconBrain,
		IconCertificate,
		IconChartCandle,
		IconChartLine,
		IconClock,
		IconFlame,
		IconPlayerPlay,
		IconSchool,
		IconShield,
		IconUsers
	} from '$lib/icons';

	// Tabler Icons (Preserving sub-path imports for tree-shaking)
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
			bgGradient: 'from-blue-950/50 to-cyan-950/30',
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
			bgGradient: 'from-emerald-950/50 to-teal-950/30',
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
			bgGradient: 'from-violet-950/50 to-fuchsia-950/30',
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
			bgGradient: 'from-amber-950/50 to-red-950/30',
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
	// ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
	let isVisible = $state(false);
	let scrollTriggerInstance: any = null;
	let prefersReducedMotion = $state(false);

	// Mouse Tracking for Spotlight Effect
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
				const visibilityObserver = new IntersectionObserver(
					(entries) => {
						if (entries[0]?.isIntersecting) {
							isVisible = true;
							visibilityObserver.disconnect();
						}
					},
					{ threshold: 0.1, rootMargin: '50px' }
				);
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
			// ICT11+ Fix: Kill only ScrollTriggers scoped to this component
			// Previously used killAll() which destroyed ALL ScrollTriggers globally,
			// causing layout breaks when scrolling to bottom of page
			if (scrollTriggerInstance && (cardsRef || sectionRef)) {
				scrollTriggerInstance.getAll().forEach((st: any) => {
					if (
						st.trigger === cardsRef ||
						st.trigger === sectionRef ||
						cardsRef?.contains(st.trigger) ||
						sectionRef?.contains(st.trigger)
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
			}
		} catch (e) {
			logger.debug('[CoursesSection] GSAP not available:', e);
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
	bind:this={sectionRef}
	aria-label="Trading Curriculum"
	class="relative py-20 sm:py-32 3xl:py-40 5xl:py-48 overflow-hidden bg-[#050812] selection:bg-violet-500/30 selection:text-violet-200"
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
			class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:60px_60px] [mask-image:radial-gradient(ellipse_at_center,black_40%,transparent_80%)]"
		></div>
	</div>

	<div
		class="relative max-w-7xl 3xl:max-w-[1800px] 4xl:max-w-[2200px] 5xl:max-w-[2600px] 6xl:max-w-[3200px] mx-auto px-4 sm:px-6 lg:px-8 3xl:px-12 5xl:px-16 6xl:px-20 z-10"
	>
		{#if isVisible}
			<div
				class="max-w-4xl 3xl:max-w-[1200px] 4xl:max-w-[1600px] 5xl:max-w-[2000px] 6xl:max-w-[2400px] mx-auto text-center mb-24 3xl:mb-32 5xl:mb-40"
				in:slideUp={{ delay: 0, duration: 1000 }}
			>
				<div
					class="inline-flex items-center gap-3 px-4 py-1.5 border border-violet-900/30 bg-violet-950/10 text-violet-500 text-[10px] font-bold tracking-[0.3em] uppercase mb-8 rounded-sm"
				>
					<IconSchool class="w-4 h-4" />
					Professional Education
				</div>

				<h2
					class="text-4xl xs:text-5xl sm:text-5xl md:text-7xl 3xl:text-8xl 4xl:text-9xl 5xl:text-[10rem] font-serif text-white mb-8 tracking-tight"
				>
					Trading <span class="text-slate-700">Curriculum.</span>
				</h2>

				<p
					class="text-lg 3xl:text-xl 5xl:text-2xl text-slate-400 font-light leading-relaxed max-w-2xl 3xl:max-w-3xl 5xl:max-w-4xl mx-auto"
				>
					We don't sell courses. We provide institutional-grade trading education. Verified by
					funded traders and prop firm graduates worldwide.
				</p>

				<div
					class="grid grid-cols-2 md:grid-cols-4 gap-6 3xl:gap-8 5xl:gap-12 max-w-4xl 3xl:max-w-[1200px] 4xl:max-w-[1600px] 5xl:max-w-[2000px] 6xl:max-w-[2400px] mx-auto mt-12 pt-8 border-t border-white/5"
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
					<div class="cs-stat">
						<div class="cs-stat-value">4.9</div>
						<div class="cs-stat-label">Rating</div>
					</div>
					<div class="cs-stat">
						<div class="cs-stat-value">89%</div>
						<div class="cs-stat-label">Completion</div>
					</div>
					<div class="cs-stat">
						<div class="cs-stat-value">24/7</div>
						<div class="cs-stat-label">Support</div>
					</div>
				</div>
			</div>
		{/if}

		<div
			bind:this={cardsRef}
			class="grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8 3xl:gap-12 5xl:gap-16 relative"
			style="--mouse-x: {mouseX}px; --mouse-y: {mouseY}px;"
		>
			<div
				class="cs-cards-spotlight"
				style="background: radial-gradient(800px circle at var(--mouse-x) var(--mouse-y), oklch(0.55 0.2 300 / 0.08), transparent 40%);"
			></div>

			{#each courses as course (course.href ?? course.title)}
				<a
					href={course.href}
					class="course-card group relative rounded-[2rem] overflow-hidden bg-zinc-900/40 border border-white/5 hover:border-white/10 transition-all duration-500 hover:shadow-2xl hover:shadow-violet-900/20 active:scale-[0.99] z-10 isolate"
				>
					<div
						class="cs-card-blur"
						style="background: {course.imagePattern}; filter: blur(40px);"
					></div>

					<div class="cs-card-inner">
						<div class="cs-card-top">
							<div class="cs-card-info">
								<div class="cs-card-icon">
									{#if course.icon}
										{@const iconStr = course.icon}
										<Icon icon={iconStr} size={28} />
									{/if}
								</div>
								<div>
									<span class="cs-level-badge">
										{#if course.level === 'Advanced'}
											<Icon icon={IconFlame} size={12} class="cs-flame" />
										{:else if course.level === 'Intermediate'}
											<Icon icon={IconActivity} size={12} />
										{/if}
										{course.level}
									</span>
									<h3 class="cs-card-title">{course.title}</h3>
								</div>
							</div>

							<span class="cs-badge-pill">{course.badge}</span>
						</div>

						<p class="cs-card-desc">{course.description}</p>

						<div class="flex gap-2 mb-8 overflow-x-auto pb-2 -mx-2 px-2 scrollbar-hide snap-x">
							{#each course.features as feature (feature)}
								<span
									class="snap-start flex-shrink-0 px-3 py-1.5 text-xs font-medium rounded-lg bg-white/5 text-zinc-300 border border-white/5 whitespace-nowrap group-hover:border-white/20 transition-colors"
								>
									{feature}
								</span>
							{/each}
						</div>

						<div class="cs-spacer"></div>

						<div class="cs-card-bottom">
							<div class="cs-price-col">
								<div class="cs-meta-row">
									<div class="cs-meta-item">
										<Icon icon={IconClock} size={16} class="cs-meta-icon" />
										<span>{course.duration}</span>
									</div>
									<div class="cs-meta-item">
										<Icon icon={IconUsers} size={16} class="cs-meta-icon" />
										<span>{course.students}</span>
									</div>
								</div>
								<div class="cs-price-row">
									<span class="cs-price">{course.price}</span>
									<span class="cs-original-price">{course.originalPrice}</span>
								</div>
							</div>

							<div class="cs-cta-btn">
								<span>Start Learning</span>
								<Icon icon={IconArrowRight} size={16} />
							</div>
						</div>
					</div>

					<div class="cs-play-overlay">
						<div class="cs-play-backdrop"></div>
						<div class="cs-play-btn">
							<Icon icon={IconPlayerPlay} size={32} class="cs-play-icon" />
						</div>
					</div>
				</a>
			{/each}
		</div>

		{#if isVisible}
			<div class="cs-footer" in:slideUp={{ delay: 400, duration: 800 }}>
				<a href="/courses" class="cs-footer-cta">
					<span class="cs-footer-shimmer"></span>
					<span class="cs-footer-text">View Full Curriculum</span>
					<Icon icon={IconArrowRight} size={20} class="cs-footer-arrow" />
				</a>

				<p class="cs-cert-note">
					<Icon icon={IconCertificate} size={16} class="cs-cert-icon" />
					Official certification included with all pathways
				</p>
			</div>
		{/if}
	</div>
</section>

<style>
	/* ─── Section ─── */
	.cs-section {
		position: relative;
		padding-block: 5rem;
		overflow: hidden;
		background-color: oklch(0.08 0.02 270);

		@media (min-width: 640px) {
			padding-block: 8rem;
		}
	}

	/* ─── Background ─── */
	.cs-bg {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	.cs-noise {
		position: absolute;
		inset: 0;
		opacity: 0.03;
		mix-blend-mode: overlay;
	}

	.cs-orb {
		position: absolute;
		inline-size: 50rem;
		block-size: 50rem;
		border-radius: 50%;
		filter: blur(120px);
		mix-blend-mode: screen;
		animation: pulse-slow 8s ease-in-out infinite;
	}

	.cs-orb-violet {
		inset-block-start: -10%;
		inset-inline-start: -10%;
		background-color: oklch(0.5 0.2 300 / 0.1);
	}

	.cs-orb-cyan {
		inset-block-end: -10%;
		inset-inline-end: -10%;
		background-color: oklch(0.6 0.15 195 / 0.1);
		animation-delay: 1s;
	}

	.cs-grid-lines {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(oklch(1 0 0 / 0.02) 1px, transparent 1px),
			linear-gradient(90deg, oklch(1 0 0 / 0.02) 1px, transparent 1px);
		background-size: 3.75rem 3.75rem;
		mask-image: radial-gradient(ellipse at center, oklch(0 0 0) 40%, transparent 80%);
	}

	/* ─── Container ─── */
	.cs-container {
		position: relative;
		max-inline-size: 80rem;
		margin-inline: auto;
		padding-inline: 1rem;
		z-index: 10;

		@media (min-width: 640px) {
			padding-inline: 1.5rem;
		}
		@media (min-width: 1024px) {
			padding-inline: 2rem;
		}
	}

	.cs-header {
		max-inline-size: 56rem;
		margin-inline: auto;
		text-align: center;
		margin-block-end: 6rem;
	}

	.cs-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding-inline: 1rem;
		padding-block: 0.375rem;
		border: 1px solid oklch(0.35 0.15 300 / 0.3);
		background-color: oklch(0.15 0.08 300 / 0.1);
		color: oklch(0.6 0.2 300);
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
		border-radius: 2px;
	}

	.cs-title {
		font-size: clamp(3rem, 6vw, 4.5rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
	}

	.cs-title-muted {
		color: oklch(0.35 0.01 265);
	}

	.cs-subtitle {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-inline: auto;
	}

	/* ─── Stats ─── */
	.cs-stats {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.5rem;
		max-inline-size: 56rem;
		margin-inline: auto;
		margin-block-start: 3rem;
		padding-block-start: 2rem;
		border-block-start: 1px solid oklch(1 0 0 / 0.05);

		@media (min-width: 768px) {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	.cs-stat {
		text-align: center;
		cursor: default;
	}

	.cs-stat-value {
		font-size: 1.875rem;
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		font-variant-numeric: tabular-nums;
		transition: transform 300ms;
	}

	.cs-stat:hover .cs-stat-value {
		transform: scale(1.1);
	}

	.cs-stat-label {
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: oklch(0.45 0.005 285);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-start: 0.25rem;
	}

	/* ─── Cards Grid ─── */
	.cs-cards {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
		position: relative;

		@media (min-width: 768px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (min-width: 1024px) {
			gap: 2rem;
		}
	}

	.cs-cards-spotlight {
		pointer-events: none;
		position: absolute;
		inset: -1px;
		opacity: 0;
		transition: opacity 300ms;
		z-index: 0;
		border-radius: 1.5rem;

		@media (min-width: 768px) {
			opacity: 1;
		}
	}

	/* ─── Card ─── */
	.cs-card {
		position: relative;
		border-radius: 2rem;
		overflow: hidden;
		background-color: oklch(0.15 0.005 285 / 0.4);
		border: 1px solid oklch(1 0 0 / 0.05);
		transition:
			border-color 500ms,
			box-shadow 500ms,
			transform 100ms;
		z-index: 10;
		isolation: isolate;
		text-decoration: none;

		&:hover {
			border-color: oklch(1 0 0 / 0.1);
			box-shadow: 0 25px 50px oklch(0.35 0.15 300 / 0.2);
		}
		&:active {
			transform: scale(0.99);
		}
		&:hover .cs-card-blur {
			opacity: 1;
		}
		&:hover .cs-card-icon {
			transform: scale(1.1) rotate(3deg);
		}
		&:hover .cs-card-title {
			color: oklch(0.85 0.1 300);
		}
		&:hover .cs-card-desc {
			color: oklch(0.8 0.01 265);
		}
		&:hover .cs-feature-tag {
			border-color: oklch(1 0 0 / 0.2);
		}
		&:hover .cs-badge-pill {
			background-color: oklch(1 0 0 / 0.2);
		}
		&:hover .cs-cta-btn {
			transform: translateY(-0.25rem);
			box-shadow: 0 20px 40px oklch(1 0 0 / 0.2);
		}
		&:hover .cs-play-overlay {
			opacity: 1;
		}
		&:hover .cs-play-btn {
			transform: scale(1);
		}
	}

	.cs-card-blur {
		position: absolute;
		inset: 0;
		opacity: 0;
		transition: opacity 700ms ease-in-out;
	}

	.cs-card-inner {
		position: relative;
		padding: 1.5rem;
		block-size: 100%;
		display: flex;
		flex-direction: column;

		@media (min-width: 640px) {
			padding: 2rem;
		}
		@media (min-width: 1024px) {
			padding: 2.5rem;
		}
	}

	/* ─── Card Top ─── */
	.cs-card-top {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		margin-block-end: 1.5rem;
	}

	.cs-card-info {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.cs-card-icon {
		inline-size: 3.5rem;
		block-size: 3.5rem;
		border-radius: 1rem;
		background-color: oklch(1 0 0 / 0.05);
		border: 1px solid oklch(1 0 0 / 0.1);
		display: flex;
		align-items: center;
		justify-content: center;
		box-shadow: 0 10px 15px oklch(0 0 0 / 0.1);
		backdrop-filter: blur(4px);
		color: oklch(1 0 0);
		transition: transform 500ms ease-out;
	}

	.cs-level-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-bold);
		color: oklch(0.75 0.12 300);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-end: 0.25rem;
	}

	:global(.cs-flame) {
		animation: pulse 1s ease-in-out infinite;
	}

	.cs-card-title {
		font-size: clamp(1.25rem, 2.5vw, 1.5rem);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		letter-spacing: -0.02em;
		line-height: 1.1;
		transition: color 200ms;
	}

	.cs-badge-pill {
		padding-inline: 0.75rem;
		padding-block: 0.25rem;
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		border-radius: 999px;
		background-color: oklch(1 0 0 / 0.1);
		color: oklch(1 0 0);
		backdrop-filter: blur(12px);
		border: 1px solid oklch(1 0 0 / 0.1);
		box-shadow: 0 10px 15px oklch(0 0 0 / 0.1);
		transition: background-color 200ms;
		white-space: nowrap;

		@media (min-width: 640px) {
			font-size: var(--text-xs);
		}
	}

	/* ─── Card Description ─── */
	.cs-card-desc {
		font-size: var(--text-base);
		color: oklch(0.55 0.01 265);
		margin-block-end: 2rem;
		line-height: 1.7;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		transition: color 300ms;
	}

	/* ─── Features Scroll ─── */
	.cs-features-scroll {
		display: flex;
		gap: 0.5rem;
		margin-block-end: 2rem;
		overflow-x: auto;
		padding-block-end: 0.5rem;
		margin-inline: -0.5rem;
		padding-inline: 0.5rem;
		scrollbar-width: none;

		&::-webkit-scrollbar {
			display: none;
		}

		@media (max-width: 767px) {
			scroll-snap-type: x mandatory;
			-webkit-overflow-scrolling: touch;
		}
	}

	.cs-feature-tag {
		scroll-snap-align: start;
		flex-shrink: 0;
		padding-inline: 0.75rem;
		padding-block: 0.375rem;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		border-radius: var(--radius-lg);
		background-color: oklch(1 0 0 / 0.05);
		color: oklch(0.8 0.01 265);
		border: 1px solid oklch(1 0 0 / 0.05);
		white-space: nowrap;
		transition: border-color 200ms;
	}

	.cs-spacer {
		flex-grow: 1;
	}

	/* ─── Card Bottom ─── */
	.cs-card-bottom {
		padding-block-start: 1.5rem;
		border-block-start: 1px solid oklch(1 0 0 / 0.05);
		display: flex;
		flex-direction: column;
		gap: 1rem;

		@media (min-width: 640px) {
			flex-direction: row;
			align-items: flex-end;
			justify-content: space-between;
		}
	}

	.cs-price-col {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.cs-meta-row {
		display: flex;
		align-items: center;
		gap: 1rem;
		font-size: var(--text-xs);
		color: oklch(0.45 0.005 285);
		font-weight: var(--weight-medium);

		@media (min-width: 640px) {
			font-size: var(--text-sm);
		}
	}

	.cs-meta-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}
	:global(.cs-meta-icon) {
		color: oklch(0.4 0.005 285);
	}

	.cs-price-row {
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
		margin-block-start: 0.25rem;
	}

	.cs-price {
		font-size: 1.5rem;
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
		letter-spacing: -0.02em;
		font-variant-numeric: tabular-nums;
	}

	.cs-original-price {
		font-size: var(--text-sm);
		color: oklch(0.4 0.005 285);
		text-decoration: line-through;
		font-variant-numeric: tabular-nums;
	}

	.cs-cta-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding-inline: 1.5rem;
		padding-block: 0.75rem;
		border-radius: var(--radius-xl);
		background-color: oklch(1 0 0);
		color: oklch(0 0 0);
		font-weight: var(--weight-bold);
		font-size: var(--text-sm);
		box-shadow: 0 20px 40px oklch(1 0 0 / 0.05);
		transition:
			transform 300ms,
			box-shadow 300ms;
	}

	/* ─── Play Overlay ─── */
	.cs-play-overlay {
		display: none;
		position: absolute;
		inset: 0;
		align-items: center;
		justify-content: center;
		opacity: 0;
		transition: opacity 500ms;
		pointer-events: none;
		z-index: 20;

		@media (min-width: 640px) {
			display: flex;
		}
	}

	.cs-play-backdrop {
		position: absolute;
		inset: 0;
		background-color: oklch(0 0 0 / 0.4);
		backdrop-filter: blur(2px);
	}

	.cs-play-btn {
		position: relative;
		inline-size: 5rem;
		block-size: 5rem;
		border-radius: 50%;
		background-color: oklch(1 0 0 / 0.1);
		backdrop-filter: blur(16px);
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid oklch(1 0 0 / 0.2);
		box-shadow: 0 25px 50px oklch(0 0 0 / 0.25);
		transform: scale(0.5);
		transition: transform 500ms cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	:global(.cs-play-icon) {
		color: oklch(1 0 0);
		margin-inline-start: 0.25rem;
	}

	/* ─── Footer ─── */
	.cs-footer {
		text-align: center;
		margin-block-start: 5rem;
	}

	.cs-footer-cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding-inline: 2rem;
		padding-block: 1rem;
		border-radius: 999px;
		background: linear-gradient(to right, oklch(0.5 0.2 300), oklch(0.45 0.2 270));
		color: oklch(1 0 0);
		font-weight: var(--weight-semibold);
		box-shadow: 0 10px 25px oklch(0.5 0.2 300 / 0.25);
		transition:
			box-shadow 300ms,
			transform 300ms;
		position: relative;
		overflow: hidden;
		text-decoration: none;

		&:hover {
			box-shadow: 0 10px 25px oklch(0.5 0.2 300 / 0.5);
			transform: translateY(-0.25rem);
		}
		&:active {
			transform: scale(0.98);
		}
	}

	.cs-footer-shimmer {
		position: absolute;
		inset: 0;
		background: linear-gradient(to right, transparent, oklch(1 0 0 / 0.2), transparent);
		transform: translateX(-100%);
		transition: transform 700ms ease-in-out;
	}

	.cs-footer-cta:hover .cs-footer-shimmer {
		transform: translateX(100%);
	}

	.cs-footer-text {
		font-size: var(--text-base);
		position: relative;
		z-index: 10;
	}
	:global(.cs-footer-arrow) {
		position: relative;
		z-index: 10;
		transition: transform 200ms;
	}
	.cs-footer-cta:hover :global(.cs-footer-arrow) {
		transform: translateX(0.25rem);
	}

	.cs-cert-note {
		margin-block-start: 1.5rem;
		font-size: var(--text-sm);
		color: oklch(0.45 0.005 285);
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		opacity: 0.6;
		transition: opacity 200ms;

		&:hover {
			opacity: 1;
		}
	}

	:global(.cs-cert-icon) {
		color: oklch(0.6 0.2 300);
	}

	/* ─── Safe Area Insets ─── */
	.cs-section {
		padding-inline-start: env(safe-area-inset-left);
		padding-inline-end: env(safe-area-inset-right);
	}

	/* ─── Touch Targets ─── */
	@media (hover: none) and (pointer: coarse) {
		.cs-card,
		.cs-footer-cta {
			min-block-size: 44px;
		}
	}

	/* ─── Reduced Motion ─── */
	@media (prefers-reduced-motion: reduce) {
		.cs-orb {
			animation: none;
		}
		.cs-card {
			transition: none;
		}
	}

	/* ─── Landscape Mobile ─── */
	@media (max-height: 500px) and (orientation: landscape) {
		.cs-section {
			padding-block: 2rem;
		}
		.cs-header {
			margin-block-end: 2rem;
		}
	}

	/* ─── Keyframes ─── */
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

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * EXTRA SMALL DEVICES (< 360px)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 359px) {
		section {
			padding-top: 3rem !important;
			padding-bottom: 3rem !important;
		}

		:global(.course-card) {
			border-radius: 1rem !important;
		}

		:global(.course-card .p-6),
		:global(.course-card .p-8),
		:global(.course-card .p-10) {
			padding: 1rem !important;
		}

		:global(.text-5xl),
		:global(.text-7xl) {
			font-size: 1.75rem !important;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SMALL MOBILE (360px - 639px)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (min-width: 360px) and (max-width: 639px) {
		section {
			padding-top: 4rem !important;
			padding-bottom: 4rem !important;
		}

		:global(.course-card .p-6),
		:global(.course-card .p-8) {
			padding: 1.25rem !important;
		}

		:global(.grid-cols-2) {
			grid-template-columns: 1fr !important;
		}

		:global(.gap-6),
		:global(.gap-8) {
			gap: 1rem !important;
		}

		:global(.text-5xl),
		:global(.text-7xl) {
			font-size: 2rem !important;
		}

		:global(.text-2xl) {
			font-size: 1.25rem !important;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TABLET (640px - 767px)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (min-width: 640px) and (max-width: 767px) {
		:global(.grid-cols-2) {
			grid-template-columns: 1fr !important;
		}

		:global(.gap-6),
		:global(.gap-8) {
			gap: 1.25rem !important;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * MEDIUM DEVICES (768px - 1023px)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (min-width: 768px) and (max-width: 1023px) {
		:global([class*='grid-cols-2']) {
			grid-template-columns: repeat(2, 1fr) !important;
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
			transition: none !important;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * LANDSCAPE MOBILE - Optimize for horizontal orientation
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-height: 500px) and (orientation: landscape) {
		section {
			padding-top: 2rem !important;
			padding-bottom: 2rem !important;
		}

		:global(.mb-24) {
			margin-bottom: 2rem !important;
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
