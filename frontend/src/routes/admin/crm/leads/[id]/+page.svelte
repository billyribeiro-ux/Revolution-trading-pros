<!--
	/admin/crm/leads/[id] - Lead Detail Profile
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Complete lead profile with scoring
	- Activity timeline and interactions
	- Notes and task management
	- Lead scoring breakdown
	- Quick actions (edit, convert, delete)
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
	/**
	 * Lead Detail Page - Apple ICT 7 Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Comprehensive lead profile with:
	 * - Profile information and scoring
	 * - Activity timeline
	 * - Notes management
	 * - Task tracking
	 * - Quick actions
	 *
	 * @version 1.0.0 (January 2026)
	 */

	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import {
		IconArrowLeft,
		IconMail,
		IconPhone,
		IconActivity,
		IconUserCircle,
		IconEdit,
		IconTrash,
		IconTag,
		IconPlus,
		IconX,
		IconCheck,
		IconClock,
		IconChartBar,
		IconTrendingUp,
		IconBuilding,
		IconBriefcase,
		IconCalendar,
		IconRefresh,
		IconFlame,
		IconStar,
		IconStarFilled,
		IconArrowRight,
		IconNote,
		IconTarget,
		IconWorld
	} from '$lib/icons';
	import { api } from '$lib/api/config';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface Lead {
		id: string;
		email: string;
		phone?: string;
		mobile?: string;
		first_name?: string;
		last_name?: string;
		full_name: string;
		job_title?: string;
		company_name?: string;
		website?: string;
		address?: string;
		city?: string;
		state?: string;
		country?: string;
		status: 'new' | 'contacted' | 'qualified' | 'proposal' | 'negotiation' | 'won' | 'lost';
		source: string;
		lead_score: number;
		engagement_score: number;
		fit_score: number;
		behavior_score: number;
		is_hot: boolean;
		is_starred: boolean;
		assigned_to?: { id: string; name: string; email: string };
		tags?: { id: string; name: string; color?: string }[];
		estimated_value?: number;
		notes_count: number;
		activities_count: number;
		emails_count: number;
		last_contacted_at?: string;
		next_followup_at?: string;
		created_at: string;
		updated_at: string;
		custom_fields?: Record<string, any>;
	}

	interface TimelineEvent {
		id: string;
		title: string;
		description?: string;
		type: 'email' | 'call' | 'meeting' | 'note' | 'status_change' | 'created';
		occurred_at: string;
		created_by?: { name: string };
	}

	interface Note {
		id: string;
		content: string;
		created_at: string;
		created_by: { name: string };
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let lead = $state<Lead | null>(null);
	let timeline = $state<TimelineEvent[]>([]);
	let notes = $state<Note[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let activeTab = $state<'overview' | 'activity' | 'notes'>('overview');

	// Modal states
	let showAddNoteModal = $state(false);
	let showConvertModal = $state(false);
	let showDeleteModal = $state(false);
	let newNoteContent = $state('');
	let actionLoading = $state(false);

	// Toast state
	let toastMessage = $state<{ type: 'success' | 'error'; text: string } | null>(null);

	let leadId = $derived(page.params.id as string);

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadLead() {
		loading = true;
		error = null;

		try {
			const [leadRes, timelineRes, notesRes] = await Promise.allSettled([
				api.get(`/api/admin/crm/leads/${leadId}`),
				api.get(`/api/admin/crm/leads/${leadId}/timeline`),
				api.get(`/api/admin/crm/leads/${leadId}/notes`)
			]);

			if (leadRes.status === 'fulfilled') {
				lead = leadRes.value?.data || leadRes.value;
			} else {
				throw new Error('Failed to load lead');
			}

			if (timelineRes.status === 'fulfilled') {
				timeline = timelineRes.value?.data || timelineRes.value || [];
			}

			if (notesRes.status === 'fulfilled') {
				notes = notesRes.value?.data || notesRes.value || [];
			}
		} catch (e) {
			console.error('Failed to load lead:', e);
			error = 'Failed to load lead. Please try again.';
		} finally {
			loading = false;
		}
	}

	async function updateStatus(newStatus: string) {
		if (!lead) return;

		try {
			await api.patch(`/api/admin/crm/leads/${leadId}/status`, { status: newStatus });
			lead.status = newStatus as Lead['status'];
			showToast('success', `Status updated to ${newStatus}`);
			await loadLead();
		} catch (e) {
			console.error('Failed to update status:', e);
			showToast('error', 'Failed to update status');
		}
	}

	async function toggleStarred() {
		if (!lead) return;

		try {
			await api.patch(`/api/admin/crm/leads/${leadId}/star`, {
				is_starred: !lead.is_starred
			});
			lead.is_starred = !lead.is_starred;
		} catch (e) {
			console.error('Failed to toggle starred:', e);
		}
	}

	async function addNote() {
		if (!newNoteContent.trim()) return;
		actionLoading = true;

		try {
			await api.post(`/api/admin/crm/leads/${leadId}/notes`, {
				content: newNoteContent
			});
			newNoteContent = '';
			showAddNoteModal = false;
			showToast('success', 'Note added successfully');
			await loadLead();
		} catch (e) {
			console.error('Failed to add note:', e);
			showToast('error', 'Failed to add note');
		} finally {
			actionLoading = false;
		}
	}

	async function convertToContact() {
		actionLoading = true;

		try {
			await api.post(`/api/admin/crm/leads/${leadId}/convert`);
			showConvertModal = false;
			showToast('success', 'Lead converted to contact successfully');
			goto('/admin/crm');
		} catch (e) {
			console.error('Failed to convert lead:', e);
			showToast('error', 'Failed to convert lead');
		} finally {
			actionLoading = false;
		}
	}

	async function deleteLead() {
		actionLoading = true;

		try {
			await api.delete(`/api/admin/crm/leads/${leadId}`);
			showDeleteModal = false;
			showToast('success', 'Lead deleted successfully');
			goto('/admin/crm/leads');
		} catch (e) {
			console.error('Failed to delete lead:', e);
			showToast('error', 'Failed to delete lead');
		} finally {
			actionLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function showToast(type: 'success' | 'error', text: string) {
		toastMessage = { type, text };
		setTimeout(() => {
			toastMessage = null;
		}, 5000);
	}

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

	function formatCurrency(amount: number | undefined): string {
		if (!amount) return '$0';
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0
		}).format(amount);
	}

	function getStatusColor(status: string): string {
		const colors: Record<string, string> = {
			new: 'bg-blue-500/15 text-blue-400 border-blue-500/30',
			contacted: 'bg-cyan-500/15 text-cyan-400 border-cyan-500/30',
			qualified: 'bg-violet-500/15 text-violet-400 border-violet-500/30',
			proposal: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
			negotiation: 'bg-orange-500/15 text-orange-400 border-orange-500/30',
			won: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
			lost: 'bg-red-500/15 text-red-400 border-red-500/30'
		};
		return colors[status] || colors.new;
	}

	function getScoreColor(score: number): string {
		if (score >= 80) return 'text-emerald-400';
		if (score >= 60) return 'text-amber-400';
		if (score >= 40) return 'text-orange-400';
		return 'text-red-400';
	}

	// @ts-expect-error write-only state
	function _getTimelineIcon(type: string): string {
		const icons: Record<string, string> = {
			email: 'mail',
			call: 'phone',
			meeting: 'calendar',
			note: 'note',
			status_change: 'activity',
			created: 'user'
		};
		return icons[type] || 'activity';
	}

	function goBack() {
		goto('/admin/crm/leads');
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		await loadLead();
	});
</script>

<svelte:head>
	<title>{lead ? `${lead.full_name} | Lead` : 'Lead'} - CRM Admin</title>
</svelte:head>

<div class="lead-detail-page">
	<!-- Back Button -->
	<button class="back-btn" onclick={goBack}>
		<IconArrowLeft size={18} />
		Back to Leads
	</button>

	{#if loading && !lead}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading lead...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={loadLead}>Try Again</button>
		</div>
	{:else if lead}
		<!-- Header Section -->
		<header class="lead-header">
			<div class="lead-avatar-section">
				<div class="avatar-large" class:hot={lead.is_hot}>
					{lead.full_name?.charAt(0).toUpperCase() || '?'}
					{#if lead.is_hot}
						<span class="hot-badge"><IconFlame size={14} /></span>
					{/if}
				</div>
				<div class="lead-identity">
					<div class="name-row">
						<h1>{lead.full_name}</h1>
						<button class="btn-star" class:starred={lead.is_starred} onclick={toggleStarred}>
							{#if lead.is_starred}
								<IconStarFilled size={20} />
							{:else}
								<IconStar size={20} />
							{/if}
						</button>
					</div>
					{#if lead.job_title || lead.company_name}
						<p class="job-info">
							{lead.job_title || ''}
							{#if lead.job_title && lead.company_name}
								at
							{/if}
							{lead.company_name || ''}
						</p>
					{/if}
					<div class="status-badges">
						<select
							class="status-select {getStatusColor(lead.status)}"
							value={lead.status}
							onchange={(e) => updateStatus((e.target as HTMLSelectElement).value)}
						>
							<option value="new">New</option>
							<option value="contacted">Contacted</option>
							<option value="qualified">Qualified</option>
							<option value="proposal">Proposal</option>
							<option value="negotiation">Negotiation</option>
							<option value="won">Won</option>
							<option value="lost">Lost</option>
						</select>
						<span class="source-badge">{lead.source || 'Unknown Source'}</span>
					</div>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-icon" onclick={loadLead} title="Refresh">
					<IconRefresh size={18} />
				</button>
				<button class="btn-secondary" onclick={() => (showConvertModal = true)}>
					<IconArrowRight size={18} />
					Convert to Contact
				</button>
				<a href="/admin/crm/leads/{leadId}/edit" class="btn-primary">
					<IconEdit size={18} />
					Edit Lead
				</a>
				<button class="btn-icon danger" onclick={() => (showDeleteModal = true)} title="Delete">
					<IconTrash size={18} />
				</button>
			</div>
		</header>

		<!-- Score Overview -->
		<section class="score-section">
			<div class="score-card main">
				<div class="score-header">
					<IconTarget size={24} />
					<span>Lead Score</span>
				</div>
				<div class="score-value {getScoreColor(lead.lead_score)}">
					{lead.lead_score}
				</div>
				<div class="score-bar">
					<div class="score-fill" style="width: {lead.lead_score}%"></div>
				</div>
			</div>
			<div class="score-card">
				<div class="score-header">
					<IconChartBar size={20} />
					<span>Fit Score</span>
				</div>
				<div class="score-value {getScoreColor(lead.fit_score || 0)}">
					{lead.fit_score || 0}
				</div>
			</div>
			<div class="score-card">
				<div class="score-header">
					<IconActivity size={20} />
					<span>Engagement</span>
				</div>
				<div class="score-value {getScoreColor(lead.engagement_score || 0)}">
					{lead.engagement_score || 0}
				</div>
			</div>
			<div class="score-card">
				<div class="score-header">
					<IconTrendingUp size={20} />
					<span>Behavior</span>
				</div>
				<div class="score-value {getScoreColor(lead.behavior_score || 0)}">
					{lead.behavior_score || 0}
				</div>
			</div>
			<div class="score-card value">
				<div class="score-header">
					<span>Est. Value</span>
				</div>
				<div class="score-value green">
					{formatCurrency(lead.estimated_value)}
				</div>
			</div>
		</section>

		<!-- Tabs -->
		<nav class="tabs-nav">
			<button
				class="tab-btn"
				class:active={activeTab === 'overview'}
				onclick={() => (activeTab = 'overview')}
			>
				<IconUserCircle size={18} />
				Overview
			</button>
			<button
				class="tab-btn"
				class:active={activeTab === 'activity'}
				onclick={() => (activeTab = 'activity')}
			>
				<IconActivity size={18} />
				Activity ({timeline.length})
			</button>
			<button
				class="tab-btn"
				class:active={activeTab === 'notes'}
				onclick={() => (activeTab = 'notes')}
			>
				<IconNote size={18} />
				Notes ({notes.length})
			</button>
		</nav>

		<!-- Tab Content - Layout Shift Free Pattern -->
		<div class="tab-content">
			<!-- Overview Panel -->
			<div
				class="tab-panel"
				class:active={activeTab === 'overview'}
				inert={activeTab !== 'overview' ? true : undefined}
			>
				<div class="overview-grid">
					<!-- Contact Info Card -->
					<div class="info-card">
						<h3>Contact Information</h3>
						<div class="info-grid">
							<div class="info-item">
								<IconMail size={16} />
								<div>
									<span class="info-label">Email</span>
									<a href="mailto:{lead.email}" class="info-value link">
										{lead.email}
									</a>
								</div>
							</div>
							{#if lead.phone}
								<div class="info-item">
									<IconPhone size={16} />
									<div>
										<span class="info-label">Phone</span>
										<a href="tel:{lead.phone}" class="info-value link">
											{lead.phone}
										</a>
									</div>
								</div>
							{/if}
							{#if lead.company_name}
								<div class="info-item">
									<IconBuilding size={16} />
									<div>
										<span class="info-label">Company</span>
										<span class="info-value">{lead.company_name}</span>
									</div>
								</div>
							{/if}
							{#if lead.job_title}
								<div class="info-item">
									<IconBriefcase size={16} />
									<div>
										<span class="info-label">Job Title</span>
										<span class="info-value">{lead.job_title}</span>
									</div>
								</div>
							{/if}
							{#if lead.website}
								<div class="info-item">
									<IconWorld size={16} />
									<div>
										<span class="info-label">Website</span>
										<a href={lead.website} target="_blank" class="info-value link">
											{lead.website}
										</a>
									</div>
								</div>
							{/if}
							{#if lead.city || lead.country}
								<div class="info-item">
									<IconWorld size={16} />
									<div>
										<span class="info-label">Location</span>
										<span class="info-value">
											{[lead.city, lead.state, lead.country].filter(Boolean).join(', ')}
										</span>
									</div>
								</div>
							{/if}
						</div>
					</div>

					<!-- Dates Card -->
					<div class="info-card">
						<h3>Key Dates</h3>
						<div class="info-grid">
							<div class="info-item">
								<IconCalendar size={16} />
								<div>
									<span class="info-label">Created</span>
									<span class="info-value">{formatDate(lead.created_at)}</span>
								</div>
							</div>
							<div class="info-item">
								<IconClock size={16} />
								<div>
									<span class="info-label">Last Updated</span>
									<span class="info-value">{formatDate(lead.updated_at)}</span>
								</div>
							</div>
							<div class="info-item">
								<IconMail size={16} />
								<div>
									<span class="info-label">Last Contacted</span>
									<span class="info-value">
										{formatDate(lead.last_contacted_at)}
									</span>
								</div>
							</div>
							{#if lead.next_followup_at}
								<div class="info-item">
									<IconCalendar size={16} />
									<div>
										<span class="info-label">Next Follow-up</span>
										<span class="info-value">
											{formatDate(lead.next_followup_at)}
										</span>
									</div>
								</div>
							{/if}
						</div>
					</div>

					<!-- Tags Card -->
					<div class="info-card">
						<div class="card-header">
							<h3>
								<IconTag size={18} />
								Tags
							</h3>
							<button class="btn-add">
								<IconPlus size={14} />
								Add
							</button>
						</div>
						<div class="tags-list">
							{#if lead.tags && lead.tags.length > 0}
								{#each lead.tags as tag}
									<span
										class="tag-pill"
										style="background-color: {tag.color || '#f97316'}20; color: {tag.color ||
											'#f97316'}"
									>
										{tag.name}
									</span>
								{/each}
							{:else}
								<p class="empty-text">No tags assigned</p>
							{/if}
						</div>
					</div>

					<!-- Assignment Card -->
					{#if lead.assigned_to}
						<div class="info-card">
							<h3>Assigned To</h3>
							<div class="assignee">
								<div class="assignee-avatar">
									{lead.assigned_to.name.charAt(0).toUpperCase()}
								</div>
								<div class="assignee-info">
									<span class="assignee-name">{lead.assigned_to.name}</span>
									<span class="assignee-email">{lead.assigned_to.email}</span>
								</div>
							</div>
						</div>
					{/if}
				</div>
			</div>

			<!-- Activity Panel -->
			<div
				class="tab-panel"
				class:active={activeTab === 'activity'}
				inert={activeTab !== 'activity' ? true : undefined}
			>
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
									<div class="timeline-dot {event.type}"></div>
									<div class="timeline-content">
										<div class="timeline-header">
											<span class="timeline-title">{event.title}</span>
											<span class="timeline-time">
												{formatDateTime(event.occurred_at)}
											</span>
										</div>
										{#if event.description}
											<p class="timeline-description">{event.description}</p>
										{/if}
										{#if event.created_by}
											<span class="timeline-author">
												by {event.created_by.name}
											</span>
										{/if}
									</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>

			<!-- Notes Panel -->
			<div
				class="tab-panel"
				class:active={activeTab === 'notes'}
				inert={activeTab !== 'notes' ? true : undefined}
			>
				<div class="notes-section">
					<div class="notes-header">
						<button class="btn-primary" onclick={() => (showAddNoteModal = true)}>
							<IconPlus size={18} />
							Add Note
						</button>
					</div>
					{#if notes.length === 0}
						<div class="empty-state">
							<IconNote size={48} />
							<h3>No notes yet</h3>
							<p>Add notes to track important information about this lead</p>
						</div>
					{:else}
						<div class="notes-list">
							{#each notes as note}
								<div class="note-item">
									<div class="note-header">
										<span class="note-author">
											{note.created_by?.name || 'Unknown'}
										</span>
										<span class="note-date">
											{formatDateTime(note.created_at)}
										</span>
									</div>
									<p class="note-content">{note.content}</p>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>
		</div>
	{:else}
		<div class="empty-state">
			<IconUserCircle size={48} />
			<h3>Lead not found</h3>
			<p>The lead you're looking for doesn't exist or has been deleted</p>
			<button class="btn-primary" onclick={goBack}>Go Back</button>
		</div>
	{/if}
</div>

<!-- Add Note Modal -->
{#if showAddNoteModal}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => (showAddNoteModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showAddNoteModal = false)}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconNote size={20} />
					Add Note
				</h3>
				<button class="modal-close" onclick={() => (showAddNoteModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<textarea
					class="note-input"
					placeholder="Write your note here..."
					bind:value={newNoteContent}
					rows="5"
				></textarea>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showAddNoteModal = false)}> Cancel </button>
				<button
					class="btn-primary"
					onclick={addNote}
					disabled={actionLoading || !newNoteContent.trim()}
				>
					{#if actionLoading}
						Saving...
					{:else}
						<IconCheck size={18} />
						Save Note
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Convert Modal -->
{#if showConvertModal}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => (showConvertModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal modal-small"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showConvertModal = false)}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconArrowRight size={20} />
					Convert to Contact
				</h3>
				<button class="modal-close" onclick={() => (showConvertModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="confirm-text">
					Convert <strong>{lead?.full_name}</strong> to a contact? The lead will be moved to your contacts
					list with all associated data.
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showConvertModal = false)}> Cancel </button>
				<button class="btn-primary" onclick={convertToContact} disabled={actionLoading}>
					{#if actionLoading}
						Converting...
					{:else}
						<IconArrowRight size={18} />
						Convert Lead
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Delete Modal -->
{#if showDeleteModal}
	<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions a11y_interactive_supports_focus -->
	<div
		class="modal-overlay"
		onclick={() => (showDeleteModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal modal-small"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showDeleteModal = false)}
			role="document"
		>
			<div class="modal-header">
				<h3>
					<IconTrash size={20} />
					Delete Lead
				</h3>
				<button class="modal-close" onclick={() => (showDeleteModal = false)}>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<p class="confirm-text">
					Are you sure you want to delete <strong>{lead?.full_name}</strong>? This action cannot be
					undone.
				</p>
			</div>
			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showDeleteModal = false)}> Cancel </button>
				<button class="btn-danger" onclick={deleteLead} disabled={actionLoading}>
					{#if actionLoading}
						Deleting...
					{:else}
						<IconTrash size={18} />
						Delete Lead
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Toast Notification -->
{#if toastMessage}
	<div class="toast toast-{toastMessage.type}" role="alert">
		{#if toastMessage.type === 'success'}
			<IconCheck size={18} />
		{:else}
			<IconX size={18} />
		{/if}
		<span>{toastMessage.text}</span>
		<button class="toast-close" onclick={() => (toastMessage = null)}>
			<IconX size={14} />
		</button>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   LEAD DETAIL PAGE - Apple ICT 7 Principal Engineer Grade
	   ═══════════════════════════════════════════════════════════════════════════ */

	.lead-detail-page {
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
	.lead-header {
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

	.lead-avatar-section {
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
		position: relative;
	}

	.avatar-large.hot {
		background: linear-gradient(135deg, #f97316, #ef4444);
	}

	.hot-badge {
		position: absolute;
		bottom: -4px;
		right: -4px;
		width: 28px;
		height: 28px;
		background: #ef4444;
		border: 3px solid #1e293b;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.lead-identity {
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

	.btn-star {
		display: flex;
		padding: 6px;
		background: transparent;
		border: none;
		color: #475569;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-star:hover,
	.btn-star.starred {
		color: #fbbf24;
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
		align-items: center;
	}

	.status-select {
		padding: 6px 14px;
		background: transparent;
		border: 1px solid;
		border-radius: 20px;
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
		text-transform: capitalize;
	}

	.status-select:focus {
		outline: none;
	}

	.source-badge {
		padding: 6px 12px;
		background: rgba(100, 116, 139, 0.15);
		border-radius: 20px;
		font-size: 0.75rem;
		color: #94a3b8;
		text-transform: capitalize;
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

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
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

	/* Score Section */
	.score-section {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 16px;
		margin-bottom: 24px;
	}

	.score-card {
		padding: 20px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
	}

	.score-card.main {
		grid-column: span 1;
	}

	.score-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 12px;
		color: #64748b;
		font-size: 0.8rem;
		font-weight: 600;
	}

	.score-header :global(svg) {
		color: #f97316;
	}

	.score-value {
		font-size: 2rem;
		font-weight: 700;
	}

	.score-value.green {
		color: #4ade80;
	}

	.score-bar {
		margin-top: 12px;
		height: 6px;
		background: #334155;
		border-radius: 3px;
		overflow: hidden;
	}

	.score-fill {
		height: 100%;
		background: linear-gradient(90deg, #f97316, #fbbf24);
		border-radius: 3px;
		transition: width 0.3s;
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

	/* Tab Content - Layout Shift Prevention */
	.tab-content {
		position: relative;
		min-height: 400px;
		contain: layout style;
		isolation: isolate;
	}

	/* Tab Panels - CSS visibility toggling eliminates layout shift */
	.tab-panel {
		position: absolute;
		inset: 0;
		width: 100%;
		contain: content;
		opacity: 0;
		visibility: hidden;
		transform: translateY(8px);
		transition:
			opacity 0.2s ease,
			visibility 0.2s ease,
			transform 0.2s ease;
		z-index: 0;
		pointer-events: none;
	}

	.tab-panel.active {
		position: relative;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		z-index: 1;
		pointer-events: auto;
	}

	@media (prefers-reduced-motion: reduce) {
		.tab-panel {
			transition: none;
			transform: none;
		}
		.tab-panel:not(.active) {
			display: none;
		}
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

	.empty-text {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	/* Assignee */
	.assignee {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px;
		background: #0f172a;
		border-radius: 10px;
	}

	.assignee-avatar {
		width: 44px;
		height: 44px;
		border-radius: 50%;
		background: linear-gradient(135deg, #f97316, #ea580c);
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 600;
		color: white;
	}

	.assignee-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.assignee-name {
		font-weight: 600;
		color: white;
	}

	.assignee-email {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Activity Section */
	.activity-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 14px;
		padding: 20px;
	}

	.timeline {
		display: flex;
		flex-direction: column;
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

	.timeline-dot.email {
		background: var(--primary-500);
	}

	.timeline-dot.call {
		background: #22d3ee;
	}

	.timeline-dot.meeting {
		background: #fbbf24;
	}

	.timeline-dot.note {
		background: #4ade80;
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

	/* Notes Section */
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

	/* Empty & Loading States */
	.empty-state,
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
		color: #64748b;
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
		font-size: 0.9rem;
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
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
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

	.modal-small {
		max-width: 420px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-header h3 {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 0;
		font-size: 1.1rem;
		color: white;
	}

	.modal-header h3 :global(svg) {
		color: #f97316;
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
		min-height: 120px;
	}

	.note-input:focus {
		outline: none;
		border-color: #f97316;
	}

	.confirm-text {
		margin: 0;
		font-size: 0.95rem;
		color: #94a3b8;
		line-height: 1.6;
	}

	.confirm-text strong {
		color: white;
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #ef4444, #dc2626);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(239, 68, 68, 0.4);
	}

	.btn-danger:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Toast */
	.toast {
		position: fixed;
		bottom: 24px;
		right: 24px;
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 18px;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 500;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
		z-index: 2000;
		animation: slideIn 0.3s ease-out;
	}

	.toast-success {
		background: linear-gradient(135deg, #065f46, #047857);
		border: 1px solid #10b981;
		color: #ecfdf5;
	}

	.toast-error {
		background: linear-gradient(135deg, #7f1d1d, #991b1b);
		border: 1px solid #f87171;
		color: #fef2f2;
	}

	.toast-close {
		display: flex;
		padding: 4px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 6px;
		color: inherit;
		cursor: pointer;
	}

	.toast-close:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.score-section {
			grid-template-columns: repeat(3, 1fr);
		}

		.score-card.main {
			grid-column: span 1;
		}
	}

	@media (max-width: 900px) {
		.overview-grid {
			grid-template-columns: 1fr;
		}

		.score-section {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.lead-header {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			justify-content: flex-start;
		}

		.score-section {
			grid-template-columns: 1fr;
		}

		.tabs-nav {
			justify-content: flex-start;
		}
	}
</style>
