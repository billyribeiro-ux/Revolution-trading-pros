<script lang="ts">
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconSend from '@tabler/icons-svelte-runes/icons/send';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import type { Contact } from './types';
	import { getStatusColor, getLifecycleColor } from './helpers';

	interface Props {
		contact: Contact;
		contactId: string;
		onRefresh: () => void;
		onSendEmail: () => void;
		onDelete: () => void;
	}

	let { contact, contactId, onRefresh, onSendEmail, onDelete }: Props = $props();
</script>

<header class="contact-header">
	<div class="contact-avatar-section">
		<div class="avatar-large">
			{contact.full_name?.charAt(0).toUpperCase() || '?'}
		</div>
		<div class="contact-identity">
			<div class="name-row">
				<h1>{contact.full_name}</h1>
				{#if contact.is_vip}
					<span class="vip-badge">VIP</span>
				{/if}
			</div>
			{#if contact.job_title || contact.company_name}
				<p class="job-info">
					{contact.job_title || ''}
					{#if contact.job_title && contact.company_name}
						at
					{/if}
					{contact.company_name || ''}
				</p>
			{/if}
			<div class="status-badges">
				<span class={['status-badge', getStatusColor(contact.status)]}>
					{contact.status}
				</span>
				<span class={['status-badge', getLifecycleColor(contact.lifecycle_stage)]}>
					{contact.lifecycle_stage}
				</span>
			</div>
		</div>
	</div>

	<div class="header-actions">
		<button class="btn-icon" onclick={onRefresh} title="Refresh">
			<IconRefresh size={18} />
		</button>
		<button class="btn-secondary" onclick={onSendEmail}>
			<IconSend size={18} />
			Send Email
		</button>
		<a href="/admin/crm/contacts/{contactId}/edit" class="btn-primary">
			<IconEdit size={18} />
			Edit Contact
		</a>
		<button class="btn-icon danger" onclick={onDelete} title="Delete">
			<IconTrash size={18} />
		</button>
	</div>
</header>

<style>
	.contact-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 24px;
		margin-bottom: 24px;
		padding: 24px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		flex-wrap: wrap;
	}

	.contact-avatar-section {
		display: flex;
		align-items: flex-start;
		gap: 20px;
	}

	.avatar-large {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 2rem;
		font-weight: 700;
		color: white;
		flex-shrink: 0;
	}

	.contact-identity {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.name-row {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.name-row h1 {
		margin: 0;
		font-size: 1.75rem;
		font-weight: 700;
		color: white;
	}

	.vip-badge {
		padding: 4px 10px;
		background: linear-gradient(135deg, #fbbf24, #f59e0b);
		border-radius: 20px;
		font-size: 0.7rem;
		font-weight: 700;
		color: #1e293b;
	}

	.job-info {
		margin: 0;
		font-size: 0.95rem;
		color: #94a3b8;
	}

	.status-badges {
		display: flex;
		gap: 8px;
		flex-wrap: wrap;
	}

	.status-badge {
		padding: 4px 12px;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
		border: 1px solid;
	}

	.header-actions {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.btn-icon {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-500);
		border-color: rgba(230, 184, 0, 0.3);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: #0f172a;
		border: 1px solid #334155;
		color: #e2e8f0;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #1e293b;
		border-color: #475569;
	}

	@media (max-width: 639.98px) {
		.contact-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			justify-content: flex-start;
		}
	}
</style>
