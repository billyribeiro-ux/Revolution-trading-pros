<script lang="ts">
	import { IconMail, IconSend } from '#lib/icons/index.js';
	import type { MemberProfile } from '#lib/api/member-profile.js';
	import { recordedInstant, recordedWallClock } from './profile-display.js';
	import HistoryControls from './HistoryControls.svelte';
	let {
		result,
		loading,
		error,
		busy = false,
		onPage,
		onPerPage,
		onRefresh,
		onSendEmail
	}: {
		result: MemberProfile['emails'] | null;
		loading: boolean;
		error: string;
		busy?: boolean;
		onPage: (page: number) => void;
		onPerPage: (size: number) => void;
		onRefresh: () => void;
		onSendEmail: () => void;
	} = $props();
</script>

<section class="panel" aria-label="Linked email receipts">
	<div class="panel-header">
		<h3>Email History</h3>
		<button onclick={onSendEmail} disabled={busy}><IconSend size={16} />Send Email</button>
	</div>
	<div class="content">
		<p class="hint">
			Provider acceptance requires a stored acknowledgement. Queued or unknown outcomes do not
			establish delivery; recorded opens and clicks can include automated activity.
		</p>
		{#if result && !loading && !error}<p class="hint">{result.coverage} {result.observations}</p>
			{#if result.emails.length === 0}<div class="empty-state">
					<IconMail size={48} />
					<h4>No linked email receipts on this page</h4>
					<p>Absence of linked receipts does not establish that no email was sent.</p>
				</div>{:else}<div class="rows">
					{#each result.emails as email (email.id)}<article class="card">
							<div class="card-header">
								<h4>{email.subject}</h4>
								<span class="status">{email.outcome}</span>
							</div>
							<p>{email.recipient}</p>
							<dl>
								<div>
									<dt>Recorded status</dt>
									<dd>{email.recorded_status}</dd>
								</div>
								<div>
									<dt>Provider</dt>
									<dd>{email.provider ?? 'Not recorded'}</dd>
								</div>
								<div>
									<dt>Provider reference</dt>
									<dd>{email.provider_message_id ?? 'Not recorded'}</dd>
								</div>
								<div>
									<dt>Provider acceptance time</dt>
									<dd>{recordedInstant(email.sent_at)}</dd>
								</div>
								<div>
									<dt>Queued</dt>
									<dd>{recordedInstant(email.queued_at)}</dd>
								</div>
								<div>
									<dt>Historical record created</dt>
									<dd>{recordedWallClock(email.created_at)}</dd>
								</div>
								<div>
									<dt>Stored open signal</dt>
									<dd>{recordedWallClock(email.opened_at)}</dd>
								</div>
								<div>
									<dt>Stored click signal</dt>
									<dd>{recordedWallClock(email.clicked_at)}</dd>
								</div>
								<div>
									<dt>Stored bounce signal</dt>
									<dd>{recordedWallClock(email.bounced_at)}</dd>
								</div>
							</dl>
							{#if email.error}<p class="error">{email.error}</p>{/if}
						</article>{/each}
				</div>{/if}{/if}
	</div>
	<HistoryControls
		label="email receipts"
		pagination={result?.pagination ?? null}
		{loading}
		{error}
		{busy}
		{onPage}
		{onPerPage}
		{onRefresh}
	/>
</section>

<style>
	.panel {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 16px;
		overflow: hidden;
	}
	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}
	h3 {
		font-size: 1rem;
		margin: 0;
		color: var(--text-primary, #f0f6fc);
	}
	button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.6rem 0.9rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 9px;
		cursor: pointer;
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	.content {
		padding: 1.25rem 1.5rem;
	}
	.hint,
	.empty-state {
		color: var(--text-secondary, #8b949e);
		line-height: 1.6;
	}
	.empty-state {
		padding: 2rem;
		text-align: center;
	}
	.rows {
		display: grid;
		gap: 1rem;
	}
	.card {
		padding: 1rem;
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 12px;
		background: var(--bg-base, #0d1117);
	}
	.card-header {
		display: flex;
		justify-content: space-between;
		gap: 0.75rem;
		align-items: center;
		flex-wrap: wrap;
	}
	h4 {
		margin: 0 0 0.5rem;
		color: var(--text-primary, #f0f6fc);
	}
	dl {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}
	dt {
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}
	dd {
		margin: 0.35rem 0 0;
		overflow-wrap: anywhere;
	}
	.status {
		border: 1px solid var(--border-default, #30363d);
		border-radius: 99px;
		padding: 0.25rem 0.65rem;
		font-size: 0.8rem;
	}
	.error {
		color: var(--error-emphasis, #f85149);
	}
	p {
		overflow-wrap: anywhere;
	}
</style>
