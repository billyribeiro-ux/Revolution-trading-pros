<!--
	AbTestResults — variant A vs B subject-line performance comparison.
	Extracted from +page.svelte (R22-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconChartBar } from '$lib/icons';

	interface AbVariant {
		subject: string;
		sent: number;
		opens: number;
		open_rate: number;
		clicks: number;
		click_rate: number;
	}

	interface AbResults {
		enabled: boolean;
		winner: 'a' | 'b' | null;
		variant_a: AbVariant;
		variant_b: AbVariant;
	}

	interface Props {
		results: AbResults;
		formatNumber: (num: number) => string;
	}

	const { results, formatNumber }: Props = $props();
</script>

<section class="glass-panel ab-panel" in:fly={{ y: 20, duration: 500, delay: 350 }}>
	<div class="panel-header">
		<div class="panel-title">
			<div class="panel-icon gold">
				<IconChartBar size={24} />
			</div>
			<div>
				<h3>A/B Test Results</h3>
				<span class="panel-subtitle">Subject line performance comparison</span>
			</div>
		</div>
		{#if results.winner}
			<span class="winner-badge">
				Winner: Variant {results.winner.toUpperCase()}
			</span>
		{/if}
	</div>

	<div class="ab-variants">
		<div class={['ab-variant', { winner: results.winner === 'a' }]}>
			<div class="variant-header">
				<span class="variant-label">A</span>
				<span class="variant-subject">{results.variant_a.subject}</span>
			</div>
			<div class="variant-stats">
				<div class="variant-stat">
					<span class="stat-value">{results.variant_a.open_rate.toFixed(1)}%</span>
					<span class="stat-label">Open Rate</span>
				</div>
				<div class="variant-stat">
					<span class="stat-value">{results.variant_a.click_rate.toFixed(1)}%</span>
					<span class="stat-label">Click Rate</span>
				</div>
				<div class="variant-stat">
					<span class="stat-value">{formatNumber(results.variant_a.sent)}</span>
					<span class="stat-label">Sent</span>
				</div>
			</div>
		</div>

		<div class={['ab-variant', { winner: results.winner === 'b' }]}>
			<div class="variant-header">
				<span class="variant-label">B</span>
				<span class="variant-subject">{results.variant_b.subject}</span>
			</div>
			<div class="variant-stats">
				<div class="variant-stat">
					<span class="stat-value">{results.variant_b.open_rate.toFixed(1)}%</span>
					<span class="stat-label">Open Rate</span>
				</div>
				<div class="variant-stat">
					<span class="stat-value">{results.variant_b.click_rate.toFixed(1)}%</span>
					<span class="stat-label">Click Rate</span>
				</div>
				<div class="variant-stat">
					<span class="stat-value">{formatNumber(results.variant_b.sent)}</span>
					<span class="stat-label">Sent</span>
				</div>
			</div>
		</div>
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

	.panel-icon.gold {
		background: var(--admin-widget-gold-bg, rgba(230, 184, 0, 0.15));
		color: var(--admin-widget-gold-icon, var(--primary-400));
	}

	.winner-badge {
		padding: 0.5rem 1rem;
		background: var(--admin-success-bg, rgba(46, 160, 67, 0.15));
		color: var(--admin-success-text, #3fb950);
		border-radius: 20px;
		font-size: 0.8125rem;
		font-weight: 600;
	}

	.ab-variants {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

	@media (max-width: 767.98px) {
		.ab-variants {
			grid-template-columns: 1fr;
		}
	}

	.ab-variant {
		padding: 1.5rem;
		background: var(--bg-elevated);
		border: 2px solid var(--border-muted);
		border-radius: 16px;
		transition: all 0.3s;
	}

	.ab-variant.winner {
		border-color: var(--admin-success, #2ea043);
		background: var(--admin-success-bg, rgba(46, 160, 67, 0.15));
	}

	.variant-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.variant-label {
		width: 32px;
		height: 32px;
		background: var(--primary-500);
		color: var(--bg-base);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		flex-shrink: 0;
	}

	.ab-variant.winner .variant-label {
		background: var(--admin-success-text, #3fb950);
	}

	.variant-subject {
		font-size: 0.9375rem;
		color: var(--text-primary);
		font-weight: 500;
	}

	.variant-stats {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
	}

	.variant-stat {
		text-align: center;
	}

	.variant-stat .stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary);
		display: block;
	}

	.ab-variant.winner .variant-stat .stat-value {
		color: var(--admin-success-text, #3fb950);
	}

	.variant-stat .stat-label {
		font-size: 0.6875rem;
		color: var(--text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}
</style>
