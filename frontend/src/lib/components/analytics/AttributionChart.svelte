<script lang="ts">
	/**
	 * AttributionChart - Channel Attribution Visualization
	 *
	 * Displays marketing channel attribution with multiple
	 * attribution models comparison.
	 */
	import type { ChannelAttribution } from '$lib/api/analytics';

	interface Props {
		channels?: ChannelAttribution[];
		title?: string;
		model?: string;
	}

	let { channels = [], title = 'Channel Attribution', model = 'linear' }: Props = $props();

	// Channel names for data-attribute styling
	const validChannels = ['organic', 'paid', 'social', 'email', 'direct', 'referral', 'affiliate'];
	function getChannelKey(channel: string): string {
		return validChannels.includes(channel) ? channel : 'default';
	}

	// Total for percentage calculations
	let totalConversions = $derived(channels.reduce((sum, c) => sum + c.attributed_conversions, 0));
	let totalRevenue = $derived(channels.reduce((sum, c) => sum + c.attributed_revenue, 0));

	function formatNumber(num: number): string {
		if (num >= 1000000) return '$' + (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return '$' + (num / 1000).toFixed(1) + 'K';
		return '$' + num.toFixed(0);
	}

	// Model display names
	const modelNames: Record<string, string> = {
		first_touch: 'First Touch',
		last_touch: 'Last Touch',
		linear: 'Linear',
		time_decay: 'Time Decay',
		position_based: 'Position Based'
	};
</script>

<div class="attr-container">
	<div class="attr-header">
		<div>
			<h3 class="attr-title">{title}</h3>
			<p class="attr-subtitle">Attribution Model: {modelNames[model] || model}</p>
		</div>
		<div class="attr-totals">
			<div class="attr-total-value">{formatNumber(totalRevenue)}</div>
			<div class="attr-total-label">{totalConversions.toFixed(0)} conversions</div>
		</div>
	</div>

	<!-- Channel bars -->
	<div class="channel-list">
		{#each channels as channel (channel.channel)}
			{@const barWidth = totalRevenue > 0 ? (channel.attributed_revenue / totalRevenue) * 100 : 0}
			<div class="channel-row">
				<div class="channel-meta">
					<div class="channel-name-group">
						<div
							class="channel-dot"
							data-channel={getChannelKey(channel.channel)}
						></div>
						<span class="channel-name">{channel.channel}</span>
					</div>
					<div class="channel-stats">
						<span>{channel.attributed_conversions.toFixed(1)} conv</span>
						<span class="channel-revenue">
							{formatNumber(channel.attributed_revenue)}
						</span>
					</div>
				</div>

				<div class="bar-track">
					<div
						class="bar-fill"
						data-channel={getChannelKey(channel.channel)}
						style="width: {barWidth}%"
					>
						{#if barWidth > 15}
							<span class="bar-label-inside">
								{channel.revenue_share.toFixed(1)}%
							</span>
						{/if}
					</div>
					{#if barWidth <= 15}
						<span class="bar-label-outside">
							{channel.revenue_share.toFixed(1)}%
						</span>
					{/if}
				</div>
			</div>
		{/each}
	</div>

	<!-- Summary stats -->
	<div class="attr-summary">
		<div class="summary-stat">
			<div class="summary-value">
				{channels.reduce((sum, c) => sum + c.touchpoints, 0).toLocaleString()}
			</div>
			<div class="summary-label">Total Touchpoints</div>
		</div>
		<div class="summary-stat">
			<div class="summary-value">
				{channels.reduce((sum, c) => sum + c.assisted_conversions, 0).toLocaleString()}
			</div>
			<div class="summary-label">Assisted Conversions</div>
		</div>
		<div class="summary-stat">
			<div class="summary-value">
				{totalConversions > 0 ? (totalRevenue / totalConversions).toFixed(0) : 0}
			</div>
			<div class="summary-label">Avg Conv Value</div>
		</div>
	</div>
</div>

<style>
	.attr-container {
		background-color: oklch(1 0 0);
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
		padding: var(--space-6);
	}

	.attr-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.attr-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.attr-subtitle { font-size: var(--text-sm); color: oklch(0.55 0.01 265); }

	.attr-totals { text-align: end; }

	.attr-total-value {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);
		color: oklch(0.15 0.01 265);
	}

	.attr-total-label { font-size: var(--text-sm); color: oklch(0.55 0.01 265); }

	.channel-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.channel-row {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.channel-meta {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--text-sm);
	}

	.channel-name-group { display: flex; align-items: center; gap: var(--space-2); }

	.channel-dot {
		inline-size: 0.75rem;
		block-size: 0.75rem;
		border-radius: 9999px;

		&[data-channel='organic'] { background-color: oklch(0.6 0.18 160); }
		&[data-channel='paid'] { background-color: oklch(0.6 0.2 260); }
		&[data-channel='social'] { background-color: oklch(0.55 0.2 300); }
		&[data-channel='email'] { background-color: oklch(0.7 0.18 55); }
		&[data-channel='direct'] { background-color: oklch(0.55 0.01 265); }
		&[data-channel='referral'] { background-color: oklch(0.7 0.15 200); }
		&[data-channel='affiliate'] { background-color: oklch(0.65 0.2 340); }
		&[data-channel='default'] { background-color: oklch(0.65 0.01 265); }
	}

	.channel-name {
		font-weight: var(--weight-medium);
		text-transform: capitalize;
	}

	.channel-stats {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		color: oklch(0.45 0.01 265);
	}

	.channel-revenue {
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.bar-track {
		position: relative;
		block-size: 1.5rem;
		background-color: oklch(0.95 0.002 265);
		border-radius: 9999px;
		overflow: hidden;
	}

	.bar-fill {
		block-size: 100%;
		border-radius: 9999px;
		transition: all 500ms ease-out;

		&[data-channel='organic'] { background-color: oklch(0.6 0.18 160); }
		&[data-channel='paid'] { background-color: oklch(0.6 0.2 260); }
		&[data-channel='social'] { background-color: oklch(0.55 0.2 300); }
		&[data-channel='email'] { background-color: oklch(0.7 0.18 55); }
		&[data-channel='direct'] { background-color: oklch(0.55 0.01 265); }
		&[data-channel='referral'] { background-color: oklch(0.7 0.15 200); }
		&[data-channel='affiliate'] { background-color: oklch(0.65 0.2 340); }
		&[data-channel='default'] { background-color: oklch(0.65 0.01 265); }
	}

	.bar-label-inside {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: var(--text-xs);
		color: oklch(1 0 0);
		font-weight: var(--weight-medium);
	}

	.bar-label-outside {
		position: absolute;
		inset-inline-end: var(--space-2);
		inset-block-start: 50%;
		transform: translateY(-50%);
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 265);
	}

	.attr-summary {
		margin-block-start: var(--space-6);
		padding-block-start: var(--space-4);
		border-block-start: 1px solid oklch(0.95 0.002 265);
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: var(--space-4);
		text-align: center;
	}

	.summary-value {
		font-size: var(--text-lg);
		font-weight: var(--weight-bold);
		color: oklch(0.15 0.01 265);
	}

	.summary-label { font-size: var(--text-xs); color: oklch(0.55 0.01 265); }
</style>
