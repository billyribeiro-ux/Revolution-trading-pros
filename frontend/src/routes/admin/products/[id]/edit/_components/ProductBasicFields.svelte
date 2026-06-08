<script lang="ts">
	/**
	 * R20-C extraction (2026-05-20): basic-info fields — name, slug,
	 * description, long_description. Mutates the parent's `formData` proxy
	 * directly so existing `bind:value` semantics carry over.
	 */
	interface FormFields {
		name: string;
		slug: string;
		description: string;
		long_description: string;
	}

	interface Props {
		formData: FormFields;
		getFieldError: (field: string) => string | undefined;
	}

	let { formData = $bindable(), getFieldError }: Props = $props();
</script>

<div class={['form-group', { 'has-error': getFieldError('name') }]}>
	<label for="name">Product Name *</label>
	<input
		id="name"
		name="name"
		type="text"
		bind:value={formData.name}
		placeholder="e.g., Advanced Trading Course"
	/>
	{#if getFieldError('name')}
		<span class="field-error">{getFieldError('name')}</span>
	{/if}
</div>

<div class={['form-group', { 'has-error': getFieldError('slug') }]}>
	<label for="slug">URL Slug *</label>
	<div class="slug-input">
		<span class="slug-prefix">/products/</span>
		<input
			id="slug"
			name="slug"
			type="text"
			bind:value={formData.slug}
			placeholder="advanced-trading-course"
		/>
	</div>
	{#if getFieldError('slug')}
		<span class="field-error">{getFieldError('slug')}</span>
	{/if}
</div>

<div class="form-group">
	<label for="description">Short Description</label>
	<textarea
		id="description"
		bind:value={formData.description}
		placeholder="Brief description of the product..."
		rows="3"
	></textarea>
</div>

<div class="form-group">
	<label for="long_description">Full Description</label>
	<textarea
		id="long_description"
		bind:value={formData.long_description}
		placeholder="Detailed description with features and benefits..."
		rows="6"
	></textarea>
</div>

<style>
	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group.has-error input {
		border-color: #ef4444;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.form-group input[type='text'],
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.25);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
	}

	.field-error {
		display: block;
		color: #f87171;
		font-size: 0.8125rem;
		margin-top: 0.375rem;
	}

	.slug-input {
		display: flex;
		align-items: center;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.25);
		border-radius: 8px;
		overflow: hidden;
	}

	.slug-prefix {
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.8);
		color: #64748b;
		font-size: 0.875rem;
		font-family: monospace;
		border-right: 1px solid rgba(148, 163, 184, 0.2);
	}

	.slug-input input {
		border: none;
		border-radius: 0;
		background: transparent;
	}

	.slug-input input:focus {
		box-shadow: none;
	}
</style>
