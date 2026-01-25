<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * WeeklyHero Component - Apple Principal Engineer ICT Level 7
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Hero section with video breakdown and trade plan tabs
	 * @version 5.0.0 - Complete rebuild with proper layout, frosted glass play button
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * Layout: 50/50 split on desktop, stacked on mobile
	 * Play Button: 80px frosted glass with vibrancy effect
	 * Typography: Montserrat headers, system UI body
	 * Grid: 8pt spacing system (8, 16, 24, 32, 48, 64px)
	 * Easing: cubic-bezier(0.16, 1, 0.3, 1) for all animations
	 */

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
	// Safe embed URL for iframe (computed from embedUrl)
	const safeEmbedUrl = $derived.by(() => getEmbedUrl(embedUrl));

	// Component state (Svelte 5 $state rune)
	let heroTab = $state<'video' | 'entries'>('video');
	let isCollapsed = $state(false);
	let expandedTradeNotes = $state(new Set<string>());

	// Video player state
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

	// Keyboard handler - Escape closes video/expanded mode
	function handleVideoKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (isVideoExpanded) {
				isVideoExpanded = false;
			} else if (isVideoPlaying) {
				closeVideo();
			}
		}
	}

	// Bunny.net embed URL generator with strict validation
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

	function toggleTradeNotes(ticker: string) {
		const newSet = new Set(expandedTradeNotes);
		if (newSet.has(ticker)) {
			newSet.delete(ticker);
		} else {
			newSet.add(ticker);
		}
		expandedTradeNotes = newSet;
	}
</script>

<svelte:window onkeydown={handleVideoKeydown} />

<section class="hero" class:collapsed={isCollapsed}>
	<!-- Collapse Toggle Header -->
	<button class="hero-collapse-toggle" onclick={toggleCollapse} type="button">
		<div class="hero-header-compact">
			<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20" class="video-icon" aria-hidden="true">
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
		<nav class="hero-tabs-bar" aria-label="Hero content tabs">
			<button
				class="hero-tab"
				class:active={heroTab === 'video'}
				onclick={() => (heroTab = 'video')}
				type="button"
				aria-pressed={heroTab === 'video'}
			>
				<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18" aria-hidden="true">
					<path d="M8 5v14l11-7z" />
				</svg>
				Video Breakdown
			</button>
			<button
				class="hero-tab"
				class:active={heroTab === 'entries'}
				onclick={() => (heroTab = 'entries')}
				type="button"
				aria-pressed={heroTab === 'entries'}
			>
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18" aria-hidden="true">
					<rect x="3" y="3" width="18" height="18" rx="2" />
					<path d="M3 9h18M9 21V9" />
				</svg>
				Trade Plan & Entries
			</button>
		</nav>

		<!-- Main Content Area -->
		<div class="hero-content">
			{#if heroTab === 'video'}
				<!-- ════════════════════════════════════════════════════════════════
				     VIDEO TAB - Player with Info Panel
				     ════════════════════════════════════════════════════════════════ -->
				<div class="video-section" class:video-expanded={isVideoExpanded}>
					<!-- Blur backdrop when video is playing -->
					{#if isVideoPlaying}
						<div class="video-backdrop" class:expanded={isVideoExpanded}></div>
					{/if}

					<div class="video-layout" class:playing={isVideoPlaying} class:expanded={isVideoExpanded}>
						<!-- Video Player Container -->
						<div class="video-player-container" class:playing={isVideoPlaying} class:expanded={isVideoExpanded}>
							{#if isVideoPlaying}
								<!-- Active Video Player with iframe -->
								<div class="video-frame">
									{#if safeEmbedUrl}
										<iframe
											src={safeEmbedUrl}
											title={weeklyContent.videoTitle}
											frameborder="0"
											allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen"
										></iframe>
									{:else}
										<div class="video-error">
											<p>Video unavailable</p>
										</div>
									{/if}
								</div>

								<!-- Video Controls Bar -->
								<div class="video-controls">
									<button
										class="control-btn close-btn"
										onclick={closeVideo}
										type="button"
										aria-label="Close video"
									>
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20" aria-hidden="true">
											<path d="M18 6L6 18M6 6l12 12" />
										</svg>
									</button>
									<span class="video-title-bar">{weeklyContent.videoTitle}</span>
									<button
										class="control-btn expand-btn"
										onclick={toggleExpand}
										type="button"
										aria-label={isVideoExpanded ? 'Exit fullscreen' : 'Enter fullscreen'}
									>
										{#if isVideoExpanded}
											<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20" aria-hidden="true">
												<path d="M8 3v3a2 2 0 01-2 2H3m18 0h-3a2 2 0 01-2-2V3m0 18v-3a2 2 0 012-2h3M3 16h3a2 2 0 012 2v3" />
											</svg>
										{:else}
											<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20" aria-hidden="true">
												<path d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3" />
											</svg>
										{/if}
									</button>
								</div>
							{:else}
								<!-- Thumbnail State with Play Button -->
								<div
									class="video-thumbnail"
									style="background-image: url('{weeklyContent.thumbnail}')"
								>
									<button
										class="play-btn"
										onclick={playVideo}
										type="button"
										aria-label="Play video: {weeklyContent.videoTitle}"
									>
										<svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
											<path d="M8 5v14l11-7z" />
										</svg>
									</button>
									<span class="duration-badge" aria-label="Duration: {weeklyContent.duration}">
										{weeklyContent.duration}
									</span>
								</div>
							{/if}
						</div>

						<!-- Video Info Panel (hidden when playing) -->
						{#if !isVideoPlaying}
							<div class="video-info">
								<h2>{weeklyContent.videoTitle}</h2>
								<p class="published-date">Published {weeklyContent.publishedDate}</p>
								<div class="video-actions">
									{#if isAdmin && onUploadVideo}
										<button class="admin-upload-btn" onclick={onUploadVideo} type="button">
											<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18" aria-hidden="true">
												<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
											</svg>
											Upload New Video
										</button>
									{/if}
									<a href={watchFullUrl} class="watch-btn">
										Watch Full Video
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18" aria-hidden="true">
											<path d="M5 12h14M12 5l7 7-7 7" />
										</svg>
									</a>
								</div>
							</div>
						{/if}
					</div>
				</div>
			{:else}
				<!-- ENTRIES TAB - Trade Plan Sheet -->
				<div class="entries-container">
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
												aria-label="Toggle notes for {trade.ticker}"
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
										<tr class="notes-row expanded">
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
	   HERO SECTION - Collapsible Accordion
	   Apple ICT 7 Responsive Design - All Breakpoints
	   ═══════════════════════════════════════════════════════════════════════════ */
	.hero {
		background: linear-gradient(135deg, #f69532 0%, #e8850d 50%, #d4790a 100%);
		overflow: hidden;
		transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.hero.collapsed {
		background: linear-gradient(135deg, #f69532 0%, #e8850d 100%);
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
		font-family: 'Montserrat', -apple-system, BlinkMacSystemFont, sans-serif;
		font-size: 17px;
		font-weight: 700;
		color: #fff;
		margin: 0;
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
		transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.collapse-chevron.rotated {
		transform: rotate(180deg);
	}

	.hero-tabs-bar {
		display: flex;
		gap: 12px;
		justify-content: center;
		padding: 0 24px 24px;
	}

	.hero-tab {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: rgba(255, 255, 255, 0.15);
		border: 2px solid rgba(255, 255, 255, 0.25);
		border-radius: 10px;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.hero-tab:hover {
		background: rgba(255, 255, 255, 0.25);
		border-color: rgba(255, 255, 255, 0.35);
	}

	.hero-tab.active {
		background: #fff;
		border-color: #fff;
		color: #f69532;
	}

	.hero-content {
		position: relative;
		padding: 40px 48px;
		min-height: 360px;
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO SECTION - Contained within max-width
	   ═══════════════════════════════════════════════════════════════════════════ */
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

	/* Blur backdrop when video is playing */
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
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO LAYOUT - 50/50 Split Desktop
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-layout {
		display: flex;
		gap: 48px;
		align-items: center;
		position: relative;
		z-index: 1;
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO PLAYER CONTAINER - 50% width, max 640px
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-player-container {
		position: relative;
		flex: 0 0 50%;
		max-width: 640px;
		aspect-ratio: 16 / 9;
		border-radius: 12px;
		overflow: hidden;
		background: #143E59;
		transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.video-player-container.playing {
		flex: none;
		width: 85%;
		max-width: 1000px;
		transform: translateY(-8px);
		box-shadow: 
			0 32px 64px rgba(0, 0, 0, 0.5),
			0 16px 32px rgba(0, 0, 0, 0.3);
		border-radius: 16px;
	}

	.video-player-container.expanded {
		width: 100%;
		max-width: 100%;
		height: 100%;
		max-height: 100%;
		aspect-ratio: unset;
		transform: none;
		border-radius: 12px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO THUMBNAIL - Navy fallback with gradient overlay
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-thumbnail {
		position: absolute;
		inset: 0;
		background-color: #143E59;
		background-size: cover;
		background-position: center;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.video-thumbnail::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			rgba(20, 62, 89, 0.4) 0%,
			rgba(0, 0, 0, 0.5) 100%
		);
		transition: background 0.3s ease;
	}

	.video-player-container:hover .video-thumbnail::before {
		background: linear-gradient(
			135deg,
			rgba(20, 62, 89, 0.3) 0%,
			rgba(0, 0, 0, 0.4) 100%
		);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PLAY BUTTON - Frosted Glass Vibrancy (CRITICAL - Must be visible)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.play-btn {
		position: relative;
		z-index: 2;
		width: 80px;
		height: 80px;
		border-radius: 50%;
		cursor: pointer;
		
		/* Frosted Glass Effect */
		background: rgba(255, 255, 255, 0.25);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		
		/* Border for definition */
		border: 2px solid rgba(255, 255, 255, 0.3);
		
		/* Shadow for depth */
		box-shadow: 
			0 8px 32px rgba(0, 0, 0, 0.3),
			0 0 0 1px rgba(255, 255, 255, 0.1) inset;
		
		/* Layout */
		display: flex;
		align-items: center;
		justify-content: center;
		
		/* Transition */
		transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.play-btn:hover {
		transform: scale(1.1);
		background: rgba(255, 255, 255, 0.4);
		border-color: rgba(255, 255, 255, 0.5);
		box-shadow: 
			0 12px 48px rgba(0, 0, 0, 0.4),
			0 0 80px rgba(255, 255, 255, 0.2),
			0 0 0 1px rgba(255, 255, 255, 0.2) inset;
	}

	.play-btn:active {
		transform: scale(0.95);
		background: rgba(255, 255, 255, 0.5);
	}

	.play-btn:focus-visible {
		outline: 3px solid rgba(255, 255, 255, 0.8);
		outline-offset: 4px;
	}

	.play-btn svg {
		width: 36px;
		height: 36px;
		color: #fff;
		margin-left: 4px; /* Optical center adjustment */
		filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.3));
		transition: all 0.3s ease;
	}

	.play-btn:hover svg {
		transform: scale(1.05);
		filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.4));
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DURATION BADGE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.duration-badge {
		position: absolute;
		bottom: 12px;
		right: 12px;
		padding: 6px 12px;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
		color: #fff;
		z-index: 5;
	}

	/* Video Frame (iframe container) */
	.video-frame {
		position: absolute;
		inset: 0;
	}

	.video-frame iframe {
		width: 100%;
		height: 100%;
		border: none;
	}

	.video-error {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #1a1a1a;
		color: #888;
		font-size: 14px;
	}

	.video-error p {
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO CONTROLS - Top Bar
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-controls {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		background: linear-gradient(to bottom, rgba(0, 0, 0, 0.8) 0%, rgba(0, 0, 0, 0.4) 60%, transparent 100%);
		z-index: 20;
		opacity: 0;
		transition: opacity 0.25s ease;
	}

	.video-player-container:hover .video-controls {
		opacity: 1;
	}

	/* Always show controls in expanded mode */
	.video-player-container.expanded .video-controls {
		opacity: 1;
		padding: 20px 24px;
	}

	.control-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(255, 255, 255, 0.2);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.control-btn:hover {
		background: rgba(255, 255, 255, 0.35);
		transform: scale(1.05);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.control-btn:active {
		transform: scale(0.95);
	}

	.video-title-bar {
		flex: 1;
		text-align: center;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		padding: 0 16px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO INFO PANEL - Typography & Buttons
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-info {
		flex: 1;
		min-width: 280px;
		color: #fff;
		text-align: center;
	}

	.video-info h2 {
		font-family: 'Montserrat', -apple-system, BlinkMacSystemFont, sans-serif;
		font-size: 28px;
		font-weight: 700;
		line-height: 1.2;
		color: #fff;
		margin: 0 0 8px 0;
		/* Modern text wrapping - balanced lines */
		text-wrap: balance; /* stylelint-disable-line property-no-unknown */
	}

	.video-info .published-date {
		font-size: 14px;
		font-weight: 500;
		color: rgba(255, 255, 255, 0.8);
		margin: 0 0 24px 0;
	}

	.video-actions {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
		justify-content: center;
	}

	/* Primary Button - Watch Full Video */
	.watch-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		padding: 14px 28px;
		background: #143E59;
		color: #fff;
		border: 2px solid #143E59;
		border-radius: 10px;
		font-size: 15px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.watch-btn:hover {
		background: #0f2d42;
		border-color: #0f2d42;
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(20, 62, 89, 0.4);
	}

	/* Secondary Button - Upload (Admin Only) */
	.admin-upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 14px 24px;
		background: transparent;
		color: #fff;
		border: 2px solid rgba(255, 255, 255, 0.4);
		border-radius: 10px;
		font-size: 15px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.admin-upload-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.6);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ENTRIES TAB - Trade Sheet
	   ═══════════════════════════════════════════════════════════════════════════ */
	.entries-container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.entries-header {
		text-align: center;
		color: #fff;
		margin-bottom: 30px;
	}

	.entries-title-row {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 16px;
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

	.admin-add-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		background: #143E59;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		color: white;
		cursor: pointer;
		transition: all 0.15s ease;
		white-space: nowrap;
	}

	.admin-add-btn:hover {
		background: #0f2d42;
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
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
		background: #143e59;
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
		color: #143e59;
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
		background: #143E59;
		border-color: #143E59;
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
		color: #143e59;
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
		border-color: #143e59;
		color: #143e59;
	}

	.table-notes-btn.expanded {
		background: #143e59;
		border-color: #143e59;
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
		border-bottom: 2px solid #143e59;
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
		background: #143e59;
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
		background: #143e59;
		color: #fff;
		padding: 14px 28px;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s;
	}

	.google-sheet-link:hover {
		background: #0f2d42;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Apple ICT 7 Standard (8pt Grid)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Ultra-wide Screens (1920px+) */
	@media (min-width: 1920px) {
		.hero-content {
			padding: 64px 80px;
		}

		.video-section,
		.entries-container {
			max-width: 1400px;
		}
	}

	/* Large Desktop (1440px+) */
	@media (min-width: 1440px) and (max-width: 1919px) {
		.hero-content {
			padding: 56px 64px;
		}
	}

	/* Desktop (1280px - 1439px) */
	@media (min-width: 1280px) and (max-width: 1439px) {
		.hero-content {
			padding: 48px 56px;
		}
	}

	/* Small Desktop (1024px - 1279px) */
	@media (min-width: 1024px) and (max-width: 1279px) {
		.hero-content {
			padding: 40px 48px;
		}

		.video-info h2 {
			font-size: 24px;
		}
	}

	/* Tablet (768px - 1023px) */
	@media (min-width: 768px) and (max-width: 1023px) {
		.hero-content {
			padding: 32px 24px;
		}

		.video-layout {
			flex-direction: column;
			gap: 32px;
		}

		.video-player-container {
			flex: none;
			width: 100%;
			max-width: 560px;
			margin: 0 auto;
		}

		.video-info {
			width: 100%;
			text-align: center;
		}

		.video-player-container.playing {
			width: 90%;
			max-width: 800px;
		}

		.trade-sheet-wrapper {
			overflow-x: auto;
		}

		.trade-sheet {
			min-width: 900px;
		}
	}

	/* Mobile (< 768px) */
	@media (max-width: 767px) {
		.hero-collapse-toggle {
			padding: 14px 16px;
		}

		.hero-header-compact h1 {
			font-size: 15px;
		}

		.hero-tabs-bar {
			flex-direction: column;
			padding: 0 16px 16px;
		}

		.hero-tab {
			width: 100%;
			justify-content: center;
		}

		.hero-content {
			padding: 24px 16px;
		}

		.video-layout {
			flex-direction: column;
			gap: 24px;
		}

		.video-player-container {
			flex: none;
			width: 100%;
			max-width: none;
		}

		.video-info {
			width: 100%;
			text-align: center;
		}

		.video-info h2 {
			font-size: 22px;
		}

		.video-actions {
			flex-direction: column;
		}

		.watch-btn,
		.admin-upload-btn {
			width: 100%;
			justify-content: center;
		}

		.video-player-container.playing {
			width: 95%;
		}

		.video-layout.expanded {
			padding: 12px;
		}

		.play-btn {
			width: 64px;
			height: 64px;
		}

		.play-btn svg {
			width: 28px;
			height: 28px;
		}

		.entries-header h2 {
			font-size: 22px;
		}

		.entries-title-row {
			flex-direction: column;
			gap: 12px;
		}

		.admin-add-btn {
			width: 100%;
			justify-content: center;
		}

		.trade-sheet-wrapper {
			overflow-x: auto;
		}

		.trade-sheet {
			min-width: 900px;
		}
	}

	/* Small Mobile (< 480px) */
	@media (max-width: 479px) {
		.hero-header-compact h1 {
			font-size: 14px;
		}

		.collapse-indicator span {
			display: none;
		}

		.video-info h2 {
			font-size: 20px;
		}

		.video-info .published-date {
			font-size: 13px;
		}

		.entries-header h2 {
			font-size: 20px;
		}

		.entries-header p {
			font-size: 13px;
		}

		.play-btn {
			width: 56px;
			height: 56px;
		}

		.play-btn svg {
			width: 24px;
			height: 24px;
		}
	}
</style>