<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * WeeklyHero Component - Collapsible Weekly Video Accordion
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Hero section with video breakdown and trade plan tabs
	 * @version 4.1.0 - Visual Polish Pass
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

	interface WeeklyContent {
		title: string;
		thumbnail: string;
		duration: string;
		videoTitle: string;
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
	}

	const {
		weeklyContent,
		tradePlan,
		videoUrl = '/dashboard/explosive-swings/video/weekly',
		sheetUrl = 'https://docs.google.com/spreadsheets/d/your-sheet-id'
	}: Props = $props();

	// Component-local state (Svelte 5 $state rune)
	let heroTab = $state<'video' | 'entries'>('video');
	let isCollapsed = $state(false);
	let expandedTradeNotes = $state(new Set<string>());

	function toggleCollapse() {
		isCollapsed = !isCollapsed;
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
				<div class="video-container-compact">
					<div class="video-player-compact" style="background-image: url('{weeklyContent.thumbnail}')">
						<div class="video-overlay">
							<button class="play-btn" aria-label="Play video">
								<svg viewBox="0 0 24 24" fill="currentColor">
									<path d="M8 5v14l11-7z" />
								</svg>
							</button>
						</div>
						<div class="video-duration">{weeklyContent.duration}</div>
					</div>
					<div class="video-info-compact">
						<h2>{weeklyContent.videoTitle}</h2>
						<p>Published {weeklyContent.publishedDate}</p>
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
				<div class="entries-container">
					<div class="entries-header">
						<h2>This Week's Trade Plan</h2>
						<p>Complete breakdown with entries, targets, stops, and options plays</p>
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
		background: #143e59;
		color: #fff;
		padding: 16px 32px;
		border-radius: 10px;
		font-size: 16px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s;
	}

	.watch-btn:hover {
		background: #0f2d42;
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

	.ticker-cell strong {
		font-size: 16px;
		color: #143e59;
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
</style>
