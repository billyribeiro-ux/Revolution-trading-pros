<script lang="ts">
	import { browser } from '$app/environment';
	import { cubicOut } from 'svelte/easing';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconNews from '@tabler/icons-svelte-runes/icons/news';
	import IconChartCandle from '@tabler/icons-svelte-runes/icons/chart-candle';
	import type { Post } from '$lib/types/post';

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
	let containerRef: HTMLElement;
	// ICT11+ Fix: Start false, set true via IntersectionObserver to trigger in: transitions
	let isVisible = $state(false);
	let mouse = $state({ x: 0, y: 0 });

	const handleMouseMove = (e: MouseEvent) => {
		if (!containerRef) return;
		const rect = containerRef.getBoundingClientRect();
		mouse.x = e.clientX - rect.left;
		mouse.y = e.clientY - rect.top;
	};

	$effect(() => {
		if (!browser) {
			isVisible = true;
			return;
		}

		// Use queueMicrotask to ensure binding is complete
		let visibilityObserver: IntersectionObserver | null = null;
		queueMicrotask(() => {
			if (!containerRef) {
				isVisible = true; // Fallback: show content
				return;
			}

			// Check if already in viewport
			const rect = containerRef.getBoundingClientRect();
			const viewportHeight = window.innerHeight || document.documentElement.clientHeight;
			if (rect.top < viewportHeight + 200) {
				isVisible = true;
				return;
			}

			visibilityObserver = new IntersectionObserver(
				(entries) => {
					if (entries[0]?.isIntersecting) {
						isVisible = true;
						visibilityObserver?.disconnect();
					}
				},
				{ threshold: 0.1 }
			);
			visibilityObserver.observe(containerRef);
		});

		return () => visibilityObserver?.disconnect();
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
	class="relative py-32 px-6 bg-[#020202] overflow-hidden border-t border-white/10"
	aria-label="Market Intelligence Wire"
>
	<div class="absolute inset-0 pointer-events-none">
		<div
			class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)]"
		></div>
		<div
			class="absolute inset-0 opacity-20"
			style="background: radial-gradient(800px circle at var(--x) var(--y), rgba(255,255,255,0.05), transparent 60%);"
		></div>
	</div>

	<div class="relative max-w-[1600px] mx-auto z-10">
		<div class="max-w-4xl mx-auto text-center mb-24">
			{#if isVisible}
				<div
					in:heavySlide={{ delay: 0, duration: 1000 }}
					class="inline-flex items-center gap-3 px-4 py-1.5 border border-amber-900/30 bg-amber-950/10 text-amber-500 text-[10px] font-bold tracking-[0.3em] uppercase mb-8 rounded-sm"
				>
					<IconNews size={14} />
					Intelligence Wire
				</div>

				<h2
					in:heavySlide={{ delay: 100 }}
					class="text-5xl md:text-7xl font-serif text-white mb-8 tracking-tight"
				>
					Market <span class="text-slate-700">Analysis.</span>
				</h2>

				<p
					in:heavySlide={{ delay: 200 }}
					class="text-lg text-slate-400 font-light leading-relaxed max-w-2xl mx-auto"
				>
					We don't publish retail content. We deliver institutional-grade market intelligence.
					Verified by quantitative analysts and professional trading desks worldwide.
				</p>
			{/if}
		</div>

		{#if posts.length > 0}
			<div class="grid lg:grid-cols-12 gap-12">
				{#if leadPost}
					<div class="lg:col-span-8 h-full">
						{#if isVisible}
							<a
								href="/blog/{leadPost.slug}"
								in:heavySlide={{ delay: 300 }}
								class="relative group block h-full bg-[#050505] border border-white/10 overflow-hidden hover:border-amber-600/50 transition-colors duration-500"
							>
								<div class="relative h-[400px] overflow-hidden">
									{#if leadPost.featured_image}
										<div
											class="absolute inset-0 bg-cover bg-center transition-transform duration-700 group-hover:scale-105 opacity-60"
											style="background-image: url('{leadPost.featured_image}')"
										></div>
									{:else}
										<div
											class="absolute inset-0 bg-gradient-to-br from-slate-900 via-black to-slate-900 opacity-60"
										></div>
									{/if}
									<div
										class="absolute inset-0 bg-gradient-to-t from-[#050505] via-[#050505]/50 to-transparent"
									></div>

									<div class="absolute top-6 left-6 flex items-center gap-2">
										<span
											class="px-3 py-1 bg-black/80 backdrop-blur border border-white/10 text-[10px] font-mono text-amber-500 uppercase tracking-widest"
										>
											Priority Brief
										</span>
									</div>
								</div>

								<div class="relative -mt-32 p-8 md:p-12 z-10">
									<div
										class="flex items-center gap-4 text-[10px] font-mono text-slate-400 uppercase tracking-widest mb-4"
									>
										<span class="flex items-center gap-2">
											<IconClock size={12} />
											{getRelativeTime(leadPost.published_at)}
										</span>
										<span class="w-px h-3 bg-white/20"></span>
										<span>{leadPost.author?.name || 'Desk Analyst'}</span>
									</div>

									<h3
										class="text-3xl md:text-5xl font-serif text-white mb-6 leading-tight group-hover:text-amber-500 transition-colors"
									>
										{leadPost.title}
									</h3>

									<p
										class="text-base text-slate-400 font-light leading-relaxed max-w-2xl mb-8 border-l-2 border-white/10 pl-6 group-hover:border-amber-500/50 transition-colors"
									>
										{leadPost.excerpt || 'Market analysis briefing. Click to access full report.'}
									</p>

									<div
										class="flex items-center gap-2 text-xs font-mono uppercase tracking-widest text-white"
									>
										<span>Read Full Protocol</span>
										<IconArrowRight
											size={14}
											class="group-hover:translate-x-2 transition-transform duration-300 text-amber-500"
										/>
									</div>
								</div>
							</a>
						{/if}
					</div>
				{/if}

				<div class="lg:col-span-4 flex flex-col h-full border-l border-white/5 lg:pl-12">
					{#if isVisible}
						<div
							in:heavySlide={{ delay: 400 }}
							class="mb-8 flex items-center gap-2 text-xs font-mono uppercase tracking-widest text-slate-500"
						>
							<IconNews size={14} />
							<span>Incoming Signals</span>
						</div>

						<div class="space-y-8">
							{#each wirePosts as post}
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
									<h4
										class="text-lg font-medium text-slate-300 group-hover:text-white transition-colors mb-2 leading-snug"
									>
										{post.title}
									</h4>
									<div class="flex items-center justify-between mt-3">
										<span class="text-[10px] font-mono text-slate-600 uppercase tracking-widest">
											{post.author?.name || 'Desk Analyst'}
										</span>
										<span
											class="read-more-link inline-flex items-center gap-1.5 text-[11px] font-mono uppercase tracking-widest text-slate-500 group-hover:text-amber-500 transition-all duration-300"
										>
											<span>Read More</span>
											<IconArrowRight
												size={12}
												class="transform group-hover:translate-x-1 transition-transform duration-300"
											/>
										</span>
									</div>
								</a>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		{:else}
			<div class="py-24 text-center border border-dashed border-white/10 rounded-sm">
				<div
					class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-white/5 text-slate-500 mb-4"
				>
					<IconNews size={24} />
				</div>
				<h3 class="text-lg font-serif text-white mb-2">Wire Silent</h3>
				<p class="text-sm font-mono text-slate-500 uppercase tracking-widest">
					No Intelligence Briefs Available
				</p>
			</div>
		{/if}
	</div>
</section>
