<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { crmAPI } from '#lib/api/crm.js';
	import { getCrmNativeTemplatePicker } from '#lib/api/crm-native-template-picker.js';
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	import { contactNumericId, contactMessageActivity } from '#lib/api/crm-contact-contract.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type {
		ContactRecord,
		ContactTimelinePage,
		ContactPage,
		ContactNote,
		ContactEmailActivity,
		ContactTag,
		ContactListMembership,
		ContactMessageIntent,
		ContactMessageReceipt
	} from '#lib/crm/contact-types.js';
	import type { TransferOptionPage } from '#lib/api/crm-transfer-contract.js';
	import type { EmailTemplateOption } from './types';
	import ContactHeader from './ContactHeader.svelte';
	import ContactStatsGrid from './ContactStatsGrid.svelte';
	import OverviewPanel from './OverviewPanel.svelte';
	import EmailsPanel from './EmailsPanel.svelte';
	import NotesPanel from './NotesPanel.svelte';
	import ActivityPanel from './ActivityPanel.svelte';
	import PickerModal from './PickerModal.svelte';
	import AddNoteModal from './AddNoteModal.svelte';
	import SendEmailModal from './SendEmailModal.svelte';
	let { contactId, back = '/admin/crm' }: { contactId: string; back?: string } = $props();
	const access = useCrmAccess();
	const denied = 'Your current CRM permissions do not allow this operation.';
	const rights = $derived({
		contact: access.can('GET', '/contacts/{id}'),
		timeline: access.can('GET', '/contacts/{id}/timeline'),
		notes: access.can('GET', '/contacts/{id}/notes'),
		emails: access.can('GET', '/contacts/{id}/emails'),
		tags: access.can('GET', '/contacts/{id}/tags'),
		lists: access.can('GET', '/contacts/{id}/lists'),
		edit: access.can('PUT', '/contacts/{id}'),
		delete: access.can('DELETE', '/contacts/{id}'),
		recalculate: access.can('POST', '/contacts/{id}/recalculate-score'),
		note: access.can('POST', '/contacts/{id}/notes'),
		send: access.can('POST', '/contacts/{id}/send-email'),
		templates: access.can('GET', '/contacts/native-templates'),
		tagOptions: access.can('GET', '/imports/options', { kind: 'tags' }),
		listOptions: access.can('GET', '/imports/options', { kind: 'lists' }),
		addTag: access.can('POST', '/contacts/{id}/tags'),
		removeTag: access.can('DELETE', '/contacts/{id}/tags/{tag}'),
		addList: access.can('POST', '/contacts/{id}/lists'),
		removeList: access.can('DELETE', '/contacts/{id}/lists/{list}')
	});
	let contact = $state<ContactRecord | null>(null),
		loading = $state(true),
		error = $state(''),
		notice = $state('');
	let timeline = $state<ContactTimelinePage | null>(null),
		notes = $state<ContactPage<ContactNote> | null>(null),
		emails = $state<ContactPage<ContactEmailActivity> | null>(null),
		tags = $state<ContactTag[]>([]),
		lists = $state<ContactPage<ContactListMembership> | null>(null);
	let timelineError = $state(''),
		notesError = $state(''),
		emailsError = $state(''),
		tagsError = $state(''),
		listsError = $state('');
	let timelineLoading = $state(true),
		notesLoading = $state(true),
		emailsLoading = $state(true),
		tagsLoading = $state(true),
		listsLoading = $state(true);
	let activeTab = $state<'overview' | 'emails' | 'notes' | 'activity'>('overview');
	let busy = $state(false),
		recalculating = $state(false),
		confirmDelete = $state(false);
	let noteOpen = $state(false),
		noteContent = $state(''),
		noteError = $state('');
	let picker = $state<'tag' | 'list' | null>(null),
		options = $state<TransferOptionPage | null>(null),
		optionSearch = $state(''),
		optionError = $state(''),
		optionLoading = $state(false);
	let emailOpen = $state(false),
		sending = $state(false),
		subject = $state(''),
		body = $state(''),
		selectedTemplateId = $state<string | number | null>(null),
		intent = $state<ContactMessageIntent | null>(null),
		receipt = $state<ContactMessageReceipt | null>(null),
		recovered = $state<ContactEmailActivity[]>([]),
		messageError = $state('');
	let selectedTemplate = $state<EmailTemplateOption | null>(null);
	let templates = $state<EmailTemplateOption[]>([]),
		templateLoading = $state(false),
		templateError = $state(''),
		templateSearch = $state(''),
		templatePage = $state(1),
		templatePages = $state(1);
	let disposed = false;
	const generations = {
		contact: 0,
		timeline: 0,
		notes: 0,
		emails: 0,
		tags: 0,
		lists: 0,
		options: 0,
		templates: 0
	};
	const message = (cause: unknown) =>
		apiErrorMessage(cause, 'The contact service could not confirm the operation');
	async function loadContact() {
		if (!rights.contact) { contact = null; loading = false; error = denied; return; }
		const generation = ++generations.contact;
		loading = true;
		error = '';
		try {
			const value = await crmAPI.getContact(contactId);
			if (!disposed && generation === generations.contact) contact = value;
		} catch (cause) {
			if (!disposed && generation === generations.contact) error = message(cause);
		} finally {
			if (!disposed && generation === generations.contact) loading = false;
		}
	}
	async function loadTimeline(page = 1) {
		if (!rights.timeline) { timeline = null; timelineLoading = false; timelineError = denied; return; }
		const generation = ++generations.timeline;
		timelineLoading = true;
		timelineError = '';
		try {
			const value = await crmAPI.getContactTimeline(contactId, page);
			if (!disposed && generation === generations.timeline) timeline = value;
		} catch (cause) {
			if (!disposed && generation === generations.timeline) timelineError = message(cause);
		} finally {
			if (!disposed && generation === generations.timeline) timelineLoading = false;
		}
	}
	async function loadNotes(page = 1) {
		if (!rights.notes) { notes = null; notesLoading = false; notesError = denied; return; }
		const generation = ++generations.notes;
		notesLoading = true;
		notesError = '';
		try {
			const value = await crmAPI.getContactNotes(contactId, page);
			if (!disposed && generation === generations.notes) notes = value;
		} catch (cause) {
			if (!disposed && generation === generations.notes) notesError = message(cause);
		} finally {
			if (!disposed && generation === generations.notes) notesLoading = false;
		}
	}
	async function loadEmails(page = 1) {
		if (!rights.emails) { emails = null; emailsLoading = false; emailsError = denied; return; }
		const generation = ++generations.emails;
		emailsLoading = true;
		emailsError = '';
		try {
			const value = await crmAPI.getContactEmails(contactId, page);
			if (!disposed && generation === generations.emails) emails = value;
		} catch (cause) {
			if (!disposed && generation === generations.emails) emailsError = message(cause);
		} finally {
			if (!disposed && generation === generations.emails) emailsLoading = false;
		}
	}
	async function loadTags() {
		if (!rights.tags) { tags = []; tagsLoading = false; tagsError = denied; return; }
		const generation = ++generations.tags;
		tagsLoading = true;
		tagsError = '';
		try {
			const value = await crmAPI.getContactTagAssignments(contactId);
			if (!disposed && generation === generations.tags) tags = value;
		} catch (cause) {
			if (!disposed && generation === generations.tags) tagsError = message(cause);
		} finally {
			if (!disposed && generation === generations.tags) tagsLoading = false;
		}
	}
	async function loadLists(page = 1) {
		if (!rights.lists) { lists = null; listsLoading = false; listsError = denied; return; }
		const generation = ++generations.lists;
		listsLoading = true;
		listsError = '';
		try {
			const value = await crmAPI.getContactListMemberships(contactId, page);
			if (!disposed && generation === generations.lists) lists = value;
		} catch (cause) {
			if (!disposed && generation === generations.lists) listsError = message(cause);
		} finally {
			if (!disposed && generation === generations.lists) listsLoading = false;
		}
	}
	async function refresh() {
		await Promise.all([
			loadContact(),
			loadTimeline(),
			loadNotes(),
			loadEmails(),
			loadTags(),
			loadLists()
		]);
	}
	async function recalculate() {
		if (busy || recalculating) return;
		if (!rights.recalculate) { notice = denied; return; }
		recalculating = true;
		notice = '';
		try {
			const value = await crmAPI.recalculateContactScore(contactId);
			if (disposed) return;
			contact = { ...value, facts: contact?.facts };
			notice = `Scoring result: ${value.scoring.status.replaceAll('_', ' ')}.`;
			await loadTimeline();
		} catch (cause) {
			if (!disposed) notice = message(cause);
		} finally {
			if (!disposed) recalculating = false;
		}
	}
	async function loadOptions(page = 1) {
		if (!picker) return;
		if (!(picker === 'tag' ? rights.tagOptions : rights.listOptions)) { options = null; optionLoading = false; optionError = denied; return; }
		const kind = picker,
			generation = ++generations.options;
		optionLoading = true;
		optionError = '';
		try {
			const value = await crmAPI.getTransferOptions(kind === 'tag' ? 'tags' : 'lists', {
				search: optionSearch,
				page,
				per_page: 25
			});
			if (!disposed && picker === kind && generation === generations.options) options = value;
		} catch (cause) {
			if (!disposed && generation === generations.options) optionError = message(cause);
		} finally {
			if (!disposed && generation === generations.options) optionLoading = false;
		}
	}
	function openPicker(kind: 'tag' | 'list') {
		if (!(kind === 'tag' ? rights.addTag && rights.tagOptions : rights.addList && rights.listOptions)) { notice = denied; return; }
		picker = kind;
		options = null;
		optionSearch = '';
		void loadOptions();
	}
	async function changeMembership(kind: 'tag' | 'list', id: string, action: 'add' | 'remove') {
		const permitted = kind === 'tag' ? (action === 'add' ? rights.addTag : rights.removeTag) : (action === 'add' ? rights.addList : rights.removeList);
		if (!permitted) { notice = denied; optionError = denied; return; }
		if (busy) return;
		busy = true;
		notice = '';
		optionError = '';
		try {
			const count = await crmAPI.changeContactMembership(contactId, kind, id, action);
			if (disposed) return;
			notice = `${kind === 'tag' ? 'Tag' : 'List'} ${action === 'add' ? 'assignment' : 'removal'} confirmed: ${count} changed.`;
			picker = null;
			await Promise.all([kind === 'tag' ? loadTags() : loadLists(), loadContact(), loadTimeline()]);
		} catch (cause) {
			if (!disposed) {
				notice = message(cause);
				optionError = notice;
			}
		} finally {
			if (!disposed) busy = false;
		}
	}
	async function saveNote() {
		if (busy || !noteContent.trim()) return;
		if (!rights.note) { noteError = denied; return; }
		busy = true;
		noteError = '';
		try {
			await crmAPI.addContactNote(contactId, noteContent);
			if (disposed) return;
			noteContent = '';
			noteOpen = false;
			notice = 'Note saved and confirmed.';
			await Promise.all([loadNotes(), loadTimeline()]);
		} catch (cause) {
			if (!disposed) noteError = message(cause);
		} finally {
			if (!disposed) busy = false;
		}
	}
	async function removeContact() {
		if (busy) return;
		if (!rights.delete) { notice = denied; return; }
		busy = true;
		notice = '';
		try {
			await crmAPI.deleteContact(contactId);
			if (!disposed) await goto(back, { refreshAll: true });
		} catch (cause) {
			if (!disposed) notice = message(cause);
		} finally {
			if (!disposed) {
				busy = false;
				confirmDelete = false;
			}
		}
	}
	async function loadTemplates(page = 1) {
		if (!rights.templates) { templates = []; templateLoading = false; templateError = 'Native template access is not permitted. You can compose a custom message.'; return; }
		const generation = ++generations.templates;
		templateLoading = true;
		templateError = '';
		try {
			const value = await getCrmNativeTemplatePicker({ search: templateSearch, page, per_page: 25 });
			if (disposed || generation !== generations.templates) return;
			templates = value.data.map((template) => ({
				id: template.id,
				name: template.name,
				subject: template.subject,
				body_html: template.body_html,
				body_text: template.body_text ?? undefined
			}));
			if (selectedTemplate && !templates.some((item) => item.id === selectedTemplate?.id))
				templates = [selectedTemplate, ...templates];
			templatePage = value.meta.current_page;
			templatePages = value.meta.last_page;
		} catch (cause) {
			if (!disposed && generation === generations.templates) templateError = message(cause);
		} finally {
			if (!disposed && generation === generations.templates) templateLoading = false;
		}
	}
	function selectTemplate(id: string | number | null) {
		if (intent || (id !== null && !rights.templates)) return;
		selectedTemplateId = id;
		const template = templates.find((item) => item.id === String(id));
		selectedTemplate = template ?? null;
		if (!template) return;
		if (!subject.trim()) subject = template.subject;
		if (!body.trim()) body = template.body_html;
	}
	function openEmail() {
		if (!rights.send) { notice = denied; return; }
		emailOpen = true;
		if (!intent && rights.templates) void loadTemplates();
	}
	function newMessage() {
		if (sending) return;
		if (!rights.send) { messageError = denied; return; }
		intent = null;
		receipt = null;
		recovered = [];
		messageError = '';
		subject = '';
		body = '';
		selectedTemplateId = null;
		selectedTemplate = null;
		if (rights.templates) void loadTemplates();
	}
	async function sendMessage() {
		if (sending || !contact || !subject.trim() || !body.trim()) return;
		if (!rights.send || ((intent?.template_id || selectedTemplateId) && !rights.templates)) { messageError = denied; return; }
		sending = true;
		messageError = '';
		try {
			if (!intent)
				intent = {
					request_id: crypto.randomUUID(),
					expected_email: contact.email.trim().toLowerCase(),
					subject,
					body_html: body,
					...(selectedTemplateId ? { template_id: contactNumericId(selectedTemplateId) } : {})
				};
			const value = await crmAPI.sendContactMessage(contactId, intent);
			if (disposed) return;
			receipt = value;
			await Promise.all([loadEmails(), loadTimeline()]);
		} catch (cause) {
			if (!disposed) messageError = message(cause);
		} finally {
			if (!disposed) sending = false;
		}
	}
	async function refreshMessage() {
		if (!intent || sending) return;
		if (!rights.emails) { messageError = denied; return; }
		sending = true;
		messageError = '';
		try {
			const value = await crmAPI.getContactEmails(contactId, 1, intent.request_id);
			if (disposed) return;
			for (const activity of value.data)
				contactMessageActivity(activity, intent, receipt?.source_hash);
			recovered = value.data;
			if (value.data.length === 0)
				messageError =
					'No saved submission was found for this request. A retry keeps the same frozen message intent.';
			await loadEmails();
		} catch (cause) {
			if (!disposed) messageError = message(cause);
		} finally {
			if (!disposed) sending = false;
		}
	}
	onMount(() => {
		void refresh();
		return () => {
			disposed = true;
		};
	});
</script>

<div class="profile">
	<a href={back}>← Back to contacts</a>
	{#if notice}<p role="status">{notice}</p>{/if}
	{#if !rights.contact}<p>{denied}</p>{:else if loading && !contact}<p role="status">Loading contact…</p>{:else if error}<p role="alert">
			{error}
		</p>
		<button onclick={() => void loadContact()}>Retry contact</button>{:else if contact}
		<ContactHeader
			{contact}
			{contactId}
			canEdit={rights.edit}
			canDelete={rights.delete}
			canSend={rights.send}
			onRefresh={() => void refresh()}
			onSendEmail={openEmail}
			onDelete={() => {
				confirmDelete = true;
			}}
		/>
		<ContactStatsGrid {contact} {recalculating} canRecalculate={rights.recalculate} onRecalculate={() => void recalculate()} />
		{#if confirmDelete}<section aria-label="Confirm contact deletion">
				<p>Delete {contact.email} and its recorded contact links?</p>
				<button disabled={busy || !rights.delete} onclick={() => void removeContact()}>Confirm delete contact</button
				><button
					disabled={busy}
					onclick={() => {
						confirmDelete = false;
					}}>Cancel deletion</button
				>
			</section>{/if}
		<nav class="tabs" aria-label="Contact sections">
			{#each ['overview', 'emails', 'notes', 'activity'] as tab (tab)}<button
					class={{ active: activeTab === tab }}
					onclick={() => {
						if (tab === 'overview' || tab === 'emails' || tab === 'notes' || tab === 'activity')
							activeTab = tab;
					}}>{tab[0].toUpperCase() + tab.slice(1)}</button
				>{/each}
		</nav>
		{#if activeTab === 'overview'}<OverviewPanel
				{contact}
				{tags}
				{lists}
				{tagsError}
				{listsError}
				{tagsLoading}
				{listsLoading}
				{busy}
				canReadTags={rights.tags}
				canReadLists={rights.lists}
				canAddTag={rights.addTag && rights.tagOptions}
				canRemoveTag={rights.removeTag}
				canAddList={rights.addList && rights.listOptions}
				canRemoveList={rights.removeList}
				onAddTag={() => openPicker('tag')}
				onRemoveTag={(id) => void changeMembership('tag', id, 'remove')}
				onAddList={() => openPicker('list')}
				onRemoveFromList={(id) => void changeMembership('list', id, 'remove')}
				onRefreshTags={() => void loadTags()}
				onRefreshLists={() => void loadLists()}
				onListsPage={(page) => void loadLists(page)}
			/>
		{:else if activeTab === 'emails' && !rights.emails}<p>Contact email history is not permitted.</p>
		{:else if activeTab === 'emails'}<EmailsPanel
				emails={emails?.data ?? []}
				error={emailsError}
				loading={emailsLoading}
				meta={emails?.meta ?? null}
				onPage={(page) => void loadEmails(page)}
				onRefresh={() => void loadEmails()}
			/>
		{:else if activeTab === 'notes' && !rights.notes}<p>Contact notes are not permitted.</p>
		{:else if activeTab === 'notes'}<NotesPanel
				canAddNote={rights.note}
				notes={notes?.data ?? []}
				error={notesError}
				loading={notesLoading}
				meta={notes?.meta ?? null}
				onPage={(page) => void loadNotes(page)}
				onRefresh={() => void loadNotes()}
				onAddNote={() => {
					if (!rights.note) return;
					noteOpen = true;
					noteError = '';
				}}
			/>
		{:else if !rights.timeline}<p>Contact activity is not permitted.</p>{:else}<ActivityPanel
				timeline={timeline?.data ?? []}
				error={timelineError}
				loading={timelineLoading}
				meta={timeline?.meta ?? null}
				onPage={(page) => void loadTimeline(page)}
				onRefresh={() => void loadTimeline()}
			/>{#if timeline}<p>
					Timeline coverage: {timeline.coverage.sources.join(', ')}. Unlinked address history and
					human engagement are not measured.
				</p>{/if}{/if}
	{/if}
</div>
<PickerModal
	open={picker !== null && (picker === 'tag' ? rights.addTag && rights.tagOptions : rights.addList && rights.listOptions)}
	kind={picker ?? 'tag'}
	items={options?.data.map((item) => ({ id: item.id, name: item.title })) ?? []}
	{busy}
	loading={optionLoading}
	error={optionError}
	bind:search={optionSearch}
	page={options?.meta.current_page ?? 1}
	pages={options?.meta.total_pages ?? 1}
	total={options?.meta.total ?? 0}
	onSearch={() => void loadOptions()}
	onPage={(page) => void loadOptions(page)}
	onClose={() => {
		if (!busy) {
			picker = null;
			generations.options++;
			optionLoading = false;
		}
	}}
	onSelect={(id) => {
		if (picker) void changeMembership(picker, id, 'add');
	}}
/>
<AddNoteModal
	open={noteOpen && rights.note}
	{busy}
	error={noteError}
	bind:content={noteContent}
	onClose={() => {
		if (!busy) noteOpen = false;
	}}
	onSave={() => void saveNote()}
/>
<SendEmailModal
	open={emailOpen}
	canUseTemplates={rights.templates}
	canSend={rights.send && (!(intent?.template_id || selectedTemplateId) || rights.templates)}
	canRefreshReceipt={rights.emails}
	{contact}
	{templates}
	loadingTemplates={templateLoading}
	{sending}
	locked={intent !== null}
	error={messageError}
	{templateError}
	bind:templateSearch
	{templatePage}
	{templatePages}
	{receipt}
	{recovered}
	bind:subject
	bind:body
	{selectedTemplateId}
	onClose={() => {
		if (!sending) emailOpen = false;
	}}
	onSend={() => void sendMessage()}
	onTemplateChange={selectTemplate}
	onRefreshReceipt={() => void refreshMessage()}
	onNewMessage={newMessage}
	onTemplateSearch={() => void loadTemplates()}
	onTemplatePage={(page) => void loadTemplates(page)}
/>

<style>
	.profile {
		max-width: 90rem;
		margin: auto;
		padding: 1.5rem;
		color: #e2e8f0;
	}
	a {
		color: #7dd3fc;
	}
	.tabs {
		display: flex;
		gap: 1rem;
		margin: 1rem 0;
	}
	button {
		padding: 0.7rem;
		color: #e2e8f0;
		border: 1px solid #475569;
		background: #0f172a;
		border-radius: 0.5rem;
	}
	.active {
		border-color: #fbbf24;
	}
	[role='alert'] {
		color: #fca5a5;
	}
</style>
