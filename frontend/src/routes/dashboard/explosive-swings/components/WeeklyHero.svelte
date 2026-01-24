<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * WeeklyHero Component - Collapsible Weekly Video Accordion
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Hero section with video breakdown and trade plan tabs
	 * @version 5.0.0 - ICT 7 Security & Accessibility Refactor
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * SVELTE 5 EVIDENCE-BASED DECISION:
	 * Per Svelte 5 docs, components are "building blocks" for self-contained units
	 * with their own state and behavior. This section has:
	 * - Own state: heroTab, isCollapsed, expandedTradeNotes
	 * - Own behavior: tab switching, collapse/expand, notes toggle
	 * - ~200 lines HTML + ~300 lines CSS = distinct UI block
	 */

	import { onMount } from 'svelte';

	// ═══════════════════════════════════════════════════════════════════════════════
	// SECURITY: XSS-Safe Video URL Validation (P0)
	// ═══════════════════════════════════════════════════════════════════════════════
	const VIDEO_EMBED_ORIGINS = [
		'iframe.mediadelivery.net',  // Bunny.net
		'player.vimeo.com',
		'www.youtube.com',
		'youtube.com',
		'vimeo.com'
	] as const;

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
		bias: string;
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
		videoUrl = weeklyContent.videoUrl || '/dashboard/explosive-swings/video/weekly',
		sheetUrl = 'https://docs.google.com/spreadsheets/d/your-sheet-id',
		isAdmin = false,
		roomSlug = 'explosive-swings',
		onAddEntry,
		onEditEntry,
		onUploadVideo
	}: Props = $props();

	// Component-local state (Svelte 5 $state rune)
	let heroTab = $state<'video' | 'entries'>('video');
	let isCollapsed = $state(false);
	let expandedTradeNotes = $state(new Set<string>());
	
	// Video player state - ICT 7 Enhancement
	let isVideoPlaying = $state(false);
	let isVideoExpanded = $state(false);
	let videoPlayerRef = $state<HTMLDivElement | null>(null);
	let videoLoadError = $state(false);
	
	// Accessibility: ARIA live region announcements
	let announcement = $state('');
	
	function announce(message: string) {
		announcement = message;
		setTimeout(() => { announcement = ''; }, 1000);
	}

	function toggleCollapse() {
		isCollapsed = !isCollapsed;
		announce(isCollapsed ? 'Section collapsed' : 'Section expanded');
	}
	
	// Video player controls
	function playVideo() {
		// Only play if we have a valid video URL
		if (!canPlayVideo) {
			console.warn('[WeeklyHero] Cannot play video: Invalid or missing video URL');
			return;
		}
		videoLoadError = false;
		isVideoPlaying = true;
		announce('Video playing');
	}
	
	function closeVideo() {
		isVideoPlaying = false;
		isVideoExpanded = false;
		announce('Video closed');
	}
	
	async function toggleFullscreen() {
		try {
			if (!isVideoExpanded && videoPlayerRef) {
				await (videoPlayerRef.requestFullscreen?.() ?? 
					(videoPlayerRef as any).webkitRequestFullscreen?.() ??
					Promise.reject(new Error('Fullscreen not supported')));
				isVideoExpanded = true;
			} else if (document.fullscreenElement) {
				await document.exitFullscreen();
				isVideoExpanded = false;
			}
		} catch (err) {
			console.warn('[WeeklyHero] Fullscreen request failed:', err);
			isVideoExpanded = false;
		}
	}
	
	function handleVideoError() {
		videoLoadError = true;
		console.error('[WeeklyHero] Video iframe failed to load');
	}
	
	function handleVideoLoad() {
		videoLoadError = false;
	}
	
	// ═══════════════════════════════════════════════════════════════════════════════
	// XSS-Safe URL Validation Functions (P0 Security)
	// ═══════════════════════════════════════════════════════════════════════════════
	function isValidVideoUrl(url: string): boolean {
		if (!url || typeof url !== 'string' || url.trim() === '') return false;
		try {
			const parsed = new URL(url);
			if (parsed.protocol !== 'https:') return false;
			return VIDEO_EMBED_ORIGINS.some(origin => 
				parsed.hostname === origin || parsed.hostname.endsWith(`.${origin}`)
			);
		} catch {
			return false;
		}
	}

	function generateEmbedUrl(url: string): string {
		if (!isValidVideoUrl(url)) return '';
		try {
			const parsed = new URL(url);
			// Bunny.net - already an embed URL
			if (parsed.hostname.includes('iframe.mediadelivery.net')) {
				parsed.searchParams.set('autoplay', 'true');
				return parsed.toString();
			}
			// Vimeo transform
			if (parsed.hostname.includes('vimeo.com') && !parsed.hostname.includes('player.')) {
				const match = parsed.pathname.match(/\/(\d+)/);
				if (match) return `https://player.vimeo.com/video/${match[1]}?autoplay=1`;
			}
			// YouTube transform  
			if (parsed.hostname.includes('youtube.com') || parsed.hostname.includes('youtu.be')) {
				const videoId = parsed.hostname.includes('youtu.be') 
					? parsed.pathname.slice(1)
					: parsed.searchParams.get('v');
				if (videoId) return `https://www.youtube.com/embed/${videoId}?autoplay=1`;
			}
			// Already embed URL
			if (parsed.pathname.includes('/embed/') || parsed.hostname.includes('player.')) {
				parsed.searchParams.set('autoplay', '1');
				return parsed.toString();
			}
			return '';
		} catch {
			return '';
		}
	}
	
	// Derived: Can we play video inline?
	const canPlayVideo = $derived(isValidVideoUrl(videoUrl));

	function toggleTradeNotes(ticker: string) {
		const newSet = new Set(expandedTradeNotes);
		const expanded = newSet.has(ticker);
		if (expanded) {
			newSet.delete(ticker);
		} else {
			newSet.add(ticker);
		}
		expandedTradeNotes = newSet;
		announce(`Notes ${expanded ? 'collapsed' : 'expanded'} for ${ticker}`);
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Lifecycle: Event Cleanup (P0 Security)
	// ═══════════════════════════════════════════════════════════════════════════════
	onMount(() => {
		const handleFullscreenChange = () => {
			if (!document.fullscreenElement && isVideoExpanded) {
				isVideoExpanded = false;
			}
		};
		
		const handleEscapeKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape' && isVideoPlaying) {
				closeVideo();
			}
		};
		
		document.addEventListener('fullscreenchange', handleFullscreenChange);
		document.addEventListener('keydown', handleEscapeKey);
		
		return () => {
			document.removeEventListener('fullscreenchange', handleFullscreenChange);
			document.removeEventListener('keydown', handleEscapeKey);
			if (document.fullscreenElement) {
				document.exitFullscreen().catch(() => {});
			}
		};
	});
</script>

<!-- Accessibility: ARIA Live Region for Screen Readers -->
<div class="sr-only" role="status" aria-live="polite" aria-atomic="true">
	{announcement}
</div>

<section class="hero" class:collapsed={isCollapsed} id="hero-content-panel">
	<button 
		class="hero-collapse-toggle" 
		onclick={toggleCollapse}
		aria-expanded={!isCollapsed}
		aria-controls="hero-content-panel"
	>
		<div class="hero-header-compact">
			<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20" class="video-icon">
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
			>
				<path d="M19 9l-7 7-7-7" />
			</svg>
		</div>
	</button>

	{#if !isCollapsed}
		<div class="hero-tabs-bar" role="tablist" aria-label="Content sections">
			<button 
				class="hero-tab" 
				class:active={heroTab === 'video'} 
				onclick={() => (heroTab = 'video')}
				role="tab"
				aria-selected={heroTab === 'video'}
				aria-controls="tab-panel-video"
				id="tab-video"
			>
				<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
					<path d="M8 5v14l11-7z" />
				</svg>
				Video Breakdown
			</button>
			<button
				class="hero-tab"
				class:active={heroTab === 'entries'}
				onclick={() => (heroTab = 'entries')}
				role="tab"
				aria-selected={heroTab === 'entries'}
				aria-controls="tab-panel-entries"
				id="tab-entries"
			>
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
					<rect x="3" y="3" width="18" height="18" rx="2" />
					<path d="M3 9h18M9 21V9" />
				</svg>
				Trade Plan & Entries
			</button>
		</div>

		<div class="hero-content">
			{#if heroTab === 'video'}
				<!-- VIDEO TAB - Enhanced In-Place Player -->
				<div 
					class="video-container-compact" 
					class:video-active={isVideoPlaying}
					role="tabpanel"
					id="tab-panel-video"
					aria-labelledby="tab-video"
				>
					<!-- Video Player with Blur Background -->
					<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
					<div 
						bind:this={videoPlayerRef}
						class="video-player-wrapper"
						class:playing={isVideoPlaying}
						class:expanded={isVideoExpanded}
						tabindex={isVideoPlaying ? 0 : -1}
						role="application"
						aria-label="Video player"
					>
						{#if isVideoPlaying && canPlayVideo}
							<!-- Active Video Player -->
							{@const safeEmbedUrl = generateEmbedUrl(videoUrl)}
							<div class="video-backdrop-blur"></div>
							<div class="video-frame-container">
								{#if safeEmbedUrl && !videoLoadError}
									<iframe
										src={safeEmbedUrl}
										title={weeklyContent.videoTitle}
										frameborder="0"
										allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen"
										allowfullscreen
										onerror={handleVideoError}
										onload={handleVideoLoad}
									></iframe>
								{/if}
								{#if !safeEmbedUrl || videoLoadError}
									<div class="video-error" role="alert">
										<p>Video failed to load</p>
										<button class="retry-btn" onclick={playVideo}>Retry</button>
									</div>
								{/if}
								
								<!-- Video Controls Overlay -->
								<div class="video-controls-bar">
									<button class="video-control-btn" onclick={closeVideo} aria-label="Close video">
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
											<path d="M18 6L6 18M6 6l12 12" />
										</svg>
									</button>
									<span class="video-title-overlay">{weeklyContent.videoTitle}</span>
									<button class="video-control-btn" onclick={toggleFullscreen} aria-label="Toggle fullscreen">
										{#if isVideoExpanded}
											<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
												<path d="M8 3v3a2 2 0 01-2 2H3m18 0h-3a2 2 0 01-2-2V3m0 18v-3a2 2 0 012-2h3M3 16h3a2 2 0 012 2v3" />
											</svg>
										{:else}
											<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
												<path d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3" />
											</svg>
										{/if}
									</button>
								</div>
							</div>
						{:else}
							<!-- Thumbnail with Play Button -->
							<div class="video-player-compact" style="background-image: url('{weeklyContent.thumbnail}')">
								<div class="video-overlay">
									<button class="play-btn" onclick={playVideo} aria-label="Play video">
										<svg viewBox="0 0 24 24" fill="currentColor">
											<path d="M8 5v14l11-7z" />
										</svg>
									</button>
								</div>
								<div class="video-duration">{weeklyContent.duration}</div>
							</div>
						{/if}
					</div>
					
					<!-- Video Info -->
					<div class="video-info-compact" class:hidden={isVideoPlaying && !isVideoExpanded}>
						<h2>{weeklyContent.videoTitle}</h2>
						<p>Published {weeklyContent.publishedDate}</p>
						{#if isAdmin && onUploadVideo}
							<button class="admin-upload-btn" onclick={onUploadVideo}>
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
									<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
								</svg>
								Upload New Video
							</button>
						{/if}
						<a href={videoUrl} class="watch-btn">
							Watch Full Video
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
								<path d="M5 12h14M12 5l7 7-7 7" />
							</svg>
						</a>
					</div>
				</div>
			{:else}
				<!-- ENTRIES TAB - Trade Plan Sheet -->
				<div 
					class="entries-container"
					role="tabpanel"
					id="tab-panel-entries"
					aria-labelledby="tab-entries"
				>
					<div class="entries-header">
						<div class="entries-title-row">
							<div>
								<h2>This Week's Trade Plan</h2>
								<p>Complete breakdown with entries, targets, stops, and options plays</p>
							</div>
							{#if isAdmin && onAddEntry}
								<button class="admin-add-btn" onclick={onAddEntry}>
									<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
										<path d="M12 5v14M5 12h14" />
									</svg>
									Add Entry
								</button>
							{/if}
							</div>
					</div>
					<div class="trade-sheet-wrapper">
						<table class="trade-sheet">
							<thead>
								<tr>
									<th>Ticker</th>
									<th>Bias</th>
									<th>Entry</th>
									<th>Target 1</th>
									<th>Target 2</th>
									<th>Target 3</th>
									<th>Runner</th>
									<th>Stop</th>
									<th>Options</th>
									<th>Exp</th>
									<th class="notes-th">Notes</th>
								</tr>
							</thead>
							<tbody>
								{#each tradePlan as trade (trade.ticker)}
									<tr class:has-notes-open={expandedTradeNotes.has(trade.ticker)}>
									<td class="ticker-cell">
										<strong>{trade.ticker}</strong>
										{#if isAdmin && onEditEntry}
											<button 
												class="edit-entry-btn"
												onclick={() => onEditEntry(trade)}
												aria-label="Edit {trade.ticker}"
											>
												<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14">
													<path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7" />
													<path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z" />
												</svg>
											</button>
										{/if}
									</td>
									<td>
										<span class="bias bias--{trade.bias.toLowerCase()}">{trade.bias}</span>
									</td>
									<td class="entry-cell">{trade.entry}</td>
									<td class="target-cell">{trade.target1}</td>
									<td class="target-cell">{trade.target2}</td>
									<td class="target-cell">{trade.target3}</td>
									<td class="runner-cell">{trade.runner}</td>
									<td class="stop-cell">{trade.stop}</td>
									<td class="options-cell">{trade.optionsStrike}</td>
									<td class="exp-cell">{trade.optionsExp}</td>
									<td class="notes-toggle-cell">
										<button
											class="table-notes-btn"
											class:expanded={expandedTradeNotes.has(trade.ticker)}
											onclick={() => toggleTradeNotes(trade.ticker)}
											aria-expanded={expandedTradeNotes.has(trade.ticker)}
											aria-controls="notes-{trade.ticker}"
											aria-label="{expandedTradeNotes.has(trade.ticker) ? 'Hide' : 'Show'} notes for {trade.ticker}"
										>
											<svg
												class="chevron-icon"
												viewBox="0 0 24 24"
												fill="none"
												stroke="currentColor"
												stroke-width="2.5"
												width="18"
												height="18"
											>
												<path d="M19 9l-7 7-7-7" />
											</svg>
										</button>
									</td>
								</tr>
									{#if expandedTradeNotes.has(trade.ticker)}
										<tr class="notes-row expanded" id="notes-{trade.ticker}">
											<td colspan="11">
												<div class="trade-notes-panel">
													<div class="trade-notes-badge">{trade.ticker}</div>
													<p>{trade.notes}</p>
												</div>
											</td>
										</tr>
									{/if}
								{/each}
							</tbody>
						</table>
					</div>
					<div class="sheet-footer">
						<a href={sheetUrl} target="_blank" class="google-sheet-link">
							<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
								<path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 17h2v-7H7v7zm4 0h2V7h-2v10zm4 0h2v-4h-2v4z" />
							</svg>
							Open in Google Sheets
						</a>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY: Screen Reader Only Utility
	   ═══════════════════════════════════════════════════════════════════════════ */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HERO SECTION - Collapsible Accordion
	   Design Tokens: Dashboard color scheme
	   ═══════════════════════════════════════════════════════════════════════════ */
	.hero {
		/* Design Tokens */
		--color-dashboard-accent: #143e59;
		--color-dashboard-accent-dark: #0f2d42;
		--color-dashboard-primary: #f69532;
		--color-dashboard-primary-dark: #e8850d;
		
		/* Aspect Ratios */
		--aspect-16-9: 56.25%;
		--aspect-compact: 28%;
		
		/* Transitions */
		--transition-fast: 0.15s ease;
		--transition-normal: 0.2s ease;
		--transition-slow: 0.3s ease;
		
		background: linear-gradient(135deg, var(--color-dashboard-primary) 0%, var(--color-dashboard-primary-dark) 50%, #d4790a 100%);
		padding: 0;
		transition: all var(--transition-slow);
	}

	.hero.collapsed {
		background: linear-gradient(135deg, #f69532 0%, #e8850d 100%);
	}

	.hero-collapse-toggle {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 18px 30px;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: background 0.2s;
	}

	.hero-collapse-toggle:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.hero-header-compact {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.hero-header-compact h1 {
		color: #fff;
		font-size: 18px;
		font-weight: 700;
		margin: 0;
		font-family: 'Montserrat', sans-serif;
	}

	.video-icon {
		color: #fff;
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
		transition: transform 0.3s ease;
	}

	.collapse-chevron.rotated {
		transform: rotate(180deg);
	}

	.hero-tabs-bar {
		display: flex;
		gap: 10px;
		padding: 0 30px 20px;
		justify-content: center;
	}

	.hero-tab {
		display: flex;
		align-items: center;
		gap: 8px;
		background: rgba(255, 255, 255, 0.15);
		border: 2px solid rgba(255, 255, 255, 0.3);
		color: #fff;
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.hero-tab:hover {
		background: rgba(255, 255, 255, 0.25);
	}

	.hero-tab.active {
		background: #fff;
		color: #f69532;
		border-color: #fff;
	}

	.hero-content {
		padding: 40px;
	}

	/* VIDEO TAB - Compact Version */
	.video-container-compact {
		display: flex;
		gap: 30px;
		align-items: center;
		max-width: 1000px;
		margin: 0 auto;
	}

	.video-player-compact {
		flex: 0 0 50%;
		position: relative;
		padding-bottom: 28%;
		background-size: cover;
		background-position: center;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 15px 40px rgba(0, 0, 0, 0.25);
	}

	.video-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.3s;
	}

	.video-player-compact:hover .video-overlay {
		background: rgba(0, 0, 0, 0.4);
	}

	.play-btn {
		width: 60px;
		height: 60px;
		background: #fff;
		border: none;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: transform 0.3s;
		box-shadow: 0 8px 30px rgba(0, 0, 0, 0.3);
	}

	.play-btn svg {
		width: 24px;
		height: 24px;
		color: #f69532;
		margin-left: 3px;
	}

	.video-player-compact:hover .play-btn {
		transform: scale(1.1);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ENHANCED VIDEO PLAYER - ICT 7 In-Place Playback
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-player-wrapper {
		flex: 0 0 50%;
		position: relative;
		transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.video-player-wrapper .video-player-compact {
		flex: none;
		padding-bottom: 56.25%;
	}

	.video-player-wrapper.playing {
		box-shadow: 0 25px 60px rgba(0, 0, 0, 0.4);
	}

	.video-player-wrapper.expanded {
		position: fixed;
		inset: 0;
		z-index: 99999;
		flex: none;
		border-radius: 0;
	}

	.video-backdrop-blur {
		position: absolute;
		inset: -50px;
		background: linear-gradient(135deg, #f69532 0%, #e8850d 100%);
		filter: blur(30px);
		opacity: 0.3;
		z-index: -1;
	}

	.video-frame-container {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		background: #000;
		border-radius: 12px;
		overflow: hidden;
	}

	.video-player-wrapper.expanded .video-frame-container {
		height: 100%;
		padding-bottom: 0;
		border-radius: 0;
	}

	.video-frame-container iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border: none;
	}

	.video-controls-bar {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 16px;
		background: linear-gradient(to bottom, rgba(0,0,0,0.7) 0%, transparent 100%);
		z-index: 10;
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.video-frame-container:hover .video-controls-bar,
	.video-player-wrapper.expanded .video-controls-bar {
		opacity: 1;
	}

	.video-control-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: rgba(255, 255, 255, 0.15);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		border: none;
		border-radius: 8px;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.video-control-btn:hover {
		background: rgba(255, 255, 255, 0.25);
		transform: scale(1.05);
	}

	.video-title-overlay {
		flex: 1;
		text-align: center;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		text-shadow: 0 1px 3px rgba(0,0,0,0.5);
		padding: 0 16px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Video Info hidden when playing inline */
	.video-info-compact.hidden {
		display: none;
	}

	/* Video Error State */
	.video-error {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 16px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		font-size: 16px;
	}

	.video-error p {
		margin: 0;
	}

	.retry-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 10px 20px;
		background: var(--color-dashboard-accent, #143e59);
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all var(--transition-normal, 0.2s ease);
	}

	.retry-btn:hover {
		background: var(--color-dashboard-accent-dark, #0f2d42);
		transform: scale(1.05);
	}

	.video-duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 5px 10px;
		border-radius: 5px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-info-compact {
		flex: 1;
		color: #fff;
		text-align: center;
	}

	.video-info-compact h2 {
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 8px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.video-info-compact p {
		font-size: 13px;
		opacity: 0.9;
		margin: 0 0 20px 0;
	}

	.watch-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: var(--color-dashboard-accent, #143e59);
		color: #fff;
		padding: 16px 32px;
		border-radius: 10px;
		font-size: 16px;
		font-weight: 700;
		text-decoration: none;
		transition: all var(--transition-slow, 0.3s ease);
	}

	.watch-btn:hover {
		background: var(--color-dashboard-accent-dark, #0f2d42);
		transform: translateY(-2px);
	}

	/* ENTRIES TAB - Trade Sheet */
	.entries-container {
		max-width: 1400px;
		margin: 0 auto;
	}

	.entries-header {
		text-align: center;
		color: #fff;
		margin-bottom: 30px;
	}

	.entries-header h2 {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 8px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.entries-header p {
		font-size: 15px;
		opacity: 0.9;
		margin: 0;
	}

	.trade-sheet-wrapper {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
	}

	.trade-sheet {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.trade-sheet thead {
		background: var(--color-dashboard-accent, #143e59);
		color: #fff;
	}

	.trade-sheet th {
		padding: 16px 12px;
		text-align: center;
		font-weight: 700;
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.trade-sheet tbody tr:nth-child(4n + 1),
	.trade-sheet tbody tr:nth-child(4n + 2) {
		background: #f8fafc;
	}

	.trade-sheet td {
		padding: 14px 12px;
		text-align: center;
		border-bottom: 1px solid #e5e7eb;
	}

	.ticker-cell {
		position: relative;
	}

	.ticker-cell strong {
		font-size: 16px;
		color: var(--color-dashboard-accent, #143e59);
	}

	.edit-entry-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		margin-left: 8px;
		padding: 0;
		background: transparent;
		border: 1px solid #e2e8f0;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s ease;
		vertical-align: middle;
	}

	.edit-entry-btn:hover {
		background: var(--color-dashboard-accent, #143e59);
		border-color: var(--color-dashboard-accent, #143e59);
		color: #fff;
		transform: scale(1.1);
	}

	.bias {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
	}

	.bias--bullish {
		background: #dcfce7;
		color: #166534;
	}

	.bias--bearish {
		background: #fee2e2;
		color: #991b1b;
	}

	.bias--neutral {
		background: #fef3c7;
		color: #92400e;
	}

	.entry-cell {
		font-weight: 700;
		color: var(--color-dashboard-accent, #143e59);
	}

	.target-cell {
		color: #22c55e;
		font-weight: 600;
	}

	.runner-cell {
		color: #0ea5e9;
		font-weight: 700;
	}

	.stop-cell {
		color: #ef4444;
		font-weight: 600;
	}

	.options-cell {
		font-weight: 600;
		color: #7c3aed;
	}

	.exp-cell {
		font-size: 12px;
		color: #666;
	}

	.notes-th {
		width: 60px;
		text-align: center;
	}

	.notes-toggle-cell {
		text-align: center;
		vertical-align: middle;
	}

	.table-notes-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #f1f5f9;
		border: 1.5px solid #e2e8f0;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.25s ease;
		color: #64748b;
	}

	.table-notes-btn:hover {
		background: #e2e8f0;
		border-color: var(--color-dashboard-accent, #143e59);
		color: var(--color-dashboard-accent, #143e59);
	}

	.table-notes-btn.expanded {
		background: var(--color-dashboard-accent, #143e59);
		border-color: var(--color-dashboard-accent, #143e59);
		color: #fff;
	}

	.table-notes-btn .chevron-icon {
		transition: transform 0.3s ease;
	}

	.table-notes-btn.expanded .chevron-icon {
		transform: rotate(180deg);
	}

	tr.has-notes-open td {
		background: #f0f9ff !important;
	}

	.notes-row.expanded td {
		text-align: left;
		padding: 0;
		background: transparent !important;
		border-bottom: 2px solid var(--color-dashboard-accent, #143e59);
	}

	.trade-notes-panel {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		padding: 18px 24px;
		background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
		animation: notesSlide 0.3s ease;
	}

	@keyframes notesSlide {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.trade-notes-badge {
		flex-shrink: 0;
		background: var(--color-dashboard-accent, #143e59);
		color: #fff;
		font-size: 12px;
		font-weight: 800;
		padding: 6px 14px;
		border-radius: 6px;
		letter-spacing: 0.05em;
	}

	.trade-notes-panel p {
		flex: 1;
		font-size: 14px;
		color: #0c4a6e;
		line-height: 1.7;
		margin: 0;
		font-weight: 500;
	}

	.sheet-footer {
		padding: 20px;
		text-align: center;
	}

	.google-sheet-link {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: var(--color-dashboard-accent, #143e59);
		color: #fff;
		padding: 14px 28px;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		transition: all var(--transition-slow, 0.3s ease);
	}

	.google-sheet-link:hover {
		background: var(--color-dashboard-accent-dark, #0f2d42);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS (Repository Standard)
	   sm: 640px, md: 768px, lg: 1024px, xl: 1280px
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1023px) {
		.video-container-compact {
			flex-direction: column;
		}

		.video-player-compact {
			flex: none;
			width: 100%;
			padding-bottom: 56.25%;
		}

		.trade-sheet-wrapper {
			overflow-x: auto;
		}

		.trade-sheet {
			min-width: 900px;
		}
	}

	@media (max-width: 767px) {
		.hero-tabs-bar {
			flex-direction: column;
			padding: 0 20px 20px;
		}

		.hero-tab {
			width: 100%;
			justify-content: center;
		}

		.hero-content {
			padding: 24px 16px;
		}

		.hero-header-compact h1 {
			font-size: 15px;
		}

		.hero-collapse-toggle {
			padding: 14px 16px;
		}

		.video-info-compact h2 {
			font-size: 20px;
		}

		.watch-btn {
			padding: 14px 24px;
			font-size: 14px;
		}

		.entries-header h2 {
			font-size: 22px;
		}
	}

	@media (max-width: 639px) {
		.hero-header-compact h1 {
			font-size: 14px;
		}

		.collapse-indicator span {
			display: none;
		}

		.video-info-compact h2 {
			font-size: 18px;
		}

		.video-info-compact p {
			font-size: 12px;
		}

		.entries-header h2 {
			font-size: 20px;
		}

		.entries-header p {
			font-size: 13px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ADMIN CONTROLS - ICT 7 Principal Engineer Standards
	   ═══════════════════════════════════════════════════════════════════════════ */
	.entries-title-row {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 16px;
	}

	.admin-add-btn,
	.admin-upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		background: var(--color-dashboard-accent, #143e59);
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		color: white;
		cursor: pointer;
		transition: all var(--transition-fast, 0.15s ease);
		white-space: nowrap;
	}

	.admin-add-btn:hover,
	.admin-upload-btn:hover {
		background: var(--color-dashboard-accent-dark, #0f2d42);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
	}

	.admin-upload-btn {
		margin-bottom: 12px;
	}

	@media (max-width: 768px) {
		.entries-title-row {
			flex-direction: column;
			gap: 12px;
		}

		.admin-add-btn,
		.admin-upload-btn {
			width: 100%;
			justify-content: center;
		}
	}
</style>
