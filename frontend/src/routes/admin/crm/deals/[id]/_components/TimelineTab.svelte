<script lang="ts">
	import IconHistory from '@tabler/icons-svelte-runes/icons/history';
	import type { TimelineEvent } from '$lib/crm/types';
	import { formatDateTime } from './helpers';

	interface Props {
		timeline: TimelineEvent[];
	}

	let { timeline }: Props = $props();
</script>

<div class="timeline-section">
	{#if timeline.length === 0}
		<div class="empty-state">
			<IconHistory size={48} />
			<h3>No activity yet</h3>
			<p>Activity timeline will appear here as events occur</p>
		</div>
	{:else}
		<div class="timeline">
			{#each timeline as event (event.id)}
				<div class="timeline-item">
					<div class="timeline-dot"></div>
					<div class="timeline-content">
						<div class="timeline-header">
							<span class="timeline-title">{event.title}</span>
							<span class="timeline-time">{formatDateTime(event.occurred_at)}</span>
						</div>
						{#if event.description}
							<p class="timeline-description">{event.description}</p>
						{/if}
						{#if event.created_by}
							<span class="timeline-author">by {event.created_by.name}</span>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.timeline-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.timeline {
		display: flex;
		flex-direction: column;
	}

	.timeline-item {
		display: flex;
		gap: 16px;
		padding: 16px 0;
		border-bottom: 1px solid #334155;
	}

	.timeline-item:last-child {
		border-bottom: none;
	}

	.timeline-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: #f97316;
		flex-shrink: 0;
		margin-top: 4px;
	}

	.timeline-content {
		flex: 1;
	}

	.timeline-header {
		display: flex;
		justify-content: space-between;
		gap: 12px;
		margin-bottom: 4px;
	}

	.timeline-title {
		font-weight: 600;
		color: white;
		font-size: 0.9rem;
	}

	.timeline-time {
		font-size: 0.75rem;
		color: #64748b;
		white-space: nowrap;
	}

	.timeline-description {
		margin: 0 0 4px;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	.timeline-author {
		font-size: 0.75rem;
		color: #64748b;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		color: white;
	}

	.empty-state p {
		margin: 0 0 20px;
	}
</style>
