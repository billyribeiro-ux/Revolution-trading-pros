<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * WeeklyHero Component - Apple Principal Engineer ICT Level 7
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Hero section with video breakdown and trade plan tabs
	 * @version 6.0.0 - Refactored into smaller components (<300 lines each)
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */

	import VideoPlayer from './VideoPlayer.svelte';
	import VideoInfo from './VideoInfo.svelte';
	import HeroTabs from './HeroTabs.svelte';
	import TradePlanTable from './TradePlanTable.svelte';

	interface WeeklyContent {
		title: string;
		thumbnail: string;
		duration: string;
		videoTitle: string;
		videoUrl: string;
		publishedDate: string;
	}

	interface TradePlanEntry {
		ticker: string;
		bias: 'Bullish' | 'Bearish' | 'Neutral' | string;
		entry: string;
		target1: string;
		target2: string;
		target3: string;
		runner: string;
		stop: string;
		optionsStrike: string;
		optionsExp: string;
		notes: string;
	}

	interface Props {
		weeklyContent: WeeklyContent;
		tradePlan: TradePlanEntry[];
		videoUrl?: string;
		fullVideoUrl?: string;
		sheetUrl?: string;
		isAdmin?: boolean;
		roomSlug?: string;
		onAddEntry?: () => void;
		onEditEntry?: (entry: TradePlanEntry) => void;
		onUploadVideo?: () => void;
	}

	const {
		weeklyContent,
		tradePlan,
		videoUrl,
		fullVideoUrl,
		sheetUrl = 'https://docs.google.com/spreadsheets/d/your-sheet-id',
		isAdmin = false,
		roomSlug = 'explosive-swings',
		onAddEntry,
		onEditEntry,
		onUploadVideo
	}: Props = $props();

	// Derived values
	const embedUrl = $derived(videoUrl || weeklyContent.videoUrl || '');
	const watchFullUrl = $derived(fullVideoUrl || `/dashboard/${roomSlug}/video/weekly`);
	const safeEmbedUrl = $derived.by(() => getEmbedUrl(embedUrl));

	// Component state
	let heroTab = $state<'video' | 'entries'>('video');
	let isCollapsed = $state(false);
	let isVideoPlaying = $state(false);
	let isVideoExpanded = $state(false);

	function toggleCollapse() {
		isCollapsed = !isCollapsed;
	}

	function playVideo() {
		isVideoPlaying = true;
	}

	function closeVideo() {
		isVideoPlaying = false;
		isVideoExpanded = false;
	}

	function toggleExpand() {
		isVideoExpanded = !isVideoExpanded;
	}

	function handleVideoKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (isVideoExpanded) {
				isVideoExpanded = false;
			} else if (isVideoPlaying) {
				closeVideo();
			}
		}
	}

	function getEmbedUrl(url: string): string {
		if (!url || url.startsWith('/')) return '';

		try {
			const parsed = new URL(url);
			if (parsed.hostname !== 'iframe.mediadelivery.net') return '';

			parsed.searchParams.set('autoplay', 'true');
			parsed.searchParams.set('preload', 'true');
			parsed.searchParams.set('responsive', 'true');

			return parsed.toString();
		} catch {
			return '';
		}
	}
</script>

<svelte:window onkeydown={handleVideoKeydown} />

<section class="hero" class:collapsed={isCollapsed}>
	<!-- Collapse Toggle Header -->
	<button class="hero-collapse-toggle" onclick={toggleCollapse} type="button">
		<div class="hero-header-compact">
			<svg
				viewBox="0 0 24 24"
				fill="currentColor"
				width="20"
				height="20"
				class="video-icon"
				aria-hidden="true"
			>
				<path d="M8 5v14l11-7z" />
			</svg>
			<h1>{weeklyContent.title} — Weekly Breakdown</h1>
		</div>
		<div class="collapse-indicator">
			<span>{isCollapsed ? 'Expand' : 'Collapse'}</span>
			<svg
				class="collapse-chevron"
				class:rotated={!isCollapsed}
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2.5"
				width="20"
				height="20"
				aria-hidden="true"
			>
				<path d="M19 9l-7 7-7-7" />
			</svg>
		</div>
	</button>

	{#if !isCollapsed}
		<!-- Tab Navigation -->
		<HeroTabs activeTab={heroTab} onTabChange={(tab) => (heroTab = tab)} />

		<!-- Main Content Area -->
		<div class="hero-content">
			{#if heroTab === 'video'}
				<!-- VIDEO TAB - Player with Info Panel -->
				<div class="video-section" class:video-expanded={isVideoExpanded}>
					{#if isVideoPlaying}
						<div class="video-backdrop" class:expanded={isVideoExpanded}></div>
					{/if}

					<div class="video-layout" class:playing={isVideoPlaying} class:expanded={isVideoExpanded}>
						<VideoPlayer
							thumbnail={weeklyContent.thumbnail}
							videoTitle={weeklyContent.videoTitle}
							duration={weeklyContent.duration}
							embedUrl={safeEmbedUrl}
							isPlaying={isVideoPlaying}
							isExpanded={isVideoExpanded}
							onPlay={playVideo}
							onClose={closeVideo}
							onToggleExpand={toggleExpand}
						/>

						{#if !isVideoPlaying}
							<VideoInfo
								videoTitle={weeklyContent.videoTitle}
								publishedDate={`Published ${weeklyContent.publishedDate}`}
								watchFullUrl={watchFullUrl}
								isAdmin={isAdmin}
								onUploadVideo={onUploadVideo}
							/>
						{/if}
					</div>
				</div>
			{:else}
				<!-- ENTRIES TAB - Trade Plan Sheet -->
				<div class="entries-container">
					<TradePlanTable
						tradePlan={tradePlan}
						sheetUrl={sheetUrl}
						isAdmin={isAdmin}
						onAddEntry={onAddEntry}
						onEditEntry={onEditEntry}
					/>
				</div>
			{/if}
		</div>
	{/if}
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HERO SECTION - Main Orchestrator Component
	   Apple ICT 7 Responsive Design - Refactored for <300 lines
	   ═══════════════════════════════════════════════════════════════════════════ */
	.hero {
		background: linear-gradient(135deg, var(--color-brand-secondary) 0%, #e8850d 50%, #d4790a 100%);
		overflow: hidden;
		transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.hero.collapsed {
		background: linear-gradient(135deg, var(--color-brand-secondary) 0%, #e8850d 100%);
	}

	.hero-collapse-toggle {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 24px;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	.hero-collapse-toggle:hover {
		background: rgba(255, 255, 255, 0.08);
	}

	.hero-header-compact {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.hero-header-compact h1 {
		font-family:
			'Montserrat',
			-apple-system,
			BlinkMacSystemFont,
			sans-serif;
		font-size: 17px;
		font-weight: 700;
		color: var(--color-bg-card);
		margin: 0;
	}

	.video-icon {
		color: var(--color-bg-card);
		opacity: 0.9;
	}

	.collapse-indicator {
		display: flex;
		align-items: center;
		gap: 8px;
		color: rgba(255, 255, 255, 0.9);
		font-size: 13px;
		font-weight: 600;
	}

	.collapse-chevron {
		transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.collapse-chevron.rotated {
		transform: rotate(180deg);
	}

	.hero-content {
		position: relative;
		padding: var(--space-2);
		min-height: auto;
		overflow: hidden;
	}

	/* Video Section */
	.video-section {
		position: relative;
		max-width: 1200px;
		margin: 0 auto;
	}

	.video-section.video-expanded {
		position: absolute;
		inset: 0;
		max-width: none;
		margin: 0;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.video-backdrop {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		z-index: 100;
		animation: fadeIn 0.3s ease forwards;
	}

	.video-backdrop.expanded {
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.video-layout {
		display: flex;
		gap: var(--space-3);
		align-items: center;
		position: relative;
		z-index: 1;
		flex-direction: column;
	}

	.video-layout.playing {
		justify-content: center;
		z-index: 101;
	}

	.video-layout.expanded {
		position: absolute;
		inset: 0;
		z-index: 101;
		padding: 24px;
	}

	/* Entries Container */
	.entries-container {
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Responsive Breakpoints */
	@media (min-width: 768px) {
		.hero-content {
			padding: var(--space-4);
			min-height: 300px;
		}

		.video-layout {
			flex-direction: row;
			gap: var(--space-4);
		}
	}

	@media (min-width: 1024px) {
		.hero-content {
			padding: var(--space-5);
			min-height: 360px;
		}

		.video-layout {
			gap: var(--space-6);
		}
	}

	@media (min-width: 1440px) {
		.hero-content {
			padding: var(--space-5);
		}

		.video-section,
		.entries-container {
			max-width: none;
		}
	}

	@media (min-width: 1920px) {
		.hero-content {
			padding: var(--space-6) var(--space-8);
			min-height: 400px;
		}

		.hero-header-compact h1 {
			font-size: 20px;
		}
	}

	@media (min-width: 2560px) {
		.hero-content {
			padding: var(--space-8) var(--space-10);
			min-height: 450px;
		}

		.hero-header-compact h1 {
			font-size: 22px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.hero,
		.collapse-chevron {
			transition: none;
		}

		.video-backdrop {
			animation: none;
		}
	}
</style>
