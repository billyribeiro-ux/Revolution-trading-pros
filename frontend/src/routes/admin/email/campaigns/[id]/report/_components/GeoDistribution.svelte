<!--
	GeoDistribution — top-8 countries with opens/clicks bars and flag emojis.
	Extracted from +page.svelte (R22-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconWorld } from '$lib/icons';

	interface GeoEntry {
		country: string;
		country_code: string;
		opens: number;
		clicks: number;
	}

	interface Props {
		distribution: GeoEntry[];
		formatNumber: (num: number) => string;
		getFlagEmoji: (countryCode: string) => string;
	}

	const { distribution, formatNumber, getFlagEmoji }: Props = $props();
</script>

<section class="glass-panel geo-panel" in:fly={{ x: 20, duration: 500, delay: 300 }}>
	<div class="panel-header">
		<div class="panel-title">
			<div class="panel-icon green">
				<IconWorld size={24} />
			</div>
			<div>
				<h3>Geographic Distribution</h3>
				<span class="panel-subtitle">Where your subscribers are located</span>
			</div>
		</div>
	</div>

	<div class="geo-list">
		{#each distribution.slice(0, 8) as geo, i (geo.country_code)}
			{const maxOpens = distribution[0]?.opens || 1}
			<div class="geo-row" in:fly={{ x: 10, duration: 300, delay: 350 + i * 40 }}>
				<div class="geo-info">
					<span class="geo-flag">{getFlagEmoji(geo.country_code)}</span>
					<span class="geo-country">{geo.country}</span>
				</div>
				<div class="geo-stats">
					<div class="geo-bar-wrap">
						<div class="geo-bar" style:width={`${(geo.opens / maxOpens) * 100}%`}></div>
					</div>
					<div class="geo-metrics">
						<span class="geo-opens">{formatNumber(geo.opens)} opens</span>
						<span class="geo-clicks">{formatNumber(geo.clicks)} clicks</span>
					</div>
				</div>
			</div>
		{/each}
	</div>
</section>

<style>
	.glass-panel {
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: 20px;
		padding: 1.75rem;
		backdrop-filter: blur(20px);
		box-shadow: var(--admin-card-shadow, 0 4px 20px rgba(0, 0, 0, 0.4));
		position: relative;
		overflow: hidden;
	}

	.glass-panel::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 1px;
		background: linear-gradient(
			90deg,
			transparent,
			var(--admin-accent-primary-soft, rgba(230, 184, 0, 0.15)),
			transparent
		);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.panel-title h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.panel-subtitle {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	.panel-icon {
		width: 44px;
		height: 44px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.panel-icon.green {
		background: var(--admin-widget-green-bg, rgba(46, 160, 67, 0.15));
		color: var(--admin-widget-green-icon, #3fb950);
	}

	.geo-list {
		display: flex;
		flex-direction: column;
		gap: 0.875rem;
	}

	.geo-row {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.geo-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		min-width: 140px;
	}

	.geo-flag {
		font-size: 1.25rem;
	}

	.geo-country {
		font-size: 0.875rem;
		color: var(--text-primary);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.geo-stats {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.geo-bar-wrap {
		height: 8px;
		background: var(--bg-hover);
		border-radius: 4px;
		overflow: hidden;
	}

	.geo-bar {
		height: 100%;
		background: linear-gradient(
			90deg,
			var(--admin-success, #2ea043),
			var(--admin-success-text, #3fb950)
		);
		border-radius: 4px;
		transition: width 1s ease-out;
	}

	.geo-metrics {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
	}

	.geo-opens {
		color: var(--text-secondary);
	}

	.geo-clicks {
		color: var(--primary-500);
	}

	@media (prefers-reduced-motion: reduce) {
		.geo-bar {
			transition: none;
		}
	}
</style>
