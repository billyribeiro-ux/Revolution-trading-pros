<!-- @migration-task Error while migrating Svelte code: `<script>` was left open
https://svelte.dev/e/element_unclosed -->
<!--
	Indicator Detail & Download Page
	═══════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7 Grade - February 2026

	Features:
	- Real API integration (no more mock data)
	- Indicator name and details
	- Available platforms display
	- Training videos
	- Platform-specific download sections
	- Secure download URL generation
	- Supporting documentation links
	- Breadcrumb navigation
	- WordPress-compatible structure
-->
<script lang="ts">
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import { page } from '$app/state';
	import { onMount } from 'svelte';

	// Platform logo mapping
	const platformLogos: Record<string, string> = {
		thinkorswim: '/logos/platforms/thinkorswim.png',
		tradingview: '/logos/platforms/tradingview.png',
		tradestation: '/logos/platforms/tradestation.png'
	};

	// Platform display names
	const platformNames: Record<string, string> = {
		thinkorswim: 'ThinkorSwim',
		tradingview: 'TradingView',
		tradestation: 'TradeStation'
	};

	interface IndicatorFile {
		id: number;
		file_name: string;
		display_name?: string;
		platform: string;
		version?: string;
		file_size_bytes?: number;
	}

	interface IndicatorVideo {
		id: number;
		title: string;
		embed_url?: string;
		play_url?: string;
		thumbnail_url?: string;
	}

	interface Indicator {
		id: number;
		name: string;
		slug: string;
		description?: string;
		platform?: string;
		version?: string;
		documentation_url?: string;
		license_key?: string;
	}

	interface PlatformDownloads {
		platform: string;
		logo: string;
		files: { id: number; name: string; downloadUrl: string | null }[];
		notes?: string;
	}

	// State
	let indicator = $state<Indicator | null>(null);
	let files = $state<IndicatorFile[]>([]);
	let videos = $state<IndicatorVideo[]>([]);
	let loading = $state(true);
	let error = $state('');
	let downloading = $state<number | null>(null);

	// License key state - ICT 7
	let licenseKey = $state('');
	let loadingLicense = $state(false);
	let showLicenseKey = $state(false);
	let copiedLicense = $state(false);

	// Installation guide state - ICT 7
	let showInstallGuide = $state(false);
	let selectedPlatform = $state('');
	let installGuide = $state('');
	let loadingGuide = $state(false);

	// Get indicator slug from URL params
	const indicatorSlug = $derived(page.params.id);

	// ICT 7: Fetch indicator data from real API
	const fetchIndicator = async () => {
		loading = true;
		error = '';
		try {
			const res = await fetch(`/api/my/indicators/${indicatorSlug}`);
			const data = await res.json();

			if (data.success) {
				indicator = data.data.indicator;
				files = data.data.files || [];
				videos = data.data.videos || [];
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

	// ICT 7: Generate secure download URL
	const downloadFile = async (fileId: number) => {
		downloading = fileId;
		try {
			const res = await fetch(`/api/my/indicators/${indicatorSlug}/download/${fileId}`);
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

	// Group files by platform for display
	const downloadsByPlatform = $derived.by(() => {
		const grouped: Record<string, PlatformDownloads> = {};

		for (const file of files) {
			const platform = file.platform.toLowerCase();
			if (!grouped[platform]) {
				grouped[platform] = {
					platform: platformNames[platform] || file.platform,
					logo: platformLogos[platform] || '/logos/platforms/generic.png',
					files: []
				};
			}
			grouped[platform].files.push({
				id: file.id,
				name: file.display_name || file.file_name,
				downloadUrl: null // Will be generated on-demand
			});
		}

		return Object.values(grouped);
	});

	// Available platforms list
	const availablePlatforms = $derived.by(() => {
		const platforms = new Set<string>();
		for (const file of files) {
			platforms.add(platformNames[file.platform.toLowerCase()] || file.platform);
		}
		if (indicator?.platform) {
			platforms.add(platformNames[indicator.platform.toLowerCase()] || indicator.platform);
		}
		return Array.from(platforms);
	});

	// ICT 7: Fetch license key
	const fetchLicenseKey = async () => {
		loadingLicense = true;
		try {
			const res = await fetch(`/api/my/indicators/${indicatorSlug}/license`);
			const data = await res.json();

			if (data.success) {
				licenseKey = data.data.license_key;
			}
		} catch (e) {
			console.error('Failed to fetch license key:', e);
		} finally {
			loadingLicense = false;
		}
	};

	// ICT 7: Copy license key to clipboard
	const copyLicenseKey = async () => {
		try {
			await navigator.clipboard.writeText(licenseKey);
			copiedLicense = true;
			setTimeout(() => (copiedLicense = false), 2000);
		} catch (e) {
			console.error('Failed to copy license key:', e);
		}
	};

	// ICT 7: Fetch installation guide for platform
	const fetchInstallGuide = async (platform: string) => {
		selectedPlatform = platform;
		showInstallGuide = true;
		loadingGuide = true;

		try {
			const res = await fetch(
				`/api/my/indicators/${indicatorSlug}/guide/${platform.toLowerCase()}`
			);
			const data = await res.json();

			if (data.success) {
				installGuide = data.data.instructions || 'No installation guide available.';
			} else {
				installGuide = 'Failed to load installation guide.';
			}
		} catch (e) {
			console.error('Failed to fetch installation guide:', e);
			installGuide = 'Failed to load installation guide.';
		} finally {
			loadingGuide = false;
		}
	};

	// ICT 7: Sanitize HTML to prevent XSS from API content
	function sanitizeHtml(html: string): string {
		return html
			.replace(/<script[\s\S]*?<\/script>/gi, '')
			.replace(/<iframe[\s\S]*?<\/iframe>/gi, '')
			.replace(/<object[\s\S]*?<\/object>/gi, '')
			.replace(/<embed[\s\S]*?\/?>|<form[\s\S]*?<\/form>/gi, '')
			.replace(/<style[\s\S]*?<\/style>/gi, '')
			.replace(/\son\w+\s*=\s*["'][^"']*["']/gi, '');
	}

	// ICT 7: Format install guide markdown to HTML
	function formatGuideHtml(text: string): string {
		return sanitizeHtml(
			text
				.replace(/\n/g, '<br>')
				.replace(/##\s*(.*?)(<br>|$)/g, '<h3>$1</h3>')
				.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
		);
	}

	onMount(fetchIndicator);
</script>

<svelte:head>
	<title>{indicator?.name || 'Indicator'} - Revolution Trading</title>
	<meta
		name="description"
		content="Download and install {indicator?.name || 'indicator'} for your trading platform"
	/>
	<meta property="og:title" content={indicator?.name || 'Indicator'} />
	<meta property="og:type" content="article" />
</svelte:head>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- Main Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<div class="indicators">
			<!-- ICT 7: Loading state -->
			{#if loading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading indicator...</p>
				</div>
			{:else if error}
				<!-- ICT 7: Error state -->
				<div class="error-state">
					<p class="error-message">{error}</p>
					<a href="/dashboard/indicators" class="btn btn-secondary">Back to My Indicators</a>
				</div>
			{:else if indicator}
				<div class="indicator-detail">
					<!-- Page Header -->
					<header class="indicator-detail__header">
						<h1>{indicator.name}</h1>
					</header>

					<!-- Available Platforms -->
					<section class="indicator-detail__platforms">
						<p class="platforms">
							<strong>Available Platforms:</strong>
							{availablePlatforms.join(', ') || indicator.platform || 'Multiple Platforms'}
						</p>
						<hr />
					</section>

					<!-- Training Videos - ICT 7: From real API -->
					{#if videos.length > 0}
						<section class="indicator-detail__videos">
							<h2>Training Videos</h2>
							<div class="videos-grid">
								{#each videos as video (video.id)}
									<div class="video-container">
										{#if video.embed_url}
											<iframe
												src={video.embed_url}
												title={video.title}
												style="aspect-ratio: 16/9; width: 100%; border: none;"
												allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
												allowfullscreen
											></iframe>
										{:else if video.play_url}
											<video
												controls
												width="100%"
												poster={video.thumbnail_url}
												style="aspect-ratio: 16/9;"
												title={video.title}
											>
												<source src={video.play_url} type="video/mp4" />
												Your browser does not support the video tag.
											</video>
										{/if}
										<p class="video-title">{video.title}</p>
									</div>
								{/each}
							</div>
						</section>
					{/if}

					<!-- Download Sections by Platform - ICT 7: From real API -->
					{#each downloadsByPlatform as platformDownload}
						<section class="st_box {platformDownload.platform.toLowerCase().replace(/\s+/g, '')}">
							<div class="platform-header">
								<img
									width="200"
									src={platformDownload.logo}
									alt={platformDownload.platform}
									onerror={(e) => {
										const img = e.currentTarget as HTMLImageElement;
										img.style.display = 'none';
									}}
								/>
							</div>

							{#if platformDownload.files.length > 0}
								<table class="downloads-table">
									<thead>
										<tr>
											<th>{platformDownload.platform} Install File:</th>
											<th></th>
										</tr>
									</thead>
									<tbody>
										{#each platformDownload.files as file}
											<tr>
												<td>{file.name}</td>
												<td class="text-right">
													<button
														class="orng_btn"
														onclick={() => downloadFile(file.id)}
														disabled={downloading === file.id}
													>
														{#if downloading === file.id}
															<span class="spinner-small"></span>
															Generating Link...
														{:else}
															<span class="fa fa-download"></span>
															Click to Download
														{/if}
													</button>
												</td>
											</tr>
										{/each}
									</tbody>
								</table>
							{:else}
								<p class="platform_notes">
									No files currently available for {platformDownload.platform}. Please contact
									support for assistance.
								</p>
							{/if}

							{#if platformDownload.notes}
								<div class="platform_notes">
									{@html sanitizeHtml(platformDownload.notes)}
								</div>
							{/if}
						</section>
					{/each}

					<!-- No downloads available message -->
					{#if downloadsByPlatform.length === 0}
						<section class="st_box">
							<p class="platform_notes">
								No download files are currently available for this indicator. Please contact support
								if you need assistance.
							</p>
						</section>
					{/if}

					<!-- Supporting Documentation - ICT 7: From indicator documentation_url -->
					{#if indicator.documentation_url}
						<section class="st_box">
							<h2>
								<strong>Supporting Documentation</strong>
							</h2>
							<table class="downloads-table">
								<tbody>
									<tr>
										<td>Official Documentation</td>
										<td class="text-right">
											<a
												class="orng_btn"
												href={indicator.documentation_url}
												target="_blank"
												rel="noopener noreferrer"
											>
												<span class="fa fa-external-link"></span>
												View Documentation
											</a>
										</td>
									</tr>
								</tbody>
							</table>
						</section>
					{/if}

					<!-- License Key Section - ICT 7 -->
					<section class="st_box license-section">
						<h2><strong>License Key</strong></h2>
						{#if licenseKey}
							<div class="license-key-display">
								<div class="license-key-value">
									{#if showLicenseKey}
										<code>{licenseKey}</code>
									{:else}
										<code>XXXX-XXXX-XXXX-XXXX</code>
									{/if}
								</div>
								<div class="license-key-actions">
									<button class="btn btn-small" onclick={() => (showLicenseKey = !showLicenseKey)}>
										{showLicenseKey ? 'Hide' : 'Show'}
									</button>
									<button
										class="btn btn-small btn-primary"
										onclick={copyLicenseKey}
										disabled={!showLicenseKey}
									>
										{copiedLicense ? 'Copied!' : 'Copy'}
									</button>
								</div>
							</div>
							<p class="license-note">
								This license key is tied to your account. Do not share it with others.
							</p>
						{:else}
							<button class="orng_btn" onclick={fetchLicenseKey} disabled={loadingLicense}>
								{loadingLicense ? 'Loading...' : 'Get License Key'}
							</button>
						{/if}
					</section>

					<!-- Installation Guide Buttons - ICT 7 -->
					{#if downloadsByPlatform.length > 0}
						<section class="st_box">
							<h2><strong>Installation Guides</strong></h2>
							<div class="install-guide-buttons">
								{#each downloadsByPlatform as platformDownload}
									<button
										class="btn btn-outline"
										onclick={() => fetchInstallGuide(platformDownload.platform)}
									>
										{platformDownload.platform} Guide
									</button>
								{/each}
							</div>
						</section>
					{/if}

					<!-- Back to Indicators -->
					<div class="indicator-detail__footer">
						<a href="/dashboard/indicators" class="btn btn-secondary">
							<span class="fa fa-arrow-left"></span>
							Back to My Indicators
						</a>
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

<!-- Installation Guide Modal - ICT 7 -->
{#if showInstallGuide}
	<div
		class="modal-overlay"
		onclick={() => (showInstallGuide = false)}
		onkeydown={(e) => e.key === 'Escape' && (showInstallGuide = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div
			class="modal install-guide-modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showInstallGuide = false)}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="modal-header">
				<h2>{selectedPlatform} Installation Guide</h2>
				<button class="btn-close" onclick={() => (showInstallGuide = false)}>X</button>
			</div>
			<div class="modal-body">
				{#if loadingGuide}
					<div class="loading-inline">
						<div class="spinner"></div>
						<span>Loading guide...</span>
					</div>
				{:else}
					<div class="guide-content">
						{@html formatGuideHtml(installGuide)}
					</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="btn btn-secondary" onclick={() => (showInstallGuide = false)}>Close</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ICT 7 Grade Styles - WordPress Compatible with Real API Integration
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* ICT 7: Loading and error states */
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 20px;
		text-align: center;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 3px solid #e5e5e5;
		border-top-color: #f69532;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	.spinner-small {
		display: inline-block;
		width: 16px;
		height: 16px;
		border: 2px solid #ffffff;
		border-top-color: transparent;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-right: 8px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-message {
		color: #dc2626;
		font-size: 16px;
		margin: 0 0 20px;
	}

	.indicators {
		max-width: 1160px;
		margin: 0 auto;
		padding: 40px 20px;
		background-color: #ffffff;
	}

	.indicator-detail {
		max-width: 100%;
		margin: 0;
		padding: 0;
		background: transparent;
		border-radius: 0;
		box-shadow: none;
	}

	.indicator-detail__header h1 {
		font-size: 32px;
		font-weight: 400;
		color: #666666;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
		text-align: center;
	}

	.indicator-detail__platforms {
		margin-bottom: 0;
	}

	.platforms {
		font-size: 16px;
		color: #666666;
		margin: 0 0 15px;
		font-family: 'Open Sans', sans-serif;
		text-align: center;
	}

	.platforms strong {
		color: #666666;
		font-weight: 700;
	}

	hr {
		border: none;
		border-top: 1px solid #dddddd;
		margin: 20px 0 40px;
	}

	/* Video Section */
	.indicator-detail__videos {
		margin-bottom: 40px;
	}

	.indicator-detail__videos h2 {
		display: none;
	}

	.videos-grid {
		display: block;
	}

	.video-container {
		position: relative;
		background: #000;
		border-radius: 0;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		margin-bottom: 25px;
	}

	.video-container video {
		width: 100%;
		display: block;
	}

	.video-title {
		display: none;
	}

	/* Platform Download Boxes - WordPress Exact */
	.st_box {
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		padding: 40px 30px;
		margin-bottom: 20px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.st_box h2 {
		font-size: 22px;
		font-weight: 700;
		color: #666666;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	.platform-header {
		text-align: center;
		margin-bottom: 30px;
	}

	.platform-header img {
		max-width: 200px;
		height: auto;
	}

	/* Downloads Table - WordPress Exact */
	.downloads-table {
		width: 100%;
		border-collapse: collapse;
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 0;
		overflow: visible;
		box-shadow: none;
	}

	.downloads-table thead th {
		background: #f5f5f5;
		color: #666666;
		padding: 15px 20px;
		text-align: left;
		font-weight: 700;
		font-size: 14px;
		text-transform: none;
		letter-spacing: 0;
		border-bottom: 1px solid #e5e5e5;
		font-family: 'Open Sans', sans-serif;
	}

	.downloads-table tbody tr {
		border-bottom: 1px solid #e5e5e5;
	}

	.downloads-table tbody tr:last-child {
		border-bottom: none;
	}

	.downloads-table tbody tr:hover {
		background-color: #fafafa;
	}

	.downloads-table td {
		padding: 15px 20px;
		color: #666666;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
	}

	.downloads-table td.text-right {
		text-align: right;
	}

	/* Orange Button - WordPress Exact */
	.orng_btn {
		display: inline-block;
		padding: 8px 20px;
		background: #f69532;
		color: #ffffff;
		text-decoration: none;
		border-radius: 20px;
		font-weight: 600;
		font-size: 13px;
		transition: all 0.2s ease;
		border: none;
		cursor: pointer;
		white-space: nowrap;
	}

	.orng_btn:hover {
		background: #dc7309;
		text-decoration: none;
	}

	.orng_btn .fa {
		display: none;
	}

	/* Platform Notes - WordPress Exact */
	.platform_notes {
		background: #f9f9f9;
		border: 1px solid #e5e5e5;
		padding: 20px;
		margin-top: 20px;
		border-radius: 4px;
		color: #666666;
		font-size: 14px;
		line-height: 1.7;
		font-family: 'Open Sans', sans-serif;
	}

	.platform_notes :global(a) {
		color: #1e73be;
		text-decoration: none;
	}

	.platform_notes :global(a:hover) {
		color: #000000;
		text-decoration: underline;
	}

	/* License Key Section - ICT 7 */
	.license-section {
		text-align: center;
	}

	.license-key-display {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: #f9f9f9;
		border-radius: 8px;
		margin-bottom: 16px;
	}

	.license-key-value {
		width: 100%;
	}

	.license-key-value code {
		display: block;
		font-family: 'Courier New', monospace;
		font-size: 20px;
		font-weight: bold;
		letter-spacing: 2px;
		padding: 16px;
		background: #ffffff;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		color: #333;
		word-break: break-all;
	}

	.license-key-actions {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
		justify-content: center;
	}

	.btn-small {
		padding: 8px 16px;
		font-size: 13px;
		min-height: 36px;
	}

	.btn-primary {
		background: #f69532;
		color: #fff;
		border: none;
	}

	.btn-primary:hover {
		background: #dc7309;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.license-note {
		font-size: 13px;
		color: #888;
		margin: 0;
		font-style: italic;
	}

	/* Installation Guide Buttons - ICT 7 */
	.install-guide-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		justify-content: center;
	}

	.btn-outline {
		padding: 10px 20px;
		background: transparent;
		color: #666;
		border: 1px solid #ddd;
		border-radius: 20px;
		font-size: 14px;
		cursor: pointer;
		transition: all 0.2s ease;
		min-height: 44px;
	}

	.btn-outline:hover {
		background: #f5f5f5;
		border-color: #143e59;
		color: #143e59;
	}

	/* Modal Styles - ICT 7 */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		background: #fff;
		border-radius: 12px;
		max-width: 600px;
		width: 100%;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.2);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e5e5e5;
	}

	.modal-header h2 {
		margin: 0;
		font-size: 18px;
		font-weight: 600;
		color: #333;
	}

	.btn-close {
		background: none;
		border: none;
		font-size: 24px;
		cursor: pointer;
		color: #999;
		padding: 4px 8px;
		line-height: 1;
	}

	.btn-close:hover {
		color: #333;
	}

	.modal-body {
		padding: 24px;
	}

	.loading-inline {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
		padding: 40px;
		color: #666;
	}

	.guide-content {
		color: #666;
		line-height: 1.8;
		font-size: 15px;
	}

	.guide-content :global(h3) {
		color: #333;
		font-size: 18px;
		margin: 24px 0 12px;
		font-weight: 600;
	}

	.guide-content :global(h3:first-child) {
		margin-top: 0;
	}

	.guide-content :global(strong) {
		color: #333;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 16px 24px;
		border-top: 1px solid #e5e5e5;
	}

	/* Footer */
	.indicator-detail__footer {
		margin-top: 40px;
		padding-top: 30px;
		border-top: 1px solid #dddddd;
		text-align: center;
	}

	.btn {
		display: inline-block;
		padding: 10px 24px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
		font-family: 'Open Sans', sans-serif;
	}

	.btn-secondary {
		background: #f5f5f5;
		color: #666666;
		border: 1px solid #dddddd;
	}

	.btn-secondary:hover {
		background: #e5e5e5;
		color: #333333;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.indicators {
			padding: 20px 15px;
		}

		.indicator-detail__header h1 {
			font-size: 24px;
		}

		.st_box {
			padding: 20px;
		}

		.downloads-table td,
		.downloads-table thead th {
			padding: 12px 15px;
		}

		.platform-header img {
			max-width: 150px;
		}
	}

	@media (max-width: 480px) {
		.downloads-table td,
		.downloads-table thead th {
			display: block;
			width: 100%;
			text-align: left !important;
		}

		.downloads-table thead {
			display: none;
		}

		.downloads-table tbody tr {
			display: block;
			margin-bottom: 15px;
			border: 1px solid #e5e5e5;
			padding: 10px;
		}

		.downloads-table td {
			padding: 8px 0;
			border-bottom: none;
		}

		.downloads-table td.text-right {
			text-align: left !important;
			margin-top: 10px;
		}

		.orng_btn {
			width: 100%;
			text-align: center;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 Mobile-First Responsive Design Enhancements
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch targets: min 44x44px, Safe areas: env(safe-area-inset-*)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Touch-friendly download buttons */
	.orng_btn {
		min-height: 44px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		-webkit-tap-highlight-color: transparent;
	}

	.btn {
		min-height: 44px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	/* Safe areas for notched devices */
	@supports (padding: max(0px)) {
		.indicators {
			padding-left: max(20px, env(safe-area-inset-left));
			padding-right: max(20px, env(safe-area-inset-right));
			padding-bottom: max(40px, env(safe-area-inset-bottom));
		}
	}

	/* xs: Extra small devices (≥ 360px) */
	@media (min-width: 360px) {
		.st_box {
			padding: 24px 20px;
		}
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		.st_box {
			padding: 32px 24px;
		}

		.downloads-table td,
		.downloads-table thead th {
			display: table-cell;
		}

		.downloads-table thead {
			display: table-header-group;
		}

		.downloads-table tbody tr {
			display: table-row;
			margin-bottom: 0;
			border: none;
			padding: 0;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.st_box {
			padding: 40px 30px;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.video-container:hover {
			box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.orng_btn {
			min-height: 48px;
			padding: 12px 24px;
		}

		.btn {
			min-height: 48px;
			padding: 14px 28px;
		}

		.downloads-table tbody tr:hover {
			background-color: transparent;
		}

		.downloads-table tbody tr:active {
			background-color: #fafafa;
		}

		/* Video touch controls */
		.video-container video::-webkit-media-controls-panel {
			min-height: 44px;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.orng_btn,
		.btn,
		.video-container {
			transition: none;
		}
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.st_box {
			border-width: 2px;
		}

		.downloads-table {
			border-width: 2px;
		}

		.orng_btn {
			border: 2px solid #fff;
		}
	}

	/* Print styles */
	@media print {
		.indicator-detail__videos,
		.indicator-detail__footer {
			display: none;
		}

		.st_box {
			break-inside: avoid;
			page-break-inside: avoid;
		}
	}
</style>
