<script lang="ts">
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import type { EmailActivity } from './types';
	import { formatDateTime, getEmailStatusColor } from './helpers';

	interface Props {
		error: string;
		loading: boolean;
		meta: { current_page: number; total_pages: number; total: number } | null;
		onPage: (page: number) => void;
		onRefresh: () => void;
		emails: EmailActivity[];
	}

	let { emails, error, loading, meta, onPage, onRefresh }: Props = $props();
</script>

<div class="emails-section">
	<p>
		Recorded provider outcomes and observation signals. These signals do not establish human
		engagement.
	</p>
	{#if loading}<p role="status">Loading emails…</p>{:else if error}<p role="alert">{error}</p>
		<button onclick={onRefresh}>Retry emails</button>{:else if emails.length === 0}
		<div class="empty-state">
			<IconMail size={48} />
			<h3>No recorded emails on this page</h3>
			<p>
				This history covers personal contact messages, CRM outbox records and linked member email
				logs.
			</p>
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
						<p>
							Scope: {email.scope.replaceAll('_', ' ')}. Provider: {email.provider ?? 'Unknown'}.
							Reference: {email.provider_message_id ?? 'Unavailable'}.
						</p>
						{#if email.error}<p role="alert">{email.error}</p>{/if}{#if email.campaign_name}
							<span class="email-campaign">Campaign: {email.campaign_name}</span>
						{/if}
						<div class="email-meta">
							<span>Provider accepted: {formatDateTime(email.accepted_at)}</span>
							{#if email.opened_at}
								<span>Open signal: {formatDateTime(email.opened_at)}</span>
							{/if}
							{#if email.clicked_at}
								<span>Click signal: {formatDateTime(email.clicked_at)}</span>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
	{#if meta && !error}<nav aria-label="emails pages">
			<button
				disabled={loading || meta.current_page <= 1}
				onclick={() => onPage(meta.current_page - 1)}>Previous</button
			><span>Page {meta.current_page} of {meta.total_pages}; {meta.total} records</span><button
				disabled={loading || meta.current_page >= meta.total_pages}
				onclick={() => onPage(meta.current_page + 1)}>Next</button
			>
		</nav>{/if}
</div>

<style>
	nav {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		margin-top: 1rem;
	}
	[role='alert'] {
		color: #fca5a5;
	}
	.emails-section {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
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
		background: var(--bg-base, #0d1117);
		border-radius: 10px;
	}

	.email-icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--primary-500);
		border-radius: 10px;
		color: var(--bg-base, #0d1117);
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
		color: #ffffff;
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
		color: var(--text-secondary, #8b949e);
	}

	.email-meta {
		display: flex;
		gap: 16px;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
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
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 16px;
	}

	.empty-state h3 {
		margin: 0 0 8px;
		color: #ffffff;
		font-size: 1.1rem;
	}

	.empty-state p {
		margin: 0 0 20px;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.9rem;
	}
</style>
