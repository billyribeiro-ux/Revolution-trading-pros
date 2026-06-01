<script lang="ts">
	import { IconArrowLeft, IconMail, IconX, IconPlus, IconFileText } from '$lib/icons';
	import { goto } from '$app/navigation';
	import type { Member } from '$lib/api/members';
	import { getMemberInitials, getStatusColor } from './helpers';

	interface Props {
		member: Member;
		tags: string[];
		onRemoveTag: (tag: string) => void;
		onOpenTagModal: () => void;
		onOpenNoteModal: () => void;
		onOpenEmailModal: () => void;
	}

	let { member, tags, onRemoveTag, onOpenTagModal, onOpenNoteModal, onOpenEmailModal }: Props =
		$props();
</script>

<div class="page-header">
	<button class="back-btn" onclick={() => goto('/admin/members')}>
		<IconArrowLeft size={20} />
		Back to Members
	</button>

	<div class="header-content">
		<div class="member-profile">
			<div class="member-avatar large">
				{#if member.avatar}
					<img src={member.avatar} alt={member.name} width="80" height="80" loading="lazy" />
				{:else}
					{getMemberInitials(member)}
				{/if}
			</div>
			<div class="member-info">
				<div class="member-name-row">
					<h1>{member.name}</h1>
					<span class="status-badge {getStatusColor(member.status)}">
						{member.status_label}
					</span>
				</div>
				<p class="member-email">{member.email || ''}</p>
				<div class="member-tags">
					{#each tags as tag (tag)}
						<span class="tag">
							{tag}
							<button class="tag-remove" onclick={() => onRemoveTag(tag)}>
								<IconX size={12} />
							</button>
						</span>
					{/each}
					<button class="tag-add" onclick={onOpenTagModal}>
						<IconPlus size={14} />
						Add Tag
					</button>
				</div>
			</div>
		</div>

		<div class="header-actions">
			<button class="btn-secondary" onclick={onOpenNoteModal}>
				<IconFileText size={18} />
				Add Note
			</button>
			<button class="btn-primary" onclick={onOpenEmailModal}>
				<IconMail size={18} />
				Send Email
			</button>
		</div>
	</div>
</div>

<style>
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: #94a3b8;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: var(--primary-400);
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
	}

	.member-profile {
		display: flex;
		gap: 1.5rem;
	}

	.member-avatar {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1.5rem;
		color: white;
		overflow: hidden;
	}

	.member-avatar img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.member-name-row {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.member-name-row h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.member-email {
		color: #64748b;
		margin: 0.25rem 0 0.75rem;
	}

	.member-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.75rem;
		background: rgba(230, 184, 0, 0.15);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--primary-400);
	}

	.tag-remove {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		color: #94a3b8;
		display: flex;
		transition: color 0.2s;
	}

	.tag-remove:hover {
		color: #ef4444;
	}

	.tag-add {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px dashed rgba(148, 163, 184, 0.3);
		border-radius: 9999px;
		font-size: 0.75rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tag-add:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
		text-transform: capitalize;
	}

	.btn-primary,
	.btn-secondary {
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
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: var(--bg-base);
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}

	@media (max-width: 767.98px) {
		.header-content {
			flex-direction: column;
			gap: 1.5rem;
		}

		.header-actions {
			width: 100%;
		}

		.header-actions button {
			flex: 1;
		}
	}
</style>
