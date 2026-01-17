<script lang="ts">
	/**
	 * MemberDetailDrawer - Full Member Profile Drawer
	 * Revolution Trading Pros - Apple ICT 11+ Principal Engineer Grade
	 *
	 * Enterprise-grade slide-out drawer showing complete member profile
	 * with tabs for overview, subscriptions, orders, activity, and notes.
	 */
	import {
		membersApi,
		type MemberFullDetails,
		type MemberNote,
		type MemberActivity
	} from '$lib/api/members';
	import {
		IconX,
		IconUser,
		IconMail,
		IconCalendar,
		IconCreditCard,
		IconReceipt,
		IconActivity,
		IconNotes,
		IconEdit,
		IconBan,
		IconPlayerPlay,
		IconTrash,
		IconSend,
		IconChartBar,
		IconTag,
		IconClock,
		IconCheck,
		IconAlertTriangle
	} from '$lib/icons';
	import ConfirmationModal from './ConfirmationModal.svelte';

	interface Props {
		isOpen: boolean;
		memberId: number | null;
		onClose: () => void;
		onEdit?: (member: MemberFullDetails['member']) => void;
		onEmail?: (member: MemberFullDetails['member']) => void;
		onRefresh?: () => void;
	}

	let { isOpen, memberId, onClose, onEdit, onEmail, onRefresh }: Props = $props();

	// State
	let data = $state<MemberFullDetails | null>(null);
	let isLoading = $state(false);
	let error = $state('');
	let activeTab = $state<'overview' | 'subscriptions' | 'orders' | 'activity' | 'notes'>(
		'overview'
	);

	// Notes state
	let newNote = $state('');
	let isAddingNote = $state(false);

	// Action modals
	let showBanModal = $state(false);
	let showSuspendModal = $state(false);
	let showUnbanModal = $state(false);
	let banReason = $state('');
	let banDuration = $state<number | undefined>(undefined);
	let isProcessingAction = $state(false);

	// Load member data when drawer opens
	$effect(() => {
		if (isOpen && memberId) {
			loadMemberData();
		} else {
			data = null;
			activeTab = 'overview';
			error = '';
		}
	});

	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
		return () => {
			document.body.style.overflow = '';
		};
	});

	async function loadMemberData() {
		if (!memberId) return;

		isLoading = true;
		error = '';

		try {
			data = await membersApi.getMemberFull(memberId);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load member data';
		} finally {
			isLoading = false;
		}
	}

	async function addNote() {
		if (!memberId || !newNote.trim()) return;

		isAddingNote = true;
		try {
			const result = await membersApi.createMemberNote(memberId, newNote.trim());
			if (data) {
				data.notes = [result.note, ...data.notes];
			}
			newNote = '';
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to add note';
		} finally {
			isAddingNote = false;
		}
	}

	async function deleteNote(noteId: number) {
		if (!memberId) return;

		try {
			await membersApi.deleteMemberNote(memberId, noteId);
			if (data) {
				data.notes = data.notes.filter((n) => n.id !== noteId);
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete note';
		}
	}

	async function handleBan() {
		if (!memberId) return;

		isProcessingAction = true;
		try {
			await membersApi.banMember(memberId, {
				reason: banReason || undefined,
				duration_days: banDuration
			});
			showBanModal = false;
			banReason = '';
			banDuration = undefined;
			await loadMemberData();
			onRefresh?.();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to ban member';
		} finally {
			isProcessingAction = false;
		}
	}

	async function handleSuspend() {
		if (!memberId) return;

		isProcessingAction = true;
		try {
			await membersApi.suspendMember(memberId, {
				reason: banReason || undefined,
				duration_days: banDuration
			});
			showSuspendModal = false;
			banReason = '';
			banDuration = undefined;
			await loadMemberData();
			onRefresh?.();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to suspend member';
		} finally {
			isProcessingAction = false;
		}
	}

	async function handleUnban() {
		if (!memberId) return;

		isProcessingAction = true;
		try {
			await membersApi.unbanMember(memberId);
			showUnbanModal = false;
			await loadMemberData();
			onRefresh?.();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to unban member';
		} finally {
			isProcessingAction = false;
		}
	}

	function formatDate(dateStr: string | null | undefined): string {
		if (!dateStr) return 'N/A';
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatDateTime(dateStr: string | null | undefined): string {
		if (!dateStr) return 'N/A';
		return new Date(dateStr).toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatCurrency(amount: number | null | undefined): string {
		if (amount === null || amount === undefined) return '$0.00';
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(amount);
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'active':
				return '#3fb950'; // success-emphasis
			case 'trial':
				return '#ffd11a'; // primary-400
			case 'cancelled':
			case 'expired':
				return '#f85149'; // error-emphasis
			case 'banned':
				return '#f85149'; // error-emphasis
			case 'suspended':
				return '#d29922'; // warning-emphasis
			default:
				return '#6e7681'; // text-tertiary
		}
	}

	function handleBackdropClick(e: MouseEvent | KeyboardEvent) {
		if (e.target === e.currentTarget) {
			onClose();
		}
	}
</script>

{#if isOpen}
	<div class="drawer-backdrop" role="presentation" onclick={handleBackdropClick} onkeydown={(e) => { if (e.key === 'Escape') handleBackdropClick(e); }}>
		<aside class="drawer" class:open={isOpen}>
			{#if isLoading}
				<div class="loading-state">
					<div class="spinner-large"></div>
					<p>Loading member data...</p>
				</div>
			{:else if error && !data}
				<div class="error-state">
					<IconAlertTriangle size={48} />
					<p>{error}</p>
					<button type="button" class="btn-retry" onclick={loadMemberData}>Try Again</button>
				</div>
			{:else if data}
				<!-- Header -->
				<header class="drawer-header">
					<div class="member-avatar">
						{data.member.name?.[0]?.toUpperCase() || 'M'}
					</div>
					<div class="member-info">
						<h2 class="member-name">{data.member.name || 'Unknown'}</h2>
						<p class="member-email">{data.member.email}</p>
						<div class="member-badges">
							<span
								class="status-badge"
								style="--badge-color: {getStatusColor(data.member.status)}"
							>
								{data.member.status}
							</span>
							{#each data.member.tags || [] as tag}
								<span class="tag-badge">{tag}</span>
							{/each}
						</div>
					</div>
					<button type="button" class="btn-close" onclick={onClose} aria-label="Close">
						<IconX size={24} />
					</button>
				</header>

				<!-- Quick Stats -->
				<div class="quick-stats">
					<div class="stat-item">
						<span class="stat-value">{formatCurrency(data.stats.total_spent)}</span>
						<span class="stat-label">Total Spent</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{data.stats.active_subscriptions}</span>
						<span class="stat-label">Active Subs</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{data.stats.member_since_days}</span>
						<span class="stat-label">Days Member</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{data.engagement_score}%</span>
						<span class="stat-label">Engagement</span>
					</div>
				</div>

				<!-- Action Buttons -->
				<div class="action-buttons">
					<button type="button" class="btn-action" onclick={() => onEdit?.(data!.member)}>
						<IconEdit size={16} />
						Edit
					</button>
					<button type="button" class="btn-action">
						<IconSend size={16} />
						Email
					</button>
					{#if data.member.status === 'banned' || data.member.status === 'suspended'}
						<button
							type="button"
							class="btn-action success"
							onclick={() => (showUnbanModal = true)}
						>
							<IconPlayerPlay size={16} />
							Restore
						</button>
					{:else}
						<button
							type="button"
							class="btn-action warning"
							onclick={() => (showSuspendModal = true)}
						>
							<IconClock size={16} />
							Suspend
						</button>
						<button type="button" class="btn-action danger" onclick={() => (showBanModal = true)}>
							<IconBan size={16} />
							Ban
						</button>
					{/if}
				</div>

				<!-- Tabs -->
				<nav class="drawer-tabs">
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'overview'}
						onclick={() => (activeTab = 'overview')}
					>
						<IconUser size={16} />
						Overview
					</button>
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'subscriptions'}
						onclick={() => (activeTab = 'subscriptions')}
					>
						<IconCreditCard size={16} />
						Subscriptions
					</button>
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'orders'}
						onclick={() => (activeTab = 'orders')}
					>
						<IconReceipt size={16} />
						Orders
					</button>
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'activity'}
						onclick={() => (activeTab = 'activity')}
					>
						<IconActivity size={16} />
						Activity
					</button>
					<button
						type="button"
						class="tab"
						class:active={activeTab === 'notes'}
						onclick={() => (activeTab = 'notes')}
					>
						<IconNotes size={16} />
						Notes
					</button>
				</nav>

				<!-- Tab Content -->
				<div class="drawer-content">
					{#if activeTab === 'overview'}
						<div class="tab-content">
							<section class="info-section">
								<h3 class="section-title">Account Details</h3>
								<div class="info-grid">
									<div class="info-item">
										<IconMail size={16} />
										<div>
											<span class="info-label">Email</span>
											<span class="info-value">{data.member.email}</span>
										</div>
									</div>
									<div class="info-item">
										<IconCalendar size={16} />
										<div>
											<span class="info-label">Joined</span>
											<span class="info-value">{formatDate(data.member.joined_at)}</span>
										</div>
									</div>
									<div class="info-item">
										<IconCheck size={16} />
										<div>
											<span class="info-label">Email Verified</span>
											<span class="info-value">{data.member.email_verified ? 'Yes' : 'No'}</span>
										</div>
									</div>
									<div class="info-item">
										<IconClock size={16} />
										<div>
											<span class="info-label">Last Login</span>
											<span class="info-value">{formatDateTime(data.member.last_login)}</span>
										</div>
									</div>
								</div>
							</section>

							{#if data.member.status === 'banned' || data.member.status === 'suspended'}
								<section class="info-section warning-section">
									<h3 class="section-title">
										<IconAlertTriangle size={16} />
										{data.member.status === 'banned' ? 'Ban Details' : 'Suspension Details'}
									</h3>
									<div class="info-grid">
										{#if data.member.ban_reason}
											<div class="info-item full-width">
												<span class="info-label">Reason</span>
												<span class="info-value">{data.member.ban_reason}</span>
											</div>
										{/if}
										{#if data.member.banned_until}
											<div class="info-item">
												<span class="info-label">Expires</span>
												<span class="info-value">{formatDateTime(data.member.banned_until)}</span>
											</div>
										{:else}
											<div class="info-item">
												<span class="info-label">Duration</span>
												<span class="info-value">Permanent</span>
											</div>
										{/if}
									</div>
								</section>
							{/if}

							<section class="info-section">
								<h3 class="section-title">Timeline</h3>
								<div class="timeline">
									{#each data.timeline.slice(0, 5) as event}
										<div class="timeline-item">
											<div
												class="timeline-dot"
												style="--dot-color: {getStatusColor(event.type)}"
											></div>
											<div class="timeline-content">
												<span class="timeline-title">{event.title}</span>
												<span class="timeline-date">{formatDateTime(event.date)}</span>
											</div>
										</div>
									{/each}
								</div>
							</section>
						</div>
					{:else if activeTab === 'subscriptions'}
						<div class="tab-content">
							{#if data.subscriptions.length === 0}
								<div class="empty-state">
									<IconCreditCard size={48} />
									<p>No subscriptions found</p>
								</div>
							{:else}
								<div class="cards-list">
									{#each data.subscriptions as sub}
										<div class="card">
											<div class="card-header">
												<span class="card-title">{sub.product_name || 'Subscription'}</span>
												<span
													class="status-badge small"
													style="--badge-color: {getStatusColor(sub.status)}"
												>
													{sub.status}
												</span>
											</div>
											<div class="card-details">
												<div class="detail-row">
													<span>Price</span>
													<span>{formatCurrency(sub.price)}/{sub.billing_period || 'month'}</span>
												</div>
												<div class="detail-row">
													<span>Started</span>
													<span>{formatDate(sub.started_at)}</span>
												</div>
												{#if sub.expires_at}
													<div class="detail-row">
														<span>Expires</span>
														<span>{formatDate(sub.expires_at)}</span>
													</div>
												{/if}
												{#if sub.cancelled_at}
													<div class="detail-row">
														<span>Cancelled</span>
														<span>{formatDate(sub.cancelled_at)}</span>
													</div>
												{/if}
											</div>
										</div>
									{/each}
								</div>
							{/if}
						</div>
					{:else if activeTab === 'orders'}
						<div class="tab-content">
							{#if data.orders.length === 0}
								<div class="empty-state">
									<IconReceipt size={48} />
									<p>No orders found</p>
								</div>
							{:else}
								<div class="cards-list">
									{#each data.orders as order}
										<div class="card">
											<div class="card-header">
												<span class="card-title">#{order.order_number || order.id}</span>
												<span
													class="status-badge small"
													style="--badge-color: {getStatusColor(order.status || 'completed')}"
												>
													{order.status || 'completed'}
												</span>
											</div>
											<div class="card-details">
												<div class="detail-row">
													<span>Total</span>
													<span class="total">{formatCurrency(order.total)}</span>
												</div>
												<div class="detail-row">
													<span>Date</span>
													<span>{formatDate(order.created_at)}</span>
												</div>
											</div>
										</div>
									{/each}
								</div>
							{/if}
						</div>
					{:else if activeTab === 'activity'}
						<div class="tab-content">
							{#if data.activity.length === 0}
								<div class="empty-state">
									<IconActivity size={48} />
									<p>No activity recorded</p>
								</div>
							{:else}
								<div class="activity-list">
									{#each data.activity as act}
										<div class="activity-item">
											<div class="activity-icon">
												<IconActivity size={14} />
											</div>
											<div class="activity-content">
												<span class="activity-action">{act.action}</span>
												{#if act.description}
													<span class="activity-desc">{act.description}</span>
												{/if}
												<span class="activity-meta">
													{formatDateTime(act.created_at)}
													{#if act.ip_address}
														<span class="ip"> - {act.ip_address}</span>
													{/if}
												</span>
											</div>
										</div>
									{/each}
								</div>
							{/if}
						</div>
					{:else if activeTab === 'notes'}
						<div class="tab-content">
							<!-- Add Note Form -->
							<div class="add-note-form">
								<textarea
									class="note-input"
									placeholder="Add a note about this member..."
									bind:value={newNote}
									rows="3"
									disabled={isAddingNote}
								></textarea>
								<button
									type="button"
									class="btn-add-note"
									onclick={addNote}
									disabled={isAddingNote || !newNote.trim()}
								>
									{isAddingNote ? 'Adding...' : 'Add Note'}
								</button>
							</div>

							{#if data.notes.length === 0}
								<div class="empty-state">
									<IconNotes size={48} />
									<p>No notes yet</p>
								</div>
							{:else}
								<div class="notes-list">
									{#each data.notes as note}
										<div class="note-item">
											<div class="note-content">{note.content}</div>
											<div class="note-meta">
												<span>{note.created_by_name || 'Admin'}</span>
												<span>{formatDateTime(note.created_at)}</span>
												<button
													type="button"
													class="btn-delete-note"
													onclick={() => deleteNote(note.id)}
													title="Delete note"
												>
													<IconTrash size={14} />
												</button>
											</div>
										</div>
									{/each}
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</aside>
	</div>
{/if}

<!-- Ban Modal -->
<ConfirmationModal
	isOpen={showBanModal}
	title="Ban Member"
	message="This will prevent the member from accessing their account. Are you sure you want to proceed?"
	confirmText="Ban Member"
	variant="danger"
	isLoading={isProcessingAction}
	showInput={true}
	inputLabel="Reason (optional)"
	inputPlaceholder="Enter ban reason..."
	bind:inputValue={banReason}
	onConfirm={handleBan}
	onCancel={() => {
		showBanModal = false;
		banReason = '';
	}}
/>

<!-- Suspend Modal -->
<ConfirmationModal
	isOpen={showSuspendModal}
	title="Suspend Member"
	message="This will temporarily restrict the member's access. You can restore their access at any time."
	confirmText="Suspend Member"
	variant="warning"
	isLoading={isProcessingAction}
	showInput={true}
	inputLabel="Reason (optional)"
	inputPlaceholder="Enter suspension reason..."
	bind:inputValue={banReason}
	onConfirm={handleSuspend}
	onCancel={() => {
		showSuspendModal = false;
		banReason = '';
	}}
/>

<!-- Unban Modal -->
<ConfirmationModal
	isOpen={showUnbanModal}
	title="Restore Member Access"
	message="This will restore the member's full access to their account."
	confirmText="Restore Access"
	variant="success"
	isLoading={isProcessingAction}
	onConfirm={handleUnban}
	onCancel={() => (showUnbanModal = false)}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * MemberDetailDrawer - Admin Design System Aligned Styles
	 * Uses RTP Admin color tokens from tokens/colors.css
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.drawer-backdrop {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		z-index: 1000;
		animation: fadeIn 0.2s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		width: 520px;
		max-width: 100%;
		height: 100vh;
		background: var(--bg-elevated, #161b22);
		border-left: 1px solid var(--border-default, #30363d);
		display: flex;
		flex-direction: column;
		transform: translateX(100%);
		transition: transform 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		z-index: 1001;
		box-shadow: -8px 0 32px rgba(0, 0, 0, 0.5);
	}

	.drawer.open {
		transform: translateX(0);
	}

	.loading-state,
	.error-state {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		color: var(--text-tertiary, #6e7681);
	}

	.spinner-large {
		width: 48px;
		height: 48px;
		border: 3px solid var(--border-muted, #21262d);
		border-top-color: var(--primary-500, #e6b800);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.btn-retry {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	.btn-retry:hover {
		background: var(--primary-400, #ffd11a);
	}

	/* Header */
	.drawer-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.5rem;
		background: var(--bg-surface, #1c2128);
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.member-avatar {
		width: 56px;
		height: 56px;
		border-radius: 50%;
		background: linear-gradient(135deg, var(--primary-500, #e6b800), var(--primary-400, #ffd11a));
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--bg-base, #0d1117);
		flex-shrink: 0;
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.member-info {
		flex: 1;
		min-width: 0;
	}

	.member-name {
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem;
	}

	.member-email {
		font-size: 0.875rem;
		color: var(--text-secondary, #8b949e);
		margin: 0 0 0.5rem;
	}

	.member-badges {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.25rem 0.625rem;
		background: color-mix(in srgb, var(--badge-color) 20%, transparent);
		color: var(--badge-color);
		border: 1px solid color-mix(in srgb, var(--badge-color) 40%, transparent);
		border-radius: 999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.small {
		padding: 0.125rem 0.5rem;
		font-size: 0.6875rem;
	}

	.tag-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.125rem 0.5rem;
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border-radius: 4px;
		font-size: 0.6875rem;
	}

	.btn-close {
		background: var(--bg-hover, #252b33);
		border: 1px solid var(--border-muted, #21262d);
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 0.5rem;
		transition: all 0.2s ease;
	}

	.btn-close:hover {
		background: var(--bg-active, #2d333b);
		color: var(--text-primary, #f0f6fc);
		border-color: var(--border-default, #30363d);
	}

	/* Quick Stats */
	.quick-stats {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
		padding: 1.25rem 1.5rem;
		background: var(--bg-base, #0d1117);
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.stat-item {
		text-align: center;
		padding: 0.75rem 0.5rem;
		background: var(--bg-surface, #1c2128);
		border-radius: 0.5rem;
		border: 1px solid var(--border-muted, #21262d);
	}

	.stat-value {
		display: block;
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--primary-400, #ffd11a);
	}

	.stat-label {
		font-size: 0.625rem;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 0.25rem;
	}

	/* Action Buttons */
	.action-buttons {
		display: flex;
		gap: 0.5rem;
		padding: 1rem 1.5rem;
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.btn-action {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.625rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		color: var(--text-secondary, #8b949e);
		border-radius: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-action:hover {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-emphasis, #8b949e);
		color: var(--text-primary, #f0f6fc);
	}

	.btn-action.danger {
		color: var(--error-emphasis, #f85149);
	}

	.btn-action.danger:hover {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
		border-color: var(--error-base, #da3633);
	}

	.btn-action.warning {
		color: var(--warning-emphasis, #d29922);
	}

	.btn-action.warning:hover {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		border-color: var(--warning-base, #bb8009);
	}

	.btn-action.success {
		color: var(--success-emphasis, #3fb950);
	}

	.btn-action.success:hover {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		border-color: var(--success-base, #2ea043);
	}

	/* Tabs */
	.drawer-tabs {
		display: flex;
		padding: 0 1rem;
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
		overflow-x: auto;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.875rem 1rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.2s ease;
	}

	.tab:hover {
		color: var(--text-secondary, #8b949e);
		background: var(--bg-hover, #252b33);
	}

	.tab.active {
		color: var(--primary-400, #ffd11a);
		border-bottom-color: var(--primary-500, #e6b800);
	}

	/* Content */
	.drawer-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
		background: var(--bg-elevated, #161b22);
	}

	.tab-content {
		animation: fadeIn 0.2s ease;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
	}

	.empty-state p {
		margin-top: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}

	/* Info Sections */
	.info-section {
		margin-bottom: 1.5rem;
	}

	.info-section.warning-section {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		border: 1px solid var(--warning-base, #bb8009);
		border-radius: 0.5rem;
		padding: 1rem;
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
		text-transform: uppercase;
		letter-spacing: 0.08em;
		margin: 0 0 1rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.info-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.info-item {
		display: flex;
		gap: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.info-item.full-width {
		grid-column: 1 / -1;
	}

	.info-item div {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.info-label {
		font-size: 0.6875rem;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.info-value {
		font-size: 0.875rem;
		color: var(--text-primary, #f0f6fc);
		font-weight: 500;
	}

	/* Timeline */
	.timeline {
		position: relative;
		padding-left: 1.5rem;
	}

	.timeline::before {
		content: '';
		position: absolute;
		left: 5px;
		top: 0;
		bottom: 0;
		width: 2px;
		background: var(--border-muted, #21262d);
	}

	.timeline-item {
		position: relative;
		padding-bottom: 1rem;
	}

	.timeline-dot {
		position: absolute;
		left: -1.5rem;
		top: 4px;
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: var(--dot-color);
		border: 2px solid var(--bg-elevated, #161b22);
	}

	.timeline-content {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.timeline-title {
		font-size: 0.875rem;
		color: var(--text-primary, #f0f6fc);
	}

	.timeline-date {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	/* Cards List */
	.cards-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.card {
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.5rem;
		padding: 1rem;
		transition: border-color 0.2s ease;
	}

	.card:hover {
		border-color: var(--border-emphasis, #8b949e);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.75rem;
	}

	.card-title {
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.card-details {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.detail-row {
		display: flex;
		justify-content: space-between;
		font-size: 0.8125rem;
	}

	.detail-row span:first-child {
		color: var(--text-tertiary, #6e7681);
	}

	.detail-row span:last-child {
		color: var(--text-secondary, #8b949e);
	}

	.detail-row .total {
		color: var(--primary-400, #ffd11a);
		font-weight: 600;
	}

	/* Activity List */
	.activity-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.activity-item {
		display: flex;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 0.5rem;
	}

	.activity-icon {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		background: var(--bg-hover, #252b33);
		color: var(--text-tertiary, #6e7681);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.activity-content {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
		min-width: 0;
	}

	.activity-action {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
	}

	.activity-desc {
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
	}

	.activity-meta {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.activity-meta .ip {
		opacity: 0.7;
	}

	/* Notes */
	.add-note-form {
		margin-bottom: 1.5rem;
	}

	.note-input {
		width: 100%;
		padding: 0.75rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.5rem;
		color: var(--text-primary, #f0f6fc);
		font-family: var(--font-body, 'Roboto', sans-serif);
		font-size: 0.875rem;
		resize: vertical;
		margin-bottom: 0.75rem;
	}

	.note-input::placeholder {
		color: var(--text-muted, #484f58);
	}

	.note-input:focus {
		outline: none;
		border-color: var(--primary-500, #e6b800);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.btn-add-note {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		border: none;
		padding: 0.625rem 1.25rem;
		border-radius: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-add-note:hover:not(:disabled) {
		background: var(--primary-400, #ffd11a);
	}

	.btn-add-note:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.notes-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.note-item {
		padding: 1rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 0.5rem;
	}

	.note-content {
		font-size: 0.875rem;
		color: var(--text-primary, #f0f6fc);
		line-height: 1.6;
		margin-bottom: 0.75rem;
		white-space: pre-wrap;
	}

	.note-meta {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
		padding-top: 0.5rem;
		border-top: 1px solid var(--border-muted, #21262d);
	}

	.btn-delete-note {
		margin-left: auto;
		background: transparent;
		border: none;
		color: var(--text-tertiary, #6e7681);
		cursor: pointer;
		padding: 0.25rem;
		border-radius: 0.25rem;
		transition: all 0.2s ease;
	}

	.btn-delete-note:hover {
		color: var(--error-emphasis, #f85149);
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
	}

	@media (max-width: 640px) {
		.drawer {
			width: 100%;
		}

		.quick-stats {
			grid-template-columns: repeat(2, 1fr);
		}

		.info-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
