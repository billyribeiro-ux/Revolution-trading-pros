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

	let {
		variant = 'primary',
		size = 'md',
		disabled = false,
		loading = false,
		type = 'button',
		fullWidth = false,
		class: className = '',
		loadingText,
		onclick,
		children
	}: ButtonProps = $props();

	const variants: Record<string, string> = {
		primary: 'bg-blue-600 hover:bg-blue-700 text-white focus:ring-blue-500 disabled:bg-blue-300',
		secondary: 'bg-gray-600 hover:bg-gray-700 text-white focus:ring-gray-500 disabled:bg-gray-300',
		danger: 'bg-red-600 hover:bg-red-700 text-white focus:ring-red-500 disabled:bg-red-300',
		success:
			'bg-green-600 hover:bg-green-700 text-white focus:ring-green-500 disabled:bg-green-300',
		ghost:
			'bg-transparent hover:bg-gray-100 text-gray-700 border border-gray-300 focus:ring-gray-500 disabled:text-gray-400',
		outline:
			'bg-white hover:bg-gray-50 text-gray-700 border border-gray-300 focus:ring-blue-500 disabled:text-gray-400'
	};

	// Size variants with touch target optimization (2026 WCAG standards)
	// Minimum 44px height for touch devices - handled via global CSS on coarse pointers
	const sizes: Record<string, string> = {
		xs: 'px-2 py-1 text-xs min-h-8', // 32px base, expands to 44px on touch via CSS
		sm: 'px-3 py-1.5 text-sm min-h-9', // 36px base, expands to 44px on touch via CSS
		md: 'px-4 py-2 text-base min-h-10', // 40px base, close to 44px target
		lg: 'px-6 py-3 text-lg min-h-11', // 44px - meets touch target
		xl: 'px-8 py-4 text-xl min-h-12' // 48px - enhanced touch target
	};

	// Compute if button is truly disabled (disabled or loading)
	let isDisabled = $derived(disabled || loading);
</script>

<button
	{type}
	class="rounded-md font-medium inline-flex items-center justify-center gap-2
		transition-colors duration-200
		focus:outline-none focus:ring-2 focus:ring-offset-2
		{variants[variant]} {sizes[size]}
		{fullWidth ? 'w-full' : ''}
		disabled:cursor-not-allowed disabled:opacity-60
		{className}"
	disabled={isDisabled}
	aria-disabled={isDisabled}
	aria-busy={loading}
	{onclick}
>
	{#if loading}
		<!-- Loading spinner with reduced motion support -->
		<svg
			class="h-5 w-5 motion-safe:animate-spin motion-reduce:hidden"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			aria-hidden="true"
		>
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
			></circle>
			<path
				class="opacity-75"
				fill="currentColor"
				d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
			></path>
		</svg>
		<!-- Static indicator for reduced motion preference -->
		<span class="motion-safe:hidden motion-reduce:inline-block" aria-hidden="true">...</span>
		<!-- Loading text for screen readers -->
		{#if loadingText}
			<span class="sr-only">{loadingText}</span>
		{:else}
			<span class="sr-only">Loading, please wait</span>
		{/if}
	{/if}
	<span class:opacity-0={loading && !loadingText} class:sr-only={loading && loadingText}>
		{@render children?.()}
	</span>
	{#if loading && loadingText}
		<span aria-hidden="true">{loadingText}</span>
	{/if}
</button>

<style>
	/* Screen reader only utility */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}

	/* Reduced motion: respect user preferences */
	@media (prefers-reduced-motion: reduce) {
		button {
			transition: none;
		}
	}
</style>
