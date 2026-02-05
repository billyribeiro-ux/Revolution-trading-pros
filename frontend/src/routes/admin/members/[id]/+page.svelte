<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { membersApi } from '$lib/api/members';
	import type { Member, Subscription, Order } from '$lib/api/members';
	import { toastStore } from '$lib/stores/toast.svelte';
	import {
		IconArrowLeft,
		IconMail,
		IconCalendar,
		IconCreditCard,
		IconReceipt,
		IconActivity,
		IconUser,
		IconEdit,
		IconSend,
		IconX,
		IconCheck,
		IconClock,
		IconAlertTriangle,
		IconTrendingUp,
		IconGift,
		IconPlus,
		IconTrash,
		IconChartBar,
		IconRefresh,
		IconDownload,
		IconExternalLink,
		IconFileText
	} from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	let memberId = $derived(Number(page.params.id));

	// State
	let member = $state<Member | null>(null);
	let timeline = $state<
		Array<{
			type: string;
			title: string;
			date: string;
			icon: string;
			meta?: Record<string, unknown>;
		}>
	>([]);
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
	let notes = $state<Array<{ id: number; content: string; created_at: string; author: string }>>(
		[]
	);
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
	let availablePlans = $state<Array<{ id: number; name: string; slug: string }>>([]);
	let selectedPlanId = $state<number | null>(null);
	let grantExpiresAt = $state('');

	// Revoke confirmation modal state
	let showRevokeModal = $state(false);
	let pendingRevokeSubId = $state<number | null>(null);

	// Email history (mock)
	let emailHistory = $state<
		Array<{
			id: number;
			subject: string;
			sent_at: string;
			status: 'sent' | 'opened' | 'clicked' | 'bounced';
			campaign_type: string;
		}>
	>([]);

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
			console.error(err);
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
			console.error('Failed to load member notes:', err);
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
			console.error('Failed to load email history:', err);
			emailHistory = []; // Empty array, not mock data
		}
	}

	async function handleSendEmail() {
		if (!emailSubject || !emailBody) return;
		emailSending = true;
		const sentSubject = emailSubject; // Capture before clearing
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
			// Save note to API
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
			} else {
				// If API fails, still add locally but warn user
				notes = [
					{
						id: Date.now(),
						content: newNote,
						created_at: new Date().toISOString(),
						author: 'Admin'
					},
					...notes
				];
				newNote = '';
				showNoteModal = false;
				toastStore.warning('Note added locally - sync pending');
			}
		} catch (err) {
			console.error('Failed to save note:', err);
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

	function addCustomTag() {
		if (newTag.trim() && !tags.includes(newTag.trim())) {
			tags = [...tags, newTag.trim()];
			newTag = '';
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'active':
				return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30';
			case 'trial':
				return 'bg-blue-500/20 text-blue-400 border-blue-500/30';
			case 'churned':
				return 'bg-red-500/20 text-red-400 border-red-500/30';
			default:
				return 'bg-slate-500/20 text-slate-400 border-slate-500/30';
		}
	}

	function getEmailStatusColor(status: string): string {
		switch (status) {
			case 'opened':
				return 'text-emerald-400';
			case 'clicked':
				return 'text-blue-400';
			case 'bounced':
				return 'text-red-400';
			default:
				return 'text-slate-400';
		}
	}

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatDateTime(dateString: string): string {
		return new Date(dateString).toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getTimelineIcon(type: string) {
		switch (type) {
			case 'subscription':
				return IconCreditCard;
			case 'payment':
				return IconReceipt;
			case 'email':
				return IconMail;
			case 'login':
				return IconUser;
			case 'support':
				return IconMail;
			default:
				return IconActivity;
		}
	}

	function getMemberInitials(): string {
		if (member?.first_name && member?.last_name) {
			return `${member.first_name[0]}${member.last_name[0]}`.toUpperCase();
		}
		return member?.name?.slice(0, 2).toUpperCase() || 'U';
	}

	function getEngagementLabel(score: number): { label: string; color: string } {
		if (score >= 80) return { label: 'Highly Engaged', color: 'text-emerald-400' };
		if (score >= 60) return { label: 'Engaged', color: 'text-blue-400' };
		if (score >= 40) return { label: 'Moderate', color: 'text-yellow-400' };
		return { label: 'Low Engagement', color: 'text-red-400' };
	}

	// Load available membership plans for granting
	async function loadPlans() {
		try {
			const response = await fetch('/api/admin/membership-plans');
			if (response.ok) {
				availablePlans = await response.json();
			}
		} catch (err) {
			console.error('Failed to load plans:', err);
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
			console.error('Failed to extend membership:', err);
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
			console.error('Failed to grant membership:', err);
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
			console.error('Failed to revoke membership:', err);
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
		<!-- Header -->
		<div class="page-header">
			<button class="back-btn" onclick={() => goto('/admin/members')}>
				<IconArrowLeft size={20} />
				Back to Members
			</button>

			<div class="header-content">
				<div class="member-profile">
					<div class="member-avatar large">
						{#if member.avatar}
							<img src={member.avatar} alt={member.name} />
						{:else}
							{getMemberInitials()}
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
							{#each tags as tag}
								<span class="tag">
									{tag}
									<button class="tag-remove" onclick={() => removeTag(tag)}>
										<IconX size={12} />
									</button>
								</span>
							{/each}
							<button class="tag-add" onclick={() => (showTagModal = true)}>
								<IconPlus size={14} />
								Add Tag
							</button>
						</div>
					</div>
				</div>

				<div class="header-actions">
					<button class="btn-secondary" onclick={() => (showNoteModal = true)}>
						<IconFileText size={18} />
						Add Note
					</button>
					<button class="btn-primary" onclick={() => (showEmailModal = true)}>
						<IconMail size={18} />
						Send Email
					</button>
				</div>
			</div>
		</div>

		<!-- Quick Stats -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon purple">
					<IconCreditCard size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{formatCurrency(member.total_spent)}</div>
					<div class="stat-label">Lifetime Value</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon emerald">
					<IconReceipt size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{member.active_subscriptions_count}</div>
					<div class="stat-label">Active Subscriptions</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconCalendar size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{formatDate(member.joined_at)}</div>
					<div class="stat-label">Member Since</div>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon {getEngagementLabel(engagementScore).color.replace('text-', '')}">
					<IconChartBar size={24} />
				</div>
				<div class="stat-content">
					<div class="stat-value">{engagementScore}%</div>
					<div class="stat-label {getEngagementLabel(engagementScore).color}">
						{getEngagementLabel(engagementScore).label}
					</div>
				</div>
			</div>
		</div>

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
								{#each timeline as event}
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
				<div class="panel">
					<div class="panel-header">
						<h3>Subscription History</h3>
						<button class="btn-primary small" onclick={openGrantModal}>
							<IconGift size={16} />
							Grant Membership
						</button>
					</div>
					{#if !member.subscriptions || member.subscriptions.length === 0}
						<div class="empty-state">
							<IconCreditCard size={48} stroke={1} />
							<h4>No Subscriptions</h4>
							<p>This member has no subscription history</p>
						</div>
					{:else}
						<div class="subscriptions-list">
							{#each member.subscriptions as sub}
								<div class="subscription-card">
									<div class="subscription-header">
										<div class="subscription-product">
											<h4>{sub.product || 'Unknown Product'}</h4>
											<span class="subscription-plan">{sub.plan || 'Standard'}</span>
										</div>
										<span
											class="status-badge {sub.status === 'active'
												? 'bg-emerald-500/20 text-emerald-400'
												: 'bg-slate-500/20 text-slate-400'}"
										>
											{sub.status}
										</span>
									</div>
									<div class="subscription-details">
										<div class="subscription-detail">
											<span class="label">Price</span>
											<span class="value">{formatCurrency(sub.price)}/{sub.interval}</span>
										</div>
										<div class="subscription-detail">
											<span class="label">Started</span>
											<span class="value"
												>{sub.start_date ? formatDate(sub.start_date) : 'N/A'}</span
											>
										</div>
										<div class="subscription-detail">
											<span class="label">Next Payment</span>
											<span class="value"
												>{sub.next_payment ? formatDate(sub.next_payment) : 'N/A'}</span
											>
										</div>
										<div class="subscription-detail">
											<span class="label">Total Paid</span>
											<span class="value">{formatCurrency(sub.total_paid)}</span>
										</div>
									</div>
									<div class="subscription-actions">
										<button class="btn-secondary small" onclick={() => openExtendModal(sub)}>
											<IconCalendar size={14} />
											Extend
										</button>
										<button class="btn-danger-outline small" onclick={() => handleRevoke(sub.id)}>
											<IconTrash size={14} />
											Revoke
										</button>
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{:else if activeTab === 'orders'}
				<div class="panel">
					<div class="panel-header">
						<h3>Order History</h3>
						<button class="btn-secondary small">
							<IconDownload size={16} />
							Export
						</button>
					</div>
					{#if !member.orders || member.orders.length === 0}
						<div class="empty-state">
							<IconReceipt size={48} stroke={1} />
							<h4>No Orders</h4>
							<p>This member has no order history</p>
						</div>
					{:else}
						<table class="orders-table">
							<thead>
								<tr>
									<th>Order #</th>
									<th>Date</th>
									<th>Status</th>
									<th>Total</th>
									<th>Actions</th>
								</tr>
							</thead>
							<tbody>
								{#each member.orders as order}
									<tr>
										<td class="order-number">{order.number}</td>
										<td>{formatDate(order.created_at)}</td>
										<td>
											<span
												class="status-badge {order.status === 'completed'
													? 'bg-emerald-500/20 text-emerald-400'
													: 'bg-yellow-500/20 text-yellow-400'}"
											>
												{order.status}
											</span>
										</td>
										<td class="order-total">{formatCurrency(order.total)}</td>
										<td>
											<button class="btn-icon small">
												<IconExternalLink size={16} />
											</button>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					{/if}
				</div>
			{:else if activeTab === 'emails'}
				<div class="panel">
					<div class="panel-header">
						<h3>Email History</h3>
						<button class="btn-primary small" onclick={() => (showEmailModal = true)}>
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
							{#each emailHistory as email}
								<div class="email-item">
									<div class="email-icon {getEmailStatusColor(email.status)}">
										<IconMail size={20} />
									</div>
									<div class="email-content">
										<div class="email-subject">{email.subject}</div>
										<div class="email-meta">
											<span class="email-campaign">{email.campaign_type}</span>
											<span class="email-date">{formatDateTime(email.sent_at)}</span>
										</div>
									</div>
									<div class="email-status {getEmailStatusColor(email.status)}">
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
			{:else if activeTab === 'notes'}
				<div class="panel">
					<div class="panel-header">
						<h3>Internal Notes</h3>
						<button class="btn-primary small" onclick={() => (showNoteModal = true)}>
							<IconPlus size={16} />
							Add Note
						</button>
					</div>
					{#if notes.length === 0}
						<div class="empty-state">
							<IconFileText size={48} stroke={1} />
							<h4>No Notes</h4>
							<p>Add internal notes about this member</p>
						</div>
					{:else}
						<div class="notes-list">
							{#each notes as note}
								<div class="note-item">
									<div class="note-content">{note.content}</div>
									<div class="note-meta">
										<span class="note-author">{note.author}</span>
										<span class="note-date">{formatDateTime(note.created_at)}</span>
									</div>
									<button class="note-delete">
										<IconTrash size={14} />
									</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>

<!-- Email Modal -->
{#if showEmailModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-overlay"
		onclick={() => (showEmailModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showEmailModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h2>Send Email to {member?.name}</h2>
				<button class="close-btn" onclick={() => (showEmailModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="form-group">
					<label for="email-subject-detail">Subject</label>
					<input
						id="email-subject-detail" name="email-subject-detail"
						type="text"
						bind:value={emailSubject}
						placeholder="Email subject..."
					/>
				</div>
				<div class="form-group">
					<label for="email-body-detail">Body</label>
					<textarea
						id="email-body-detail"
						bind:value={emailBody}
						rows="10"
						placeholder="Email body..."
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showEmailModal = false)}>Cancel</button>
				<button
					class="btn-primary"
					onclick={handleSendEmail}
					disabled={!emailSubject || !emailBody || emailSending}
				>
					<IconSend size={18} />
					{emailSending ? 'Sending...' : 'Send Email'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Note Modal -->
{#if showNoteModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-overlay"
		onclick={() => (showNoteModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showNoteModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h2>Add Note</h2>
				<button class="close-btn" onclick={() => (showNoteModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="form-group">
					<label for="note-content">Note</label>
					<textarea
						id="note-content"
						bind:value={newNote}
						rows="5"
						placeholder="Add internal note about this member..."
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showNoteModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={addNote} disabled={!newNote.trim()}>
					<IconPlus size={18} />
					Add Note
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Tag Modal -->
{#if showTagModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-overlay"
		onclick={() => (showTagModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showTagModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h2>Manage Tags</h2>
				<button class="close-btn" onclick={() => (showTagModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="available-tags">
					<span class="tags-label">Available Tags</span>
					<div class="tags-grid">
						{#each availableTags as tag}
							<button
								class="tag-option"
								class:selected={tags.includes(tag)}
								onclick={() => (tags.includes(tag) ? removeTag(tag) : addTag(tag))}
							>
								{#if tags.includes(tag)}
									<IconCheck size={14} />
								{:else}
									<IconPlus size={14} />
								{/if}
								{tag}
							</button>
						{/each}
					</div>
				</div>
				<div class="custom-tag">
					<span class="tags-label">Custom Tag</span>
					<div class="custom-tag-input">
						<input
							id="page-newtag" name="page-newtag" type="text"
							bind:value={newTag}
							placeholder="Enter custom tag..."
							onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && addCustomTag()}
						/>
						<button class="btn-primary small" onclick={addCustomTag} disabled={!newTag.trim()}>
							Add
						</button>
					</div>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-primary" onclick={() => (showTagModal = false)}>Done</button>
			</div>
		</div>
	</div>
{/if}

<!-- Extend Membership Modal -->
{#if showExtendModal && selectedSubscription}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-overlay"
		onclick={() => (showExtendModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showExtendModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h2>Extend Membership</h2>
				<button class="close-btn" onclick={() => (showExtendModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="extend-info">
					Extending: <strong>{selectedSubscription.product || 'Membership'}</strong>
				</p>
				<div class="form-group">
					<label for="extend-days">Extend by (days)</label>
					<div class="extend-options">
						{#each [7, 14, 30, 60, 90, 365] as days}
							<button
								type="button"
								class="extend-option"
								class:selected={extendDays === days}
								onclick={() => (extendDays = days)}
							>
								{days} days
							</button>
						{/each}
					</div>
					<input
						id="extend-days" name="extend-days"
						type="number"
						bind:value={extendDays}
						min="1"
						max="3650"
						class="extend-custom"
						placeholder="Custom days..."
					/>
				</div>
				<p class="extend-preview">
					New expiration: <strong
						>{selectedSubscription.next_payment
							? formatDate(
									new Date(
										new Date(selectedSubscription.next_payment).getTime() +
											extendDays * 24 * 60 * 60 * 1000
									).toISOString()
								)
							: formatDate(
									new Date(Date.now() + extendDays * 24 * 60 * 60 * 1000).toISOString()
								)}</strong
					>
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showExtendModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleExtend} disabled={extending || extendDays < 1}>
					<IconCalendar size={18} />
					{extending ? 'Extending...' : `Extend by ${extendDays} Days`}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Grant Membership Modal -->
{#if showGrantModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="modal-overlay"
		onclick={() => (showGrantModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showGrantModal = false)}
		role="dialog"
		tabindex="-1"
		aria-modal="true"
	>
		<div
			class="modal-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header">
				<h2>Grant Membership</h2>
				<button class="close-btn" onclick={() => (showGrantModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="grant-info">
					Granting membership to: <strong>{member?.name}</strong>
				</p>
				<div class="form-group">
					<label for="grant-plan">Select Plan</label>
					<select id="grant-plan" bind:value={selectedPlanId}>
						<option value={null}>Select a plan...</option>
						{#each availablePlans as plan}
							<option value={plan.id}>{plan.name}</option>
						{/each}
					</select>
				</div>
				<div class="form-group">
					<label for="grant-expires">Expiration Date (optional)</label>
					<input
						id="grant-expires" name="grant-expires"
						type="date"
						bind:value={grantExpiresAt}
						min={new Date().toISOString().split('T')[0]}
					/>
					<small class="form-hint">Leave empty for no expiration (lifetime)</small>
				</div>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showGrantModal = false)}>Cancel</button>
				<button class="btn-primary" onclick={handleGrant} disabled={granting || !selectedPlanId}>
					<IconGift size={18} />
					{granting ? 'Granting...' : 'Grant Membership'}
				</button>
			</div>
		</div>
	</div>
{/if}

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

	/* Header */
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

	/* Status Badge */
	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		border: 1px solid;
		text-transform: capitalize;
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}
	.stat-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}
	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.emerald-400 {
		background: rgba(52, 211, 153, 0.15);
		color: #34d399;
	}
	.stat-icon.yellow-400 {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.stat-icon.red-400 {
		background: rgba(248, 113, 113, 0.15);
		color: #f87171;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
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

	/* Panels */
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

	/* Empty State */
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

	/* Subscriptions */
	.subscriptions-list {
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.subscription-card {
		background: rgba(15, 23, 42, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 12px;
		padding: 1.25rem;
	}

	.subscription-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1rem;
	}

	.subscription-product h4 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.subscription-plan {
		font-size: 0.75rem;
		color: #64748b;
	}

	.subscription-details {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
	}

	.subscription-detail .label {
		display: block;
		font-size: 0.6875rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.25rem;
	}

	.subscription-detail .value {
		font-size: 0.875rem;
		color: #f1f5f9;
		font-weight: 500;
	}

	/* Orders Table */
	.orders-table {
		width: 100%;
		border-collapse: collapse;
	}

	.orders-table thead {
		background: rgba(15, 23, 42, 0.6);
	}

	.orders-table th {
		padding: 1rem 1.5rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.orders-table tbody tr {
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.orders-table td {
		padding: 1rem 1.5rem;
		color: #cbd5e1;
	}

	.order-number {
		font-weight: 600;
		color: var(--primary-400);
	}

	.order-total {
		font-weight: 600;
		color: #34d399;
	}

	/* Email List */
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

	/* Notes */
	.notes-list {
		padding: 1rem;
	}

	.note-item {
		position: relative;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 10px;
		margin-bottom: 0.75rem;
	}

	.note-content {
		font-size: 0.875rem;
		color: #cbd5e1;
		line-height: 1.5;
		margin-bottom: 0.5rem;
	}

	.note-meta {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.note-delete {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 0.25rem;
		opacity: 0;
		transition: all 0.2s;
	}

	.note-item:hover .note-delete {
		opacity: 1;
	}

	.note-delete:hover {
		color: #ef4444;
	}

	/* Buttons */
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

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
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

	.btn-primary.small,
	.btn-secondary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
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

	.btn-icon.small {
		width: 28px;
		height: 28px;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 20px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: inherit;
		resize: vertical;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	/* Tag Modal */
	.available-tags {
		margin-bottom: 1.5rem;
	}

	.tags-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.tags-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.tag-option {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tag-option:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.tag-option.selected {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
		color: var(--primary-400);
	}

	.custom-tag-input {
		display: flex;
		gap: 0.5rem;
	}

	.custom-tag-input input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.overview-grid {
			grid-template-columns: 1fr;
		}

		.subscription-details {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
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

		.tabs {
			overflow-x: auto;
			padding-bottom: 0.5rem;
		}

		.tabs button {
			white-space: nowrap;
		}
	}

	/* Subscription Actions */
	.subscription-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.btn-danger-outline {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.4);
		border-radius: 8px;
		color: #f87171;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger-outline:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.6);
	}

	/* Extend Modal Styles */
	.extend-info,
	.grant-info {
		color: #94a3b8;
		font-size: 0.9375rem;
		margin-bottom: 1.25rem;
	}

	.extend-info strong,
	.grant-info strong {
		color: #f1f5f9;
	}

	.extend-options {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.extend-option {
		padding: 0.5rem 0.875rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.extend-option:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.extend-option.selected {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.5);
		color: var(--primary-400);
	}

	.extend-custom {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.extend-preview {
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.extend-preview strong {
		color: #34d399;
	}

	/* Grant Modal Styles */
	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: inherit;
		cursor: pointer;
	}

	.form-group select:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
	}

	.form-hint {
		display: block;
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #64748b;
	}
</style>

<ConfirmationModal
	isOpen={showRevokeModal}
	title="Revoke Membership"
	message="Are you sure you want to revoke this membership?"
	confirmText="Revoke"
	variant="danger"
	onConfirm={confirmRevoke}
	onCancel={() => { showRevokeModal = false; pendingRevokeSubId = null; }}
/>
