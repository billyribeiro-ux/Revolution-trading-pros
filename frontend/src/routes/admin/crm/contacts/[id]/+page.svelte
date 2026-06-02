<!--
	/admin/crm/contacts/[id] - Contact Detail Profile

	R15-C extraction pass (May 2026): page-shell only — every section
	now lives in _components/. See /tmp/maint-reports/R15-C-extractions.md.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import IconArrowLeft from '@tabler/icons-svelte-runes/icons/arrow-left';
	import IconUserCircle from '@tabler/icons-svelte-runes/icons/user-circle';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconActivity from '@tabler/icons-svelte-runes/icons/activity';
	import IconNotes from '@tabler/icons-svelte-runes/icons/notes';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { api } from '$lib/api/config';
	import { logger } from '$lib/utils/logger';

	import ContactHeader from './_components/ContactHeader.svelte';
	import ContactStatsGrid from './_components/ContactStatsGrid.svelte';
	import OverviewPanel from './_components/OverviewPanel.svelte';
	import EmailsPanel from './_components/EmailsPanel.svelte';
	import NotesPanel from './_components/NotesPanel.svelte';
	import ActivityPanel from './_components/ActivityPanel.svelte';
	import PickerModal from './_components/PickerModal.svelte';
	import AddNoteModal from './_components/AddNoteModal.svelte';
	import SendEmailModal from './_components/SendEmailModal.svelte';
	import type {
		Contact,
		TimelineEvent,
		EmailActivity,
		Note,
		TagOption,
		ListOption,
		EmailTemplateOption
	} from './_components/types';

	// STATE

	let contact = $state<Contact | null>(null);
	let timeline = $state<TimelineEvent[]>([]);
	let emailHistory = $state<EmailActivity[]>([]);
	let notes = $state<Note[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);
	let activeTab = $state<'overview' | 'emails' | 'notes' | 'activity'>('overview');

	// Delete confirmation modal state
	let showDeleteModal = $state(false);

	// Modal states
	let showAddTagModal = $state(false);
	let showAddListModal = $state(false);
	let showAddNoteModal = $state(false);
	let showSendEmailModal = $state(false);
	let newNoteContent = $state('');
	let availableTags = $state<TagOption[]>([]);
	let availableLists = $state<ListOption[]>([]);

	// Send Email Modal State
	let availableEmailTemplates = $state<EmailTemplateOption[]>([]);
	let emailSubject = $state('');
	let emailBody = $state('');
	let selectedTemplateId = $state<string | number | null>(null);
	let sendingEmail = $state(false);
	let loadingTemplates = $state(false);

	// Toast notification state (inline for component isolation)
	let toastMessage = $state<{ type: 'success' | 'error'; text: string } | null>(null);

	let contactId = $derived(page.params.id as string);

	// API FUNCTIONS

	async function loadContact() {
		loading = true;
		error = null;
		try {
			type ContactRes = { data?: Contact } | Contact;
			type TimelineRes = { data?: TimelineEvent[] } | TimelineEvent[];
			type EmailsRes = { data?: EmailActivity[] } | EmailActivity[];
			type NotesRes = { data?: Note[] } | Note[];
			const [contactRes, timelineRes, emailsRes, notesRes] = await Promise.allSettled([
				api.get<ContactRes>(`/api/admin/crm/contacts/${contactId}`),
				api.get<TimelineRes>(`/api/admin/crm/contacts/${contactId}/timeline`),
				api.get<EmailsRes>(`/api/admin/crm/contacts/${contactId}/emails`),
				api.get<NotesRes>(`/api/admin/crm/contacts/${contactId}/notes`)
			]);

			if (contactRes.status === 'fulfilled') {
				const v = contactRes.value;
				contact = v && 'data' in v && v.data ? v.data : (v as Contact);
			} else {
				throw new Error('Failed to load contact');
			}

			if (timelineRes.status === 'fulfilled') {
				const v = timelineRes.value;
				timeline = Array.isArray(v) ? v : v?.data || [];
			}

			if (emailsRes.status === 'fulfilled') {
				const v = emailsRes.value;
				emailHistory = Array.isArray(v) ? v : v?.data || [];
			}

			if (notesRes.status === 'fulfilled') {
				const v = notesRes.value;
				notes = Array.isArray(v) ? v : v?.data || [];
			}
		} catch (e) {
			logger.error('Failed to load contact', { error: e });
			error = 'Failed to load contact. Please try again.';
		} finally {
			loading = false;
		}
	}

	async function loadAvailableTagsAndLists() {
		try {
			type TagsRes = { data?: TagOption[] } | TagOption[];
			type ListsRes = { data?: ListOption[] } | ListOption[];
			const [tagsRes, listsRes] = await Promise.allSettled([
				api.get<TagsRes>('/api/admin/crm/tags'),
				api.get<ListsRes>('/api/admin/crm/lists')
			]);

			if (tagsRes.status === 'fulfilled') {
				const v = tagsRes.value;
				availableTags = Array.isArray(v) ? v : v?.data || [];
			}

			if (listsRes.status === 'fulfilled') {
				const v = listsRes.value;
				availableLists = Array.isArray(v) ? v : v?.data || [];
			}
		} catch (e) {
			logger.error('Failed to load tags/lists', { error: e });
		}
	}

	async function addTag(tagId: string) {
		try {
			await api.post(`/api/admin/crm/contacts/${contactId}/tags`, { tag_id: tagId });
			await loadContact();
			showAddTagModal = false;
		} catch (e) {
			logger.error('Failed to add tag', { error: e });
		}
	}

	async function removeTag(tagId: string) {
		try {
			await api.delete(`/api/admin/crm/contacts/${contactId}/tags/${tagId}`);
			await loadContact();
		} catch (e) {
			logger.error('Failed to remove tag', { error: e });
		}
	}

	async function addToList(listId: string) {
		try {
			await api.post(`/api/admin/crm/contacts/${contactId}/lists`, { list_id: listId });
			await loadContact();
			showAddListModal = false;
		} catch (e) {
			logger.error('Failed to add to list', { error: e });
		}
	}

	async function removeFromList(listId: string) {
		try {
			await api.delete(`/api/admin/crm/contacts/${contactId}/lists/${listId}`);
			await loadContact();
		} catch (e) {
			logger.error('Failed to remove from list', { error: e });
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
			logger.error('Failed to add note', { error: e });
		}
	}

	function deleteContact() {
		showDeleteModal = true;
	}

	async function confirmDeleteContact() {
		showDeleteModal = false;
		try {
			await api.delete(`/api/admin/crm/contacts/${contactId}`);
			goto('/admin/crm');
		} catch (e) {
			logger.error('Failed to delete contact', { error: e });
		}
	}

	// SEND EMAIL FUNCTIONS

	async function loadEmailTemplates(): Promise<void> {
		if (availableEmailTemplates.length > 0) return; // Cache check
		loadingTemplates = true;
		try {
			type TemplateRow = {
				id: string | number;
				name?: string;
				title?: string;
				subject?: string;
				body_html?: string;
				body?: string;
				body_text?: string;
			};
			type TemplatesRes = { data?: TemplateRow[] } | TemplateRow[];
			const response = await api.get<TemplatesRes>('/api/admin/email/templates');
			const templates = Array.isArray(response) ? response : response?.data || [];
			availableEmailTemplates = templates.map((t) => ({
				id: t.id,
				name: t.name || t.title || 'Untitled Template',
				subject: t.subject || '',
				body_html: t.body_html || t.body || '',
				body_text: t.body_text || ''
			}));
		} catch (e) {
			logger.error('Failed to load email templates', { error: e });
			availableEmailTemplates = [];
		} finally {
			loadingTemplates = false;
		}
	}

	function handleTemplateSelect(templateId: string | number | null): void {
		selectedTemplateId = templateId;
		if (!templateId) return;

		const template = availableEmailTemplates.find((t) => String(t.id) === String(templateId));
		if (template) {
			// Only auto-populate if fields are empty (preserve user edits)
			if (!emailSubject.trim()) {
				emailSubject = template.subject || '';
			}
			if (!emailBody.trim()) {
				// Prefer plain text for textarea, fallback to HTML
				emailBody = template.body_text || template.body_html || '';
			}
		}
	}

	async function sendEmail(): Promise<void> {
		// Validation
		if (!contact?.email) {
			showToast('error', 'Contact does not have a valid email address');
			return;
		}
		if (!emailSubject.trim()) {
			showToast('error', 'Please enter an email subject');
			return;
		}
		if (!emailBody.trim()) {
			showToast('error', 'Please enter an email body');
			return;
		}

		sendingEmail = true;
		try {
			const payload = {
				contact_id: contactId,
				to_email: contact.email,
				subject: emailSubject.trim(),
				body: emailBody.trim(),
				body_html: emailBody.trim(), // Fallback for APIs expecting HTML
				template_id: selectedTemplateId || undefined
			};

			await api.post(`/api/admin/crm/contacts/${contactId}/send-email`, payload);

			showToast('success', `Email sent successfully to ${contact.email}`);
			resetEmailForm();
			showSendEmailModal = false;

			// Refresh email history
			await loadContact();
		} catch (e: unknown) {
			const errorMessage =
				e instanceof Error ? e.message : 'Failed to send email. Please try again.';
			logger.error('Failed to send email', { error: e });
			showToast('error', errorMessage);
		} finally {
			sendingEmail = false;
		}
	}

	function resetEmailForm(): void {
		emailSubject = '';
		emailBody = '';
		selectedTemplateId = null;
	}

	function openSendEmailModal(): void {
		resetEmailForm();
		showSendEmailModal = true;
		loadEmailTemplates();
	}

	function closeSendEmailModal(): void {
		showSendEmailModal = false;
		resetEmailForm();
	}

	function showToast(type: 'success' | 'error', text: string): void {
		toastMessage = { type, text };
		setTimeout(() => {
			toastMessage = null;
		}, 5000);
	}

	function goBack() {
		goto('/admin/crm');
	}

	// LIFECYCLE

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
		<ContactHeader
			{contact}
			{contactId}
			onRefresh={loadContact}
			onSendEmail={openSendEmailModal}
			onDelete={deleteContact}
		/>

		<ContactStatsGrid {contact} />

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
				class:active={activeTab === 'emails'}
				onclick={() => (activeTab = 'emails')}
			>
				<IconMail size={18} />
				Emails ({emailHistory.length})
			</button>
			<button
				class="tab-btn"
				class:active={activeTab === 'notes'}
				onclick={() => (activeTab = 'notes')}
			>
				<IconNotes size={18} />
				Notes ({notes.length})
			</button>
			<button
				class="tab-btn"
				class:active={activeTab === 'activity'}
				onclick={() => (activeTab = 'activity')}
			>
				<IconActivity size={18} />
				Activity ({timeline.length})
			</button>
		</nav>

		<!-- Tab Content - Layout Shift Free Pattern -->
		<div class="tab-content">
			<div
				class="tab-panel"
				class:active={activeTab === 'overview'}
				inert={activeTab !== 'overview' ? true : undefined}
			>
				<OverviewPanel
					{contact}
					onAddTag={() => (showAddTagModal = true)}
					onRemoveTag={removeTag}
					onAddList={() => (showAddListModal = true)}
					onRemoveFromList={removeFromList}
				/>
			</div>

			<div
				class="tab-panel"
				class:active={activeTab === 'emails'}
				inert={activeTab !== 'emails' ? true : undefined}
			>
				<EmailsPanel emails={emailHistory} />
			</div>

			<div
				class="tab-panel"
				class:active={activeTab === 'notes'}
				inert={activeTab !== 'notes' ? true : undefined}
			>
				<NotesPanel {notes} onAddNote={() => (showAddNoteModal = true)} />
			</div>

			<div
				class="tab-panel"
				class:active={activeTab === 'activity'}
				inert={activeTab !== 'activity' ? true : undefined}
			>
				<ActivityPanel {timeline} />
			</div>
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

<PickerModal
	open={showAddTagModal}
	kind="tag"
	items={availableTags}
	onClose={() => (showAddTagModal = false)}
	onSelect={addTag}
/>

<PickerModal
	open={showAddListModal}
	kind="list"
	items={availableLists}
	onClose={() => (showAddListModal = false)}
	onSelect={addToList}
/>

<AddNoteModal
	open={showAddNoteModal}
	bind:content={newNoteContent}
	onClose={() => (showAddNoteModal = false)}
	onSave={addNote}
/>

<SendEmailModal
	open={showSendEmailModal}
	{contact}
	templates={availableEmailTemplates}
	{loadingTemplates}
	sending={sendingEmail}
	bind:subject={emailSubject}
	bind:body={emailBody}
	{selectedTemplateId}
	onClose={closeSendEmailModal}
	onSend={sendEmail}
	onTemplateChange={handleTemplateSelect}
/>

<!-- Toast Notification -->
{#if toastMessage}
	<div class="toast toast-{toastMessage.type}" role="alert" aria-live="polite">
		{#if toastMessage.type === 'success'}
			<IconCheck size={18} />
		{:else}
			<IconX size={18} />
		{/if}
		<span>{toastMessage.text}</span>
		<button class="toast-close" onclick={() => (toastMessage = null)} aria-label="Dismiss">
			<IconX size={14} />
		</button>
	</div>
{/if}

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Contact"
	message="Are you sure you want to delete this contact? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteContact}
	onCancel={() => {
		showDeleteModal = false;
	}}
/>

<style>
	.contact-detail-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 24px;
	}

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
		color: var(--primary-500);
		border-color: var(--primary-500);
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
		background: rgba(230, 184, 0, 0.1);
	}

	.tab-btn.active {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
	}

	/* Tab Content - Layout Shift Prevention */
	.tab-content {
		position: relative;
		min-height: 400px;
		contain: layout style;
		isolation: isolate;
	}

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

	/* Empty State (for "Contact not found") */
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
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(230, 184, 0, 0.4);
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
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
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
		background: var(--primary-500);
		color: white;
		border: none;
		border-radius: 8px;
		cursor: pointer;
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

	.toast-success :global(svg) {
		color: #34d399;
	}

	.toast-error {
		background: linear-gradient(135deg, #7f1d1d, #991b1b);
		border: 1px solid #f87171;
		color: #fef2f2;
	}

	.toast-error :global(svg) {
		color: #fca5a5;
	}

	.toast-close {
		display: flex;
		padding: 4px;
		margin-left: 8px;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 6px;
		color: inherit;
		cursor: pointer;
		transition: background 0.2s;
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

	@media (max-width: 639.98px) {
		.tabs-nav {
			justify-content: flex-start;
		}
	}

	@media (max-width: 479.98px) {
		.toast {
			left: 16px;
			right: 16px;
			bottom: 16px;
		}
	}
</style>
