<!--
	MetricsGrid — 6 key metric cards (sent, delivered, opens, clicks, bounced, unsubscribed).
	Extracted from +page.svelte (R22-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import { fly, scale } from 'svelte/transition';
	import {
		IconSend,
		IconCheck,
		IconEye,
		IconClick,
		IconAlertTriangle,
		IconUserMinus
	} from '$lib/icons';

	interface Metrics {
		sent: number;
		delivered: number;
		unique_opens: number;
		unique_clicks: number;
		bounced: number;
		unsubscribed: number;
	}

	interface Rates {
		delivery_rate: number;
		unique_open_rate: number;
		unique_click_rate: number;
		bounce_rate: number;
		unsubscribe_rate: number;
	}

	interface Props {
		metrics: Metrics;
		rates: Rates;
		formatNumber: (num: number) => string;
	}

	const { metrics, rates, formatNumber }: Props = $props();
</script>

<section class="metrics-section" in:fly={{ y: 20, duration: 500, delay: 150 }}>
	<div class="metrics-grid">
		<div class="metric-card purple" in:scale={{ duration: 400, delay: 200 }}>
			<div class="metric-icon purple">
				<IconSend size={24} />
			</div>
			<div class="metric-content">
				<span class="metric-value">{formatNumber(metrics.sent)}</span>
				<span class="metric-label">Emails Sent</span>
			</div>
			<div class="metric-glow purple"></div>
		</div>

		<div class="metric-card green" in:scale={{ duration: 400, delay: 250 }}>
			<div class="metric-icon green">
				<IconCheck size={24} />
			</div>
			<div class="metric-content">
				<span class="metric-value">{formatNumber(metrics.delivered)}</span>
				<span class="metric-label">Delivered</span>
				<span class="metric-rate">{rates.delivery_rate.toFixed(1)}%</span>
			</div>
			<div class="metric-glow green"></div>
		</div>

		<div class="metric-card gold" in:scale={{ duration: 400, delay: 300 }}>
			<div class="metric-icon gold">
				<IconEye size={24} />
			</div>
			<div class="metric-content">
				<span class="metric-value">{formatNumber(metrics.unique_opens)}</span>
				<span class="metric-label">Unique Opens</span>
				<span class="metric-rate">{rates.unique_open_rate.toFixed(1)}%</span>
			</div>
			<div class="metric-glow gold"></div>
		</div>

		<div class="metric-card blue" in:scale={{ duration: 400, delay: 350 }}>
			<div class="metric-icon blue">
				<IconClick size={24} />
			</div>
			<div class="metric-content">
				<span class="metric-value">{formatNumber(metrics.unique_clicks)}</span>
				<span class="metric-label">Unique Clicks</span>
				<span class="metric-rate">{rates.unique_click_rate.toFixed(1)}%</span>
			</div>
			<div class="metric-glow blue"></div>
		</div>

		<div class="metric-card orange" in:scale={{ duration: 400, delay: 400 }}>
			<div class="metric-icon orange">
				<IconAlertTriangle size={24} />
			</div>
			<div class="metric-content">
				<span class="metric-value">{formatNumber(metrics.bounced)}</span>
				<span class="metric-label">Bounced</span>
				<span class="metric-rate">{rates.bounce_rate.toFixed(1)}%</span>
			</div>
			<div class="metric-glow orange"></div>
		</div>

		<div class="metric-card red" in:scale={{ duration: 400, delay: 450 }}>
			<div class="metric-icon red">
				<IconUserMinus size={24} />
			</div>
			<div class="metric-content">
				<span class="metric-value">{formatNumber(metrics.unsubscribed)}</span>
				<span class="metric-label">Unsubscribed</span>
				<span class="metric-rate">{rates.unsubscribe_rate.toFixed(2)}%</span>
			</div>
			<div class="metric-glow red"></div>
		</div>
	</div>
</section>

<style>
	.metrics-section {
		margin-bottom: 1.5rem;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 1rem;
	}

	@media (max-width: 1400px) {
		.metrics-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 767.98px) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.metric-card {
		position: relative;
		background: var(--bg-elevated);
		border: 1px solid var(--border-muted);
		border-radius: 16px;
		padding: 1.25rem;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		overflow: hidden;
	}

	.metric-card:hover {
		transform: translateY(-4px);
		border-color: var(--admin-border, var(--border-default));
		box-shadow: var(--admin-card-shadow-hover, 0 8px 32px rgba(0, 0, 0, 0.5));
	}

	.metric-card.gold:hover {
		border-color: var(--primary-500);
	}
	.metric-card.blue:hover {
		border-color: var(--admin-info-border, rgba(56, 139, 253, 0.3));
	}
	.metric-card.green:hover {
		border-color: var(--admin-success-border, rgba(46, 160, 67, 0.3));
	}
	.metric-card.purple:hover {
		border-color: rgba(139, 92, 246, 0.5);
	}
	.metric-card.orange:hover {
		border-color: var(--admin-warning-border, rgba(187, 128, 9, 0.3));
	}
	.metric-card.red:hover {
		border-color: var(--admin-error-border, rgba(218, 54, 51, 0.3));
	}

	.metric-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 1rem;
	}

	.metric-icon.gold {
		background: var(--admin-widget-gold-bg);
		color: var(--admin-widget-gold-icon);
	}
	.metric-icon.blue {
		background: var(--admin-widget-blue-bg);
		color: var(--admin-widget-blue-icon);
	}
	.metric-icon.green {
		background: var(--admin-widget-green-bg);
		color: var(--admin-widget-green-icon);
	}
	.metric-icon.purple {
		background: var(--admin-widget-purple-bg);
		color: var(--admin-widget-purple-icon);
	}
	.metric-icon.orange {
		background: var(--admin-widget-orange-bg);
		color: var(--admin-widget-orange-icon);
	}
	.metric-icon.red {
		background: var(--admin-widget-red-bg);
		color: var(--admin-widget-red-icon);
	}

	.metric-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.metric-value {
		font-size: 1.75rem;
		font-weight: 800;
		color: var(--text-primary);
		line-height: 1;
	}

	.metric-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metric-rate {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--primary-500);
		margin-top: 0.25rem;
	}

	.metric-glow {
		position: absolute;
		bottom: -50%;
		left: -50%;
		width: 200%;
		height: 200%;
		opacity: 0;
		transition: opacity 0.3s;
		pointer-events: none;
	}

	.metric-card:hover .metric-glow {
		opacity: 1;
	}

	.metric-glow.gold {
		background: radial-gradient(circle, var(--admin-widget-gold-bg) 0%, transparent 70%);
	}
	.metric-glow.blue {
		background: radial-gradient(circle, var(--admin-widget-blue-bg) 0%, transparent 70%);
	}
	.metric-glow.green {
		background: radial-gradient(circle, var(--admin-widget-green-bg) 0%, transparent 70%);
	}
	.metric-glow.purple {
		background: radial-gradient(circle, var(--admin-widget-purple-bg) 0%, transparent 70%);
	}
	.metric-glow.orange {
		background: radial-gradient(circle, var(--admin-widget-orange-bg) 0%, transparent 70%);
	}
	.metric-glow.red {
		background: radial-gradient(circle, var(--admin-widget-red-bg) 0%, transparent 70%);
	}

	@media (max-width: 479.98px) {
		.metric-value {
			font-size: 1.5rem;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.metric-card {
			transition: none;
		}

		.metric-card:hover {
			transform: none;
		}
	}
</style>
