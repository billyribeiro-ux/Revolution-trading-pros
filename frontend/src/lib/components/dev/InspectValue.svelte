<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * InspectValue Component - Development Value Inspector
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Wrapper for svelte-inspect-value for debugging reactive values
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 *
	 * USAGE:
	 * <InspectValue value={myReactiveValue} label="My Value" />
	 *
	 * Only renders in development mode.
	 */
	import { dev } from '$app/environment';
	import { Inspect } from 'svelte-inspect-value';

	interface Props {
		value: any;
		label?: string;
	}

	const { value, label = 'Value' }: Props = $props();
</script>

{#if dev}
	<div class="inspect-wrapper">
		<Inspect {value} name={label} />
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
