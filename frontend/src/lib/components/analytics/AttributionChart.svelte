<script lang="ts">
	/**
	 * AttributionChart - Channel Attribution Visualization
	 *
	 * Displays marketing channel attribution with multiple
	 * attribution models comparison.
	 */
	import type { ChannelAttribution } from '$lib/api/analytics';

	export let channels: ChannelAttribution[] = [];
	export let title: string = 'Channel Attribution';
	export let model: string = 'linear';

	// Channel colors
	const channelColors: Record<string, string> = {
		organic: 'bg-green-500',
		paid: 'bg-blue-500',
		social: 'bg-purple-500',
		email: 'bg-orange-500',
		direct: 'bg-gray-500',
		referral: 'bg-cyan-500',
		affiliate: 'bg-pink-500'
	};

	// Total for percentage calculations
	$: totalConversions = channels.reduce((sum, c) => sum + c.attributed_conversions, 0);
	$: totalRevenue = channels.reduce((sum, c) => sum + c.attributed_revenue, 0);

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

<div class="bg-white rounded-xl border border-gray-200 p-6">
	<div class="flex items-center justify-between mb-6">
		<div>
			<h3 class="text-lg font-semibold text-gray-900">{title}</h3>
			<p class="text-sm text-gray-500">Attribution Model: {modelNames[model] || model}</p>
		</div>
		<div class="text-right">
			<div class="text-2xl font-bold text-gray-900">{formatNumber(totalRevenue)}</div>
			<div class="text-sm text-gray-500">{totalConversions.toFixed(0)} conversions</div>
		</div>
	</div>

	<!-- Channel bars -->
	<div class="space-y-4">
		{#each channels as channel (channel.channel)}
			{@const barWidth = totalRevenue > 0 ? (channel.attributed_revenue / totalRevenue) * 100 : 0}
			<div class="space-y-2">
				<div class="flex items-center justify-between text-sm">
					<div class="flex items-center gap-2">
						<div class="w-3 h-3 rounded-full {channelColors[channel.channel] || 'bg-gray-400'}"></div>
						<span class="font-medium capitalize">{channel.channel}</span>
					</div>
					<div class="flex items-center gap-4 text-gray-600">
						<span>{channel.attributed_conversions.toFixed(1)} conv</span>
						<span class="font-semibold text-gray-900">
							{formatNumber(channel.attributed_revenue)}
						</span>
					</div>
				</div>

				<div class="relative h-6 bg-gray-100 rounded-full overflow-hidden">
					<div
						class="{channelColors[channel.channel] || 'bg-gray-400'} h-full rounded-full
							transition-all duration-500 ease-out"
						style="width: {barWidth}%"
					>
						{#if barWidth > 15}
							<span class="absolute inset-0 flex items-center justify-center text-xs text-white font-medium">
								{channel.revenue_share.toFixed(1)}%
							</span>
						{/if}
					</div>
					{#if barWidth <= 15}
						<span class="absolute right-2 top-1/2 transform -translate-y-1/2 text-xs text-gray-500">
							{channel.revenue_share.toFixed(1)}%
						</span>
					{/if}
				</div>
			</div>
		{/each}
	</div>

	<!-- Summary stats -->
	<div class="mt-6 pt-4 border-t border-gray-100 grid grid-cols-3 gap-4 text-center">
		<div>
			<div class="text-lg font-bold text-gray-900">
				{channels.reduce((sum, c) => sum + c.touchpoints, 0).toLocaleString()}
			</div>
			<div class="text-xs text-gray-500">Total Touchpoints</div>
		</div>
		<div>
			<div class="text-lg font-bold text-gray-900">
				{channels.reduce((sum, c) => sum + c.assisted_conversions, 0).toLocaleString()}
			</div>
			<div class="text-xs text-gray-500">Assisted Conversions</div>
		</div>
		<div>
			<div class="text-lg font-bold text-gray-900">
				{totalConversions > 0 ? (totalRevenue / totalConversions).toFixed(0) : 0}
			</div>
			<div class="text-xs text-gray-500">Avg Conv Value</div>
		</div>
	</div>
</div>
