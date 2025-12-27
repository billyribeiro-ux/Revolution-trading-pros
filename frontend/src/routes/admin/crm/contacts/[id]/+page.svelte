<script lang="ts">
	/**
	 * Contact Detail Page - FluentCRM Pro Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Full contact profile with:
	 * - Profile information and scoring
	 * - Lists, Tags, and Segments
	 * - Activity Timeline
	 * - Email History
	 * - Notes section
	 * - Quick actions
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	// Svelte 5 individual icon imports (Dec 2025 pattern)
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconPhone from '@tabler/icons-svelte/icons/phone';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconUserCircle from '@tabler/icons-svelte/icons/user-circle';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconTag from '@tabler/icons-svelte/icons/tag';
	import IconList from '@tabler/icons-svelte/icons/list';
	import IconSend from '@tabler/icons-svelte/icons/send';
	import IconNotes from '@tabler/icons-svelte/icons/notes';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconCurrencyDollar from '@tabler/icons-svelte/icons/currency-dollar';
	import IconMailFast from '@tabler/icons-svelte/icons/mail-fast';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconMailOpen from '@tabler/icons-svelte/icons/mail-opened';
	import IconClick from '@tabler/icons-svelte/icons/click';
	import IconWorld from '@tabler/icons-svelte/icons/world';
	import IconBuilding from '@tabler/icons-svelte/icons/building';
	import IconBriefcase from '@tabler/icons-svelte/icons/briefcase';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconDotsVertical from '@tabler/icons-svelte/icons/dots-vertical';
	import { api } from '$lib/api/config';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface Contact {
		id: string;
		email: string;
		phone?: string;
		mobile?: string;
		first_name?: string;
		last_name?: string;
		full_name: string;
		job_title?: string;
		department?: string;
		company_name?: string;
		company_id?: string;
		status: 'subscribed' | 'pending' | 'unsubscribed' | 'bounced' | 'complained';
		lifecycle_stage: 'subscriber' | 'lead' | 'mql' | 'sql' | 'opportunity' | 'customer' | 'evangelist';
		lead_score: number;
		health_score: number;
		engagement_score: number;
		value_score: number;
		email_opens: number;
		email_clicks: number;
		total_sessions: number;
		avg_engagement_score: number;
		avg_intent_score: number;
		friction_events_count: number;
		subscription_status: string;
		subscription_mrr: number;
		lifetime_value: number;
		deals_count: number;
		activities_count: number;
		notes_count: number;
		tasks_count: number;
		is_verified: boolean;
		is_unsubscribed: boolean;
		do_not_contact: boolean;
		is_vip: boolean;
		address_line1?: string;
		address_line2?: string;
		city?: string;
		state?: string;
		postal_code?: string;
		country?: string;
		timezone?: string;
		website?: string;
		linkedin_url?: string;
		twitter_handle?: string;
		last_activity_at?: string;
		last_contacted_at?: string;
		next_followup_at?: string;
		first_touch_channel?: string;
		last_touch_channel?: string;
		created_at: string;
		updated_at: string;
		tags?: { id: string; name: string; color?: string }[];
		lists?: { id: string; name: string }[];
		custom_fields?: Record<string, any>;
	}

	interface TimelineEvent {
		id: string;
		title: string;
		description?: string;
		type: string;
		occurred_at: string;
		created_by?: { name: string };
	}

	interface EmailActivity {
		id: string;
		subject: string;
		status: 'sent' | 'opened' | 'clicked' | 'bounced' | 'failed';
		sent_at: string;
		opened_at?: string;
		clicked_at?: string;
		campaign_name?: string;
	}

	interface Note {
		id: string;
		content: string;
		created_at: string;
		created_by: { name: string };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let contact = $state<Contact | null>(null);
	let timeline = $state<TimelineEvent[]>([]);
	let emailHistory = $state<EmailActivity[]>([]);
	let notes = $state<Note[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let activeTab = $state<'overview' | 'emails' | 'notes' | 'activity'>('overview');

	// Modal states
	let showAddTagModal = $state(false);
	let showAddListModal = $state(false);
	let showAddNoteModal = $state(false);
	let showSendEmailModal = $state(false);
	let newNoteContent = $state('');
	let availableTags = $state<{ id: string; name: string; color?: string }[]>([]);
	let availableLists = $state<{ id: string; name: string }[]>([]);

	let contactId = $derived($page.params.id as string);

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadContact() {
		loading = true;
		error = null;
		try {
			const [contactRes, timelineRes, emailsRes, notesRes] = await Promise.allSettled([
				api.get(`/api/admin/crm/contacts/${contactId}`),
				api.get(`/api/admin/crm/contacts/${contactId}/timeline`),
				api.get(`/api/admin/crm/contacts/${contactId}/emails`),
				api.get(`/api/admin/crm/contacts/${contactId}/notes`)
			]);

			if (contactRes.status === 'fulfilled') {
				contact = contactRes.value?.data || contactRes.value;
			} else {
				throw new Error('Failed to load contact');
			}

			if (timelineRes.status === 'fulfilled') {
				timeline = timelineRes.value?.data || timelineRes.value || [];
			}

			if (emailsRes.status === 'fulfilled') {
				emailHistory = emailsRes.value?.data || emailsRes.value || [];
			}

			if (notesRes.status === 'fulfilled') {
				notes = notesRes.value?.data || notesRes.value || [];
			}
		} catch (e) {
			console.error('Failed to load contact', e);
			error = 'Failed to load contact. Please try again.';
		} finally {
			loading = false;
		}
	}

	async function loadAvailableTagsAndLists() {
		try {
			const [tagsRes, listsRes] = await Promise.allSettled([
				api.get('/api/admin/crm/tags'),
				api.get('/api/admin/crm/lists')
			]);

			if (tagsRes.status === 'fulfilled') {
				availableTags = tagsRes.value?.data || tagsRes.value || [];
			}

			if (listsRes.status === 'fulfilled') {
				availableLists = listsRes.value?.data || listsRes.value || [];
			}
		} catch (e) {
			console.error('Failed to load tags/lists', e);
		}
	}

	async function addTag(tagId: string) {
		try {
			await api.post(`/api/admin/crm/contacts/${contactId}/tags`, { tag_id: tagId });
			await loadContact();
			showAddTagModal = false;
		} catch (e) {
			console.error('Failed to add tag', e);
		}
	}

	async function removeTag(tagId: string) {
		try {
			await api.delete(`/api/admin/crm/contacts/${contactId}/tags/${tagId}`);
			await loadContact();
		} catch (e) {
			console.error('Failed to remove tag', e);
		}
	}

	async function addToList(listId: string) {
		try {
			await api.post(`/api/admin/crm/contacts/${contactId}/lists`, { list_id: listId });
			await loadContact();
			showAddListModal = false;
		} catch (e) {
			console.error('Failed to add to list', e);
		}
	}

	async function removeFromList(listId: string) {
		try {
			await api.delete(`/api/admin/crm/contacts/${contactId}/lists/${listId}`);
			await loadContact();
		} catch (e) {
			console.error('Failed to remove from list', e);
		}
	}

	async function addNote() {
		if (!newNoteContent.trim()) return;
		try {
			await api.post(`/api/admin/crm/contacts/${contactId}/notes`, { content: newNoteContent });
			newNoteContent = '';
			showAddNoteModal = false;
			await loadContact();
		} catch (e) {
			console.error('Failed to add note', e);
		}
	}

	async function deleteContact() {
		if (!confirm('Are you sure you want to delete this contact? This action cannot be undone.')) return;
		try {
			await api.delete(`/api/admin/crm/contacts/${contactId}`);
			goto('/admin/crm');
		} catch (e) {
			console.error('Failed to delete contact', e);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function formatDateTime(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStatusColor(status: string): string {
		const colors: Record<string, string> = {
			subscribed: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
			pending: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
			unsubscribed: 'bg-slate-500/15 text-slate-400 border-slate-500/30',
			bounced: 'bg-red-500/15 text-red-400 border-red-500/30',
			complained: 'bg-red-500/15 text-red-400 border-red-500/30'
		};
		return colors[status] || colors.subscribed;
	}

	function getLifecycleColor(stage: string): string {
		const colors: Record<string, string> = {
			subscriber: 'bg-blue-500/15 text-blue-400 border-blue-500/30',
			lead: 'bg-cyan-500/15 text-cyan-400 border-cyan-500/30',
			mql: 'bg-violet-500/15 text-violet-400 border-violet-500/30',
			sql: 'bg-purple-500/15 text-purple-400 border-purple-500/30',
			opportunity: 'bg-orange-500/15 text-orange-400 border-orange-500/30',
			customer: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
			evangelist: 'bg-pink-500/15 text-pink-400 border-pink-500/30'
		};
		return colors[stage] || colors.subscriber;
	}

	function getEmailStatusColor(status: string): string {
		const colors: Record<string, string> = {
			sent: 'bg-blue-500/15 text-blue-400',
			opened: 'bg-emerald-500/15 text-emerald-400',
			clicked: 'bg-violet-500/15 text-violet-400',
			bounced: 'bg-red-500/15 text-red-400',
			failed: 'bg-red-500/15 text-red-400'
		};
		return colors[status] || colors.sent;
	}

	function goBack() {
		goto('/admin/crm');
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		await Promise.all([loadContact(), loadAvailableTagsAndLists()]);
	});
</script>

<svelte:head>
	<title>{contact ? `${contact.full_name} | Contact` : 'Contact'} - CRM Admin</title>
</svelte:head>

<div class="contact-detail-page">
	<!-- Back Button -->
	<button class="back-btn" onclick={goBack}>
		<IconArrowLeft size={18} />
		Back to Contacts
	</button>

	{#if loading && !contact}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading contact...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={loadContact}>Try Again</button>
		</div>
	{:else if contact}
		<!-- Header Section -->
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
							{#if contact.job_title && contact.company_name} at {/if}
							{contact.company_name || ''}
						</p>
					{/if}
					<div class="status-badges">
						<span class="status-badge {getStatusColor(contact.status)}">
							{contact.status}
						</span>
						<span class="status-badge {getLifecycleColor(contact.lifecycle_stage)}">
							{contact.lifecycle_stage}
						</span>
					</div>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-icon" onclick={() => loadContact()} title="Refresh">
					<IconRefresh size={18} />
				</button>
				<button class="btn-secondary" onclick={() => showSendEmailModal = true}>
					<IconSend size={18} />
					Send Email
				</button>
				<a href="/admin/crm/contacts/{contactId}/edit" class="btn-primary">
					<IconEdit size={18} />
					Edit Contact
				</a>
				<button class="btn-icon danger" onclick={deleteContact} title="Delete">
					<IconTrash size={18} />
				</button>
			</div>
		</header>

		<!-- Quick Stats -->
		<section class="quick-stats">
			<div class="stat-box">
				<IconTrendingUp size={20} class="stat-icon green" />
				<div class="stat-info">
					<span class="stat-value">{contact.lead_score}</span>
					<span class="stat-label">Lead Score</span>
				</div>
			</div>
			<div class="stat-box">
				<IconChartBar size={20} class="stat-icon blue" />
				<div class="stat-info">
					<span class="stat-value">{contact.health_score}</span>
					<span class="stat-label">Health Score</span>
				</div>
			</div>
			<div class="stat-box">
				<IconActivity size={20} class="stat-icon purple" />
				<div class="stat-info">
					<span class="stat-value">{contact.engagement_score}</span>
					<span class="stat-label">Engagement</span>
				</div>
			</div>
			<div class="stat-box">
				<IconMailOpen size={20} class="stat-icon cyan" />
				<div class="stat-info">
					<span class="stat-value">{contact.email_opens}</span>
					<span class="stat-label">Email Opens</span>
				</div>
			</div>
			<div class="stat-box">
				<IconClick size={20} class="stat-icon amber" />
				<div class="stat-info">
					<span class="stat-value">{contact.email_clicks}</span>
					<span class="stat-label">Email Clicks</span>
				</div>
			</div>
			<div class="stat-box">
				<IconCurrencyDollar size={20} class="stat-icon emerald" />
				<div class="stat-info">
					<span class="stat-value">${(contact.lifetime_value || 0).toLocaleString()}</span>
					<span class="stat-label">Lifetime Value</span>
				</div>
			</div>
		</section>

		<!-- Tabs -->
		<nav class="tabs-nav">
			<button class="tab-btn" class:active={activeTab === 'overview'} onclick={() => activeTab = 'overview'}>
				<IconUserCircle size={18} />
				Overview
			</button>
			<button class="tab-btn" class:active={activeTab === 'emails'} onclick={() => activeTab = 'emails'}>
				<IconMail size={18} />
				Emails ({emailHistory.length})
			</button>
			<button class="tab-btn" class:active={activeTab === 'notes'} onclick={() => activeTab = 'notes'}>
				<IconNotes size={18} />
				Notes ({notes.length})
			</button>
			<button class="tab-btn" class:active={activeTab === 'activity'} onclick={() => activeTab = 'activity'}>
				<IconActivity size={18} />
				Activity ({timeline.length})
			</button>
		</nav>

		<!-- Tab Content -->
		<div class="tab-content">
			{#if activeTab === 'overview'}
				<div class="overview-grid">
					<!-- Contact Info Card -->
					<div class="info-card">
						<h3>Contact Information</h3>
						<div class="info-grid">
							<div class="info-item">
								<IconMail size={16} />
								<div>
									<span class="info-label">Email</span>
									<a href="mailto:{contact.email || ''}" class="info-value link">{contact.email || ''}</a>
								</div>
							</div>
							{#if contact.phone}
								<div class="info-item">
									<IconPhone size={16} />
									<div>
										<span class="info-label">Phone</span>
										<a href="tel:{contact.phone}" class="info-value link">{contact.phone}</a>
									</div>
								</div>
							{/if}
							{#if contact.mobile}
								<div class="info-item">
									<IconPhone size={16} />
									<div>
										<span class="info-label">Mobile</span>
										<a href="tel:{contact.mobile}" class="info-value link">{contact.mobile}</a>
									</div>
								</div>
							{/if}
							{#if contact.company_name}
								<div class="info-item">
									<IconBuilding size={16} />
									<div>
										<span class="info-label">Company</span>
										<span class="info-value">{contact.company_name}</span>
									</div>
								</div>
							{/if}
							{#if contact.job_title}
								<div class="info-item">
									<IconBriefcase size={16} />
									<div>
										<span class="info-label">Job Title</span>
										<span class="info-value">{contact.job_title}</span>
									</div>
								</div>
							{/if}
							{#if contact.website}
								<div class="info-item">
									<IconWorld size={16} />
									<div>
										<span class="info-label">Website</span>
										<a href="{contact.website}" target="_blank" class="info-value link">{contact.website}</a>
									</div>
								</div>
							{/if}
							{#if contact.city || contact.country}
								<div class="info-item">
									<IconWorld size={16} />
									<div>
										<span class="info-label">Location</span>
										<span class="info-value">
											{[contact.city, contact.state, contact.country].filter(Boolean).join(', ')}
										</span>
									</div>
								</div>
							{/if}
							<div class="info-item">
								<IconCalendar size={16} />
								<div>
									<span class="info-label">Created</span>
									<span class="info-value">{formatDate(contact.created_at)}</span>
								</div>
							</div>
							<div class="info-item">
								<IconClock size={16} />
								<div>
									<span class="info-label">Last Activity</span>
									<span class="info-value">{formatDate(contact.last_activity_at)}</span>
								</div>
							</div>
						</div>
					</div>

					<!-- Tags Card -->
					<div class="info-card">
						<div class="card-header">
							<h3>
								<IconTag size={18} />
								Tags
							</h3>
							<button class="btn-add" onclick={() => showAddTagModal = true}>
								<IconPlus size={14} />
								Add
							</button>
						</div>
						<div class="tags-list">
							{#if contact.tags && contact.tags.length > 0}
								{#each contact.tags as tag}
									<span class="tag-pill" style="background-color: {tag.color || '#f97316'}20; color: {tag.color || '#f97316'}">
										{tag.name}
										<button class="tag-remove" onclick={() => removeTag(tag.id)}>
											<IconX size={12} />
										</button>
									</span>
								{/each}
							{:else}
								<p class="empty-text">No tags assigned</p>
							{/if}
						</div>
					</div>

					<!-- Lists Card -->
					<div class="info-card">
						<div class="card-header">
							<h3>
								<IconList size={18} />
								Lists
							</h3>
							<button class="btn-add" onclick={() => showAddListModal = true}>
								<IconPlus size={14} />
								Add
							</button>
						</div>
						<div class="lists-section">
							{#if contact.lists && contact.lists.length > 0}
								{#each contact.lists as list}
									<div class="list-item">
										<IconList size={16} />
										<span>{list.name}</span>
										<button class="list-remove" onclick={() => removeFromList(list.id)}>
											<IconX size={12} />
										</button>
									</div>
								{/each}
							{:else}
								<p class="empty-text">Not added to any list</p>
							{/if}
						</div>
					</div>

					<!-- Engagement Card -->
					<div class="info-card">
						<h3>Engagement Metrics</h3>
						<div class="metrics-grid">
							<div class="metric-item">
								<span class="metric-label">Total Sessions</span>
								<span class="metric-value">{contact.total_sessions}</span>
							</div>
							<div class="metric-item">
								<span class="metric-label">Avg Engagement</span>
								<span class="metric-value">{contact.avg_engagement_score?.toFixed(1) || 0}</span>
							</div>
							<div class="metric-item">
								<span class="metric-label">Avg Intent</span>
								<span class="metric-value">{contact.avg_intent_score?.toFixed(1) || 0}</span>
							</div>
							<div class="metric-item">
								<span class="metric-label">Friction Events</span>
								<span class="metric-value">{contact.friction_events_count || 0}</span>
							</div>
							<div class="metric-item">
								<span class="metric-label">Deals</span>
								<span class="metric-value">{contact.deals_count || 0}</span>
							</div>
							<div class="metric-item">
								<span class="metric-label">MRR</span>
								<span class="metric-value">${contact.subscription_mrr?.toFixed(0) || 0}</span>
							</div>
						</div>
					</div>
				</div>

			{:else if activeTab === 'emails'}
				<div class="emails-section">
					{#if emailHistory.length === 0}
						<div class="empty-state">
							<IconMail size={48} />
							<h3>No emails yet</h3>
							<p>Email history will appear here once emails are sent to this contact</p>
						</div>
					{:else}
						<div class="emails-list">
							{#each emailHistory as email}
								<div class="email-item">
									<div class="email-icon">
										<IconMail size={20} />
									</div>
									<div class="email-content">
										<div class="email-header">
											<span class="email-subject">{email.subject}</span>
											<span class="email-status {getEmailStatusColor(email.status)}">
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

			{:else if activeTab === 'notes'}
				<div class="notes-section">
					<div class="notes-header">
						<button class="btn-primary" onclick={() => showAddNoteModal = true}>
							<IconPlus size={18} />
							Add Note
						</button>
					</div>
					{#if notes.length === 0}
						<div class="empty-state">
							<IconNotes size={48} />
							<h3>No notes yet</h3>
							<p>Add notes to keep track of important information about this contact</p>
						</div>
					{:else}
						<div class="notes-list">
							{#each notes as note}
								<div class="note-item">
									<div class="note-header">
										<span class="note-author">{note.created_by?.name || 'Unknown'}</span>
										<span class="note-date">{formatDateTime(note.created_at)}</span>
									</div>
									<p class="note-content">{note.content}</p>
								</div>
							{/each}
						</div>
					{/if}
				</div>

			{:else if activeTab === 'activity'}
				<div class="activity-section">
					{#if timeline.length === 0}
						<div class="empty-state">
							<IconActivity size={48} />
							<h3>No activity yet</h3>
							<p>Activity timeline will appear here as events occur</p>
						</div>
					{:else}
						<div class="timeline">
							{#each timeline as event}
								<div class="timeline-item">
									<div class="timeline-dot"></div>
									<div class="timeline-content">
										<div class="timeline-header">
											<span class="timeline-title">{event.title}</span>
											<span class="timeline-time">{formatDateTime(event.occurred_at)}</span>
										</div>
										{#if event.description}
											<p class="timeline-description">{event.description}</p>
										{/if}
										{#if event.created_by}
											<span class="timeline-author">by {event.created_by.name}</span>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{:else}
		<div class="empty-state">
			<IconUserCircle size={48} />
			<h3>Contact not found</h3>
			<p>The contact you're looking for doesn't exist or has been deleted</p>
			<button class="btn-primary" onclick={goBack}>Go Back</button>
		</div>
	{/if}
</div>

<!-- Add Tag Modal -->
{#if showAddTagModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_interactive_supports_focus -->
	<div class="modal-overlay" onclick={() => showAddTagModal = false} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showAddTagModal = false)} role="dialog" aria-modal="true" aria-labelledby="add-tag-title" tabindex="-1">
		<div class="modal" role="document" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()}>
			<div class="modal-header">
				<h3 id="add-tag-title">Add Tag</h3>
				<button class="modal-close" onclick={() => showAddTagModal = false}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				{#if availableTags.length === 0}
					<p class="empty-text">No tags available</p>
				{:else}
					<div class="tag-options">
						{#each availableTags as tag}
							<button class="tag-option" onclick={() => addTag(tag.id)}>
								<IconTag size={16} />
								{tag.name}
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<!-- Add List Modal -->
{#if showAddListModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div class="modal-overlay" onclick={() => showAddListModal = false} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showAddListModal = false)} role="dialog" aria-modal="true" aria-labelledby="add-list-title" tabindex="-1">
		<div class="modal" role="document" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()}>
			<div class="modal-header">
				<h3 id="add-list-title">Add to List</h3>
				<button class="modal-close" onclick={() => showAddListModal = false}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				{#if availableLists.length === 0}
					<p class="empty-text">No lists available</p>
				{:else}
					<div class="list-options">
						{#each availableLists as list}
							<button class="list-option" onclick={() => addToList(list.id)}>
								<IconList size={16} />
								{list.name}
							</button>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<!-- Add Note Modal -->
{#if showAddNoteModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions a11y_interactive_supports_focus -->
	<div class="modal-overlay" onclick={() => showAddNoteModal = false} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showAddNoteModal = false)} role="dialog" aria-modal="true" aria-labelledby="add-note-title" tabindex="-1">
		<div class="modal" role="document" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()}>
			<div class="modal-header">
				<h3 id="add-note-title">Add Note</h3>
				<button class="modal-close" onclick={() => showAddNoteModal = false}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<textarea
					class="note-input"
					placeholder="Write your note here..."
					bind:value={newNoteContent}
					rows="4"
				></textarea>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => showAddNoteModal = false}>Cancel</button>
				<button class="btn-primary" onclick={addNote} disabled={!newNoteContent.trim()}>
					<IconCheck size={18} />
					Save Note
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.contact-detail-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 24px;
	}

	/* Back Button */
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 8px 16px;
		background: transparent;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: 24px;
	}

	.back-btn:hover {
		background: #1e293b;
		color: #f97316;
		border-color: #f97316;
	}

	/* Header */
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
		background: linear-gradient(135deg, #f97316, #ea580c);
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
		background: rgba(249, 115, 22, 0.1);
		color: #f97316;
		border-color: rgba(249, 115, 22, 0.3);
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
		background: linear-gradient(135deg, #f97316, #ea580c);
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
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
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

	/* Quick Stats */
	.quick-stats {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 12px;
		margin-bottom: 24px;
	}

	.stat-box {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
	}

	.stat-box :global(.stat-icon) {
		flex-shrink: 0;
	}

	.stat-box :global(.green) { color: #4ade80; }
	.stat-box :global(.blue) { color: #60a5fa; }
	.stat-box :global(.purple) { color: #a78bfa; }
	.stat-box :global(.cyan) { color: #22d3ee; }
	.stat-box :global(.amber) { color: #fbbf24; }
	.stat-box :global(.emerald) { color: #34d399; }

	.stat-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
	}

	.stat-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Tabs */
	.tabs-nav {
		display: flex;
		gap: 4px;
		margin-bottom: 24px;
		padding: 4px;
		background: #1e293b;
		border-radius: 12px;
		overflow-x: auto;
	}

	.tab-btn {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.tab-btn:hover {
		color: #e2e8f0;
		background: rgba(249, 115, 22, 0.1);
	}

	.tab-btn.active {
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	/* Tab Content */
	.tab-content {
		min-height: 400px;
	}

	/* Overview Grid */
	.overview-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 20px;
	}

	.info-card {
		padding: 20px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
	}

	.info-card h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 0 0 16px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.info-card h3 :global(svg) {
		color: #f97316;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}

	.card-header h3 {
		margin: 0;
	}

	.btn-add {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		background: rgba(249, 115, 22, 0.1);
		border: 1px solid rgba(249, 115, 22, 0.3);
		border-radius: 6px;
		color: #f97316;
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add:hover {
		background: rgba(249, 115, 22, 0.2);
	}

	.info-grid {
		display: grid;
		gap: 12px;
	}

	.info-item {
		display: flex;
		align-items: flex-start;
		gap: 12px;
		padding: 10px;
		background: #0f172a;
		border-radius: 8px;
	}

	.info-item :global(svg) {
		color: #64748b;
		flex-shrink: 0;
		margin-top: 2px;
	}

	.info-item > div {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.info-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.info-value {
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.info-value.link {
		color: #60a5fa;
		text-decoration: none;
	}

	.info-value.link:hover {
		text-decoration: underline;
	}

	/* Tags */
	.tags-list {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.tag-pill {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		border-radius: 20px;
		font-size: 0.8rem;
		font-weight: 500;
	}

	.tag-remove {
		display: flex;
		padding: 2px;
		background: transparent;
		border: none;
		color: inherit;
		opacity: 0.6;
		cursor: pointer;
	}

	.tag-remove:hover {
		opacity: 1;
	}

	/* Lists */
	.lists-section {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px 12px;
		background: #0f172a;
		border-radius: 8px;
		font-size: 0.875rem;
		color: #e2e8f0;
	}

	.list-item :global(svg) {
		color: #64748b;
	}

	.list-remove {
		margin-left: auto;
		display: flex;
		padding: 4px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.list-item:hover .list-remove {
		opacity: 1;
	}

	.list-remove:hover {
		color: #f87171;
	}

	/* Metrics */
	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 12px;
	}

	.metric-item {
		display: flex;
		flex-direction: column;
		gap: 4px;
		padding: 12px;
		background: #0f172a;
		border-radius: 8px;
	}

	.metric-label {
		font-size: 0.7rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.metric-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: white;
	}

	/* Emails */
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
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

	/* Notes */
	.notes-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.notes-header {
		margin-bottom: 20px;
	}

	.notes-list {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.note-item {
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
	}

	.note-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 8px;
	}

	.note-author {
		font-weight: 600;
		color: #e2e8f0;
		font-size: 0.875rem;
	}

	.note-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.note-content {
		margin: 0;
		color: #94a3b8;
		font-size: 0.9rem;
		line-height: 1.6;
		white-space: pre-wrap;
	}

	/* Activity */
	.activity-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.timeline {
		display: flex;
		flex-direction: column;
		gap: 0;
	}

	.timeline-item {
		display: flex;
		gap: 16px;
		padding: 16px 0;
		border-bottom: 1px solid #334155;
	}

	.timeline-item:last-child {
		border-bottom: none;
	}

	.timeline-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background: #f97316;
		flex-shrink: 0;
		margin-top: 4px;
	}

	.timeline-content {
		flex: 1;
	}

	.timeline-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 12px;
		margin-bottom: 4px;
	}

	.timeline-title {
		font-weight: 600;
		color: white;
		font-size: 0.9rem;
	}

	.timeline-time {
		font-size: 0.75rem;
		color: #64748b;
		white-space: nowrap;
	}

	.timeline-description {
		margin: 0 0 4px;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	.timeline-author {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Empty States */
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

	.empty-text {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	/* Loading State */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		color: #64748b;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(249, 115, 22, 0.2);
		border-top-color: #f97316;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 60px 20px;
		text-align: center;
		color: #f87171;
	}

	.error-state button {
		margin-top: 16px;
		padding: 10px 20px;
		background: #f97316;
		color: white;
		border: none;
		border-radius: 8px;
		cursor: pointer;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		width: 100%;
		max-width: 480px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-header h3 {
		margin: 0;
		font-size: 1.1rem;
		color: white;
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #334155;
	}

	.tag-options, .list-options {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.tag-option, .list-option {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 16px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tag-option:hover, .list-option:hover {
		background: rgba(249, 115, 22, 0.1);
		border-color: rgba(249, 115, 22, 0.3);
		color: #f97316;
	}

	.note-input {
		width: 100%;
		padding: 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
		min-height: 100px;
	}

	.note-input:focus {
		outline: none;
		border-color: #f97316;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.quick-stats {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 900px) {
		.overview-grid {
			grid-template-columns: 1fr;
		}

		.quick-stats {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.contact-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			justify-content: flex-start;
		}

		.quick-stats {
			grid-template-columns: 1fr;
		}

		.tabs-nav {
			justify-content: flex-start;
		}
	}
</style>
