<script lang="ts">
	import { IconArrowLeft, IconMail, IconX, IconPlus, IconFileText } from '#lib/icons/index.js';
	import type { MemberProfileRecord } from '#lib/api/member-profile.js';
	import {
		profileName,
		profileInitials,
		cohortLabel,
		accountActivityLabel,
		recordedInstant
	} from './profile-display.js';
	let {
		member,
		tagRecords,
		tagsBusy = false,
		tagsError = '',
		onRemoveTag,
		onOpenTagModal,
		onOpenNoteModal,
		onOpenEmailModal
	}: {
		member: MemberProfileRecord;
		tagRecords: MemberProfileRecord['tag_records'];
		tagsBusy?: boolean;
		tagsError?: string;
		onRemoveTag: (id: string) => void;
		onOpenTagModal: () => void;
		onOpenNoteModal: () => void;
		onOpenEmailModal: () => void;
	} = $props();
	const unmatchedTags = $derived(
		member.tags.filter((name) => !tagRecords.some((tag) => tag.name === name))
	);
</script>

<div class="page-header">
	<a class="back-btn" href="/admin/members"><IconArrowLeft size={20} />Back to Members</a>
	<div class="header-content">
		<div class="member-profile">
			<div class="member-avatar">
				{#if member.avatar}<img
						src={member.avatar}
						alt={profileName(member)}
						width="80"
						height="80"
						loading="lazy"
					/>{:else}{profileInitials(member)}{/if}
			</div>
			<div class="member-info">
				<h1>{profileName(member)}</h1>
				<p>{member.email}</p>
				<div class="facts">
					<span>Membership cohort: {cohortLabel(member.status)}</span><span
						>{accountActivityLabel(member.account.is_active)}</span
					><span>Global role: {member.role ?? 'Not recorded'}</span>
				</div>
				{#if member.account.recorded_restriction}<p>
						Recorded account status: {member.account.recorded_restriction}
					</p>{/if}{#if member.account.reason || member.account.ban_reason}<p>
						{member.account.reason ?? member.account.ban_reason}
					</p>{/if}{#if member.account.banned_until}<p>
						Recorded restriction expiry: {recordedInstant(member.account.banned_until)}
					</p>{/if}
				<div class="member-tags">
					{#each tagRecords as tag (tag.id)}<span class="tag"
							>{tag.name}<button
								class="tag-remove"
								aria-label={`Remove tag ${tag.name}`}
								disabled={tagsBusy}
								onclick={() => onRemoveTag(tag.id)}><IconX size={12} /></button
							></span
						>{/each}{#each unmatchedTags as tag (tag)}<span
							class="tag"
							title="No saved tag identity was supplied">{tag}</span
						>{/each}<button class="tag-add" disabled={tagsBusy} onclick={onOpenTagModal}
						><IconPlus size={14} />Add Tag</button
					>
				</div>
				{#if tagsError}<p role="alert">{tagsError}</p>{/if}
			</div>
		</div>
		<div class="header-actions">
			<button onclick={onOpenNoteModal}><IconFileText size={18} />Add Note</button><button
				class="primary"
				onclick={onOpenEmailModal}><IconMail size={18} />Send Email</button
			>
		</div>
	</div>
</div>

<style>
	.page-header {
		margin-bottom: 2rem;
	}
	.back-btn {
		display: inline-flex;
		gap: 0.5rem;
		align-items: center;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 1rem;
		text-decoration: none;
	}
	.header-content,
	.member-profile,
	.header-actions,
	.member-tags,
	.facts {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}
	.header-content {
		justify-content: space-between;
		align-items: flex-start;
	}
	.member-profile {
		flex-wrap: nowrap;
	}
	.member-avatar {
		width: 80px;
		height: 80px;
		flex-shrink: 0;
		background: var(--primary-500, #e6b800);
		color: #0d1117;
		display: grid;
		place-items: center;
		border-radius: 50%;
		overflow: hidden;
		font-size: 1.5rem;
		font-weight: 700;
	}
	img {
		object-fit: cover;
	}
	h1 {
		font-size: 1.75rem;
		margin: 0;
	}
	p {
		color: var(--text-secondary, #8b949e);
		overflow-wrap: anywhere;
	}
	.facts {
		font-size: 0.85rem;
	}
	.member-tags {
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
	}
	.tag {
		display: inline-flex;
		gap: 0.4rem;
		align-items: center;
		border: 1px solid var(--border-default, #30363d);
		padding: 0.25rem 0.5rem;
		border-radius: 7px;
	}
	.tag-remove {
		border: 0;
		padding: 0.2rem;
	}
	.header-actions {
		gap: 0.5rem;
	}
	button {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
		border: 1px solid var(--border-default, #30363d);
		padding: 0.6rem 0.8rem;
		border-radius: 9px;
		cursor: pointer;
	}
	.primary {
		background: var(--primary-500, #e6b800);
		color: #0d1117;
	}
	button:disabled {
		opacity: 0.5;
		cursor: default;
	}
	[role='alert'] {
		color: #f85149;
	}
</style>
