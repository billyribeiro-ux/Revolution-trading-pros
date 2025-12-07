<script lang="ts">
	/**
	 * PostExcerptField Component (FluentForms Pro)
	 *
	 * Textarea for capturing post excerpt/summary.
	 */

	interface Props {
		name?: string;
		value?: string;
		label?: string;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		maxLength?: number;
		rows?: number;
		error?: string;
		helpText?: string;
		onchange?: (value: string) => void;
	}

	let {
		name = 'post_excerpt',
		value = '',
		label = 'Excerpt',
		placeholder = 'Write a brief summary...',
		required = false,
		disabled = false,
		maxLength = 500,
		rows = 3,
		error = '',
		helpText = 'A short summary of your post (optional)',
		onchange
	}: Props = $props();

	let excerpt = $state('');
	const charCount = $derived(excerpt.length);

	// Sync excerpt with value prop changes
	$effect(() => {
		excerpt = value;
	});

	function handleInput(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		excerpt = target.value;
		if (onchange) onchange(excerpt);
	}
</script>

<div class="post-excerpt-field" class:disabled class:has-error={error}>
	{#if label}
		<label for={name} class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</label>
	{/if}

	<textarea
		id={name}
		{name}
		value={excerpt}
		{placeholder}
		{disabled}
		{rows}
		maxlength={maxLength}
		oninput={handleInput}
		class="excerpt-textarea"
	></textarea>

	{#if maxLength}
		<div class="char-counter" class:near-limit={charCount > maxLength * 0.8}>
			{charCount}/{maxLength}
		</div>
	{/if}

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.post-excerpt-field {
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

	.excerpt-textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		line-height: 1.5;
		resize: vertical;
		transition: border-color 0.15s, box-shadow 0.15s;
	}

	.excerpt-textarea:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.excerpt-textarea::placeholder {
		color: #9ca3af;
	}

	.char-counter {
		align-self: flex-end;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.char-counter.near-limit {
		color: #f59e0b;
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

	.disabled .excerpt-textarea {
		background-color: #f3f4f6;
		cursor: not-allowed;
	}

	.has-error .excerpt-textarea {
		border-color: #fca5a5;
	}
</style>
