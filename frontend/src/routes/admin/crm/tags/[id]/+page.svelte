<script lang="ts">
	import { afterNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import {
		IconTag,
		IconUsers,
		IconArrowLeft,
		IconSearch,
		IconRefresh,
		IconUser,
		IconX,
		IconChevronLeft,
		IconChevronRight
	} from '#lib/icons/index.js';
	import { crmAPI } from '#lib/api/crm.js';
	import type { ContactTag, Contact } from '#lib/crm/types.js';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';

	// Get tag ID from route params
	let tagId = $derived(page.params.id ?? '');

	// State
	let tag = $state<ContactTag | null>(null);
	let contacts = $state<Contact[]>([]);
	let isLoading = $state(true);
	let isLoadingContacts = $state(false);
	let error = $state('');
	let searchQuery = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);
	let perPage = $state(20);
	let loadedTagId = '';

	// Remove tag confirmation modal state
	let showRemoveTagModal = $state(false);
	let pendingRemoveContactId = $state<string | null>(null);

	// Computed contacts count
	let contactsCount = $state(0);

	// Filter contacts locally
	let filteredContacts = $derived(
		contacts.filter((contact) => {
			if (!searchQuery) return true;
			const query = searchQuery.toLowerCase();

			return (
				contact.email.toLowerCase().includes(query) ||
				contact.full_name.toLowerCase().includes(query) ||
				(contact.phone && contact.phone.includes(query))
			);
		})
	);

	async function loadTag() {
		isLoading = true;
		error = '';

		try {
			const response = await crmAPI.getContactTag(tagId);
			tag = response.tag;
			contactsCount = response.contacts_count;
			await loadContacts();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load tag';
		} finally {
			isLoading = false;
		}
	}

	async function loadContacts() {
		isLoadingContacts = true;

		try {
			const response = await crmAPI.getTagContacts(tagId, { per_page: perPage, page: currentPage });
			contacts = response.data || [];
			if (response.meta) {
				totalPages = Math.ceil(response.meta.total / perPage) || 1;
			}
		} catch (err) {
			console.error('Failed to load contacts:', err);
		} finally {
			isLoadingContacts = false;
		}
	}

	function removeTagFromContact(contactId: string) {
		pendingRemoveContactId = contactId;
		showRemoveTagModal = true;
	}

	async function confirmRemoveTagFromContact() {
		if (!pendingRemoveContactId) return;
		showRemoveTagModal = false;
		const contactId = pendingRemoveContactId;
		pendingRemoveContactId = null;

		try {
			await crmAPI.removeTagFromContacts(tagId, [contactId]);
			await loadTag();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to remove tag';
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getInitials(name: string): string {
		if (!name) return 'U';

		return (
			name
				.split(' ')
				.filter((n) => n.length > 0)
				.map((n) => n[0])
				.join('')
				.toUpperCase()
				.slice(0, 2) || 'U'
		);
	}

	function getStatusColor(status: string): string {
		const colors: Record<string, string> = {
			lead: '#3b82f6',
			prospect: '#8b5cf6',
			customer: '#22c55e',
			churned: '#ef4444',
			unqualified: '#64748b'
		};
		return colors[status] || '#64748b';
	}

	// Load tag on initial mount and when SvelteKit reuses this page for another tag id.
	afterNavigate(({ shallow }) => {
		if (shallow) return;

		if (tagId && tagId !== loadedTagId) {
			loadedTagId = tagId;
			void loadTag();
		}
	});
</script>

<!--
	/admin/crm/tags/[id] - Tag Detail & Contacts View
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- View tag details
	- List contacts with this tag
	- Remove tag from contacts
	- Search/filter contacts
	- Full Svelte 5 $state/$derived/$effect reactivity
-->
<svelte:head><title>{tag?.title || 'Tag'} - Contact Tags - FluentCRM Pro</title></svelte:head>

<div class="tag-detail-page">
	<!-- Back Button & Header -->
	<div class="page-header">
		<div class="header-left">
			<a href="/admin/crm/tags" class="back-link">
				<IconArrowLeft size={20} />
				Back to Tags
			</a>
			{#if tag}
				<div class="tag-header-info">
					<div class="tag-badge" style:background-color={tag.color || '#6366f1'}>
						<IconTag size={20} />
					</div>
					<div>
						<h1>{tag.title}</h1>
						{#if tag.description}
							<p class="tag-desc">{tag.description}</p>
						{/if}
					</div>
				</div>
			{/if}
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadTag()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
		</div>
	</div>

	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading tag...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadTag()}>Try Again</button>
		</div>
	{:else if tag}
		<!-- Tag Stats -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-icon blue">
					<IconUsers size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{contactsCount.toLocaleString()}</span>
					<span class="stat-label">Tagged Contacts</span>
				</div>
			</div>
			<div class="stat-card">
				<div class="stat-icon purple">
					<IconTag size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{formatDate(tag.created_at)}</span>
					<span class="stat-label">Created</span>
				</div>
			</div>
		</div>

		<!-- Contacts Section -->
		<div class="contacts-section">
			<div class="section-header">
				<h2>Contacts with this Tag</h2>
				<div class="search-box">
					<IconSearch size={18} />
					<input autocomplete="off"
						id="page-searchquery"
						name="page-searchquery"
						type="text"
						placeholder="Search contacts..."
						bind:value={searchQuery}
					/>
				</div>
			</div>

			{#if isLoadingContacts}
				<div class="loading-state compact">
					<div class="spinner small"></div>
					<p>Loading contacts...</p>
				</div>
			{:else if filteredContacts.length === 0}
				<div class="empty-state">
					<IconUsers size={48} />
					<h3>No contacts found</h3>
					<p>{searchQuery ? 'Try a different search term' : 'No contacts have this tag yet'}</p>
				</div>
			{:else}
				<div class="contacts-table">
					<table>
						<thead>
							<tr>
								<th>Contact</th>
								<th>Email</th>
								<th>Status</th>
								<th>Phone</th>
								<th>Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each filteredContacts as contact (contact.id)}
								<tr>
									<td>
										<div class="contact-cell">
											<div class="contact-avatar">
												{getInitials(contact.full_name)}
											</div>
											<span class="contact-name">{contact.full_name}</span>
										</div>
									</td>

									<td><a href="mailto:{contact.email}" class="email-link">{contact.email}</a></td>

									<td>
										<span
											class="status-badge"
											style:background-color={`${getStatusColor(contact.status)}20`}
											style:color={getStatusColor(contact.status)}
										>
											{contact.status}
										</span>
									</td>
									<td>
										{#if contact.phone}
											<span class="phone">{contact.phone}</span>
										{:else}
											<span class="no-data">-</span>
										{/if}
									</td>
									<td>
										<div class="action-buttons">
											<a
												href="/admin/crm/contacts/{contact.id}"
												class="btn-icon"
												title="View Contact"
											>
												<IconUser size={16} />
											</a>
											<button
												class="btn-icon danger"
												title="Remove Tag"
												onclick={() => removeTagFromContact(contact.id)}
											>
												<IconX size={16} />
											</button>
										</div>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>

				<!-- Pagination -->
				{#if totalPages > 1}
					<div class="pagination">
						<button
							class="page-btn"
							disabled={currentPage === 1}
							onclick={() => {
								currentPage--;
								loadContacts();
							}}
						>
							<IconChevronLeft size={18} />
						</button>
						<span class="page-info">Page {currentPage} of {totalPages}</span>
						<button
							class="page-btn"
							disabled={currentPage === totalPages}
							onclick={() => {
								currentPage++;
								loadContacts();
							}}
						>
							<IconChevronRight size={18} />
						</button>
					</div>
				{/if}
			{/if}
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showRemoveTagModal}
	title="Remove Tag"
	message="Remove this tag from the contact?"
	confirmText="Remove"
	variant="warning"
	onConfirm={confirmRemoveTagFromContact}
	onCancel={() => {
		showRemoveTagModal = false;
		pendingRemoveContactId = null;
	}}
/>

<style>
	.tag-detail-page {
		max-width: 1400px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.header-left {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--text-secondary, #8b949e);
		text-decoration: none;
		font-size: 0.875rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: var(--primary-400, #ffd11a);
	}

	.tag-header-info {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.tag-badge {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #ffffff;
	}

	.tag-header-info h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.tag-desc {
		color: var(--text-tertiary, #6e7681);
		margin: 0;
		font-size: 0.9rem;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-400, #ffd11a);
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
		max-width: 500px;
	}

	@media (max-width: 639.98px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400, #ffd11a);
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
	}

	.contacts-section {
		background: var(--bg-base, #0d1117);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		overflow: hidden;
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
		gap: 1rem;
		flex-wrap: wrap;
	}

	.section-header h2 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		min-width: 250px;
	}

	.search-box :global(svg) {
		color: var(--text-tertiary, #6e7681);
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: var(--text-tertiary, #6e7681);
	}

	.contacts-table {
		overflow-x: auto;
	}

	.contacts-table table {
		width: 100%;
		border-collapse: collapse;
	}

	.contacts-table th {
		padding: 1rem;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: rgba(230, 184, 0, 0.05);
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.contacts-table td {
		padding: 1rem;
		font-size: 0.9rem;
		color: var(--text-primary, #f0f6fc);
		border-bottom: 1px solid rgba(230, 184, 0, 0.05);
	}

	.contacts-table tbody tr:hover {
		background: rgba(230, 184, 0, 0.05);
	}

	.contact-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.contact-avatar {
		width: 36px;
		height: 36px;
		border-radius: 50%;
		background: var(--primary-500);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--bg-base, #0d1117);
		font-size: 0.75rem;
		font-weight: 600;
	}

	.contact-name {
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
	}

	.email-link {
		color: var(--primary-400, #ffd11a);
		text-decoration: none;
		transition: color 0.2s;
	}

	.email-link:hover {
		color: var(--primary-400, #ffd11a);
		text-decoration: underline;
	}

	.status-badge {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.phone {
		color: var(--text-secondary, #8b949e);
	}

	.no-data {
		color: var(--text-tertiary, #6e7681);
	}

	.action-buttons {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-400, #ffd11a);
	}

	.btn-icon.danger:hover {
		background: rgba(248, 81, 73, 0.1);
		color: var(--error-emphasis, #f85149);
		border-color: rgba(248, 81, 73, 0.3);
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 1rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	.page-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.page-btn:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-400, #ffd11a);
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-info {
		font-size: 0.875rem;
		color: var(--text-secondary, #8b949e);
	}

	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: var(--text-tertiary, #6e7681);
	}

	.loading-state.compact {
		padding: 2rem;
	}

	.empty-state :global(svg) {
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	.spinner.small {
		width: 24px;
		height: 24px;
		border-width: 2px;
	}
</style>
