<script lang="ts">
	import { GraduationCap } from '@lucide/svelte';
	import ConceptTooltip from './ConceptTooltip.svelte';
	import { getEducation } from './education-content.js';
	import type { EducationEntry } from '../../engine/types.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let activeEntry = $state<EducationEntry | null>(null);

	/**
	 * Show the education tooltip for a given concept ID.
	 * Called by parent components when education indicators are clicked.
	 */
	export function showEntry(id: string): void {
		const entry = getEducation(id);
		if (entry) activeEntry = entry;
	}

	function closeEntry(): void {
		activeEntry = null;
	}
</script>

<!-- Active tooltip (centered overlay) -->
{#if calc.educationMode && activeEntry}
	<div class="fixed inset-0 z-[9990]" role="presentation">
		<button
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

<!-- Education toggle button (always visible in header area) -->
<button
	onclick={() => { calc.educationMode = !calc.educationMode; }}
	class="flex items-center gap-1.5 text-[10px] font-medium px-2.5 py-1.5 rounded-lg transition-all duration-200 cursor-pointer"
	style={calc.educationMode
		? 'background: var(--calc-accent-glow); color: var(--calc-accent); border: 1px solid var(--calc-accent); box-shadow: 0 0 12px rgba(99,102,241,0.2);'
		: 'background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);'}
	aria-label="Toggle education mode"
	aria-pressed={calc.educationMode}
	title="Toggle education mode (E)"
>
	<GraduationCap size={12} />
	{calc.educationMode ? 'Learning' : 'Learn'}
</button>
