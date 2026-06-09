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

<section {@attach captureSection} class="courses-section">
	<div class="ambient-layers" aria-hidden="true">
		<div class="noise-layer"></div>

		<div class="ambient-orb ambient-orb-violet"></div>
		<div class="ambient-orb ambient-orb-cyan"></div>

		<div class="grid-layer"></div>
	</div>

	<div class="courses-shell">
		{#if isVisible}
			<div class="section-heading" in:slideUp={{ delay: 0, duration: 1000 }}>
				<div class="section-eyebrow">
					<IconSchool size={16} />
					Professional Education
				</div>

				<h2 class="section-title">
					Trading <span>Curriculum.</span>
				</h2>

				<p class="section-copy">
					We don't sell courses. We provide institutional-grade trading education. Verified by
					funded traders and prop firm graduates worldwide.
				</p>

				<div class="courses-stats-grid" in:slideUp={{ delay: 200, duration: 800 }}>
					<div class="stat-card">
						<div class="stat-value">12k+</div>
						<div class="stat-label">Students</div>
					</div>
					<div class="stat-card">
						<div class="stat-value">4.9</div>
						<div class="stat-label">Rating</div>
					</div>
					<div class="stat-card">
						<div class="stat-value">89%</div>
						<div class="stat-label">Completion</div>
					</div>
					<div class="stat-card">
						<div class="stat-value">24/7</div>
						<div class="stat-label">Support</div>
					</div>
				</div>
			</div>
		{/if}

		<div
			{@attach captureCards}
			class="courses-grid"
			style:--mouse-x={`${mouseX}px`}
			style:--mouse-y={`${mouseY}px`}
		>
			<div class="cursor-spotlight" aria-hidden="true"></div>

			{#each courses as course (course.id)}
				<a href={course.href} class="course-card">
					<div
						class="course-pattern"
						style:background={course.imagePattern}
						style:filter="blur(40px)"
					></div>

					<div class="course-gradient" style:background={course.backgroundGradient}></div>

					<div class="course-content">
						<div class="course-header">
							<div class="course-identity">
								<div class="course-icon">
									{#if course.icon}
										{@const IconComponent = course.icon}
										<IconComponent class="course-icon-svg" size={28} />
									{/if}
								</div>
								<div>
									<span class="course-level">
										{#if course.level === 'Advanced'}
											<IconFlame size={12} class="level-icon-pulse" />
										{:else if course.level === 'Intermediate'}
											<IconActivity size={12} />
										{/if}
										{course.level}
									</span>
									<h3 class="course-title">{course.title}</h3>
								</div>
							</div>

							<span class="course-badge">{course.badge}</span>
						</div>

						<p class="course-description">
							{course.description}
						</p>

						<div class="feature-strip">
							{#each course.features as feature (feature)}
								<span class="feature-chip">
									{feature}
								</span>
							{/each}
						</div>

						<div class="course-spacer"></div>

						<div class="course-footer">
							<div class="course-meta-block">
								<div class="course-meta">
									<div class="course-meta-item">
										<IconClock size={16} />
										<span>{course.duration}</span>
									</div>
									<div class="course-meta-item">
										<IconUsers size={16} />
										<span>{course.students}</span>
									</div>
								</div>
								<div class="course-pricing">
									<span class="course-price">{course.price}</span>
									<span class="course-original-price">{course.originalPrice}</span>
								</div>
							</div>

							<div class="course-cta">
								<span>Start Learning</span>
								<IconArrowRight size={16} />
							</div>
						</div>
					</div>

					<div class="play-overlay">
						<div class="play-overlay-scrim"></div>
						<div class="play-button">
							<IconPlayerPlay class="play-icon" size={32} />
						</div>
					</div>
				</a>
			{/each}
		</div>

		{#if isVisible}
			<div class="section-actions" in:slideUp={{ delay: 400, duration: 800 }}>
				<a href="/courses" class="curriculum-link">
					<span class="curriculum-link-sheen"></span>
					<span class="curriculum-link-label">View Full Curriculum</span>
					<IconArrowRight class="curriculum-link-icon" size={20} />
				</a>

				<p class="certification-note">
					<IconCertificate size={16} />
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

	.courses-section {
		position: relative;
		overflow: hidden;
		padding: 5rem env(safe-area-inset-right) 5rem env(safe-area-inset-left);
		background: #050812;
	}

	.courses-section::selection {
		background: rgba(139, 92, 246, 0.3);
		color: #ddd6fe;
	}

	.ambient-layers {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	.noise-layer,
	.grid-layer {
		position: absolute;
		inset: 0;
	}

	.noise-layer {
		background-image: url('/textures/noise.svg');
		mix-blend-mode: overlay;
		opacity: 0.03;
	}

	.ambient-orb {
		position: absolute;
		width: 50rem;
		height: 50rem;
		border-radius: 999px;
		filter: blur(120px);
		mix-blend-mode: screen;
		animation: pulse-slow 8s ease-in-out infinite;
	}

	.ambient-orb-violet {
		top: -10%;
		left: -10%;
		background: rgba(124, 58, 237, 0.1);
	}

	.ambient-orb-cyan {
		right: -10%;
		bottom: -10%;
		background: rgba(8, 145, 178, 0.1);
		animation-delay: 1000ms;
	}

	.grid-layer {
		background-image:
			linear-gradient(rgba(255, 255, 255, 0.02) 1px, transparent 1px),
			linear-gradient(90deg, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
		background-size: 60px 60px;
		mask-image: radial-gradient(ellipse at center, black 40%, transparent 80%);
	}

	.courses-shell {
		position: relative;
		z-index: 10;
		width: min(100% - 2rem, 80rem);
		margin-inline: auto;
	}

	.section-heading {
		width: min(100%, 56rem);
		margin: 0 auto 6rem;
		text-align: center;
	}

	.section-eyebrow {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 2rem;
		padding: 0.375rem 1rem;
		border: 1px solid rgba(76, 29, 149, 0.3);
		border-radius: 2px;
		background: rgba(46, 16, 101, 0.1);
		color: #8b5cf6;
		font-size: 0.625rem;
		font-weight: 700;
		letter-spacing: 0.3em;
		text-transform: uppercase;
	}

	.section-title {
		margin: 0 0 2rem;
		color: white;
		font-family: var(--font-serif, Georgia, serif);
		font-size: clamp(3rem, 8vw, 4.5rem);
		font-weight: 400;
		line-height: 0.95;
		letter-spacing: 0;
	}

	.section-title span {
		color: #334155;
	}

	.section-copy {
		width: min(100%, 42rem);
		margin: 0 auto;
		color: #94a3b8;
		font-size: 1.125rem;
		font-weight: 300;
		line-height: 1.625;
	}

	.courses-stats-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1.5rem;
		width: min(100%, 56rem);
		margin: 3rem auto 0;
		padding-top: 2rem;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.stat-card {
		cursor: default;
		text-align: center;
	}

	.stat-value {
		color: white;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.15;
		font-variant-numeric: tabular-nums;
		transition: transform 300ms ease;
	}

	.stat-card:hover .stat-value {
		transform: scale(1.1);
	}

	.stat-label {
		margin-top: 0.25rem;
		color: #71717a;
		font-size: 0.75rem;
		font-weight: 500;
		letter-spacing: 0.05em;
		text-transform: uppercase;
	}

	.courses-grid {
		position: relative;
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.5rem;
	}

	.cursor-spotlight {
		position: absolute;
		z-index: 0;
		inset: -1px;
		border-radius: 1.5rem;
		pointer-events: none;
		background: radial-gradient(
			800px circle at var(--mouse-x) var(--mouse-y),
			rgba(139, 92, 246, 0.08),
			transparent 40%
		);
		opacity: 0;
		transition: opacity 300ms ease;
	}

	.course-card {
		position: relative;
		z-index: 10;
		isolation: isolate;
		display: block;
		overflow: hidden;
		min-height: 44px;
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 2rem;
		background: rgba(24, 24, 27, 0.4);
		color: inherit;
		text-decoration: none;
		transition:
			border-color 500ms ease,
			box-shadow 500ms ease,
			transform 500ms ease;
	}

	.course-card:hover {
		border-color: rgba(255, 255, 255, 0.1);
		box-shadow: 0 25px 50px -12px rgba(76, 29, 149, 0.2);
	}

	.course-card:active {
		transform: scale(0.99);
	}

	.course-pattern,
	.course-gradient,
	.play-overlay,
	.play-overlay-scrim {
		position: absolute;
		inset: 0;
	}

	.course-pattern {
		opacity: 0;
		transition: opacity 700ms ease-in-out;
	}

	.course-card:hover .course-pattern {
		opacity: 1;
	}

	.course-gradient {
		opacity: 0.5;
		transition: opacity 500ms ease;
	}

	.course-card:hover .course-gradient {
		opacity: 0.6;
	}

	.course-content {
		position: relative;
		display: flex;
		min-height: 100%;
		flex-direction: column;
		padding: 1.5rem;
	}

	.course-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.course-identity {
		display: flex;
		align-items: center;
		gap: 1rem;
		min-width: 0;
	}

	.course-icon {
		display: flex;
		width: 3.5rem;
		height: 3.5rem;
		flex: 0 0 auto;
		align-items: center;
		justify-content: center;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		background: rgba(255, 255, 255, 0.05);
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.35);
		color: white;
		backdrop-filter: blur(4px);
		transition:
			transform 500ms ease-out,
			border-color 500ms ease,
			background 500ms ease;
	}

	.course-card:hover .course-icon {
		transform: scale(1.1) rotate(3deg);
	}

	:global(.course-icon-svg) {
		filter: drop-shadow(0 0 8px rgba(255, 255, 255, 0.3));
	}

	.course-level {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		margin-bottom: 0.25rem;
		color: #c4b5fd;
		font-size: 0.75rem;
		font-weight: 700;
		letter-spacing: 0.05em;
		text-transform: uppercase;
	}

	:global(.level-icon-pulse) {
		animation: icon-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
	}

	.course-title {
		margin: 0;
		color: white;
		font-size: 1.25rem;
		font-weight: 700;
		line-height: 1;
		letter-spacing: 0;
		transition: color 300ms ease;
	}

	.course-card:hover .course-title {
		color: #ddd6fe;
	}

	.course-badge {
		flex: 0 0 auto;
		padding: 0.25rem 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.1);
		color: white;
		font-size: 0.625rem;
		font-weight: 700;
		box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.25);
		backdrop-filter: blur(12px);
		transition: background 300ms ease;
	}

	.course-card:hover .course-badge {
		background: rgba(255, 255, 255, 0.2);
	}

	.course-description {
		display: -webkit-box;
		overflow: hidden;
		margin: 0 0 2rem;
		color: #a1a1aa;
		font-size: 1rem;
		line-height: 1.625;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		transition: color 300ms ease;
	}

	.course-card:hover .course-description {
		color: #d4d4d8;
	}

	.feature-strip {
		display: flex;
		gap: 0.5rem;
		overflow-x: auto;
		margin: 0 -0.5rem 2rem;
		padding: 0 0.5rem 0.5rem;
		scrollbar-width: none;
		scroll-snap-type: x mandatory;
		-webkit-overflow-scrolling: touch;
	}

	.feature-strip::-webkit-scrollbar {
		display: none;
	}

	.feature-chip {
		flex: 0 0 auto;
		scroll-snap-align: start;
		padding: 0.375rem 0.75rem;
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 0.5rem;
		background: rgba(255, 255, 255, 0.05);
		color: #d4d4d8;
		font-size: 0.75rem;
		font-weight: 500;
		white-space: nowrap;
		transition: border-color 300ms ease;
	}

	.course-card:hover .feature-chip {
		border-color: rgba(255, 255, 255, 0.2);
	}

	.course-spacer {
		flex-grow: 1;
	}

	.course-footer {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 1rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.course-meta-block {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.course-meta {
		display: flex;
		align-items: center;
		gap: 1rem;
		color: #71717a;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.course-meta-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.course-meta-item :global(svg) {
		color: #52525b;
	}

	.course-pricing {
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
		margin-top: 0.25rem;
		font-variant-numeric: tabular-nums;
	}

	.course-price {
		color: white;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1;
		letter-spacing: 0;
	}

	.course-original-price {
		color: #52525b;
		font-size: 0.875rem;
		text-decoration-line: line-through;
		text-decoration-color: rgba(82, 82, 91, 0.5);
	}

	.course-cta {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 0.75rem;
		background: white;
		color: black;
		font-size: 0.875rem;
		font-weight: 700;
		box-shadow: 0 20px 25px -5px rgba(255, 255, 255, 0.05);
		transition:
			transform 300ms ease,
			box-shadow 300ms ease;
	}

	.course-card:hover .course-cta {
		transform: translateY(-0.25rem);
		box-shadow: 0 20px 25px -5px rgba(255, 255, 255, 0.2);
	}

	.play-overlay {
		z-index: 20;
		display: none;
		align-items: center;
		justify-content: center;
		pointer-events: none;
		opacity: 0;
		transition: opacity 500ms ease;
	}

	.course-card:hover .play-overlay {
		opacity: 1;
	}

	.play-overlay-scrim {
		background: rgba(0, 0, 0, 0.4);
		backdrop-filter: blur(2px);
	}

	.play-button {
		position: relative;
		display: flex;
		width: 5rem;
		height: 5rem;
		align-items: center;
		justify-content: center;
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 999px;
		background: rgba(255, 255, 255, 0.1);
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.45);
		color: white;
		backdrop-filter: blur(24px);
		transform: scale(0.5);
		transition: transform 500ms cubic-bezier(0.34, 1.56, 0.64, 1);
	}

	.course-card:hover .play-button {
		transform: scale(1);
	}

	:global(.play-icon) {
		margin-left: 0.25rem;
		filter: drop-shadow(0 10px 8px rgba(0, 0, 0, 0.04)) drop-shadow(0 4px 3px rgba(0, 0, 0, 0.1));
	}

	.section-actions {
		margin-top: 5rem;
		text-align: center;
	}

	.curriculum-link {
		position: relative;
		display: inline-flex;
		overflow: hidden;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 1rem 2rem;
		border-radius: 999px;
		background: linear-gradient(to right, #7c3aed, #4f46e5);
		color: white;
		font-weight: 600;
		text-decoration: none;
		box-shadow: 0 10px 15px -3px rgba(139, 92, 246, 0.25);
		transition:
			transform 300ms ease,
			box-shadow 300ms ease;
	}

	.curriculum-link:hover {
		transform: translateY(-0.25rem);
		box-shadow: 0 10px 15px -3px rgba(139, 92, 246, 0.5);
	}

	.curriculum-link:active {
		transform: scale(0.98);
	}

	.curriculum-link-sheen {
		position: absolute;
		inset: 0;
		background: linear-gradient(to right, transparent, rgba(255, 255, 255, 0.2), transparent);
		transform: translateX(-100%);
		transition: transform 700ms ease-in-out;
	}

	.curriculum-link:hover .curriculum-link-sheen {
		transform: translateX(100%);
	}

	.curriculum-link-label,
	:global(.curriculum-link-icon) {
		position: relative;
		z-index: 10;
	}

	:global(.curriculum-link-icon) {
		transition: transform 300ms ease;
	}

	.curriculum-link:hover :global(.curriculum-link-icon) {
		transform: translateX(0.25rem);
	}

	.certification-note {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin: 1.5rem 0 0;
		color: #71717a;
		font-size: 0.875rem;
		opacity: 0.6;
		transition: opacity 300ms ease;
	}

	.certification-note:hover {
		opacity: 1;
	}

	.certification-note :global(svg) {
		color: #8b5cf6;
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

	@keyframes icon-pulse {
		50% {
			opacity: 0.5;
		}
	}

	@media (max-width: 359px) {
		.courses-section {
			padding-top: 3rem;
			padding-bottom: 3rem;
		}

		.course-card {
			border-radius: 1rem;
		}

		.course-content {
			padding: 1rem;
		}

		.section-title {
			font-size: 1.75rem;
		}

		.course-title,
		.course-price {
			font-size: 1.25rem;
		}
	}

	@media (min-width: 360px) and (max-width: 639px) {
		.courses-section {
			padding-top: 4rem;
			padding-bottom: 4rem;
		}

		.course-content {
			padding: 1.25rem;
		}

		.courses-stats-grid {
			grid-template-columns: 1fr;
		}

		.courses-stats-grid,
		.courses-grid {
			gap: 1rem;
		}

		.section-title {
			font-size: 2rem;
		}

		.course-title,
		.course-price {
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

	@media (min-width: 640px) {
		.courses-section {
			padding-top: 8rem;
			padding-bottom: 8rem;
		}

		.course-content {
			padding: 2rem;
		}

		.course-title {
			font-size: 1.5rem;
		}

		.course-badge {
			font-size: 0.75rem;
		}

		.course-footer {
			flex-direction: row;
			align-items: flex-end;
		}

		.course-meta {
			font-size: 0.875rem;
		}

		.play-overlay {
			display: flex;
		}
	}

	@media (min-width: 768px) {
		.courses-stats-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.cursor-spotlight {
			opacity: 1;
		}
	}

	@media (min-width: 1024px) {
		.courses-shell {
			width: min(100% - 4rem, 80rem);
		}

		.courses-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 2rem;
		}

		.course-content {
			padding: 2.5rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DYNAMIC VIEWPORT HEIGHT - Use dvh for modern mobile browsers
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@supports (min-height: 100dvh) {
		.courses-section {
			min-height: auto;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * REDUCED MOTION - Accessibility
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (prefers-reduced-motion: reduce) {
		.ambient-orb,
		:global(.level-icon-pulse) {
			animation: none;
		}

		.course-card,
		.course-icon,
		.stat-value,
		.course-cta,
		.play-button,
		.curriculum-link,
		.curriculum-link-sheen,
		:global(.curriculum-link-icon) {
			transition: none;
		}
	}

	@media (max-height: 500px) and (orientation: landscape) {
		.courses-section {
			padding-top: 2rem;
			padding-bottom: 2rem;
		}

		.section-heading {
			margin-bottom: 2rem;
		}
	}
</style>
