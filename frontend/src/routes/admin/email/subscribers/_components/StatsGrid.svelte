<script lang="ts">
	import { IconUsers, IconCheck, IconX, IconMail } from '#lib/icons/index.js';

	import type { SubscriberStats } from '#lib/api/email.js';
	interface Props {
		stats: SubscriberStats | null;
	}

	let { stats }: Props = $props();
</script>

<div class="stats-grid">
	<div class="stat-card">
		<div class="stat-icon blue">
			<IconUsers size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats?.total.toLocaleString() ?? 'Unavailable'}</div>
			<div class="stat-label">Total Subscribers</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon emerald">
			<IconCheck size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats?.subscribed.toLocaleString() ?? 'Unavailable'}</div>
			<div class="stat-label">Active</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon slate">
			<IconX size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats?.unsubscribed.toLocaleString() ?? 'Unavailable'}</div>
			<div class="stat-label">Unsubscribed</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon red">
			<IconMail size={24} />
		</div>
		<div class="stat-content">
			<div class="stat-value">{stats?.bounced.toLocaleString() ?? 'Unavailable'}</div>
			<div class="stat-label">Bounced</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-content">
			<div class="stat-value">{stats?.pending.toLocaleString() ?? 'Unavailable'}</div>
			<div class="stat-label">Pending confirmation</div>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-content">
			<div class="stat-value">{stats?.complained.toLocaleString() ?? 'Unavailable'}</div>
			<div class="stat-label">Complaints</div>
		</div>
	</div>
</div>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 16px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.emerald {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}
	.stat-icon.slate {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}
	.stat-icon.red {
		background: rgba(248, 81, 73, 0.15);
		color: var(--error-emphasis, #f85149);
	}

	.stat-content .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.stat-content .stat-label {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
	}

	@media (max-width: 767.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
