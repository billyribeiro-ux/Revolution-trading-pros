<script lang="ts">
	import IconSchool from '@tabler/icons-svelte-runes/icons/school';
	import ConceptTooltip from './ConceptTooltip.svelte';
	import { getEducation } from './education-content.js';
	import { getCalculator } from '../../state/context.js';
	import type { EducationEntry } from '../../engine/types.js';

	const calc = getCalculator();

	/** Static content that is only ever swapped wholesale, so it is not made deeply reactive. */
	let activeEntry = $state.raw<EducationEntry | null>(null);

	/**
	 * Show the education tooltip for a given concept ID.
	 * Called by parent components (through `bind:this`) when education indicators are clicked.
	 */
	export function showEntry(id: string): void {
		const entry = getEducation(id);
		if (entry) activeEntry = entry;
	}

	function closeEntry(): void {
		activeEntry = null;
	}

	/** Keyboard parity for the backdrop click. */
	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape' && calc.educationMode && activeEntry) closeEntry();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Active tooltip (centered overlay) -->
{#if calc.educationMode && activeEntry}
	<div class="fixed inset-0 z-[9990]" role="presentation">
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.3); backdrop-filter: blur(2px);"
			onclick={closeEntry}
			aria-label="Close tooltip"
			tabindex={-1}
		></button>
		<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-10">
			<ConceptTooltip entry={activeEntry} visible={true} onClose={closeEntry} />
		</div>
	</div>
{/if}

<!-- Education toggle button (always visible in header area). Its name is its visible label. -->
<button
	type="button"
	onclick={() => (calc.educationMode = !calc.educationMode)}
	class="flex items-center gap-1.5 text-[10px] font-medium px-2.5 py-1.5 rounded-lg transition-all motion-reduce:transition-none duration-200 cursor-pointer"
	style:background={calc.educationMode ? 'var(--calc-accent-glow)' : 'var(--calc-surface)'}
	style:color={calc.educationMode ? 'var(--calc-accent)' : 'var(--calc-text-muted)'}
	style:border="1px solid {calc.educationMode ? 'var(--calc-accent)' : 'var(--calc-border)'}"
	style:box-shadow={calc.educationMode ? '0 0 12px rgba(99,102,241,0.2)' : undefined}
	aria-pressed={calc.educationMode}
	title="Toggle education mode (E)"
>
	<IconSchool size={12} aria-hidden="true" />
	Learn
</button>
