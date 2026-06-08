<script lang="ts">
	import {
		IconMail,
		IconSend,
		IconCheck,
		IconClock,
		IconAlertTriangle,
		IconTrendingUp
	} from '$lib/icons';
	import type { EmailHistoryItem } from './helpers';
	import { formatDateTime, getEmailStatusColor } from './helpers';

	interface Props {
		emailHistory: EmailHistoryItem[];
		onSendEmail: () => void;
	}

	let { emailHistory, onSendEmail }: Props = $props();
</script>

<div class="panel">
	<div class="panel-header">
		<h3>Email History</h3>
		<button class="btn-primary small" onclick={onSendEmail}>
			<IconSend size={16} />
			Send Email
		</button>
	</div>
	{#if emailHistory.length === 0}
		<div class="empty-state">
			<IconMail size={48} stroke={1} />
			<h4>No Emails Sent</h4>
			<p>No emails have been sent to this member yet</p>
		</div>
	{:else}
		<div class="email-list">
			{#each emailHistory as email (email.id)}
				<div class="email-item">
					<div class={['email-icon', getEmailStatusColor(email.status)]}>
						<IconMail size={20} />
					</div>
					<div class="email-content">
						<div class="email-subject">{email.subject}</div>
						<div class="email-meta">
							<span class="email-campaign">{email.campaign_type}</span>
							<span class="email-date">{formatDateTime(email.sent_at)}</span>
						</div>
					</div>
					<div class={['email-status', getEmailStatusColor(email.status)]}>
						{#if email.status === 'opened'}
							<IconCheck size={16} />
							Opened
						{:else if email.status === 'clicked'}
							<IconTrendingUp size={16} />
							Clicked
						{:else if email.status === 'bounced'}
							<IconAlertTriangle size={16} />
							Bounced
						{:else}
							<IconClock size={16} />
							Sent
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.panel {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.panel-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
	}

	.empty-state h4 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.email-list {
		padding: 1rem;
	}

	.email-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.email-item:last-child {
		border-bottom: none;
	}

	.email-icon {
		width: 40px;
		height: 40px;
		border-radius: 10px;
		background: rgba(230, 184, 0, 0.1);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.email-content {
		flex: 1;
	}

	.email-subject {
		font-size: 0.875rem;
		color: #f1f5f9;
		font-weight: 500;
	}

	.email-meta {
		display: flex;
		gap: 1rem;
		margin-top: 0.25rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.email-status {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-primary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}
</style>
