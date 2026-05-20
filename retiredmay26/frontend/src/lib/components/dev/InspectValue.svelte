<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * InspectValue Component - Development Value Inspector
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Wrapper for svelte-inspect-value for debugging reactive values
	 * @version 1.1.0 - ICT 7 Fix: Use expandLevel instead of collapsed
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * USAGE:
	 * <InspectValue value={myReactiveValue} label="My Value" />
	 * <InspectValue value={myReactiveValue} label="My Value" expandLevel={0} /> <!-- collapsed -->
	 *
	 * Only renders in development mode.
	 */
	import { dev } from '$app/environment';
	import { Inspect } from 'svelte-inspect-value';

	interface Props {
		value: unknown;
		label?: string;
		expandLevel?: number;
	}

	let props: Props = $props();
</script>

{#if dev}
	<div class="inspect-wrapper">
		<Inspect
			value={props.value}
			name={props.label ?? 'Value'}
			expandLevel={props.expandLevel ?? 1}
		/>
	</div>
{/if}

<style>
	.inspect-wrapper {
		margin: 8px 0;
		padding: 8px;
		background: rgba(0, 0, 0, 0.05);
		border-radius: 4px;
		font-family:
			'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
		font-size: 12px;
	}
</style>
