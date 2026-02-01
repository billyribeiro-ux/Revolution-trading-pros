<script lang="ts">
	/**
	 * My Indicators Dashboard
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Shows user's owned indicators with download links
	 */

	import { onMount } from 'svelte';

	interface OwnedIndicator {
		id: string;
		name: string;
		slug: string;
		tagline?: string;
		logo_url?: string;
		card_image_url?: string;
		access_granted_at: string;
	}

	let indicators = $state<OwnedIndicator[]>([]);
	let loading = $state(true);
	let error = $state('');

	const fetchMyIndicators = async () => {
		loading = true;
		try {
			const res = await fetch('/api/my/indicators');
			const data = await res.json();
			if (data.success) {
				indicators = data.data;
			} else {
				error = data.error || 'Failed to load indicators';
			}
		} catch (e) {
			error = 'Failed to load your indicators';
			console.error(e);
		} finally {
			loading = false;
		}
	};

	const formatDate = (date: string) => {
		return new Date(date).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	};

	onMount(fetchMyIndicators);
</script>

<svelte:head>
	<title>My Indicators | Revolution Trading Pros</title>
</svelte:head>

<div class="my-indicators-page">
	<header class="page-header">
		<h1>My Indicators</h1>
		<p class="subtitle">Download your trading indicators for all platforms</p>
	</header>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading your indicators...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={fetchMyIndicators}>Try Again</button>
		</div>
	{:else if indicators.length === 0}
		<div class="empty-state">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="64"
				height="64"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1"
			>
				<path d="M3 3v18h18" /><path d="m19 9-5 5-4-4-3 3" />
			</svg>
			<h2>No Indicators Yet</h2>
			<p>You haven't purchased any trading indicators yet.</p>
			<a href="/indicators" class="btn-primary">Browse Indicators</a>
		</div>
	{:else}
		<div class="indicators-grid">
			{#each indicators as indicator}
				<a href="/my/indicators/{indicator.slug}" class="indicator-card">
					<div class="card-image">
						{#if indicator.logo_url}
							<img src={indicator.logo_url} alt={indicator.name} />
						{:else}
							<div class="placeholder">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="48"
									height="48"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="1"
								>
									<path d="M3 3v18h18" /><path d="m19 9-5 5-4-4-3 3" />
								</svg>
							</div>
						{/if}
					</div>
					<div class="card-content">
						<h3>{indicator.name}</h3>
						{#if indicator.tagline}
							<p class="tagline">{indicator.tagline}</p>
						{/if}
						<div class="card-footer">
							<span class="owned-badge">
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
								Owned
							</span>
							<span class="date">Since {formatDate(indicator.access_granted_at)}</span>
						</div>
					</div>
					<div class="download-arrow">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
								points="7 10 12 15 17 10"
							/><line x1="12" x2="12" y1="15" y2="3" />
						</svg>
					</div>
				</a>
			{/each}
		</div>
	{/if}
</div>

<style>
	.my-indicators-page {
		padding: 32px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 32px;
	}
	.page-header h1 {
		font-size: 28px;
		font-weight: 700;
		color: #1f2937;
		margin: 0 0 8px;
	}
	.subtitle {
		font-size: 16px;
		color: #6b7280;
		margin: 0;
	}

	.loading,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 80px 32px;
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
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state svg {
		color: #d1d5db;
		margin-bottom: 24px;
	}
	.empty-state h2 {
		font-size: 20px;
		color: #1f2937;
		margin: 0 0 8px;
	}
	.empty-state p {
		color: #6b7280;
		margin: 0 0 24px;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
	}
	.btn-primary:hover {
		background: #0f2d42;
	}

	.indicators-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		gap: 20px;
	}

	.indicator-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		text-decoration: none;
		color: inherit;
		transition: all 0.2s;
	}
	.indicator-card:hover {
		border-color: #143e59;
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.1);
	}

	.card-image {
		flex-shrink: 0;
	}
	.card-image img {
		width: 72px;
		height: 72px;
		border-radius: 12px;
		object-fit: cover;
	}
	.placeholder {
		width: 72px;
		height: 72px;
		background: #f3f4f6;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #9ca3af;
	}

	.card-content {
		flex: 1;
		min-width: 0;
	}
	.card-content h3 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 4px;
	}
	.tagline {
		font-size: 14px;
		color: #6b7280;
		margin: 0 0 12px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.card-footer {
		display: flex;
		align-items: center;
		gap: 12px;
	}
	.owned-badge {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-size: 12px;
		font-weight: 500;
		color: #065f46;
		background: #d1fae5;
		padding: 4px 8px;
		border-radius: 4px;
	}
	.date {
		font-size: 12px;
		color: #9ca3af;
	}

	.download-arrow {
		flex-shrink: 0;
		/* Touch-friendly 44x44px minimum */
		width: 44px;
		height: 44px;
		background: #f3f4f6;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #143e59;
		transition: all 0.2s;
	}
	.indicator-card:hover .download-arrow {
		background: #143e59;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 Mobile-First Responsive Design
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch targets: min 44x44px, Safe areas: env(safe-area-inset-*)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile base styles (< 640px) */
	@media (max-width: 639px) {
		.my-indicators-page {
			padding: 20px 16px;
			padding-left: max(16px, env(safe-area-inset-left));
			padding-right: max(16px, env(safe-area-inset-right));
			padding-bottom: max(20px, env(safe-area-inset-bottom));
		}

		.page-header h1 {
			font-size: 24px;
		}

		.subtitle {
			font-size: 14px;
		}

		/* Single column on mobile */
		.indicators-grid {
			grid-template-columns: 1fr;
			gap: 16px;
		}

		.indicator-card {
			padding: 16px;
			flex-wrap: wrap;
		}

		.card-image img,
		.placeholder {
			width: 56px;
			height: 56px;
		}

		.card-content h3 {
			font-size: 15px;
		}

		.card-footer {
			flex-wrap: wrap;
			gap: 8px;
		}

		.loading,
		.error-state,
		.empty-state {
			padding: 48px 24px;
		}
	}

	/* xs: Extra small devices (≥ 360px) */
	@media (min-width: 360px) {
		.indicator-card {
			flex-wrap: nowrap;
		}
	}

	/* sm: Small devices (≥ 640px) - 2 columns */
	@media (min-width: 640px) {
		.my-indicators-page {
			padding: 24px;
		}

		.indicators-grid {
			grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.my-indicators-page {
			padding: 32px;
		}

		.page-header h1 {
			font-size: 28px;
		}

		.indicators-grid {
			grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
			gap: 20px;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.indicator-card:hover {
			transform: translateY(-2px);
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.btn-primary {
			min-height: 48px;
			padding: 14px 28px;
		}

		.indicator-card {
			min-height: 88px;
		}

		.indicator-card:hover {
			transform: none;
		}

		.indicator-card:active {
			border-color: #143e59;
			background: #f9fafb;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.spinner {
			animation: none;
		}

		.indicator-card,
		.download-arrow {
			transition: none;
		}
	}

	/* Safe areas for notched devices */
	@supports (padding: max(0px)) {
		.my-indicators-page {
			padding-left: max(16px, env(safe-area-inset-left));
			padding-right: max(16px, env(safe-area-inset-right));
		}
	}
</style>
