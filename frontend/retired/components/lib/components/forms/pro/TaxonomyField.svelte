<script lang="ts">
	/**
	 * TaxonomyField Component (FluentForms Pro 6.1.8)
	 *
	 * Select field for categories, tags, and custom taxonomies.
	 * Supports hierarchical (categories) and non-hierarchical (tags) taxonomies.
	 */

	interface TaxonomyTerm {
		id: string | number;
		name: string;
		slug: string;
		parent?: string | number;
		count?: number;
		children?: TaxonomyTerm[];
	}

	interface Props {
		name: string;
		taxonomy?: string;
		value?: string | string[];
		label?: string;
		required?: boolean;
		disabled?: boolean;
		multiple?: boolean;
		hierarchical?: boolean;
		showCount?: boolean;
		placeholder?: string;
		maxSelections?: number;
		allowCreate?: boolean;
		terms?: TaxonomyTerm[];
		error?: string;
		helpText?: string;
		fetchEndpoint?: string;
		onchange?: (value: string | string[]) => void;
		oncreate?: (name: string) => Promise<TaxonomyTerm>;
	}

	let {
		name,
		taxonomy = 'category',
		value = '',
		label = 'Category',
		required = false,
		disabled = false,
		multiple = false,
		hierarchical = true,
		showCount = false,
		placeholder = 'Select...',
		maxSelections,
		allowCreate = false,
		terms = [],
		error = '',
		helpText = '',
		fetchEndpoint,
		onchange,
		oncreate
	}: Props = $props();

	let selectedIds = $state<string[]>([]);
	let searchQuery = $state('');
	let showDropdown = $state(false);
	let loading = $state(false);
	let fetchedTerms = $state<TaxonomyTerm[]>([]);
	let newTermName = $state('');
	let creating = $state(false);

	// Sync selectedIds with value prop changes
	$effect(() => {
		selectedIds = Array.isArray(value) ? value.map(String) : value ? [String(value)] : [];
	});

	// Sync fetchedTerms with terms prop changes
	$effect(() => {
		if (terms.length > 0) {
			fetchedTerms = terms;
		}
	});

	$effect(() => {
		if (fetchEndpoint && fetchedTerms.length === 0) {
			fetchTerms();
		}
	});

	async function fetchTerms() {
		if (!fetchEndpoint) return;

		loading = true;
		try {
			const response = await fetch(`${fetchEndpoint}?taxonomy=${taxonomy}`);
			const data = await response.json();
			fetchedTerms = data.terms || [];
		} catch {
			// Failed to fetch terms
		} finally {
			loading = false;
		}
	}

	function buildTree(terms: TaxonomyTerm[]): TaxonomyTerm[] {
		if (!hierarchical) return terms;

		const map = new Map<string | number, TaxonomyTerm>();
		const roots: TaxonomyTerm[] = [];

		terms.forEach((term) => {
			map.set(term.id, { ...term, children: [] });
		});

		terms.forEach((term) => {
			const node = map.get(term.id)!;
			if (term.parent && map.has(term.parent)) {
				map.get(term.parent)!.children!.push(node);
			} else {
				roots.push(node);
			}
		});

		return roots;
	}

	const termTree = $derived(buildTree(fetchedTerms));

	const filteredTerms = $derived(
		searchQuery
			? fetchedTerms.filter((t) => t.name.toLowerCase().includes(searchQuery.toLowerCase()))
			: []
	);

	const selectedTerms = $derived(fetchedTerms.filter((t) => selectedIds.includes(String(t.id))));

	const canSelect = $derived(!maxSelections || selectedIds.length < maxSelections);

	function toggleTerm(termId: string) {
		if (disabled) return;

		if (selectedIds.includes(termId)) {
			selectedIds = selectedIds.filter((id) => id !== termId);
		} else if (multiple) {
			if (canSelect) {
				selectedIds = [...selectedIds, termId];
			}
		} else {
			selectedIds = [termId];
			showDropdown = false;
		}

		notifyChange();
	}

	function removeTerm(termId: string) {
		if (disabled) return;
		selectedIds = selectedIds.filter((id) => id !== termId);
		notifyChange();
	}

	function notifyChange() {
		if (onchange) {
			onchange(multiple ? selectedIds : selectedIds[0] || '');
		}
	}

	async function handleCreate() {
		if (!allowCreate || !oncreate || !newTermName.trim() || creating) return;

		creating = true;
		try {
			const newTerm = await oncreate(newTermName.trim());
			fetchedTerms = [...fetchedTerms, newTerm];
			selectedIds = [...selectedIds, String(newTerm.id)];
			newTermName = '';
			notifyChange();
		} catch {
			// Failed to create term
		} finally {
			creating = false;
		}
	}

	function renderTermOption(term: TaxonomyTerm, depth: number = 0): string {
		const prefix = hierarchical ? '—'.repeat(depth) + (depth > 0 ? ' ' : '') : '';
		const count = showCount && term.count !== undefined ? ` (${term.count})` : '';
		return `${prefix}${term.name}${count}`;
	}
</script>

<div class="taxonomy-field" class:disabled class:has-error={error}>
	{#if label}
		<label for={multiple ? `${name}-search` : name} class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</label>
	{/if}

	{#if loading}
		<div class="loading-state">
			<svg class="spinner" viewBox="0 0 24 24">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
			</svg>
			<span>Loading {taxonomy}...</span>
		</div>
	{:else if multiple}
		<!-- Multi-select with checkboxes -->
		<div class="multi-select-wrapper">
			{#if selectedTerms.length > 0}
				<div class="selected-tags">
					{#each selectedTerms as term}
						<span class="tag">
							{term.name}
							<button
								type="button"
								class="tag-remove"
								onclick={() => removeTerm(String(term.id))}
								{disabled}
								aria-label="Remove {term.name}"
							>
								×
							</button>
						</span>
					{/each}
				</div>
			{/if}

			<div class="search-wrapper">
				<input
					type="text"
					id="{name}-search"
					class="search-input"
					placeholder="Search {taxonomy}..."
					bind:value={searchQuery}
					onfocus={() => (showDropdown = true)}
					{disabled}
				/>
			</div>

			{#if showDropdown}
				<div class="dropdown">
					{#if searchQuery && filteredTerms.length > 0}
						{#each filteredTerms as term}
							<label class="term-option">
								<input
									type="checkbox"
									checked={selectedIds.includes(String(term.id))}
									disabled={disabled || (!selectedIds.includes(String(term.id)) && !canSelect)}
									onchange={() => toggleTerm(String(term.id))}
								/>
								<span>{term.name}</span>
								{#if showCount && term.count !== undefined}
									<span class="term-count">({term.count})</span>
								{/if}
							</label>
						{/each}
					{:else if !searchQuery}
						{#each termTree as term}
							{@render termCheckbox(term, 0)}
						{/each}
					{:else}
						<div class="no-results">No {taxonomy} found</div>
					{/if}

					{#if allowCreate && searchQuery && !filteredTerms.find((t) => t.name.toLowerCase() === searchQuery.toLowerCase())}
						<div class="create-option">
							<button
								type="button"
								class="create-btn"
								onclick={() => {
									newTermName = searchQuery;
									handleCreate();
								}}
								disabled={creating}
							>
								{creating ? 'Creating...' : `Create "${searchQuery}"`}
							</button>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{:else}
		<!-- Single select dropdown -->
		<select
			id={name}
			{name}
			{disabled}
			onchange={(e: Event) => toggleTerm((e.target as HTMLSelectElement).value)}
			class="select-input"
		>
			<option value="">{placeholder}</option>
			{#each termTree as term}
				{@render termOption(term, 0)}
			{/each}
		</select>
	{/if}

	{#if maxSelections && multiple}
		<p class="selection-count">
			{selectedIds.length}/{maxSelections} selected
		</p>
	{/if}

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}

	<!-- Hidden inputs for form submission -->
	{#each selectedIds as id}
		<input type="hidden" name={multiple ? `${name}[]` : name} value={id} />
	{/each}
</div>

{#snippet termCheckbox(term: TaxonomyTerm, depth: number)}
	<label class="term-option" style="padding-left: {depth * 1.25}rem">
		<input
			type="checkbox"
			checked={selectedIds.includes(String(term.id))}
			disabled={disabled || (!selectedIds.includes(String(term.id)) && !canSelect)}
			onchange={() => toggleTerm(String(term.id))}
		/>
		<span>{term.name}</span>
		{#if showCount && term.count !== undefined}
			<span class="term-count">({term.count})</span>
		{/if}
	</label>
	{#if term.children && term.children.length > 0}
		{#each term.children as child}
			{@render termCheckbox(child, depth + 1)}
		{/each}
	{/if}
{/snippet}

{#snippet termOption(term: TaxonomyTerm, depth: number)}
	<option value={term.id} selected={selectedIds.includes(String(term.id))}>
		{renderTermOption(term, depth)}
	</option>
	{#if term.children && term.children.length > 0}
		{#each term.children as child}
			{@render termOption(child, depth + 1)}
		{/each}
	{/if}
{/snippet}

<style>
	.taxonomy-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.required {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.loading-state {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		color: #6b7280;
	}

	.spinner {
		width: 18px;
		height: 18px;
		animation: spin 1s linear infinite;
	}

	.spinner circle {
		stroke-dasharray: 60;
		stroke-dashoffset: 45;
		stroke-linecap: round;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.select-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		background-color: white;
	}

	.select-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.multi-select-wrapper {
		position: relative;
	}

	.selected-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
		margin-bottom: 0.5rem;
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background-color: #dbeafe;
		color: #1e40af;
		border-radius: 0.25rem;
		font-size: 0.8125rem;
	}

	.tag-remove {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 16px;
		height: 16px;
		padding: 0;
		border: none;
		background: none;
		color: #1e40af;
		cursor: pointer;
		font-size: 1rem;
		line-height: 1;
	}

	.tag-remove:hover {
		color: #1e3a8a;
	}

	.search-wrapper {
		position: relative;
	}

	.search-input {
		width: 100%;
		padding: 0.625rem 0.875rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
	}

	.search-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.dropdown {
		position: absolute;
		top: 100%;
		left: 0;
		right: 0;
		max-height: 250px;
		overflow-y: auto;
		background-color: white;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
		z-index: 50;
		margin-top: 0.25rem;
	}

	.term-option {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		cursor: pointer;
		transition: background-color 0.1s;
	}

	.term-option:hover {
		background-color: #f3f4f6;
	}

	.term-option input[type='checkbox'] {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	.term-count {
		font-size: 0.75rem;
		color: #6b7280;
		margin-left: auto;
	}

	.no-results {
		padding: 1rem;
		text-align: center;
		color: #6b7280;
	}

	.create-option {
		border-top: 1px solid #e5e7eb;
		padding: 0.5rem;
	}

	.create-btn {
		width: 100%;
		padding: 0.5rem;
		border: none;
		background-color: #dbeafe;
		color: #1e40af;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.create-btn:hover:not(:disabled) {
		background-color: #bfdbfe;
	}

	.create-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.selection-count {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.help-text {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.error-text {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
	}

	.disabled {
		opacity: 0.6;
	}

	.has-error .select-input,
	.has-error .search-input {
		border-color: #fca5a5;
	}
</style>
