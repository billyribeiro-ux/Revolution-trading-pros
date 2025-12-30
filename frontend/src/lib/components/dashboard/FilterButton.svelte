<script lang="ts">
	/**
	 * FilterButton - Shared Filter Toggle Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for filter buttons used in learning center,
	 * platform tutorials, alerts, and other filterable pages.
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */

	interface Props {
		/** Button label text */
		label: string;
		/** Value to emit when clicked */
		value: string;
		/** Whether this filter is currently active */
		isActive?: boolean;
		/** Click handler */
		onclick?: (value: string) => void;
		/** Optional variant for different color schemes */
		variant?: 'default' | 'dark';
	}

	let {
		label,
		value,
		isActive = false,
		onclick,
		variant = 'default'
	}: Props = $props();

	function handleClick() {
		onclick?.(value);
	}
</script>

<button
	type="button"
	class="filter-btn"
	class:is-active={isActive}
	class:variant--dark={variant === 'dark'}
	onclick={handleClick}
>
	{label}
</button>

<style>
	.filter-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 500;
		font-family: 'Open Sans', sans-serif;
		color: #666;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 20px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
		white-space: nowrap;
	}

	.filter-btn:hover {
		background: #f4f4f4;
		border-color: #999;
		color: #333;
	}

	.filter-btn.is-active {
		background: #0984ae;
		border-color: #0984ae;
		color: #fff;
	}

	.filter-btn.is-active:hover {
		background: #077a9e;
		border-color: #077a9e;
	}

	/* Dark variant for dark-themed pages (alerts) */
	.filter-btn.variant--dark {
		background: rgba(30, 41, 59, 0.6);
		border-color: rgba(148, 163, 184, 0.3);
		color: #94a3b8;
	}

	.filter-btn.variant--dark:hover {
		background: rgba(30, 41, 59, 0.8);
		border-color: rgba(148, 163, 184, 0.5);
		color: #cbd5e1;
	}

	.filter-btn.variant--dark.is-active {
		background: #a5b4fc;
		border-color: #a5b4fc;
		color: #1e1b4b;
	}
</style>
