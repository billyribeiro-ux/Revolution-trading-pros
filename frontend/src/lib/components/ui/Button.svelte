<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let variant: 'primary' | 'secondary' | 'danger' | 'ghost' | 'outline' = 'primary';
	export let size: 'sm' | 'md' | 'lg' = 'md';
	export let disabled: boolean = false;
	export let loading: boolean = false;
	export let type: 'button' | 'submit' = 'button';
	export let fullWidth: boolean = false;
	let className: string = '';
	export { className as class };

	const dispatch = createEventDispatcher();

	const variants = {
		primary: 'bg-blue-600 hover:bg-blue-700 text-white disabled:bg-blue-300',
		secondary: 'bg-gray-600 hover:bg-gray-700 text-white disabled:bg-gray-300',
		danger: 'bg-red-600 hover:bg-red-700 text-white disabled:bg-red-300',
		ghost:
			'bg-transparent hover:bg-gray-100 text-gray-700 border border-gray-300 disabled:text-gray-400',
		outline: 'bg-white hover:bg-gray-50 text-gray-700 border border-gray-300 disabled:text-gray-400'
	};

	const sizes = {
		sm: 'px-3 py-1.5 text-sm',
		md: 'px-4 py-2 text-base',
		lg: 'px-6 py-3 text-lg'
	};
</script>

<button
	{type}
	class="rounded-md font-medium transition-colors duration-200 inline-flex items-center justify-center gap-2
    {variants[variant]} {sizes[size]} {fullWidth ? 'w-full' : ''}
    disabled:cursor-not-allowed disabled:opacity-60 {className}"
	disabled={disabled || loading}
	on:click={() => dispatch('click')}
>
	{#if loading}
		<svg
			class="animate-spin h-5 w-5"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
		>
			<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
			></circle>
			<path
				class="opacity-75"
				fill="currentColor"
				d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
			></path>
		</svg>
	{/if}
	<slot />
</button>
