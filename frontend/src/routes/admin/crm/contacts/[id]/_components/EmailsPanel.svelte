<script lang="ts">
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import type { EmailActivity } from './types';
	import { formatDateTime, getEmailStatusColor } from './helpers';

	interface Props {
		emails: EmailActivity[];
	}

	let { emails }: Props = $props();
</script>

<div class="emails-section">
	{#if emails.length === 0}
		<div class="empty-state">
			<IconMail size={48} />
			<h3>No emails yet</h3>
			<p>Email history will appear here once emails are sent to this contact</p>
		</div>
	{:else}
		<div class="emails-list">
			{#each emails as email (email.id)}
				<div class="email-item">
					<div class="email-icon">
						<IconMail size={20} />
					</div>
					<div class="email-content">
						<div class="email-header">
							<span class="email-subject">{email.subject}</span>
							<span class={['email-status', getEmailStatusColor(email.status)]}>
								{email.status}
							</span>
						</div>
						{#if email.campaign_name}
							<span class="email-campaign">Campaign: {email.campaign_name}</span>
						{/if}
						<div class="email-meta">
							<span>Sent: {formatDateTime(email.sent_at)}</span>
							{#if email.opened_at}
								<span>Opened: {formatDateTime(email.opened_at)}</span>
							{/if}
							{#if email.clicked_at}
								<span>Clicked: {formatDateTime(email.clicked_at)}</span>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.emails-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.emails-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.email-item {
		display: flex;
		gap: 16px;
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
	}

	.email-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		border-radius: 10px;
		color: white;
		flex-shrink: 0;
	}

	.email-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.email-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 12px;
	}

	.email-subject {
		font-weight: 600;
		color: white;
	}

	.email-status {
		padding: 4px 10px;
		border-radius: 20px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.email-campaign {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.email-meta {
		display: flex;
		gap: 16px;
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
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		color: white;
		font-size: 1.1rem;
	}

	.empty-state p {
		margin: 0 0 20px;
		color: #64748b;
		font-size: 0.9rem;
	}
</style>
