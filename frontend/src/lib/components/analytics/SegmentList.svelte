<script lang="ts">
	/**
	 * SegmentList - User Segments Display
	 *
	 * Lists user segments with counts and percentages.
	 */
	import type { Segment } from '$lib/api/analytics';

	interface Props {
		segments?: Segment[];
		onSelect?: ((segment: Segment) => void) | null;
	}

	let { segments = [], onSelect = null }: Props = $props();

	// Segment type labels
	const typeLabels: Record<string, string> = {
		static: 'Static',
		dynamic: 'Dynamic',
		computed: 'Computed'
	};

	// Icon mapping
	const iconMap: Record<string, string> = {
		star: '⭐',
		'alert-triangle': '⚠️',
		'user-x': '🚫',
		'user-plus': '➕',
		crown: '👑',
		'credit-card': '💳',
		user: '👤',
		search: '🔍',
		target: '🎯'
	};

	// Sort segments: system first, then by user count
	let sortedSegments = $derived([...segments].sort((a, b) => {
		if (a.is_system !== b.is_system) return a.is_system ? -1 : 1;
		return b.user_count - a.user_count;
	}));

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}
</script>

<div class="bg-white rounded-xl border border-gray-200">
	<div class="p-4 border-b border-gray-100">
		<h3 class="text-lg font-semibold text-gray-900">User Segments</h3>
		<p class="text-sm text-gray-500 mt-1">{segments.length} segments defined</p>
	</div>

	<div class="divide-y divide-gray-100">
		{#each sortedSegments as segment (segment.key)}
			<button
				class="w-full p-4 hover:bg-gray-50 transition-colors text-left flex items-center gap-4
					{onSelect ? 'cursor-pointer' : ''}"
				onclick={() => onSelect?.(segment)}
				disabled={!onSelect}
			>
				<!-- Icon -->
				<div
					class="w-10 h-10 rounded-lg flex items-center justify-center text-lg"
					style="background-color: {segment.color || '#6B7280'}20"
				>
					{iconMap[segment.icon || ''] || '📊'}
				</div>

				<!-- Info -->
				<div class="flex-1 min-w-0">
					<div class="flex items-center gap-2">
						<span class="font-medium text-gray-900">{segment.name}</span>
						{#if segment.is_system}
							<span class="text-xs bg-blue-100 text-blue-700 px-1.5 py-0.5 rounded"> System </span>
						{/if}
						<span class="text-xs text-gray-400">
							{typeLabels[segment.type] || segment.type}
						</span>
					</div>
					{#if segment.description}
						<p class="text-sm text-gray-500 truncate">{segment.description}</p>
					{/if}
				</div>

				<!-- Stats -->
				<div class="text-right flex-shrink-0">
					<div class="font-semibold text-gray-900">
						{formatNumber(segment.user_count)}
					</div>
					<div class="text-xs text-gray-500">
						{segment.percentage.toFixed(1)}% of users
					</div>
				</div>

				<!-- Progress bar -->
				<div class="w-24 h-2 bg-gray-100 rounded-full overflow-hidden flex-shrink-0">
					<div
						class="h-full rounded-full transition-all"
						style="width: {Math.min(segment.percentage, 100)}%; background-color: {segment.color ||
							'#6B7280'}"
					></div>
				</div>
			</button>
		{/each}
	</div>

	{#if segments.length === 0}
		<div class="p-8 text-center text-gray-500">
			<p>No segments defined</p>
		</div>
	{/if}
</div>
