<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * WeeklyHero Component - Collapsible Weekly Video Accordion
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Hero section with video breakdown and trade plan tabs
	 * @version 4.2.0 - Fixed video player UX (play button vibrancy, lift-off, maximize)
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @standards Apple Principal Engineer ICT 7+ Standards
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
	
	// Derived values after props are destructured
	const embedUrl = $derived(videoUrl || weeklyContent.videoUrl || '');
	const watchFullUrl = $derived(fullVideoUrl || `/dashboard/${roomSlug}/video/weekly`);

	// Component-local state (Svelte 5 $state rune)
	let heroTab = $state<'video' | 'entries'>('video');
	let isCollapsed = $state(false);
	let expandedTradeNotes = $state(new Set<string>());
	
	// Video player state
	let isVideoPlaying = $state(false);
	let isVideoExpanded = $state(false);

	function toggleCollapse() {
		isCollapsed = !isCollapsed;
	}
	
	// Video player controls
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
	
	// Keyboard handler for video player
	function handleVideoKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			if (isVideoExpanded) {
				isVideoExpanded = false;
			} else if (isVideoPlaying) {
				closeVideo();
			}
		}
	}
	
	// Generate embed URL from video URL prop with autoplay
	function getEmbedUrl(url: string): string {
		if (!url || url.startsWith('/')) return '';
		
		// Bunny.net iframe URL - strict validation
		if (url.includes('iframe.mediadelivery.net')) {
			try {
				const parsed = new URL(url);
				if (parsed.hostname !== 'iframe.mediadelivery.net') return '';
				parsed.searchParams.set('autoplay', 'true');
				return parsed.toString();
			} catch {
				return '';
			}
		}
		return '';
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

<svelte:window on:keydown={handleVideoKeydown} />

<section class="hero" class:collapsed={isCollapsed}>
	<button class="hero-collapse-toggle" onclick={toggleCollapse}>
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
		<div class="hero-tabs-bar">
			<button class="hero-tab" class:active={heroTab === 'video'} onclick={() => (heroTab = 'video')}>
				<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
					<path d="M8 5v14l11-7z" />
				</svg>
				Video Breakdown
			</button>
			<button
				class="hero-tab"
				class:active={heroTab === 'entries'}
				onclick={() => (heroTab = 'entries')}
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
				<!-- VIDEO TAB -->
				<div class="video-section" class:video-active={isVideoPlaying} class:video-expanded={isVideoExpanded}>
					
					<!-- Blur backdrop when video is playing -->
					{#if isVideoPlaying}
						<div class="video-backdrop" class:expanded={isVideoExpanded}></div>
					{/if}
					
					<div class="video-layout" class:playing={isVideoPlaying} class:expanded={isVideoExpanded}>
						<!-- Video Player -->
						<div class="video-player-container" class:playing={isVideoPlaying} class:expanded={isVideoExpanded}>
							{#if isVideoPlaying}
								<!-- Active Video Player -->
								{@const safeEmbedUrl = getEmbedUrl(embedUrl)}
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
								
								<!-- Video Controls -->
								<div class="video-controls">
									<button class="control-btn close-btn" onclick={closeVideo} aria-label="Close video">
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
											<path d="M18 6L6 18M6 6l12 12" />
										</svg>
									</button>
									<span class="video-title-bar">{weeklyContent.videoTitle}</span>
									<button class="control-btn expand-btn" onclick={toggleExpand} aria-label={isVideoExpanded ? 'Exit fullscreen' : 'Fullscreen'}>
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
							{:else}
								<!-- Thumbnail with Play Button -->
								<div class="video-thumbnail" style="background-image: url('{weeklyContent.thumbnail}')">
									<button class="play-btn" onclick={playVideo} aria-label="Play video">
										<span class="play-btn-glow"></span>
										<span class="play-btn-inner">
											<svg viewBox="0 0 24 24" fill="currentColor">
												<path d="M8 5v14l11-7z" />
											</svg>
										</span>
									</button>
									<span class="duration-badge">{weeklyContent.duration}</span>
								</div>
							{/if}
						</div>
						
						<!-- Video Info (hidden when playing) -->
						{#if !isVideoPlaying}
							<div class="video-info">
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
								<a href={watchFullUrl} class="watch-btn">
									Watch Full Video
									<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
										<path d="M5 12h14M12 5l7 7-7 7" />
									</svg>
								</a>
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
	   ═══════════════════════════════════════════════════════════════════════════ */
	.hero {
		background: linear-gradient(135deg, #f69532 0%, #e8850d 50%, #d4790a 100%);
		padding: 0;
		transition: all 0.3s ease;
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO SECTION - Lift-off Player with Vibrancy Effect
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-section {
		position: relative;
		max-width: 1000px;
		margin: 0 auto;
	}

	/* Blur backdrop when video is playing */
	.video-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		z-index: 100;
		opacity: 0;
		animation: fadeIn 0.3s ease forwards;
	}

	.video-backdrop.expanded {
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
	}

	@keyframes fadeIn {
		to { opacity: 1; }
	}

	.video-layout {
		display: flex;
		gap: 30px;
		align-items: center;
		position: relative;
		z-index: 1;
	}

	.video-layout.playing {
		justify-content: center;
		z-index: 101;
	}

	.video-layout.expanded {
		position: fixed;
		inset: 0;
		z-index: 101;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 40px;
	}

	/* Video Player Container */
	.video-player-container {
		position: relative;
		flex: 0 0 55%;
		aspect-ratio: 16 / 9;
		border-radius: 12px;
		overflow: hidden;
		background: #000;
		transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.video-player-container.playing {
		flex: none;
		width: 75%;
		max-width: 900px;
		transform: translateY(-12px);
		box-shadow: 
			0 25px 50px rgba(0, 0, 0, 0.4),
			0 12px 24px rgba(0, 0, 0, 0.3);
		border-radius: 16px;
	}

	.video-player-container.expanded {
		width: 100%;
		max-width: 100%;
		height: 100%;
		max-height: 100%;
		transform: none;
		border-radius: 0;
		box-shadow: none;
	}

	/* Video Thumbnail */
	.video-thumbnail {
		position: absolute;
		inset: 0;
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
		background: rgba(0, 0, 0, 0.25);
		transition: background 0.3s ease;
	}

	.video-player-container:hover .video-thumbnail::before {
		background: rgba(0, 0, 0, 0.15);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PLAY BUTTON - Apple Vibrancy Effect
	   ═══════════════════════════════════════════════════════════════════════════ */
	.play-btn {
		position: relative;
		width: 72px;
		height: 72px;
		border: none;
		border-radius: 50%;
		cursor: pointer;
		background: transparent;
		z-index: 2;
		transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.play-btn:hover {
		transform: scale(1.08);
	}

	.play-btn:active {
		transform: scale(0.95);
	}

	/* Glow layer - the vibrancy/blur effect */
	.play-btn-glow {
		position: absolute;
		inset: -4px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.4);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		opacity: 0.8;
		transition: all 0.3s ease;
	}

	.play-btn:hover .play-btn-glow {
		inset: -8px;
		background: rgba(255, 255, 255, 0.6);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		opacity: 1;
		box-shadow: 0 0 40px rgba(255, 255, 255, 0.4);
	}

	/* Inner button face */
	.play-btn-inner {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.95);
		border-radius: 50%;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
		transition: all 0.3s ease;
	}

	.play-btn:hover .play-btn-inner {
		background: #fff;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
	}

	.play-btn-inner svg {
		width: 28px;
		height: 28px;
		color: #f69532;
		margin-left: 4px;
		transition: color 0.2s ease;
	}

	.play-btn:hover .play-btn-inner svg {
		color: #e8850d;
	}

	/* Duration Badge */
	.duration-badge {
		position: absolute;
		bottom: 12px;
		right: 12px;
		padding: 6px 10px;
		background: rgba(0, 0, 0, 0.75);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		color: #fff;
		font-size: 12px;
		font-weight: 600;
		border-radius: 6px;
		font-variant-numeric: tabular-nums;
		z-index: 2;
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
		padding: 12px 16px;
		background: linear-gradient(to bottom, rgba(0, 0, 0, 0.7) 0%, transparent 100%);
		z-index: 10;
		opacity: 0;
		transition: opacity 0.25s ease;
	}

	.video-player-container:hover .video-controls,
	.video-player-container.expanded .video-controls {
		opacity: 1;
	}

	.control-btn {
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

	.control-btn:hover {
		background: rgba(255, 255, 255, 0.25);
		transform: scale(1.05);
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
	   VIDEO INFO PANEL
	   ═══════════════════════════════════════════════════════════════════════════ */
	.video-info {
		flex: 1;
		color: #fff;
		text-align: center;
	}

	.video-info h2 {
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 8px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.video-info p {
		font-size: 13px;
		opacity: 0.9;
		margin: 0 0 20px 0;
	}

	.watch-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: #143e59;
		color: #fff;
		padding: 16px 32px;
		border-radius: 10px;
		font-size: 16px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s ease;
	}

	.watch-btn:hover {
		background: #0f2d42;
		transform: translateY(-2px);
	}

	.admin-upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		margin-bottom: 12px;
		background: #143E59;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		color: white;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.admin-upload-btn:hover {
		background: #0f2d42;
		transform: translateY(-1px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ENTRIES TAB - Trade Sheet
	   ═══════════════════════════════════════════════════════════════════════════ */
	.entries-container {
		max-width: 1400px;
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
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1023px) {
		.video-layout {
			flex-direction: column;
		}

		.video-player-container {
			flex: none;
			width: 100%;
		}

		.video-player-container.playing {
			width: 90%;
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

		.video-info h2 {
			font-size: 20px;
		}

		.watch-btn {
			padding: 14px 24px;
			font-size: 14px;
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

		.video-layout.expanded {
			padding: 16px;
		}

		.play-btn {
			width: 64px;
			height: 64px;
		}

		.play-btn-inner svg {
			width: 24px;
			height: 24px;
		}
	}

	@media (max-width: 639px) {
		.hero-header-compact h1 {
			font-size: 14px;
		}

		.collapse-indicator span {
			display: none;
		}

		.video-info h2 {
			font-size: 18px;
		}

		.video-info p {
			font-size: 12px;
		}

		.entries-header h2 {
			font-size: 20px;
		}

		.entries-header p {
			font-size: 13px;
		}
	}
</style>