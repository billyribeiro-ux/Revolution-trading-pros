<!--
	CampaignOverview — campaign name, status badge, subject, and meta row.
	Extracted from +page.svelte (R22-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconMail, IconCalendar, IconUsers } from '$lib/icons';

	interface Overview {
		name: string;
		subject: string;
		status: string;
		sent_at: string | null;
		segment_name: string | null;
		template_name: string | null;
	}

	interface Props {
		overview: Overview;
		formatDate: (dateStr: string | null) => string;
		getStatusColor: (status: string) => string;
	}

	const { overview, formatDate, getStatusColor }: Props = $props();
</script>

<section class="overview-section glass-panel" in:fly={{ y: 20, duration: 500, delay: 100 }}>
	<div class="overview-header">
		<div class="campaign-info">
			<div class="campaign-status-row">
				<h2>{overview.name}</h2>
				<span class={['status-badge', getStatusColor(overview.status)]}>
					{overview.status}
				</span>
			</div>
			<p class="campaign-subject">{overview.subject}</p>
		</div>
		<div class="campaign-meta">
			<div class="meta-item">
				<IconCalendar size={16} />
				<span>Sent: {formatDate(overview.sent_at)}</span>
			</div>
			{#if overview.segment_name}
				<div class="meta-item">
					<IconUsers size={16} />
					<span>Segment: {overview.segment_name}</span>
				</div>
			{/if}
			{#if overview.template_name}
				<div class="meta-item">
					<IconMail size={16} />
					<span>Template: {overview.template_name}</span>
				</div>
			{/if}
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

	.overview-section {
		margin-bottom: 1.5rem;
	}

	.overview-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.campaign-info h2 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.5rem;
	}

	.campaign-status-row {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.status-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.success {
		background: var(--admin-success-bg, rgba(46, 160, 67, 0.15));
		color: var(--admin-success-text, #3fb950);
	}

	.status-badge.info {
		background: var(--admin-info-bg, rgba(56, 139, 253, 0.15));
		color: var(--admin-info-text, #58a6ff);
	}

	.status-badge.warning {
		background: var(--admin-warning-bg, rgba(187, 128, 9, 0.15));
		color: var(--admin-warning-text, #d29922);
	}

	.status-badge.muted {
		background: var(--admin-widget-muted-bg, var(--bg-hover));
		color: var(--admin-widget-muted-icon, var(--text-secondary));
	}

	.status-badge.error {
		background: var(--admin-error-bg, rgba(218, 54, 51, 0.15));
		color: var(--admin-error-text, #f85149);
	}

	.campaign-subject {
		color: var(--text-secondary);
		font-size: 1rem;
		margin: 0;
	}

	.campaign-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: var(--text-secondary);
	}
</style>
