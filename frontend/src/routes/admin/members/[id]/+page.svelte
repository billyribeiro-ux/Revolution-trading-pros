<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { membersApi } from '$lib/api/members';
	import type { Member, Subscription } from '$lib/api/members';
	import { toastStore } from '$lib/stores/toast.svelte';
	import {
		IconMail,
		IconCreditCard,
		IconReceipt,
		IconActivity,
		IconEdit,
		IconCheck,
		IconAlertTriangle,
		IconRefresh,
		IconFileText
	} from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { logger } from '$lib/utils/logger';

	import MemberHeader from './_components/MemberHeader.svelte';
	import MemberStatsGrid from './_components/MemberStatsGrid.svelte';
	import SubscriptionsTab from './_components/SubscriptionsTab.svelte';
	import OrdersTab from './_components/OrdersTab.svelte';
	import EmailsTab from './_components/EmailsTab.svelte';
	import NotesTab from './_components/NotesTab.svelte';
	import EmailModal from './_components/EmailModal.svelte';
	import NoteModal from './_components/NoteModal.svelte';
	import TagModal from './_components/TagModal.svelte';
	import ExtendMembershipModal from './_components/ExtendMembershipModal.svelte';
	import GrantMembershipModal from './_components/GrantMembershipModal.svelte';
	import type {
		EmailHistoryItem,
		MembershipPlan,
		NoteItem,
		TimelineEvent
	} from './_components/helpers';
	import { formatDateTime, getTimelineIcon } from './_components/helpers';

	let memberId = $derived(Number(page.params.id));

	// State
	let member = $state<Member | null>(null);
	let timeline = $state<TimelineEvent[]>([]);
	let engagementScore = $state(0);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Modal states
	let showEmailModal = $state(false);
	let showNoteModal = $state(false);
	let showTagModal = $state(false);

	// Email form
	let emailSubject = $state('');
	let emailBody = $state('');
	let emailSending = $state(false);

	// Notes
	let notes = $state<NoteItem[]>([]);
	let newNote = $state('');

	// Tags
	let tags = $state<string[]>([]);
	let newTag = $state('');
	let availableTags = ['VIP', 'High Value', 'At Risk', 'New', 'Engaged', 'Support Priority'];

	// Active tab
	let activeTab = $state<'overview' | 'subscriptions' | 'orders' | 'emails' | 'notes'>('overview');

	// Membership management modals
	let showExtendModal = $state(false);
	let showGrantModal = $state(false);
	let selectedSubscription = $state<Subscription | null>(null);
	let extendDays = $state(30);
	let extending = $state(false);
	let granting = $state(false);
	let availablePlans = $state<MembershipPlan[]>([]);
	let selectedPlanId = $state<number | null>(null);
	let grantExpiresAt = $state('');

	// Revoke confirmation modal state
	let showRevokeModal = $state(false);
	let pendingRevokeSubId = $state<number | null>(null);

	// Email history
	let emailHistory = $state<EmailHistoryItem[]>([]);

	onMount(async () => {
		await loadMember();
	});

	async function loadMember() {
		loading = true;
		error = null;
		try {
			const response = await membersApi.getMember(memberId);
			member = response.member;
			timeline = response.timeline || [];
			engagementScore = response.engagement_score || 0;

			// Load REAL data from API - NO MOCK DATA
			// Tags from member data
			tags = member?.tags || [];

			// Load notes from API
			await loadMemberNotes();

			// Load email history from API
			await loadEmailHistory();
		} catch (err) {
			error = 'Failed to load member details';
			logger.error('[AdminMember] Load member failed', { error: err });
		} finally {
			loading = false;
		}
	}

	async function loadMemberNotes() {
		try {
			const response = await fetch(`/api/admin/members/${memberId}/notes`);
			if (response.ok) {
				const data = await response.json();
				notes = Array.isArray(data) ? data : [];
			} else {
				// No notes available - that's okay, show empty state
				notes = [];
			}
		} catch (err) {
			logger.error('[AdminMember] Load member notes failed', { error: err });
			notes = []; // Empty array, not mock data
		}
	}

	async function loadEmailHistory() {
		try {
			const response = await fetch(`/api/admin/members/${memberId}/emails`);
			if (response.ok) {
				const data = await response.json();
				emailHistory = Array.isArray(data) ? data : [];
			} else {
				// No email history available - that's okay, show empty state
				emailHistory = [];
			}
		} catch (err) {
			logger.error('[AdminMember] Load email history failed', { error: err });
			emailHistory = []; // Empty array, not mock data
		}
	}

	async function handleSendEmail() {
		if (!emailSubject || !emailBody) return;
		emailSending = true;
		try {
			await membersApi.sendEmail(memberId, {
				subject: emailSubject,
				body: emailBody,
				campaign_type: 'general'
			});
			toastStore.success('Email sent successfully');
			showEmailModal = false;
			emailSubject = '';
			emailBody = '';
			// Refresh email history from API to get real data
			await loadEmailHistory();
		} catch {
			toastStore.error('Failed to send email');
		} finally {
			emailSending = false;
		}
	}

	async function addNote() {
		if (!newNote.trim()) return;
		try {
			const response = await fetch(`/api/admin/members/${memberId}/notes`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ content: newNote })
			});

			if (response.ok) {
				const savedNote = await response.json();
				notes = [savedNote, ...notes];
				newNote = '';
				showNoteModal = false;
				toastStore.success('Note added');
				return;
			}

			// FIX-2026-04-26 (audit 02 §P1-1): the previous else-branch inserted a
			// fake `id: Date.now()` note into local state and showed a "Note added
			// locally - sync pending" toast — but no sync ever happened, so the
			// admin would believe the note was saved while the next refresh would
			// silently drop it. Now we surface the real error and leave the input
			// populated so the admin can retry.
			let detail = `Backend returned ${response.status}`;
			try {
				const data = await response.json();
				if (data?.error) detail = String(data.error);
			} catch {
				// non-JSON error body — keep the status-code default
			}
			toastStore.error(`Failed to save note: ${detail}`);
		} catch (err) {
			logger.error('[AdminMember] Save note failed', { error: err });
			toastStore.error('Failed to save note');
		}
	}

	function addTag(tag: string) {
		if (!tags.includes(tag)) {
			tags = [...tags, tag];
			toastStore.success(`Tag "${tag}" added`);
		}
	}

	function removeTag(tag: string) {
		tags = tags.filter((t) => t !== tag);
		toastStore.success(`Tag "${tag}" removed`);
	}

	function toggleTag(tag: string) {
		if (tags.includes(tag)) {
			removeTag(tag);
		} else {
			addTag(tag);
		}
	}

	function addCustomTag() {
		if (newTag.trim() && !tags.includes(newTag.trim())) {
			tags = [...tags, newTag.trim()];
			newTag = '';
		}
	}

	// Load available membership plans for granting
	async function loadPlans() {
		try {
			const response = await fetch('/api/admin/membership-plans');
			if (response.ok) {
				availablePlans = await response.json();
			}
		} catch (err) {
			logger.error('[AdminMember] Load plans failed', { error: err });
		}
	}

	// Open extend modal
	function openExtendModal(sub: Subscription) {
		selectedSubscription = sub;
		extendDays = 30;
		showExtendModal = true;
	}

	// Open grant modal
	async function openGrantModal() {
		if (availablePlans.length === 0) {
			await loadPlans();
		}
		selectedPlanId = null;
		grantExpiresAt = '';
		showGrantModal = true;
	}

	// Extend membership
	async function handleExtend() {
		if (!selectedSubscription) return;
		extending = true;
		try {
			// Calculate new expiration date
			const currentExpiry = selectedSubscription.next_payment
				? new Date(selectedSubscription.next_payment)
				: new Date();
			currentExpiry.setDate(currentExpiry.getDate() + extendDays);

			const response = await fetch(`/api/admin/user-memberships/${selectedSubscription.id}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					expires_at: currentExpiry.toISOString()
				})
			});

			if (response.ok) {
				toastStore.success(`Membership extended by ${extendDays} days`);
				showExtendModal = false;
				await loadMember(); // Refresh data
			} else {
				const data = await response.json();
				toastStore.error(data.error || 'Failed to extend membership');
			}
		} catch (err) {
			logger.error('[AdminMember] Extend membership failed', { error: err });
			toastStore.error('Failed to extend membership');
		} finally {
			extending = false;
		}
	}

	// Grant new membership
	async function handleGrant() {
		if (!selectedPlanId) {
			toastStore.error('Please select a plan');
			return;
		}
		granting = true;
		try {
			const response = await fetch('/api/admin/user-memberships', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					user_id: memberId,
					plan_id: selectedPlanId,
					expires_at: grantExpiresAt || null,
					status: 'active'
				})
			});

			if (response.ok) {
				toastStore.success('Membership granted successfully');
				showGrantModal = false;
				await loadMember(); // Refresh data
			} else {
				const data = await response.json();
				toastStore.error(data.error || 'Failed to grant membership');
			}
		} catch (err) {
			logger.error('[AdminMember] Grant membership failed', { error: err });
			toastStore.error('Failed to grant membership');
		} finally {
			granting = false;
		}
	}

	// Revoke membership
	function handleRevoke(subId: number) {
		pendingRevokeSubId = subId;
		showRevokeModal = true;
	}

	async function confirmRevoke() {
		if (!pendingRevokeSubId) return;
		showRevokeModal = false;
		const subId = pendingRevokeSubId;
		pendingRevokeSubId = null;

		try {
			const response = await fetch(`/api/admin/user-memberships/${subId}`, {
				method: 'DELETE'
			});

			if (response.ok) {
				toastStore.success('Membership revoked');
				await loadMember(); // Refresh data
			} else {
				const data = await response.json();
				toastStore.error(data.error || 'Failed to revoke membership');
			}
		} catch (err) {
			logger.error('[AdminMember] Revoke membership failed', { error: err });
			toastStore.error('Failed to revoke membership');
		}
	}
</script>

<svelte:head>
	<title>{member?.name || 'Member'} | Revolution Trading Pros</title>
</svelte:head>

<div class="member-detail-page">
	{#if loading}
		<div class="loading-container">
			<div class="skeleton-header">
				<div class="skeleton skeleton-avatar"></div>
				<div class="skeleton-info">
					<div class="skeleton skeleton-title"></div>
					<div class="skeleton skeleton-subtitle"></div>
				</div>
			</div>
			<div class="skeleton-grid">
				<div class="skeleton skeleton-card"></div>
				<div class="skeleton skeleton-card"></div>
				<div class="skeleton skeleton-card"></div>
				<div class="skeleton skeleton-card"></div>
			</div>
		</div>
	{:else if error}
		<div class="error-state">
			<IconAlertTriangle size={48} />
			<h2>Failed to load member</h2>
			<p>{error}</p>
			<button class="btn-primary" onclick={loadMember}>
				<IconRefresh size={18} />
				Retry
			</button>
		</div>
	{:else if member}
		<MemberHeader
			{member}
			{tags}
			onRemoveTag={removeTag}
			onOpenTagModal={() => (showTagModal = true)}
			onOpenNoteModal={() => (showNoteModal = true)}
			onOpenEmailModal={() => (showEmailModal = true)}
		/>

		<MemberStatsGrid {member} {engagementScore} />

		<!-- Tabs -->
		<div class="tabs-container">
			<div class="tabs">
				<button class:active={activeTab === 'overview'} onclick={() => (activeTab = 'overview')}>
					<IconActivity size={18} />
					Overview
				</button>
				<button
					class:active={activeTab === 'subscriptions'}
					onclick={() => (activeTab = 'subscriptions')}
				>
					<IconCreditCard size={18} />
					Subscriptions
				</button>
				<button class:active={activeTab === 'orders'} onclick={() => (activeTab = 'orders')}>
					<IconReceipt size={18} />
					Orders
				</button>
				<button class:active={activeTab === 'emails'} onclick={() => (activeTab = 'emails')}>
					<IconMail size={18} />
					Emails
				</button>
				<button class:active={activeTab === 'notes'} onclick={() => (activeTab = 'notes')}>
					<IconFileText size={18} />
					Notes
				</button>
			</div>
		</div>

		<!-- Tab Content -->
		<div class="tab-content">
			{#if activeTab === 'overview'}
				<div class="overview-grid">
					<!-- Activity Timeline -->
					<div class="panel timeline-panel">
						<div class="panel-header">
							<h3>Activity Timeline</h3>
							<button class="btn-icon" onclick={loadMember}>
								<IconRefresh size={18} />
							</button>
						</div>
						<div class="timeline">
							{#if timeline.length === 0}
								<div class="empty-timeline">
									<IconActivity size={32} stroke={1} />
									<p>No activity recorded yet</p>
								</div>
							{:else}
								{#each timeline as event, i (i)}
									{@const Icon = getTimelineIcon(event.type)}
									<div class="timeline-item">
										<div class="timeline-icon">
											<Icon size={16} />
										</div>
										<div class="timeline-content">
											<div class="timeline-title">{event.title}</div>
											<div class="timeline-date">{formatDateTime(event.date)}</div>
										</div>
									</div>
								{/each}
							{/if}
						</div>
					</div>

					<!-- Member Details -->
					<div class="panel details-panel">
						<div class="panel-header">
							<h3>Member Details</h3>
							<button class="btn-icon">
								<IconEdit size={18} />
							</button>
						</div>
						<div class="details-list">
							<div class="detail-row">
								<span class="detail-label">Full Name</span>
								<span class="detail-value">{member.name}</span>
							</div>
							<div class="detail-row">
								<span class="detail-label">Email</span>
								<span class="detail-value">
									{member.email || ''}
									{#if member.email_verified}
										<IconCheck size={14} class="verified-icon" />
									{/if}
								</span>
							</div>
							<div class="detail-row">
								<span class="detail-label">Current Plan</span>
								<span class="detail-value">{member.current_plan || 'None'}</span>
							</div>
							<div class="detail-row">
								<span class="detail-label">Last Login</span>
								<span class="detail-value">
									{member.last_login ? formatDateTime(member.last_login) : 'Never'}
								</span>
							</div>
							<div class="detail-row">
								<span class="detail-label">Total Orders</span>
								<span class="detail-value">{member.orders?.length || 0}</span>
							</div>
						</div>
					</div>
				</div>
			{:else if activeTab === 'subscriptions'}
				<SubscriptionsTab
					subscriptions={member.subscriptions}
					onGrant={openGrantModal}
					onExtend={openExtendModal}
					onRevoke={handleRevoke}
				/>
			{:else if activeTab === 'orders'}
				<OrdersTab orders={member.orders} />
			{:else if activeTab === 'emails'}
				<EmailsTab {emailHistory} onSendEmail={() => (showEmailModal = true)} />
			{:else if activeTab === 'notes'}
				<NotesTab {notes} onAddNote={() => (showNoteModal = true)} />
			{/if}
		</div>
	{/if}
</div>

<EmailModal
	open={showEmailModal}
	recipientName={member?.name || ''}
	subject={emailSubject}
	body={emailBody}
	sending={emailSending}
	onSubjectChange={(v) => (emailSubject = v)}
	onBodyChange={(v) => (emailBody = v)}
	onClose={() => (showEmailModal = false)}
	onSend={handleSendEmail}
/>

<NoteModal
	open={showNoteModal}
	{newNote}
	onNoteChange={(v) => (newNote = v)}
	onClose={() => (showNoteModal = false)}
	onAdd={addNote}
/>

<TagModal
	open={showTagModal}
	{tags}
	{availableTags}
	{newTag}
	onNewTagChange={(v) => (newTag = v)}
	onToggleTag={toggleTag}
	onAddCustomTag={addCustomTag}
	onClose={() => (showTagModal = false)}
/>

<ExtendMembershipModal
	open={showExtendModal}
	subscription={selectedSubscription}
	{extendDays}
	{extending}
	onDaysChange={(v) => (extendDays = v)}
	onClose={() => (showExtendModal = false)}
	onExtend={handleExtend}
/>

<GrantMembershipModal
	open={showGrantModal}
	memberName={member?.name || ''}
	{availablePlans}
	{selectedPlanId}
	expiresAt={grantExpiresAt}
	{granting}
	onPlanChange={(v) => (selectedPlanId = v)}
	onExpiresAtChange={(v) => (grantExpiresAt = v)}
	onClose={() => (showGrantModal = false)}
	onGrant={handleGrant}
/>

<ConfirmationModal
	isOpen={showRevokeModal}
	title="Revoke Membership"
	message="Are you sure you want to revoke this membership?"
	confirmText="Revoke"
	variant="danger"
	onConfirm={confirmRevoke}
	onCancel={() => {
		showRevokeModal = false;
		pendingRevokeSubId = null;
	}}
/>

<style>
	.member-detail-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Loading Skeleton */
	.loading-container {
		padding: 2rem 0;
	}

	.skeleton {
		background: linear-gradient(
			90deg,
			rgba(148, 163, 184, 0.1) 25%,
			rgba(148, 163, 184, 0.2) 50%,
			rgba(148, 163, 184, 0.1) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	.skeleton-header {
		display: flex;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.skeleton-avatar {
		width: 80px;
		height: 80px;
		border-radius: 50%;
	}

	.skeleton-info {
		flex: 1;
	}

	.skeleton-title {
		height: 32px;
		width: 200px;
		margin-bottom: 0.75rem;
	}

	.skeleton-subtitle {
		height: 20px;
		width: 150px;
	}

	.skeleton-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.skeleton-card {
		height: 100px;
	}

	/* Error State */
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #ef4444;
		text-align: center;
	}

	.error-state h2 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.error-state p {
		color: #94a3b8;
		margin-bottom: 1.5rem;
	}

	/* Tabs */
	.tabs-container {
		margin-bottom: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
	}

	.tabs button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: none;
		border: none;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		transition: all 0.2s;
	}

	.tabs button:hover {
		color: var(--primary-400);
	}

	.tabs button.active {
		color: var(--primary-400);
		border-bottom-color: var(--primary-500);
	}

	/* Panels (Overview tab keeps these — Activity Timeline + Member Details) */
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

	/* Overview Grid */
	.overview-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
	}

	/* Timeline */
	.timeline {
		padding: 1.5rem;
	}

	.empty-timeline {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2rem;
		color: #64748b;
		text-align: center;
	}

	.timeline-item {
		display: flex;
		gap: 1rem;
		padding: 0.75rem 0;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.timeline-item:last-child {
		border-bottom: none;
	}

	.timeline-icon {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.timeline-title {
		font-size: 0.875rem;
		color: #f1f5f9;
	}

	.timeline-date {
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.25rem;
	}

	/* Details */
	.details-list {
		padding: 1rem 1.5rem;
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		padding: 0.75rem 0;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.detail-row:last-child {
		border-bottom: none;
	}

	.detail-label {
		color: #64748b;
		font-size: 0.875rem;
	}

	.detail-value {
		color: #f1f5f9;
		font-size: 0.875rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	:global(.verified-icon) {
		color: #34d399;
	}

	/* Buttons (parent-only: retry button + btn-icon for panel-header refresh/edit) */
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

	.btn-icon {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	@media (max-width: 1023.98px) {
		.overview-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 767.98px) {
		.tabs {
			overflow-x: auto;
			padding-bottom: 0.5rem;
		}

		.tabs button {
			white-space: nowrap;
		}
	}
</style>
