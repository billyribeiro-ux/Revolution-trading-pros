<script lang="ts">
	import type { ActivityLog } from '$lib/types/dashboard';

	export let data: { activities?: ActivityLog[]; total_count?: number };
	export let config: {
		limit?: number;
		filter_actions?: string[];
		filter_entity_types?: string[];
		show_user?: boolean;
		show_time?: boolean;
		group_by_date?: boolean;
	} = {};

	// Apply filters
	$: filteredActivities = (data?.activities || [])
		.filter((activity) => {
			if (config.filter_actions && !config.filter_actions.includes(activity.action)) {
				return false;
			}
			if (
				config.filter_entity_types &&
				!config.filter_entity_types.includes(activity.entity_type)
			) {
				return false;
			}
			return true;
		})
		.slice(0, config.limit || 20);

	$: showUser = config.show_user !== false;
	$: showTime = config.show_time !== false;

	function getActivityIcon(action: string): string {
		switch (action) {
			case 'created':
				return '➕';
			case 'updated':
				return '✏️';
			case 'deleted':
				return '🗑️';
			case 'viewed':
				return '👁️';
			default:
				return '📝';
		}
	}

	function formatTime(timestamp: string): string {
		const date = new Date(timestamp);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);

		if (diffMins < 1) return 'Just now';
		if (diffMins < 60) return `${diffMins}m ago`;

		const diffHours = Math.floor(diffMins / 60);
		if (diffHours < 24) return `${diffHours}h ago`;

		const diffDays = Math.floor(diffHours / 24);
		return `${diffDays}d ago`;
	}
</script>

<div class="recent-activity">
	{#if filteredActivities.length > 0}
		<div class="activity-list">
			{#each filteredActivities as activity}
				<div class="activity-item">
					<div class="activity-icon">{getActivityIcon(activity.action)}</div>
					<div class="activity-content">
						<div class="activity-description">{activity.description}</div>
						<div class="activity-meta">
							{#if showUser && activity.user}
								<span class="activity-user">{activity.user.name}</span>
								<span class="separator">•</span>
							{/if}
							{#if showTime}
								<span class="activity-time">{formatTime(activity.created_at)}</span>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<div class="no-activity">No recent activity</div>
	{/if}
</div>

<style>
	.recent-activity {
		height: 100%;
		overflow-y: auto;
	}

	.activity-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.activity-item {
		display: flex;
		gap: 0.75rem;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 8px;
		transition: background 0.2s;
	}

	.activity-item:hover {
		background: #f3f4f6;
	}

	.activity-icon {
		font-size: 1.25rem;
		flex-shrink: 0;
	}

	.activity-content {
		flex: 1;
		min-width: 0;
	}

	.activity-description {
		font-size: 0.875rem;
		color: #1f2937;
		margin-bottom: 0.25rem;
	}

	.activity-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.activity-user {
		font-weight: 500;
	}

	.separator {
		color: #d1d5db;
	}

	.no-activity {
		text-align: center;
		color: #6b7280;
		padding: 2rem;
	}
</style>
