<script lang="ts">
	/**
	 * PostTitleField Component (FluentForms Pro)
	 *
	 * Text input for capturing post titles in frontend post creation forms.
	 */

	interface Props {
		name?: string;
		value?: string;
		label?: string;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		maxLength?: number;
		error?: string;
		helpText?: string;
		onchange?: (value: string) => void;
	}

	let {
		name = 'post_title',
		value = '',
		label = 'Post Title',
		placeholder = 'Enter title...',
		required = false,
		disabled = false,
		maxLength = 200,
		error = '',
		helpText = '',
		onchange
	}: Props = $props();

	let inputValue = $state('');
	let charCount = $derived(inputValue.length);

	// Sync inputValue with value prop changes
	$effect(() => {
		inputValue = value;
	});

	function handleInput(e: Event) {
		const target = e.target as HTMLInputElement;
		inputValue = target.value;
		if (onchange) onchange(inputValue);
	}
</script>

<div class="post-title-field" class:disabled class:has-error={error}>
	{#if label}
		<label for={name} class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</label>
	{/if}

	<input
		type="text"
		id={name}
		{name}
		value={inputValue}
		{placeholder}
		{disabled}
		maxlength={maxLength}
		oninput={handleInput}
		class="title-input"
	/>

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
	.post-title-field {
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

	.title-input {
		width: 100%;
		padding: 0.75rem 1rem;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		font-size: 1.125rem;
		font-weight: 500;
		transition: border-color 0.15s, box-shadow 0.15s;
	}

	.title-input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.title-input::placeholder {
		color: #9ca3af;
		font-weight: 400;
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

	.disabled .title-input {
		background-color: #f3f4f6;
		cursor: not-allowed;
	}

	.has-error .title-input {
		border-color: #fca5a5;
	}
</style>
