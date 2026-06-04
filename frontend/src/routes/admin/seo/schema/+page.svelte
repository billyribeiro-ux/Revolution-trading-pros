<script lang="ts">
	import { onMount } from 'svelte';
	import { IconPlus, IconCode, IconEye, IconTrash, IconCopy } from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { toastStore } from '$lib/stores/toast.svelte';

	interface SchemaItem {
		id: number;
		schema_type?: string;
		name?: string;
		entity_type?: string;
		entity_id?: string | number;
	}

	// State using Svelte 5 runes
	let schemas = $state<SchemaItem[]>([]);
	let loading = $state(false);
	let showPreview = $state<unknown>(null);

	// Delete confirmation modal state
	let showDeleteSchemaModal = $state(false);
	let pendingDeleteSchemaId = $state<number | null>(null);

	type SchemaTypeOption = {
		name: string;
		deprecated?: boolean;
		note?: string;
	};

	const schemaTypes: SchemaTypeOption[] = [
		{ name: 'Article' },
		{ name: 'Product' },
		{ name: 'Organization' },
		{ name: 'Person' },
		{ name: 'LocalBusiness' },
		{ name: 'Event' },
		{ name: 'Recipe' },
		{ name: 'VideoObject' },
		{
			name: 'FAQPage',
			deprecated: true,
			note: 'Google removed FAQ rich results on May 7, 2026 and limits remaining eligibility to well-known government and health sites.'
		},
		{ name: 'Course' },
		{ name: 'JobPosting' },
		{ name: 'Review' },
		{ name: 'BreadcrumbList' },
		{ name: 'Website' },
		{ name: 'WebPage' }
	];

	onMount(() => {
		loadSchemas();
	});

	async function loadSchemas() {
		loading = true;
		try {
			const response = await fetch('/api/seo/schema');
			const data = await response.json();
			schemas = data.data || [];
		} catch (error) {
			console.error('Failed to load schemas:', error);
		} finally {
			loading = false;
		}
	}

	function deleteSchema(id: number) {
		pendingDeleteSchemaId = id;
		showDeleteSchemaModal = true;
	}

	async function confirmDeleteSchema() {
		const id = pendingDeleteSchemaId;
		if (id == null) return;
		showDeleteSchemaModal = false;
		pendingDeleteSchemaId = null;
		try {
			await fetch(`/api/seo/schema/${id}`, { method: 'DELETE' });
			loadSchemas();
		} catch (error) {
			console.error('Failed to delete schema:', error);
			toastStore.error('Failed to delete schema');
		}
	}

	function cancelDeleteSchema() {
		showDeleteSchemaModal = false;
		pendingDeleteSchemaId = null;
	}

	async function viewJsonLd(id: number) {
		try {
			const response = await fetch(`/api/seo/schema/${id}/json-ld`);
			const data = await response.json();
			showPreview = data.json_ld;
		} catch (error) {
			console.error('Failed to generate JSON-LD:', error);
		}
	}

	function copyJsonLd() {
		navigator.clipboard.writeText(JSON.stringify(showPreview, null, 2));
		toastStore.success('JSON-LD copied to clipboard!');
	}
</script>

<svelte:head>
	<title>Schema Markup | SEO</title>
</svelte:head>

<div class="schema-page">
	<header class="page-header">
		<div>
			<h1>Schema Markup Builder</h1>
			<p>Generate JSON-LD structured data for better search visibility</p>
		</div>
		<a href="/admin/seo/schema/create" class="btn-primary">
			<IconPlus size={18} />
			Add Schema
		</a>
	</header>

	<div class="schema-grid">
		{#if loading}
			<div class="loading">Loading schemas...</div>
		{:else if schemas.length === 0}
			<div class="empty-state">
				<IconCode size={64} />
				<h3>No schema markup created yet</h3>
				<p>Create structured data to help search engines understand your content</p>
				<a href="/admin/seo/schema/create" class="btn-primary">
					<IconPlus size={18} />
					Create Your First Schema
				</a>
			</div>
		{:else}
			{#each schemas as schema (schema.id)}
				<div class="schema-card">
					<div class="card-header">
						<div class="schema-type">{schema.schema_type}</div>
						<div class="actions">
							<button class="action-btn" onclick={() => viewJsonLd(schema.id)} title="View JSON-LD">
								<IconEye size={18} />
							</button>
							<button
								class="action-btn danger"
								onclick={() => deleteSchema(schema.id)}
								title="Delete"
							>
								<IconTrash size={18} />
							</button>
						</div>
					</div>

					<div class="card-body">
						<h4>{schema.name || 'Untitled Schema'}</h4>
						{#if schema.entity_type}
							<div class="meta">
								Applied to: <strong>{schema.entity_type}</strong>
								{#if schema.entity_id}
									(ID: {schema.entity_id})
								{/if}
							</div>
						{/if}
					</div>

					<div class="card-footer">
						<a href="/admin/seo/schema/{schema.id}/edit" class="edit-link"> Edit Schema → </a>
					</div>
				</div>
			{/each}
		{/if}
	</div>

	<div class="templates-section">
		<h2>Available Schema Types</h2>
		<div class="type-grid">
			{#each schemaTypes as type (type.name)}
				<div class="type-card" class:type-card--deprecated={type.deprecated}>
					<div class="type-name">
						{type.name}
						{#if type.deprecated}
							<span class="deprecated-badge" title={type.note}>Deprecated</span>
						{/if}
					</div>
					{#if type.note}
						<p class="type-note">{type.note}</p>
					{/if}
					<a href="/admin/seo/schema/create?type={type.name}" class="use-template">
						Use Template
					</a>
				</div>
			{/each}
		</div>
	</div>
</div>

{#if showPreview}
	<div
		class="modal-overlay"
		role="button"
		tabindex="0"
		onclick={() => (showPreview = null)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showPreview = null)}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h3><IconCode size={20} /> JSON-LD Preview</h3>
				<div class="modal-actions">
					<button class="btn-copy" onclick={copyJsonLd}>
						<IconCopy size={18} />
						Copy
					</button>
					<button class="close-btn" onclick={() => (showPreview = null)}>×</button>
				</div>
			</div>
			<div class="modal-body">
				<pre><code>{JSON.stringify(showPreview, null, 2)}</code></pre>
			</div>
		</div>
	</div>
{/if}

<ConfirmationModal
	isOpen={showDeleteSchemaModal}
	title="Delete schema?"
	message="Are you sure you want to delete this schema? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteSchema}
	onCancel={cancelDeleteSchema}
/>

<style>
	.schema-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #666;
		font-size: 0.95rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		color: white;
		border-radius: 6px;
		font-weight: 500;
		text-decoration: none;
		transition: background 0.2s;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.schema-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1.5rem;
		margin-bottom: 3rem;
	}

	.schema-card {
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		overflow: hidden;
		transition: box-shadow 0.2s;
	}

	.schema-card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		background: #f8f9fa;
		border-bottom: 1px solid #e5e5e5;
	}

	.schema-type {
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--primary-600);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		padding: 0.5rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		color: #666;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #f0f0f0;
	}

	.action-btn.danger {
		color: #ef4444;
		border-color: #fee2e2;
	}

	.action-btn.danger:hover {
		background: #fee2e2;
	}

	.card-body {
		padding: 1.5rem;
	}

	.card-body h4 {
		font-size: 1.1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.meta {
		font-size: 0.85rem;
		color: #666;
	}

	.meta strong {
		color: #1a1a1a;
	}

	.card-footer {
		padding: 1rem 1.5rem;
		border-top: 1px solid #f0f0f0;
	}

	.edit-link {
		color: #3b82f6;
		text-decoration: none;
		font-weight: 500;
		font-size: 0.9rem;
	}

	.edit-link:hover {
		text-decoration: underline;
	}

	.empty-state {
		grid-column: 1 / -1;
		text-align: center;
		padding: 4rem 2rem;
		color: #999;
	}

	.empty-state h3 {
		color: #1a1a1a;
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		color: #666;
		margin-bottom: 1.5rem;
	}

	.templates-section {
		margin-top: 3rem;
	}

	.templates-section h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.5rem;
	}

	.type-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 1rem;
	}

	.type-card {
		padding: 1.25rem;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		transition: all 0.2s;
	}

	.type-card:hover {
		border-color: var(--primary-600);
		box-shadow: 0 2px 8px rgba(230, 184, 0, 0.1);
	}

	.type-card--deprecated {
		background: #fbfaf6;
		border-color: #e8d8a8;
	}

	.type-name {
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.75rem;
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
	}

	.deprecated-badge {
		font-size: 0.65rem;
		font-weight: 700;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		padding: 0.15rem 0.45rem;
		border-radius: 999px;
		background: #fef3c7;
		color: #92400e;
		border: 1px solid #f3d791;
	}

	.type-note {
		font-size: 0.75rem;
		color: #6b6b6b;
		margin: 0 0 0.75rem;
		line-height: 1.35;
	}

	.use-template {
		font-size: 0.85rem;
		color: var(--primary-600);
		text-decoration: none;
	}

	.use-template:hover {
		text-decoration: underline;
	}

	.loading {
		grid-column: 1 / -1;
		text-align: center;
		padding: 3rem;
		color: #999;
	}

	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		background: white;
		border-radius: 12px;
		width: 100%;
		max-width: 700px;
		max-height: 90vh;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid #e5e5e5;
	}

	.modal-header h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-copy {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
	}

	.btn-copy:hover {
		background: #2563eb;
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 2rem;
		color: #666;
		cursor: pointer;
		line-height: 1;
	}

	.modal-body {
		padding: 1.5rem;
		overflow: auto;
		max-height: calc(90vh - 100px);
	}

	.modal-body pre {
		background: #1e293b;
		color: #e2e8f0;
		padding: 1.5rem;
		border-radius: 6px;
		overflow: auto;
		font-size: 0.9rem;
		line-height: 1.6;
	}

	.modal-body code {
		font-family: 'Courier New', monospace;
	}
</style>
