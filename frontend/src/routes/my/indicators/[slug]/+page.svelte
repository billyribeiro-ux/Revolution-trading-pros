<script lang="ts">
	/**
	 * My Indicator Downloads Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Platform-specific downloads with secure URLs
	 */

	import { onMount } from 'svelte';

	interface Indicator {
		id: string;
		name: string;
		slug: string;
		tagline?: string;
		logo_url?: string;
		description?: string;
		version?: string;
	}

	interface IndicatorFile {
		id: number;
		file_name: string;
		platform: string;
		file_size_bytes?: number;
		version?: string;
		display_name?: string;
	}

	interface IndicatorVideo {
		id: number;
		title: string;
		embed_url?: string;
		thumbnail_url?: string;
		is_featured?: boolean;
	}

	interface Ownership {
		access_granted_at?: string;
		is_lifetime_access?: boolean;
	}

	let indicator = $state<Indicator | null>(null);
	let files = $state<IndicatorFile[]>([]);
	let videos = $state<IndicatorVideo[]>([]);
	let ownership = $state<Ownership | null>(null);
	let loading = $state(true);
	let downloading = $state<number | null>(null);
	let error = $state('');
	let slug = $state('');

	onMount(() => {
		const pathParts = window.location.pathname.split('/');
		slug = pathParts[pathParts.length - 1];
		fetchIndicator();
	});

	const platformIcons: Record<string, string> = {
		thinkorswim: '📊',
		tradingview: '📈',
		mt4: '💹',
		mt5: '💹',
		ninjatrader: '🎯',
		tradestation: '📉',
		sierrachart: '📋',
		ctrader: '⚡'
	};

	const platformNames: Record<string, string> = {
		thinkorswim: 'ThinkorSwim',
		tradingview: 'TradingView',
		mt4: 'MetaTrader 4',
		mt5: 'MetaTrader 5',
		ninjatrader: 'NinjaTrader 8',
		tradestation: 'TradeStation',
		sierrachart: 'Sierra Chart',
		ctrader: 'cTrader'
	};

	const fetchIndicator = async () => {
		loading = true;
		try {
			const res = await fetch(`/api/my/indicators/${slug}`);
			const data = await res.json();

			if (data.success) {
				indicator = data.data.indicator;
				files = data.data.files || [];
				videos = data.data.videos || [];
				ownership = data.data.ownership;
			} else {
				error = data.error || 'Failed to load indicator';
			}
		} catch (e) {
			error = 'Failed to load indicator';
			console.error(e);
		} finally {
			loading = false;
		}
	};

	const downloadFile = async (fileId: number) => {
		downloading = fileId;
		try {
			const res = await fetch(`/api/my/indicators/${slug}/download/${fileId}`);
			const data = await res.json();

			if (data.success) {
				// Open secure download URL
				window.open(data.data.download_url, '_blank');
			} else {
				error = data.error || 'Failed to generate download link';
			}
		} catch (e) {
			error = 'Failed to download file';
			console.error(e);
		} finally {
			downloading = null;
		}
	};

	const formatFileSize = (bytes?: number) => {
		if (!bytes) return '';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / 1048576).toFixed(1)} MB`;
	};

	const formatDate = (date?: string) => {
		if (!date) return '';
		return new Date(date).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	};

	// Group files by platform
	const filesByPlatform = $derived(() => {
		const grouped: Record<string, IndicatorFile[]> = {};
		for (const file of files) {
			if (!grouped[file.platform]) grouped[file.platform] = [];
			grouped[file.platform].push(file);
		}
		return grouped;
	});
</script>

<svelte:head>
	<title>{indicator?.name || 'Indicator'} Downloads | Revolution Trading Pros</title>
</svelte:head>

<div class="indicator-downloads-page">
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<a href="/my/indicators" class="btn-secondary">Back to My Indicators</a>
		</div>
	{:else if indicator}
		<header class="page-header">
			<a href="/my/indicators" class="back-link">← My Indicators</a>
			<div class="indicator-info">
				{#if indicator.logo_url}
					<img src={indicator.logo_url} alt="" class="logo" />
				{/if}
				<div>
					<h1>{indicator.name}</h1>
					{#if indicator.tagline}
						<p class="tagline">{indicator.tagline}</p>
					{/if}
				</div>
			</div>
			{#if ownership}
				<div class="ownership-info">
					<span class="badge">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="14"
							height="14"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" /><polyline
								points="22 4 12 14.01 9 11.01"
							/>
						</svg>
						{ownership.is_lifetime_access ? 'Lifetime Access' : 'Owned'}
					</span>
					{#if ownership.access_granted_at}
						<span class="since">Since {formatDate(ownership.access_granted_at)}</span>
					{/if}
				</div>
			{/if}
		</header>

		<section class="downloads-section">
			<h2>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="24"
					height="24"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
						points="7 10 12 15 17 10"
					/><line x1="12" x2="12" y1="15" y2="3" />
				</svg>
				Platform Downloads
			</h2>
			<p class="section-desc">
				Download the indicator for your trading platform. Each download link is secure and expires
				after 24 hours.
			</p>

			{#if files.length === 0}
				<div class="empty-state">
					<p>No download files available yet.</p>
				</div>
			{:else}
				<div class="platforms-grid">
					{#each Object.entries(filesByPlatform()) as [platform, platformFiles]}
						<div class="platform-card">
							<div class="platform-header">
								<span class="platform-icon">{platformIcons[platform] || '📦'}</span>
								<h3>{platformNames[platform] || platform}</h3>
							</div>
							<div class="platform-files">
								{#each platformFiles as file}
									<button
										class="download-btn"
										onclick={() => downloadFile(file.id)}
										disabled={downloading === file.id}
									>
										{#if downloading === file.id}
											<div class="spinner-small"></div>
											<span>Generating link...</span>
										{:else}
											<svg
												xmlns="http://www.w3.org/2000/svg"
												width="18"
												height="18"
												viewBox="0 0 24 24"
												fill="none"
												stroke="currentColor"
												stroke-width="2"
											>
												<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
													points="7 10 12 15 17 10"
												/><line x1="12" x2="12" y1="15" y2="3" />
											</svg>
											<span>{file.display_name || file.file_name}</span>
										{/if}
										{#if file.file_size_bytes}
											<span class="file-size">{formatFileSize(file.file_size_bytes)}</span>
										{/if}
									</button>
								{/each}
							</div>
							{#if platformFiles[0]?.version}
								<div class="platform-footer">
									<span class="version">v{platformFiles[0].version}</span>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</section>

		{#if videos.length > 0}
			<section class="videos-section">
				<h2>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<polygon points="5 3 19 12 5 21 5 3" />
					</svg>
					Tutorial Videos
				</h2>
				<div class="videos-grid">
					{#each videos as video}
						<div class="video-card">
							{#if video.embed_url}
								<iframe
									src={video.embed_url}
									title={video.title}
									allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
									allowfullscreen
								></iframe>
							{:else if video.thumbnail_url}
								<img src={video.thumbnail_url} alt={video.title} />
							{/if}
							<h4>{video.title}</h4>
						</div>
					{/each}
				</div>
			</section>
		{/if}

		<section class="support-section">
			<h2>Need Help?</h2>
			<p>
				If you're having trouble installing or using this indicator, our support team is here to
				help.
			</p>
			<div class="support-links">
				<a href="/support" class="btn-secondary">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<circle cx="12" cy="12" r="10" /><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" /><path
							d="M12 17h.01"
						/>
					</svg>
					Get Support
				</a>
				<a href="/docs/indicators/{indicator.slug}" class="btn-secondary">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="18"
						height="18"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
					</svg>
					Documentation
				</a>
			</div>
		</section>
	{/if}
</div>

<style>
	.indicator-downloads-page {
		padding: 32px;
		max-width: 1000px;
		margin: 0 auto;
	}

	.loading,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px;
		text-align: center;
	}
	.spinner {
		width: 48px;
		height: 48px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}
	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.back-link {
		display: inline-block;
		color: #6b7280;
		text-decoration: none;
		font-size: 14px;
		margin-bottom: 16px;
	}
	.back-link:hover {
		color: #143e59;
	}

	.page-header {
		margin-bottom: 40px;
	}
	.indicator-info {
		display: flex;
		align-items: center;
		gap: 16px;
		margin-bottom: 16px;
	}
	.logo {
		width: 64px;
		height: 64px;
		border-radius: 12px;
		object-fit: cover;
	}
	h1 {
		font-size: 28px;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 4px;
	}
	.tagline {
		font-size: 16px;
		color: #6b7280;
		margin: 0;
	}

	.ownership-info {
		display: flex;
		align-items: center;
		gap: 12px;
	}
	.badge {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: #d1fae5;
		color: #065f46;
		font-size: 13px;
		font-weight: 500;
		border-radius: 6px;
	}
	.since {
		font-size: 13px;
		color: #9ca3af;
	}

	section {
		margin-bottom: 48px;
	}
	section h2 {
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 20px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 8px;
	}
	section h2 svg {
		color: #143e59;
	}
	.section-desc {
		color: #6b7280;
		margin: 0 0 24px;
	}

	.empty-state {
		text-align: center;
		padding: 40px;
		background: #f9fafb;
		border-radius: 12px;
		color: #6b7280;
	}

	.platforms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 20px;
	}

	.platform-card {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}
	.platform-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px 20px;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
	}
	.platform-icon {
		font-size: 24px;
	}
	.platform-header h3 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.platform-files {
		padding: 16px 20px;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.download-btn {
		display: flex;
		align-items: center;
		gap: 10px;
		width: 100%;
		padding: 12px 16px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}
	.download-btn:hover:not(:disabled) {
		background: #0f2d42;
	}
	.download-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}
	.download-btn svg {
		flex-shrink: 0;
	}
	.download-btn span {
		flex: 1;
		text-align: left;
	}
	.file-size {
		font-size: 12px;
		opacity: 0.8;
	}

	.platform-footer {
		padding: 12px 20px;
		border-top: 1px solid #f3f4f6;
	}
	.version {
		font-size: 12px;
		color: #9ca3af;
	}

	.videos-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 20px;
	}
	.video-card {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}
	.video-card iframe,
	.video-card img {
		width: 100%;
		aspect-ratio: 16/9;
		border: none;
	}
	.video-card h4 {
		padding: 12px 16px;
		font-size: 14px;
		font-weight: 500;
		margin: 0;
	}

	.support-section {
		background: #f9fafb;
		border-radius: 12px;
		padding: 32px;
		text-align: center;
	}
	.support-section h2 {
		justify-content: center;
		margin-bottom: 8px;
	}
	.support-section p {
		color: #6b7280;
		margin: 0 0 20px;
	}
	.support-links {
		display: flex;
		justify-content: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #fff;
		color: #374151;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
		/* Touch-friendly minimum size */
		min-height: 44px;
	}
	.btn-secondary:hover {
		background: #f3f4f6;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch targets: min 44x44px, Safe areas: env(safe-area-inset-*)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile base styles (< 640px) */
	@media (max-width: 639px) {
		.indicator-downloads-page {
			padding: 20px 16px;
			padding-left: max(16px, env(safe-area-inset-left));
			padding-right: max(16px, env(safe-area-inset-right));
			padding-bottom: max(20px, env(safe-area-inset-bottom));
		}

		.indicator-info {
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
		}

		.logo {
			width: 48px;
			height: 48px;
		}

		h1 {
			font-size: 22px;
		}

		.tagline {
			font-size: 14px;
		}

		.ownership-info {
			flex-direction: column;
			align-items: flex-start;
			gap: 8px;
		}

		/* Single column platform cards */
		.platforms-grid {
			grid-template-columns: 1fr;
			gap: 16px;
		}

		.platform-header {
			padding: 12px 16px;
		}

		.platform-files {
			padding: 12px 16px;
		}

		.download-btn {
			padding: 14px 16px;
			min-height: 48px;
		}

		/* Single column videos */
		.videos-grid {
			grid-template-columns: 1fr;
			gap: 16px;
		}

		.support-section {
			padding: 24px 16px;
		}

		.support-links {
			flex-direction: column;
			width: 100%;
		}

		.support-links .btn-secondary {
			width: 100%;
			justify-content: center;
		}

		.loading,
		.error-state {
			padding: 48px 16px;
		}

		section h2 {
			font-size: 18px;
		}

		.section-desc {
			font-size: 14px;
		}
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		.indicator-downloads-page {
			padding: 24px;
		}

		.platforms-grid {
			grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.indicator-downloads-page {
			padding: 32px;
		}

		h1 {
			font-size: 28px;
		}

		.platforms-grid {
			grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
			gap: 20px;
		}

		.videos-grid {
			grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.platform-card:hover {
			box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.download-btn {
			min-height: 52px;
			padding: 16px 20px;
		}

		.btn-secondary {
			min-height: 48px;
			padding: 14px 24px;
		}

		.back-link {
			min-height: 44px;
			display: inline-flex;
			align-items: center;
		}

		.platform-card:hover {
			box-shadow: none;
		}

		.platform-card:active {
			background: #f9fafb;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.spinner,
		.spinner-small {
			animation: none;
		}

		.download-btn,
		.btn-secondary,
		.platform-card {
			transition: none;
		}
	}

	/* Safe areas for notched devices */
	@supports (padding: max(0px)) {
		.indicator-downloads-page {
			padding-left: max(16px, env(safe-area-inset-left));
			padding-right: max(16px, env(safe-area-inset-right));
			padding-bottom: max(32px, env(safe-area-inset-bottom));
		}
	}

	/* Print styles */
	@media print {
		.download-btn,
		.btn-secondary {
			display: none;
		}

		.platform-card {
			break-inside: avoid;
		}
	}
</style>
