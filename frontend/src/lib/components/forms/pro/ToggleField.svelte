<script lang="ts">
	import type { FormField } from '$lib/api/forms';

	interface Props {
		field: FormField;
		value?: boolean;
		error?: string[];
		onchange?: (value: boolean) => void;
	}

	let props: Props = $props();

	const onLabel = $derived(props.field.attributes?.on_label || 'Yes');
	const offLabel = $derived(props.field.attributes?.off_label || 'No');
	const showLabels = $derived(props.field.attributes?.show_labels !== false);
	const size = $derived<'sm' | 'md' | 'lg'>(props.field.attributes?.size || 'md');

	function handleToggle() {
		props.onchange?.(!(props.value ?? false));
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			handleToggle();
		}
	}
</script>

<div class="toggle-field">
	<label for="toggle-{props.field.name}" class="field-label">
		{props.field.label}
		{#if props.field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="field-help">{props.field.help_text}</p>
	{/if}

	<div class="toggle-wrapper">
		{#if showLabels}
			<span class="toggle-label off" class:active={!(props.value ?? false)}>{offLabel}</span>
		{/if}

		<button
			type="button"
			class={`toggle-switch ${size}`}
			class:active={value}
			role="switch"
			aria-checked={value}
			aria-label="{props.field.label} toggle"
			onclick={handleToggle}
			onkeydown={handleKeyDown}
		>
			<span class="toggle-track">
				<span class="toggle-thumb"></span>
			</span>
		</button>

		{#if showLabels}
			<span class="toggle-label on" class:active={value}>{onLabel}</span>
		{/if}
	</div>

	<input id="toggle-{props.field.name}" type="hidden" name={props.field.name} value={value ? '1' : '0'} />

	{#if error && error.length > 0}
		<div class="field-error">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.toggle-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-weight: 500;
		font-size: 0.875rem;
		color: #374151;
	}

	.required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.field-help {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	/* 2026 Mobile-First: Enhanced touch area */
	.toggle-wrapper {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		min-height: 44px; /* Touch target */
	}

	.toggle-label {
		font-size: 1rem; /* 16px for readability */
		color: #9ca3af;
		transition: color 0.2s;
		padding: 0.5rem 0; /* Touch-friendly padding */
	}

	.toggle-label.active {
		color: #111827;
		font-weight: 500;
	}

	.toggle-switch {
		position: relative;
		display: inline-flex;
		align-items: center;
		background: none;
		border: none;
		padding: 0.5rem; /* Touch-friendly padding around switch */
		margin: -0.5rem; /* Offset to maintain visual position */
		cursor: pointer;
		touch-action: manipulation;
	}

	.toggle-switch:focus {
		outline: none;
	}

	.toggle-switch:focus .toggle-track {
		box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.2);
	}

	.toggle-track {
		position: relative;
		background-color: #d1d5db;
		border-radius: 9999px;
		transition: background-color 0.2s;
	}

	.toggle-switch.active .toggle-track {
		background-color: #2563eb;
	}

	.toggle-thumb {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		background-color: white;
		border-radius: 50%;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
		transition: left 0.2s;
	}

	/* Size variants */
	.toggle-switch.sm .toggle-track {
		width: 36px;
		height: 20px;
	}

	.toggle-switch.sm .toggle-thumb {
		width: 16px;
		height: 16px;
		left: 2px;
	}

	.toggle-switch.sm.active .toggle-thumb {
		left: 18px;
	}

	.toggle-switch.md .toggle-track {
		width: 44px;
		height: 24px;
	}

	.toggle-switch.md .toggle-thumb {
		width: 20px;
		height: 20px;
		left: 2px;
	}

	.toggle-switch.md.active .toggle-thumb {
		left: 22px;
	}

	.toggle-switch.lg .toggle-track {
		width: 56px;
		height: 30px;
	}

	.toggle-switch.lg .toggle-thumb {
		width: 26px;
		height: 26px;
		left: 2px;
	}

	.toggle-switch.lg.active .toggle-thumb {
		left: 28px;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>
