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

	const channelTones: Record<string, string> = {
		organic: 'organic',
		paid: 'paid',
		social: 'social',
		email: 'email',
		direct: 'direct',
		referral: 'referral',
		affiliate: 'affiliate'
	};

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

<div class="attribution-chart">
	<div class="attribution-chart__header">
		<div>
			<h3>{title}</h3>
			<p>Attribution Model: {modelNames[model] || model}</p>
		</div>
		<div class="attribution-chart__total">
			<div>{formatNumber(totalRevenue)}</div>
			<span>{totalConversions.toFixed(0)} conversions</span>
		</div>
	</div>

	<!-- Channel bars -->
	<div class="attribution-chart__channels">
		{#each channels as channel (channel.channel)}
			{@const barWidth = totalRevenue > 0 ? (channel.attributed_revenue / totalRevenue) * 100 : 0}
			{@const channelTone = channelTones[channel.channel] || 'unknown'}
			<div class="attribution-chart__channel">
				<div class="attribution-chart__channel-header">
					<div class="attribution-chart__channel-name">
						<div class="attribution-chart__swatch" data-tone={channelTone}></div>
						<span>{channel.channel}</span>
					</div>
					<div class="attribution-chart__channel-stats">
						<span>{channel.attributed_conversions.toFixed(1)} conv</span>
						<span>
							{formatNumber(channel.attributed_revenue)}
						</span>
					</div>
				</div>

				<div class="attribution-chart__track">
					<div class="attribution-chart__bar" data-tone={channelTone} style:width={`${barWidth}%`}>
						{#if barWidth > 15}
							<span class="attribution-chart__share attribution-chart__share--inside">
								{channel.revenue_share.toFixed(1)}%
							</span>
						{/if}
					</div>
					{#if barWidth <= 15}
						<span class="attribution-chart__share attribution-chart__share--outside">
							{channel.revenue_share.toFixed(1)}%
						</span>
					{/if}
				</div>
			</div>
		{:else}
			<div class="attribution-chart__empty">No attribution channels available.</div>
		{/each}
	</div>

	<!-- Summary stats -->
	<div class="attribution-chart__summary">
		<div class="attribution-chart__summary-item">
			<div>
				{channels.reduce((sum, c) => sum + c.touchpoints, 0).toLocaleString()}
			</div>
			<span>Total Touchpoints</span>
		</div>
		<div class="attribution-chart__summary-item">
			<div>
				{channels.reduce((sum, c) => sum + c.assisted_conversions, 0).toLocaleString()}
			</div>
			<span>Assisted Conversions</span>
		</div>
		<div class="attribution-chart__summary-item">
			<div>
				{totalConversions > 0 ? (totalRevenue / totalConversions).toFixed(0) : 0}
			</div>
			<span>Avg Conv Value</span>
		</div>
	</div>
</div>

<style>
	.attribution-chart {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
		padding: 1.5rem;
	}

	.attribution-chart__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.attribution-chart h3 {
		margin: 0;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.attribution-chart p,
	.attribution-chart__total span,
	.attribution-chart__summary-item span {
		margin: 0;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.attribution-chart__total {
		text-align: right;
	}

	.attribution-chart__total div {
		color: #111827;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 2rem;
	}

	.attribution-chart__channels {
		display: grid;
		gap: 1rem;
	}

	.attribution-chart__channel {
		display: grid;
		gap: 0.5rem;
	}

	.attribution-chart__channel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.attribution-chart__channel-name,
	.attribution-chart__channel-stats {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.attribution-chart__channel-name span {
		text-transform: capitalize;
		font-weight: 500;
	}

	.attribution-chart__channel-stats {
		color: #4b5563;
		gap: 1rem;
	}

	.attribution-chart__channel-stats span:last-child {
		color: #111827;
		font-weight: 600;
	}

	.attribution-chart__swatch {
		width: 0.75rem;
		height: 0.75rem;
		border-radius: 999px;
		flex: 0 0 auto;
	}

	.attribution-chart__track {
		position: relative;
		overflow: hidden;
		height: 1.5rem;
		border-radius: 999px;
		background: #f3f4f6;
	}

	.attribution-chart__bar {
		position: relative;
		height: 100%;
		border-radius: 999px;
		transition: width 500ms ease-out;
	}

	.attribution-chart__swatch[data-tone='organic'],
	.attribution-chart__bar[data-tone='organic'] {
		background: #22c55e;
	}

	.attribution-chart__swatch[data-tone='paid'],
	.attribution-chart__bar[data-tone='paid'] {
		background: #3b82f6;
	}

	.attribution-chart__swatch[data-tone='social'],
	.attribution-chart__bar[data-tone='social'] {
		background: #a855f7;
	}

	.attribution-chart__swatch[data-tone='email'],
	.attribution-chart__bar[data-tone='email'] {
		background: #f97316;
	}

	.attribution-chart__swatch[data-tone='direct'],
	.attribution-chart__bar[data-tone='direct'] {
		background: #6b7280;
	}

	.attribution-chart__swatch[data-tone='referral'],
	.attribution-chart__bar[data-tone='referral'] {
		background: #06b6d4;
	}

	.attribution-chart__swatch[data-tone='affiliate'],
	.attribution-chart__bar[data-tone='affiliate'] {
		background: #ec4899;
	}

	.attribution-chart__swatch[data-tone='unknown'],
	.attribution-chart__bar[data-tone='unknown'] {
		background: #9ca3af;
	}

	.attribution-chart__share {
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.attribution-chart__share--inside {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #ffffff;
		font-weight: 500;
	}

	.attribution-chart__share--outside {
		position: absolute;
		top: 50%;
		right: 0.5rem;
		color: #6b7280;
		transform: translateY(-50%);
	}

	.attribution-chart__summary {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 1rem;
		border-top: 1px solid #f3f4f6;
		margin-top: 1.5rem;
		padding-top: 1rem;
		text-align: center;
	}

	.attribution-chart__summary-item div {
		color: #111827;
		font-size: 1.125rem;
		font-weight: 700;
		line-height: 1.75rem;
	}

	.attribution-chart__summary-item span {
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.attribution-chart__empty {
		padding: 2rem 1rem;
		color: #6b7280;
		text-align: center;
	}

	@media (max-width: 640px) {
		.attribution-chart {
			padding: 1rem;
		}

		.attribution-chart__header,
		.attribution-chart__channel-header {
			align-items: flex-start;
			flex-direction: column;
		}

		.attribution-chart__total {
			text-align: left;
		}

		.attribution-chart__summary {
			grid-template-columns: 1fr;
		}
	}
</style>
