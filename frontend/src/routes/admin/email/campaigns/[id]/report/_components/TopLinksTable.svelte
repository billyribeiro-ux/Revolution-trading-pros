<!--
	TopLinksTable — ranked list of top-clicked links with click rates.
	Extracted from +page.svelte (R22-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconLink } from '$lib/icons';

	interface TopLink {
		url: string;
		clicks: number;
		unique_clicks: number;
	}

	interface Props {
		links: TopLink[];
		uniqueOpens: number;
		formatNumber: (num: number) => string;
		truncateUrl: (url: string, maxLength?: number) => string;
	}

	const { links, uniqueOpens, formatNumber, truncateUrl }: Props = $props();
</script>

<section class="glass-panel links-panel" in:fly={{ y: 20, duration: 500, delay: 250 }}>
	<div class="panel-header">
		<div class="panel-title">
			<div class="panel-icon orange">
				<IconLink size={24} />
			</div>
			<div>
				<h3>Top Links Clicked</h3>
				<span class="panel-subtitle">Most popular links in your campaign</span>
			</div>
		</div>
	</div>

	<div class="links-table">
		<div class="links-header">
			<span class="link-col url-col">URL</span>
			<span class="link-col clicks-col">Total Clicks</span>
			<span class="link-col unique-col">Unique Clicks</span>
			<span class="link-col rate-col">Click Rate</span>
		</div>
		{#each links as link, i (link.url)}
			{@const clickRate = (link.unique_clicks / uniqueOpens) * 100 || 0}
			<div class="link-row" in:fly={{ x: -10, duration: 300, delay: 300 + i * 50 }}>
				<span class="link-col url-col">
					<span class="link-rank">{i + 1}</span>
					<a href={link.url} target="_blank" rel="noopener noreferrer" class="link-url">
						{truncateUrl(link.url, 45)}
					</a>
				</span>
				<span class="link-col clicks-col">{formatNumber(link.clicks)}</span>
				<span class="link-col unique-col">{formatNumber(link.unique_clicks)}</span>
				<span class="link-col rate-col">
					<span class="click-rate">{clickRate.toFixed(1)}%</span>
				</span>
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
		margin-bottom: 1.5rem;
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

	.panel-icon.orange {
		background: var(--admin-widget-orange-bg, rgba(187, 128, 9, 0.15));
		color: var(--admin-widget-orange-icon, #d29922);
	}

	.links-table {
		overflow-x: auto;
	}

	.links-header,
	.link-row {
		display: grid;
		grid-template-columns: 2fr 1fr 1fr 1fr;
		gap: 1rem;
		align-items: center;
		padding: 0.875rem 1rem;
	}

	.links-header {
		background: var(--bg-elevated);
		border-radius: 10px;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.link-row {
		border-bottom: 1px solid var(--border-muted);
		transition: background 0.2s;
	}

	.link-row:hover {
		background: var(--bg-hover);
	}

	.link-row:last-child {
		border-bottom: none;
	}

	.url-col {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.link-rank {
		width: 24px;
		height: 24px;
		background: var(--admin-accent-primary-soft, rgba(230, 184, 0, 0.15));
		color: var(--primary-500);
		border-radius: 6px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 700;
		flex-shrink: 0;
	}

	.link-url {
		color: var(--text-primary);
		text-decoration: none;
		font-size: 0.875rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		transition: color 0.2s;
	}

	.link-url:hover {
		color: var(--primary-500);
	}

	.clicks-col,
	.unique-col,
	.rate-col {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.click-rate {
		color: var(--admin-success-text, #3fb950);
		font-weight: 600;
	}

	@media (max-width: 767.98px) {
		.links-header,
		.link-row {
			grid-template-columns: 1.5fr 1fr 1fr;
		}

		.unique-col {
			display: none;
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.link-row {
			padding: 1rem;
		}
	}
</style>
