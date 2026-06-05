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
	let sortedSegments = $derived(
		[...segments].sort((a, b) => {
			if (a.is_system !== b.is_system) return a.is_system ? -1 : 1;
			return b.user_count - a.user_count;
		})
	);

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}
</script>

<div class="segment-list">
	<div class="segment-list__header">
		<h3>User Segments</h3>
		<p>{segments.length} segments defined</p>
	</div>

	<div class="segment-list__items">
		{#each sortedSegments as segment (segment.key)}
			<button
				class={{
					'segment-list__item': true,
					'segment-list__item--selectable': Boolean(onSelect)
				}}
				onclick={() => onSelect?.(segment)}
				disabled={!onSelect}
			>
				<!-- Icon -->
				<div class="segment-list__icon" style:background-color={`${segment.color || '#6b7280'}20`}>
					{iconMap[segment.icon || ''] || '📊'}
				</div>

				<!-- Info -->
				<div class="segment-list__info">
					<div class="segment-list__meta">
						<span class="segment-list__name">{segment.name}</span>
						{#if segment.is_system}
							<span class="segment-list__system-badge">System</span>
						{/if}
						<span class="segment-list__type">
							{typeLabels[segment.type] || segment.type}
						</span>
					</div>
					{#if segment.description}
						<p class="segment-list__description">{segment.description}</p>
					{/if}
				</div>

				<!-- Stats -->
				<div class="segment-list__stats">
					<div class="segment-list__count">
						{formatNumber(segment.user_count)}
					</div>
					<div class="segment-list__percentage">
						{segment.percentage.toFixed(1)}% of users
					</div>
				</div>

				<!-- Progress bar -->
				<div class="segment-list__progress">
					<div
						class="segment-list__progress-value"
						style:width={`${Math.min(segment.percentage, 100)}%`}
						style:background-color={segment.color || '#6b7280'}
					></div>
				</div>
			</button>
		{/each}
	</div>

	{#if segments.length === 0}
		<div class="segment-list__empty">
			<p>No segments defined</p>
		</div>
	{/if}
</div>

<style>
	.segment-list {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
	}

	.segment-list__header {
		border-bottom: 1px solid #f3f4f6;
		padding: 1rem;
	}

	.segment-list__header h3 {
		margin: 0;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.segment-list__header p {
		margin: 0.25rem 0 0;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.segment-list__items {
		display: grid;
	}

	.segment-list__item {
		display: flex;
		width: 100%;
		align-items: center;
		gap: 1rem;
		border: 0;
		border-top: 1px solid #f3f4f6;
		background: transparent;
		color: inherit;
		font: inherit;
		padding: 1rem;
		text-align: left;
		transition: background 160ms ease;
	}

	.segment-list__item:first-child {
		border-top: 0;
	}

	.segment-list__item--selectable {
		cursor: pointer;
	}

	.segment-list__item--selectable:hover {
		background: #f9fafb;
	}

	.segment-list__item:disabled {
		cursor: default;
	}

	.segment-list__icon {
		display: flex;
		width: 2.5rem;
		height: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: 0.5rem;
		font-size: 1.125rem;
		line-height: 1.75rem;
		flex: 0 0 auto;
	}

	.segment-list__info {
		min-width: 0;
		flex: 1 1 auto;
	}

	.segment-list__meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-width: 0;
	}

	.segment-list__name {
		overflow: hidden;
		color: #111827;
		font-weight: 500;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.segment-list__system-badge {
		border-radius: 0.25rem;
		background: #dbeafe;
		color: #1d4ed8;
		font-size: 0.75rem;
		line-height: 1rem;
		padding: 0.125rem 0.375rem;
	}

	.segment-list__type {
		color: #9ca3af;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.segment-list__description {
		overflow: hidden;
		margin: 0;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.25rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.segment-list__stats {
		text-align: right;
		flex: 0 0 auto;
	}

	.segment-list__count {
		color: #111827;
		font-weight: 600;
	}

	.segment-list__percentage {
		color: #6b7280;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.segment-list__progress {
		overflow: hidden;
		width: 6rem;
		height: 0.5rem;
		border-radius: 999px;
		background: #f3f4f6;
		flex: 0 0 auto;
	}

	.segment-list__progress-value {
		height: 100%;
		border-radius: 999px;
		transition: width 160ms ease;
	}

	.segment-list__empty {
		padding: 2rem;
		color: #6b7280;
		text-align: center;
	}

	.segment-list__empty p {
		margin: 0;
	}

	@media (max-width: 640px) {
		.segment-list__item {
			align-items: flex-start;
			flex-wrap: wrap;
		}

		.segment-list__stats {
			text-align: left;
		}

		.segment-list__progress {
			width: 100%;
		}
	}
</style>
