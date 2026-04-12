<script lang="ts">
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';
	import type { Post } from '$lib/types/post';
	import { Icon, IconArrowRight, IconChartCandle, IconClock, IconNews } from '$lib/icons';

	// ─────────────────────────────────────────────────────────────────────────
	// Props & Logic
	// ─────────────────────────────────────────────────────────────────────────
	interface Props {
		posts?: Post[];
	}

	let { posts = [] }: Props = $props();

	// Separate the Lead Story (First Post) from the Wire (Rest of Posts)
	let leadPost = $derived(posts.length > 0 ? posts[0] : null);
	let wirePosts = $derived(posts.length > 1 ? posts.slice(1, 4) : []);

	// --- Animation Logic ---
	let containerRef = $state<HTMLElement | null>(null);
	// ICT11+ Fix: Start false, set true via IntersectionObserver to trigger in: transitions
	let isVisible = $state(false);
	let mouse = $state({ x: 0, y: 0 });

	const handleMouseMove = (e: MouseEvent) => {
		if (!containerRef) return;
		const rect = containerRef.getBoundingClientRect();
		mouse.x = e.clientX - rect.left;
		mouse.y = e.clientY - rect.top;
	};

	// Trigger entrance animations when section scrolls into viewport
	$effect(() => {
		if (!browser || !containerRef) {
			isVisible = true;
			return;
		}

		const observer = new IntersectionObserver(
			(entries) => {
				if (entries[0]?.isIntersecting) {
					isVisible = true;
					observer.disconnect();
				}
			},
			{ threshold: 0.1, rootMargin: '200px' }
		);

		observer.observe(containerRef);

		return () => observer.disconnect();
	});

	function heavySlide(_node: Element, { delay = 0, duration = 1000 }) {
		return {
			delay,
			duration,
			css: (t: number) => {
				const eased = cubicOut(t);
				return `opacity: ${eased}; transform: translateY(${(1 - eased) * 20}px);`;
			}
		};
	}

	// Helper: Format relative time for the "Wire" feel
	function getRelativeTime(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diffInHours = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60));

		if (diffInHours < 24) return `${diffInHours} HRS AGO`;
		return date.toLocaleDateString('en-US', { day: 'numeric', month: 'short' }).toUpperCase();
	}
</script>

<section
	bind:this={containerRef}
	onmousemove={handleMouseMove}
	class="relative py-32 3xl:py-40 5xl:py-48 px-6 3xl:px-12 5xl:px-16 6xl:px-20 bg-[#020202] overflow-hidden border-t border-white/10"
	aria-label="Market Intelligence Wire"
>
	<div class="lb-bg">
		<div class="lb-grid-lines"></div>
		<div
			class="lb-spotlight"
			style="background: radial-gradient(800px circle at var(--x) var(--y), oklch(1 0 0 / 0.05), transparent 60%);"
		></div>
	</div>

	<div
		class="relative max-w-[1600px] 3xl:max-w-[1800px] 4xl:max-w-[2200px] 5xl:max-w-[2600px] 6xl:max-w-[3200px] mx-auto z-10"
	>
		<div
			class="max-w-4xl 3xl:max-w-[1200px] 4xl:max-w-[1600px] 5xl:max-w-[2000px] 6xl:max-w-[2400px] mx-auto text-center mb-24 3xl:mb-32 5xl:mb-40"
		>
			{#if isVisible}
				<div in:heavySlide={{ delay: 0, duration: 1000 }} class="lb-badge">
					<Icon icon={IconNews} size={14} />
					Intelligence Wire
				</div>

				<h2
					in:heavySlide={{ delay: 100 }}
					class="text-4xl xs:text-5xl sm:text-5xl md:text-7xl 3xl:text-8xl 4xl:text-9xl 5xl:text-[10rem] font-serif text-white mb-8 tracking-tight"
				>
					Market <span class="text-slate-700">Analysis.</span>
				</h2>

				<p
					in:heavySlide={{ delay: 200 }}
					class="text-lg 3xl:text-xl 5xl:text-2xl text-slate-400 font-light leading-relaxed max-w-2xl 3xl:max-w-3xl 5xl:max-w-4xl mx-auto"
				>
					We don't publish retail content. We deliver institutional-grade market intelligence.
					Verified by quantitative analysts and professional trading desks worldwide.
				</p>
			{/if}
		</div>

		{#if posts.length > 0}
			<div class="grid lg:grid-cols-12 gap-12 3xl:gap-16 5xl:gap-20">
				{#if leadPost}
					<div class="lb-lead-col">
						{#if isVisible}
							<a href="/blog/{leadPost.slug}" in:heavySlide={{ delay: 300 }} class="lb-lead-card">
								<div class="lb-lead-image">
									{#if leadPost.featured_image}
										<div
											class="lb-lead-bg"
											style="background-image: url('{leadPost.featured_image}')"
										></div>
									{:else}
										<div class="lb-lead-bg-fallback"></div>
									{/if}
									<div class="lb-lead-overlay"></div>

									<div class="lb-lead-tag-row">
										<span class="lb-lead-tag">Priority Brief</span>
									</div>
								</div>

								<div class="lb-lead-content">
									<div class="lb-lead-meta">
										<span class="lb-meta-time">
											<Icon icon={IconClock} size={12} />
											{getRelativeTime(leadPost.published_at)}
										</span>
										<span class="lb-meta-sep"></span>
										<span>{leadPost.author?.name || 'Desk Analyst'}</span>
									</div>

									<h3 class="lb-lead-title">{leadPost.title}</h3>

									<p class="lb-lead-excerpt">
										{leadPost.excerpt || 'Market analysis briefing. Click to access full report.'}
									</p>

									<div class="lb-read-row">
										<span>Read Full Protocol</span>
										<Icon icon={IconArrowRight} size={14} class="lb-read-arrow" />
									</div>
								</div>
							</a>
						{/if}
					</div>
				{/if}

				<div class="lb-wire-col">
					{#if isVisible}
						<div in:heavySlide={{ delay: 400 }} class="lb-wire-header">
							<Icon icon={IconNews} size={14} />
							<span>Incoming Signals</span>
						</div>

						<div class="space-y-8">
							{#each wirePosts as post (post.slug)}
								<a
									href="/blog/{post.slug}"
									class="group block border-b border-white/5 pb-8 last:border-0 hover:pl-4 transition-all duration-300"
								>
									<div class="flex items-center justify-between mb-2">
										<span class="text-[10px] font-mono text-amber-600 uppercase tracking-widest">
											{getRelativeTime(post.published_at)}
										</span>
										<IconChartCandle
											size={14}
											class="text-slate-600 group-hover:text-amber-500 transition-colors"
										/>
									</div>
									<h4 class="lb-wire-title">{post.title}</h4>
									<div class="lb-wire-bottom">
										<span class="lb-wire-author">{post.author?.name || 'Desk Analyst'}</span>
										<span class="lb-wire-read">
											<span>Read More</span>
											<Icon icon={IconArrowRight} size={12} class="lb-wire-arrow" />
										</span>
									</div>
								</a>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		{:else}
			<div class="lb-empty">
				<div class="lb-empty-icon">
					<Icon icon={IconNews} size={24} />
				</div>
				<h3 class="lb-empty-title">Wire Silent</h3>
				<p class="lb-empty-text">No Intelligence Briefs Available</p>
			</div>
		{/if}
	</div>
</section>

<style>
	/* ─── Section ─── */
	.lb-section {
		position: relative;
		padding-block: 8rem;
		padding-inline: 1.5rem;
		background-color: oklch(0.05 0 0);
		overflow: hidden;
		border-block-start: 1px solid oklch(1 0 0 / 0.1);
	}

	/* ─── Background ─── */
	.lb-bg {
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	.lb-grid-lines {
		position: absolute;
		inset: 0;
		background-image:
			linear-gradient(oklch(1 0 0 / 0.02) 1px, transparent 1px),
			linear-gradient(90deg, oklch(1 0 0 / 0.02) 1px, transparent 1px);
		background-size: 4rem 4rem;
		mask-image: radial-gradient(ellipse 60% 50% at 50% 0%, oklch(0 0 0) 70%, transparent 100%);
	}

	.lb-spotlight {
		position: absolute;
		inset: 0;
		opacity: 0.2;
	}

	/* ─── Container ─── */
	.lb-container {
		position: relative;
		max-inline-size: 100rem;
		margin-inline: auto;
		z-index: 10;
	}

	.lb-header {
		max-inline-size: 56rem;
		margin-inline: auto;
		text-align: center;
		margin-block-end: 6rem;
	}

	.lb-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding-inline: 1rem;
		padding-block: 0.375rem;
		border: 1px solid oklch(0.45 0.12 70 / 0.3);
		background-color: oklch(0.2 0.06 70 / 0.1);
		color: oklch(0.75 0.15 85);
		font-size: 0.625rem;
		font-weight: var(--weight-bold);
		letter-spacing: 0.3em;
		text-transform: uppercase;
		margin-block-end: 2rem;
		border-radius: 2px;
	}

	.lb-title {
		font-size: clamp(3rem, 6vw, 4.5rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 2rem;
		letter-spacing: -0.02em;
	}

	.lb-title-muted {
		color: oklch(0.35 0.01 265);
	}

	.lb-subtitle {
		font-size: var(--text-lg);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-inline: auto;
	}

	/* ─── Layout ─── */
	.lb-layout {
		display: grid;
		gap: 3rem;

		@media (min-width: 1024px) {
			grid-template-columns: 8fr 4fr;
		}
	}

	/* ─── Lead Card ─── */
	.lb-lead-col {
		block-size: 100%;
	}

	.lb-lead-card {
		position: relative;
		display: block;
		block-size: 100%;
		background-color: oklch(0.08 0 0);
		border: 1px solid oklch(1 0 0 / 0.1);
		overflow: hidden;
		transition: border-color 500ms;
		text-decoration: none;

		&:hover {
			border-color: oklch(0.6 0.15 70 / 0.5);
		}
		&:hover .lb-lead-bg {
			transform: scale(1.05);
		}
		&:hover .lb-lead-title {
			color: oklch(0.75 0.15 85);
		}
		&:hover .lb-lead-excerpt {
			border-inline-start-color: oklch(0.75 0.15 85 / 0.5);
		}
	}

	.lb-lead-image {
		position: relative;
		block-size: 25rem;
		overflow: hidden;
	}

	.lb-lead-bg {
		position: absolute;
		inset: 0;
		background-size: cover;
		background-position: center;
		transition: transform 700ms;
		opacity: 0.6;
	}

	.lb-lead-bg-fallback {
		position: absolute;
		inset: 0;
		background: linear-gradient(
			to bottom right,
			oklch(0.25 0.01 265),
			oklch(0 0 0),
			oklch(0.25 0.01 265)
		);
		opacity: 0.6;
	}

	.lb-lead-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(to top, oklch(0.08 0 0), oklch(0.08 0 0 / 0.5), transparent);
	}

	.lb-lead-tag-row {
		position: absolute;
		inset-block-start: 1.5rem;
		inset-inline-start: 1.5rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.lb-lead-tag {
		padding-inline: 0.75rem;
		padding-block: 0.25rem;
		background-color: oklch(0 0 0 / 0.8);
		backdrop-filter: blur(8px);
		border: 1px solid oklch(1 0 0 / 0.1);
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.75 0.15 85);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	.lb-lead-content {
		position: relative;
		margin-block-start: -8rem;
		padding: 2rem;
		z-index: 10;

		@media (min-width: 768px) {
			padding: 3rem;
		}
	}

	.lb-lead-meta {
		display: flex;
		align-items: center;
		gap: 1rem;
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.55 0.01 265);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		margin-block-end: 1rem;
	}

	.lb-meta-time {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.lb-meta-sep {
		inline-size: 1px;
		block-size: 0.75rem;
		background-color: oklch(1 0 0 / 0.2);
	}

	.lb-lead-title {
		font-size: clamp(1.875rem, 4vw, 3rem);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 1.5rem;
		line-height: 1.2;
		transition: color 300ms;
	}

	.lb-lead-excerpt {
		font-size: var(--text-base);
		color: oklch(0.55 0.01 265);
		font-weight: 300;
		line-height: 1.7;
		max-inline-size: 32rem;
		margin-block-end: 2rem;
		border-inline-start: 2px solid oklch(1 0 0 / 0.1);
		padding-inline-start: 1.5rem;
		transition: border-color 300ms;
	}

	.lb-read-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: oklch(1 0 0);
	}

	:global(.lb-read-arrow) {
		color: oklch(0.75 0.15 85);
		transition: transform 300ms;
	}
	.lb-lead-card:hover :global(.lb-read-arrow) {
		transform: translateX(0.5rem);
	}

	/* ─── Wire Column ─── */
	.lb-wire-col {
		display: flex;
		flex-direction: column;
		block-size: 100%;
		border-inline-start: 1px solid oklch(1 0 0 / 0.05);

		@media (min-width: 1024px) {
			padding-inline-start: 3rem;
		}
	}

	.lb-wire-header {
		margin-block-end: 2rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: var(--text-xs);
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: oklch(0.55 0.01 265);
	}

	.lb-wire-list {
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.lb-wire-item {
		display: block;
		border-block-end: 1px solid oklch(1 0 0 / 0.05);
		padding-block-end: 2rem;
		transition: padding-inline-start 300ms;
		text-decoration: none;

		&:last-child {
			border-block-end: none;
		}
		&:hover {
			padding-inline-start: 1rem;
		}
		&:hover .lb-wire-title {
			color: oklch(1 0 0);
		}
		&:hover .lb-wire-read {
			color: oklch(0.75 0.15 85);
		}
	}

	.lb-wire-top {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: 0.5rem;
	}

	.lb-wire-time {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.6 0.15 70);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	:global(.lb-wire-candle) {
		color: oklch(0.4 0.01 265);
		transition: color 200ms;
	}
	.lb-wire-item:hover :global(.lb-wire-candle) {
		color: oklch(0.75 0.15 85);
	}

	.lb-wire-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-medium);
		color: oklch(0.8 0.01 265);
		transition: color 200ms;
		margin-block-end: 0.5rem;
		line-height: 1.4;
	}

	.lb-wire-bottom {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-start: 0.75rem;
	}

	.lb-wire-author {
		font-size: 0.625rem;
		font-family: var(--font-mono, monospace);
		color: oklch(0.4 0.01 265);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	.lb-wire-read {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.6875rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: oklch(0.55 0.01 265);
		transition: color 300ms;
	}

	:global(.lb-wire-arrow) {
		transition: transform 300ms;
	}
	.lb-wire-item:hover :global(.lb-wire-arrow) {
		transform: translateX(0.25rem);
	}

	/* ─── Empty ─── */
	.lb-empty {
		padding-block: 6rem;
		text-align: center;
		border: 1px dashed oklch(1 0 0 / 0.1);
		border-radius: 2px;
	}

	.lb-empty-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		inline-size: 3rem;
		block-size: 3rem;
		border-radius: 50%;
		background-color: oklch(1 0 0 / 0.05);
		color: oklch(0.55 0.01 265);
		margin-block-end: 1rem;
	}

	.lb-empty-title {
		font-size: var(--text-lg);
		font-family: var(--font-serif, serif);
		color: oklch(1 0 0);
		margin-block-end: 0.5rem;
	}

	.lb-empty-text {
		font-size: var(--text-sm);
		font-family: var(--font-mono, monospace);
		color: oklch(0.55 0.01 265);
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}
</style>
