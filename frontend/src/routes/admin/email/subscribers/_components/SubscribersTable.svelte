<script lang="ts">
	import { IconCheck, IconX, IconTrash } from '#lib/icons/index.js';
	import type { EmailSubscriber } from '#lib/api/email.js';

	export type RowAction =
		| { type: 'unsubscribe'; id: string }
		| { type: 'resubscribe'; id: string }
		| { type: 'delete'; id: string };

	interface Props {
		subscribers: EmailSubscriber[];
		selectedSubscribers: Set<string>;
		getStatusColor: (status: string) => string;
		formatDate: (dateStr: string | undefined) => string;
		ontoggleAll: () => void;
		ontoggleOne: (id: string) => void;
		onaction: (action: RowAction) => void;
	}

	let {
		subscribers,
		selectedSubscribers = $bindable(),
		getStatusColor,
		formatDate,
		ontoggleAll,
		ontoggleOne,
		onaction
	}: Props = $props();
</script>

<table class="subscribers-table">
	<thead>
		<tr>
			<th class="checkbox-col">
				<input
					id="page-checkbox"
					name="page-checkbox"
					type="checkbox"
					checked={selectedSubscribers.size === subscribers.length}
					onchange={ontoggleAll}
				/>
			</th>
			<th>Subscriber</th>
			<th>Status</th>
			<th>Tags</th>
			<th>Email Score</th>
			<th>Subscribed</th>
			<th>Last Activity</th>
			<th></th>
		</tr>
	</thead>
	<tbody>
		{#each subscribers as subscriber (subscriber.id)}
			<tr>
				<td class="checkbox-col">
					<input
						id="page-checkbox"
						name="page-checkbox"
						type="checkbox"
						checked={selectedSubscribers.has(subscriber.id)}
						onchange={() => ontoggleOne(subscriber.id)}
					/>
				</td>
				<td>
					<div class="subscriber-info">
						<div class="subscriber-avatar">
							{(subscriber.first_name?.[0] || subscriber.email?.[0] || 'U').toUpperCase()}
						</div>
						<div>
							<div class="subscriber-name">
								{subscriber.first_name || ''}
								{subscriber.last_name || ''}
							</div>
							<div class="subscriber-email">{subscriber.email || ''}</div>
						</div>
					</div>
				</td>
				<td>
					<span class={['status-badge', getStatusColor(subscriber.status)]}>
						{subscriber.status}
					</span>
				</td>
				<td>
					<div class="tags-list">
						{#each (subscriber.tags || []).slice(0, 3) as tag (tag)}
							<span class="tag">{tag}</span>
						{/each}
						{#if (subscriber.tags || []).length > 3}
							<span class="tag more">+{(subscriber.tags || []).length - 3}</span>
						{/if}
					</div>
				</td>
				<td>
					<div class="score-bar">
						<div class="score-fill" style:width={`${subscriber.email_score}%`}></div>
						<span>{subscriber.email_score}</span>
					</div>
				</td>
				<td>{formatDate(subscriber.subscribed_at)}</td>
				<td>
					{formatDate(subscriber.last_opened_at || subscriber.last_clicked_at)}
				</td>
				<td>
					<div class="row-actions">
						{#if subscriber.status === 'subscribed'}
							<button
								class="btn-icon small"
								title="Unsubscribe"
								onclick={() => onaction({ type: 'unsubscribe', id: subscriber.id })}
							>
								<IconX size={16} />
							</button>
						{:else if subscriber.status === 'unsubscribed'}
							<button
								class="btn-icon small"
								title="Resubscribe"
								onclick={() => onaction({ type: 'resubscribe', id: subscriber.id })}
							>
								<IconCheck size={16} />
							</button>
						{/if}
						<button
							class="btn-icon small danger"
							title="Delete"
							onclick={() => onaction({ type: 'delete', id: subscriber.id })}
						>
							<IconTrash size={16} />
						</button>
					</div>
				</td>
			</tr>
		{/each}
	</tbody>
</table>

<style>
	.subscribers-table {
		width: 100%;
		border-collapse: collapse;
	}

	.subscribers-table th,
	.subscribers-table td {
		padding: 1rem;
		text-align: left;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.subscribers-table th {
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-tertiary, #6e7681);
		background: var(--bg-base, #0d1117);
	}

	.subscribers-table tbody tr:hover {
		background: var(--bg-surface, #1c2128);
	}

	.checkbox-col {
		width: 40px;
	}

	.subscriber-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.subscriber-avatar {
		width: 36px;
		height: 36px;
		background: var(--primary-500);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--bg-base, #0d1117);
		font-weight: 600;
		font-size: 0.875rem;
	}

	.subscriber-name {
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
	}

	.subscriber-email {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
	}

	.status-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.tag {
		padding: 0.125rem 0.5rem;
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
		border-radius: 4px;
		font-size: 0.6875rem;
	}

	.tag.more {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
	}

	.score-bar {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.score-bar .score-fill {
		height: 6px;
		width: 60px;
		background: var(--bg-hover, #252b33);
		border-radius: 3px;
		position: relative;
	}

	.score-bar .score-fill::after {
		content: '';
		position: absolute;
		left: 0;
		top: 0;
		height: 100%;
		width: var(--width, 0%);
		background: var(--primary-500);
		border-radius: 3px;
	}

	.score-bar span {
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
	}

	.row-actions {
		display: flex;
		gap: 0.25rem;
	}

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
	}

	.btn-icon.small {
		width: 28px;
		height: 28px;
	}

	.btn-icon.danger {
		color: var(--error-emphasis, #f85149);
	}
</style>
