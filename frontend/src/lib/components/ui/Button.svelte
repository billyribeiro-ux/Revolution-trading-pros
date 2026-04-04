<script lang="ts">
	/**
	 * Button - Accessible Button Component
	 * =====================================
	 * WCAG 2.1 AA compliant button with:
	 * - aria-busy for loading states
	 * - aria-disabled for disabled states
	 * - Reduced motion support
	 * - Proper focus states
	 *
	 * @version 2.0.0 - Accessibility Enhanced
	 * @accessibility WCAG 2.1 AA compliant
	 */
	import type { Snippet } from 'svelte';

	// Export Props interface for external use
	export interface ButtonProps {
		variant?: 'primary' | 'secondary' | 'danger' | 'ghost' | 'outline' | 'success';
		size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
		disabled?: boolean;
		loading?: boolean;
		type?: 'button' | 'submit' | 'reset';
		fullWidth?: boolean;
		class?: string;
		loadingText?: string;
		onclick?: (e: MouseEvent) => void;
		children?: Snippet;
	}

	let props: ButtonProps = $props();
	let variant = $derived(props.variant ?? 'primary');
	let size = $derived(props.size ?? 'md');
	let disabled = $derived(props.disabled ?? false);
	let loading = $derived(props.loading ?? false);
	let type = $derived(props.type ?? 'button');
	let fullWidth = $derived(props.fullWidth ?? false);
	let className = $derived(props.class ?? '');
	let loadingText = $derived(props.loadingText);
	let onclick = $derived(props.onclick);

	// Compute if button is truly disabled (disabled or loading)
	let isDisabled = $derived(disabled || loading);
</script>

<button
	{type}
	class="standalone-btn {className}"
	data-variant={variant}
	data-size={size}
	data-full-width={fullWidth || undefined}
	disabled={isDisabled}
	aria-disabled={isDisabled}
	aria-busy={loading}
	{onclick}
>
	{#if loading}
		<!-- Loading spinner with reduced motion support -->
		<svg
			class="standalone-btn-spinner"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			aria-hidden="true"
		>
			<circle class="standalone-btn-spinner-track" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
			></circle>
			<path
				class="standalone-btn-spinner-head"
				fill="currentColor"
				d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
			></path>
		</svg>
		<!-- Static indicator for reduced motion preference -->
		<span class="standalone-btn-dots" aria-hidden="true">...</span>
		<!-- Loading text for screen readers -->
		{#if loadingText}
			<span class="sr-only">{loadingText}</span>
		{:else}
			<span class="sr-only">Loading, please wait</span>
		{/if}
	{/if}
	<span class="standalone-btn-content" class:standalone-btn-hidden={loading && !loadingText} class:sr-only={loading && loadingText}>
		{@render props.children?.()}
	</span>
	{#if loading && loadingText}
		<span aria-hidden="true">{loadingText}</span>
	{/if}
</button>

<style>
	/* Screen reader only utility */
	.sr-only {
		position: absolute;
		inline-size: 1px;
		block-size: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}

	.standalone-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		border-radius: var(--radius-md);
		font-weight: var(--weight-medium);
		touch-action: manipulation;
		transition: color var(--duration-fast) var(--ease-default),
			background-color var(--duration-fast) var(--ease-default);
		inline-size: 100%;

		@media (min-width: 640px) {
			inline-size: auto;
		}

		&:focus {
			outline: none;
			box-shadow: 0 0 0 2px var(--background), 0 0 0 4px var(--ring);
		}

		&:disabled {
			cursor: not-allowed;
			opacity: 0.6;
		}

		&[data-full-width] {
			inline-size: 100%;
		}

		/* Variant styles */
		&[data-variant='primary'] {
			background-color: oklch(0.55 0.2 260);
			color: oklch(1 0 0);
			&:hover:not(:disabled) { background-color: oklch(0.48 0.2 260); }
			&:focus { box-shadow: 0 0 0 2px var(--background), 0 0 0 4px oklch(0.55 0.2 260); }
			&:disabled { background-color: oklch(0.75 0.1 260); }
		}

		&[data-variant='secondary'] {
			background-color: oklch(0.45 0.015 265);
			color: oklch(1 0 0);
			&:hover:not(:disabled) { background-color: oklch(0.4 0.015 265); }
			&:disabled { background-color: oklch(0.7 0.01 265); }
		}

		&[data-variant='danger'] {
			background-color: oklch(0.58 0.24 27);
			color: oklch(1 0 0);
			&:hover:not(:disabled) { background-color: oklch(0.5 0.24 27); }
			&:disabled { background-color: oklch(0.75 0.12 27); }
		}

		&[data-variant='success'] {
			background-color: oklch(0.55 0.17 150);
			color: oklch(1 0 0);
			&:hover:not(:disabled) { background-color: oklch(0.48 0.17 150); }
			&:disabled { background-color: oklch(0.75 0.08 150); }
		}

		&[data-variant='ghost'] {
			background-color: transparent;
			color: oklch(0.4 0.01 265);
			border: 1px solid var(--border);
			&:hover:not(:disabled) { background-color: oklch(0.97 0.002 265); }
			&:disabled { color: oklch(0.65 0.01 265); }
		}

		&[data-variant='outline'] {
			background-color: var(--background);
			color: oklch(0.4 0.01 265);
			border: 1px solid var(--border);
			&:hover:not(:disabled) { background-color: oklch(0.98 0.002 265); }
			&:disabled { color: oklch(0.65 0.01 265); }
		}

		/* Size styles */
		&[data-size='xs'] {
			padding-inline: var(--space-3);
			padding-block: var(--space-2);
			font-size: var(--text-sm);
			min-block-size: 44px;
		}

		&[data-size='sm'] {
			padding-inline: var(--space-4);
			padding-block: 0.625rem;
			font-size: var(--text-sm);
			min-block-size: 44px;
		}

		&[data-size='md'] {
			padding-inline: var(--space-4);
			padding-block: var(--space-3);
			font-size: var(--text-base);
			min-block-size: 44px;
		}

		&[data-size='lg'] {
			padding-inline: var(--space-6);
			padding-block: 0.875rem;
			font-size: var(--text-lg);
			min-block-size: 48px;
		}

		&[data-size='xl'] {
			padding-inline: var(--space-8);
			padding-block: var(--space-4);
			font-size: var(--text-xl);
			min-block-size: 52px;
		}
	}

	.standalone-btn-spinner {
		inline-size: 1.25rem;
		block-size: 1.25rem;
	}

	.standalone-btn-spinner-track { opacity: 0.25; }
	.standalone-btn-spinner-head { opacity: 0.75; }
	.standalone-btn-hidden { opacity: 0; }

	.standalone-btn-dots { display: none; }

	@media (prefers-reduced-motion: no-preference) {
		.standalone-btn-spinner {
			animation: spin 1s linear infinite;
		}
		.standalone-btn-dots { display: none; }
	}

	@media (prefers-reduced-motion: reduce) {
		.standalone-btn { transition: none; }
		.standalone-btn-spinner { display: none; }
		.standalone-btn-dots { display: inline-block; }
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>
