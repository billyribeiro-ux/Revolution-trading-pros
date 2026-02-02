<script lang="ts">
	/**
	 * Form Template Selector - Svelte 5
	 *
	 * Updated: December 2025 - Migrated to Svelte 5 runes ($props, $state, $derived)
	 */
	import type { FormTemplate } from '$lib/data/formTemplates';
	import { templates } from '$lib/data/formTemplates';

	// Svelte 5: Props using $props() rune with callback events
	interface Props {
		onSelect?: (template: FormTemplate) => void;
		onCancel?: () => void;
	}

	let props: Props = $props();

	// Svelte 5: Reactive state using $state() rune
	let selectedCategory: string = $state('all');
	let searchQuery = $state('');

	const categories = [
		{ id: 'all', label: 'All Templates', icon: '📋' },
		{ id: 'contact', label: 'Contact', icon: '📧' },
		{ id: 'survey', label: 'Survey', icon: '📊' },
		{ id: 'registration', label: 'Registration', icon: '🎫' },
		{ id: 'feedback', label: 'Feedback', icon: '💭' },
		{ id: 'order', label: 'Order', icon: '🛒' },
		{ id: 'application', label: 'Application', icon: '💼' }
	];

	// Svelte 5: Derived value using $derived() rune
	let filteredTemplates = $derived(
		templates.filter((template) => {
			const matchesCategory = selectedCategory === 'all' || template.category === selectedCategory;
			const matchesSearch =
				searchQuery === '' ||
				template.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				template.description.toLowerCase().includes(searchQuery.toLowerCase());
			return matchesCategory && matchesSearch;
		})
	);

	function handleSelect(template: FormTemplate) {
		props.onSelect?.(template);
	}

	function handleStartFromScratch() {
		props.onCancel?.();
	}
</script>

<div class="template-selector">
	<div class="selector-header">
		<div>
			<h2>Choose a Template</h2>
			<p class="header-description">Start with a pre-designed form or build from scratch</p>
		</div>
		<button class="btn-scratch" onclick={handleStartFromScratch}> ✨ Start from Scratch </button>
	</div>

	<div class="search-filter">
		<input
			type="text"
			class="search-input"
			placeholder="Search templates..."
			bind:value={searchQuery}
		/>
	</div>

	<div class="categories">
		{#each categories as category}
			<button
				class="category-btn"
				class:active={selectedCategory === category.id}
				onclick={() => (selectedCategory = category.id)}
			>
				<span class="category-icon">{category.icon}</span>
				<span>{category.label}</span>
			</button>
		{/each}
	</div>

	<div class="templates-grid">
		{#if filteredTemplates.length === 0}
			<div class="empty-state">
				<p>No templates found matching your criteria.</p>
			</div>
		{:else}
			{#each filteredTemplates as template}
				<div
					class="template-card"
					role="button"
					tabindex="0"
					onclick={() => handleSelect(template)}
					onkeypress={(e: KeyboardEvent) => e.key === 'Enter' && handleSelect(template)}
				>
					<div class="template-icon">{template.icon}</div>
					<div class="template-content">
						<h3>{template.name}</h3>
						<p class="template-description">{template.description}</p>
						<div class="template-meta">
							<span class="field-count">{template.fields.length} fields</span>
							<span
								class="theme-badge"
								style="background: {template.theme.colors.primary}20; color: {template.theme.colors
									.primary}"
							>
								{template.theme.name}
							</span>
						</div>
					</div>
					<div class="template-preview">
						<div class="preview-fields">
							{#each template.fields.slice(0, 3) as field}
								<div class="preview-field">
									<div class="preview-label">{field.label}</div>
									<div class="preview-input"></div>
								</div>
							{/each}
							{#if template.fields.length > 3}
								<div class="preview-more">+{template.fields.length - 3} more fields</div>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	.template-selector {
		max-width: 1200px;
		margin: 0 auto;
	}

	.selector-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		padding-bottom: 1.5rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	h2 {
		font-size: 1.875rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.header-description {
		color: #94a3b8;
		font-size: 0.9375rem;
		margin: 0;
	}

	.btn-scratch {
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.9375rem;
		cursor: pointer;
		transition: all 0.2s;
		box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
	}

	.btn-scratch:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(99, 102, 241, 0.4);
	}

	.search-filter {
		margin-bottom: 1.5rem;
	}

	.search-input {
		width: 100%;
		padding: 0.875rem 1.25rem;
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	.search-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.categories {
		display: flex;
		gap: 0.75rem;
		margin-bottom: 2rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.category-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.category-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border-color: rgba(99, 102, 241, 0.4);
	}

	.category-btn.active {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.2), rgba(139, 92, 246, 0.2));
		color: #a5b4fc;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.category-icon {
		font-size: 1.125rem;
	}

	.templates-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.template-card {
		background: #1e293b;
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 12px;
		padding: 1.5rem;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
		overflow: hidden;
	}

	.template-card:hover {
		border-color: rgba(99, 102, 241, 0.4);
		transform: translateY(-4px);
		box-shadow: 0 8px 24px rgba(99, 102, 241, 0.2);
	}

	.template-icon {
		font-size: 3rem;
		margin-bottom: 1rem;
	}

	.template-content h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.template-description {
		color: #94a3b8;
		font-size: 0.875rem;
		line-height: 1.5;
		margin: 0 0 1rem 0;
	}

	.template-meta {
		display: flex;
		gap: 0.75rem;
		align-items: center;
		flex-wrap: wrap;
	}

	.field-count {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.theme-badge {
		padding: 0.25rem 0.625rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.template-preview {
		margin-top: 1.25rem;
		padding-top: 1.25rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.preview-fields {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.preview-field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.preview-label {
		font-size: 0.75rem;
		color: #94a3b8;
		font-weight: 500;
	}

	.preview-input {
		height: 32px;
		background: rgba(99, 102, 241, 0.05);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 6px;
	}

	.preview-more {
		text-align: center;
		font-size: 0.75rem;
		color: #64748b;
		font-style: italic;
		padding-top: 0.5rem;
	}

	.empty-state {
		grid-column: 1 / -1;
		text-align: center;
		padding: 4rem 2rem;
		color: #64748b;
	}

	@media (max-width: 768px) {
		.selector-header {
			flex-direction: column;
			gap: 1rem;
		}

		.btn-scratch {
			width: 100%;
		}

		.templates-grid {
			grid-template-columns: 1fr;
		}

		.categories {
			flex-wrap: nowrap;
			overflow-x: auto;
		}
	}
</style>
