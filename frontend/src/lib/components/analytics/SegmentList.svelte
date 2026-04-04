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

<div class="segment-container">
	<div class="segment-header">
		<h3 class="segment-title">User Segments</h3>
		<p class="segment-subtitle">{segments.length} segments defined</p>
	</div>

	<div class="segment-list">
		{#each sortedSegments as segment (segment.key)}
			<button
				class="segment-item"
				data-clickable={onSelect ? '' : undefined}
				onclick={() => onSelect?.(segment)}
				disabled={!onSelect}
			>
				<!-- Icon -->
				<div
					class="segment-icon"
					style:--seg-color={segment.color || '#6B7280'}
				>
					{iconMap[segment.icon || ''] || '📊'}
				</div>

				<!-- Info -->
				<div class="segment-info">
					<div class="segment-name-row">
						<span class="segment-name">{segment.name}</span>
						{#if segment.is_system}
							<span class="system-badge">System</span>
						{/if}
						<span class="segment-type">
							{typeLabels[segment.type] || segment.type}
						</span>
					</div>
					{#if segment.description}
						<p class="segment-desc">{segment.description}</p>
					{/if}
				</div>

				<!-- Stats -->
				<div class="segment-stats">
					<div class="segment-count">
						{formatNumber(segment.user_count)}
					</div>
					<div class="segment-pct">
						{segment.percentage.toFixed(1)}% of users
					</div>
				</div>

				<!-- Progress bar -->
				<div class="progress-track">
					<div
						class="progress-fill"
						style="width: {Math.min(segment.percentage, 100)}%; background-color: {segment.color || '#6B7280'}"
					></div>
				</div>
			</button>
		{/each}
	</div>

	{#if segments.length === 0}
		<div class="empty-state">
			<p>No segments defined</p>
		</div>
	{/if}
</div>

<style>
	.segment-container {
		background-color: oklch(1 0 0);
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
	}

	.segment-header {
		padding: var(--space-4);
		border-block-end: 1px solid oklch(0.95 0.002 265);
	}

	.segment-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.segment-subtitle {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		margin-block-start: var(--space-1);
	}

	.segment-list {
		& > :not(:first-child) {
			border-block-start: 1px solid oklch(0.95 0.002 265);
		}
	}

	.segment-item {
		inline-size: 100%;
		padding: var(--space-4);
		text-align: start;
		display: flex;
		align-items: center;
		gap: var(--space-4);
		background: none;
		border: none;
		transition: background-color var(--duration-fast) var(--ease-default);

		&:hover { background-color: oklch(0.97 0.002 265); }
		&[data-clickable] { cursor: pointer; }
	}

	.segment-icon {
		inline-size: 2.5rem;
		block-size: 2.5rem;
		border-radius: var(--radius-lg);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: var(--text-lg);
		background-color: color-mix(in oklch, var(--seg-color) 12%, transparent);
	}

	.segment-info { flex: 1; min-inline-size: 0; }

	.segment-name-row { display: flex; align-items: center; gap: var(--space-2); }

	.segment-name {
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
	}

	.system-badge {
		font-size: var(--text-xs);
		background-color: oklch(0.92 0.04 260);
		color: oklch(0.45 0.15 260);
		padding-inline: 0.375rem;
		padding-block: 0.125rem;
		border-radius: var(--radius-sm);
	}

	.segment-type { font-size: var(--text-xs); color: oklch(0.65 0.01 265); }

	.segment-desc {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.segment-stats { text-align: end; flex-shrink: 0; }

	.segment-count {
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.segment-pct { font-size: var(--text-xs); color: oklch(0.55 0.01 265); }

	.progress-track {
		inline-size: 6rem;
		block-size: 0.5rem;
		background-color: oklch(0.95 0.002 265);
		border-radius: 9999px;
		overflow: hidden;
		flex-shrink: 0;
	}

	.progress-fill {
		block-size: 100%;
		border-radius: 9999px;
		transition: all var(--duration-fast) var(--ease-default);
	}

	.empty-state {
		padding: var(--space-8);
		text-align: center;
		color: oklch(0.55 0.01 265);
	}
</style>
