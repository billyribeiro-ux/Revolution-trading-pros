<script lang="ts">
	import { trapFocus } from '../../utils/focus-trap.js';
	import IconDeviceFloppy from '@tabler/icons-svelte-runes/icons/device-floppy';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { backOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import { saveConfig } from '../../utils/saved-configs.js';
	import { formatCurrency } from '../../utils/formatters.js';
	import { getCalculator } from '../../state/context.js';

	interface Props {
		/** Called after a configuration has been written to storage. */
		onSaved: () => void;
	}

	let { onSaved }: Props = $props();

	const calc = getCalculator();
	const uid = $props.id();

	interface Draft {
		name: string;
		description: string;
		tags: string;
	}

	/** Pre-fills the name from the current ticker and option type, e.g. "AAPL Call Analysis". */
	function suggestedName(): string {
		const type = calc.optionType === 'call' ? 'Call' : 'Put';
		return calc.activeTicker ? `${calc.activeTicker} ${type} Analysis` : `${type} Analysis`;
	}

	/** "earnings, bullish, earnings" → ["earnings", "bullish"]; tags key a list, so no repeats. */
	function parseTags(raw: string): string[] | undefined {
		const tags = new Set(
			raw
				.split(',')
				.map((tag) => tag.trim())
				.filter(Boolean)
		);
		return tags.size > 0 ? [...tags] : undefined;
	}

	function close(): void {
		calc.showSaveModal = false;
	}

	function handleSubmit(event: SubmitEvent, draft: Draft): void {
		event.preventDefault();
		const name = draft.name.trim();
		if (!name) return;

		try {
			saveConfig({
				name,
				description: draft.description.trim() || undefined,
				inputs: { ...calc.inputs },
				optionType: calc.optionType,
				mode: calc.calculatorMode,
				ticker: calc.activeTicker || undefined,
				strategyLegs: calc.strategyLegs.length > 0 ? [...calc.strategyLegs] : undefined,
				tags: parseTags(draft.tags)
			});
		} catch {
			// Storage is full or blocked (private mode): keep the dialog open with the draft intact.
			calc.addToast('error', 'Could not save — browser storage is full or unavailable.');
			return;
		}

		calc.addToast('success', `Configuration "${name}" saved!`);
		onSaved();
		close();
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (calc.showSaveModal && event.key === 'Escape') close();
	}

	/** Focuses and selects the suggested name; hands focus back to the opener on close. */
	const focusName: Attachment<HTMLInputElement> = (input) => {
		const opener = document.activeElement;
		input.focus();
		input.select();
		return () => {
			if (opener instanceof HTMLElement && opener.isConnected) opener.focus();
		};
	};
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showSaveModal}
	<!-- The draft lives exactly as long as the dialog, so every opening starts fresh. -->
	{let name = $state(suggestedName())}
	{let description = $state('')}
	{let tags = $state('')}
	{const canSave = $derived(name.trim().length > 0)}

	<div
		{@attach trapFocus}
		class="fixed inset-0 z-[9996] flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-labelledby="{uid}-title"
	>
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={close}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- A real form: Enter in any field submits (the browser skips it while Save is disabled). -->
		<form
			in:scale={{
				start: 0.92,
				duration: prefersReducedMotion.current ? 0 : 250,
				easing: backOut
			}}
			onsubmit={(event) => handleSubmit(event, { name, description, tags })}
			class="relative z-10 w-full max-w-sm rounded-2xl p-5 flex flex-col gap-4"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 24px 80px rgba(0,0,0,0.5);
			"
		>
			<!-- Header -->
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<IconDeviceFloppy size={16} style="color: var(--calc-accent);" aria-hidden="true" />
					<h3
						id="{uid}-title"
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Save Configuration
					</h3>
				</div>
				<button
					type="button"
					onclick={close}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<IconX size={16} aria-hidden="true" />
				</button>
			</div>

			<!-- Preview of current state -->
			<div
				class="rounded-lg px-3 py-2 text-[10px] flex items-center gap-2 flex-wrap"
				style="background: var(--calc-surface-hover); color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
			>
				<span>{calc.optionType.toUpperCase()}</span>
				<span>S={formatCurrency(calc.spotPrice)}</span>
				<span>K={formatCurrency(calc.strikePrice)}</span>
				<span>σ={(calc.volatility * 100).toFixed(1)}%</span>
			</div>

			<!-- Name field -->
			<label class="flex flex-col gap-1">
				<span
					class="text-[10px] uppercase tracking-wider font-medium"
					style="color: var(--calc-text-muted);"
				>
					Name *
				</span>
				<input
					{@attach focusName}
					type="text"
					bind:value={name}
					aria-required="true"
					class="text-xs px-3 py-2 rounded-lg outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border);"
					placeholder="e.g., AAPL Iron Condor"
				/>
			</label>

			<!-- Description field -->
			<label class="flex flex-col gap-1">
				<span
					class="text-[10px] uppercase tracking-wider font-medium"
					style="color: var(--calc-text-muted);"
				>
					Description
				</span>
				<input
					type="text"
					bind:value={description}
					class="text-xs px-3 py-2 rounded-lg outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border);"
					placeholder="Optional notes..."
				/>
			</label>

			<!-- Tags field -->
			<label class="flex flex-col gap-1">
				<span
					class="text-[10px] uppercase tracking-wider font-medium"
					style="color: var(--calc-text-muted);"
				>
					Tags (comma-separated)
				</span>
				<input
					type="text"
					bind:value={tags}
					class="text-xs px-3 py-2 rounded-lg outline-none"
					style="background: var(--calc-surface-hover); color: var(--calc-text); border: 1px solid var(--calc-border);"
					placeholder="earnings, bullish, weekly"
				/>
			</label>

			<!-- Actions -->
			<div class="flex gap-2">
				<button
					type="button"
					onclick={close}
					class="flex-1 text-xs py-2.5 rounded-lg cursor-pointer"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					>Cancel</button
				>
				<button
					type="submit"
					disabled={!canSave}
					class={[
						'flex-1 flex items-center justify-center gap-1.5 text-xs font-semibold py-2.5 rounded-lg transition-all motion-reduce:transition-none duration-150',
						canSave ? 'cursor-pointer' : 'cursor-not-allowed'
					]}
					style:background="var(--calc-accent)"
					style:color="white"
					style:opacity={canSave ? 1 : 0.5}
				>
					<IconDeviceFloppy size={12} aria-hidden="true" />
					Save
				</button>
			</div>
		</form>
	</div>
{/if}
