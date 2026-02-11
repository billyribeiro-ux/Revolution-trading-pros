<script lang="ts">
	import { Save, X } from '@lucide/svelte';
	import gsap from 'gsap';
	import { saveConfig } from '../../utils/saved-configs.js';
	import { formatCurrency } from '../../utils/formatters.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
		onSaved: () => void;
	}

	let { calc, onSaved }: Props = $props();

	let modalEl: HTMLDivElement | undefined = $state();
	let nameInputEl: HTMLInputElement | undefined = $state();
	let name = $state('');
	let description = $state('');
	let tags = $state('');

	$effect(() => {
		if (calc.showSaveModal) {
			const ticker = calc.activeTicker;
			const type = calc.optionType === 'call' ? 'Call' : 'Put';
			name = ticker ? `${ticker} ${type} Analysis` : `${type} Analysis`;
			description = '';
			tags = '';

			requestAnimationFrame(() => {
				if (modalEl) {
					gsap.fromTo(
						modalEl,
						{ scale: 0.92, opacity: 0 },
						{ scale: 1, opacity: 1, duration: 0.25, ease: 'back.out(1.5)' }
					);
				}
				nameInputEl?.focus();
				nameInputEl?.select();
			});
		}
	});

	function handleSave(): void {
		const trimmedName = name.trim();
		if (!trimmedName) return;

		saveConfig({
			name: trimmedName,
			description: description.trim() || undefined,
			inputs: { ...calc.inputs },
			optionType: calc.optionType,
			mode: calc.calculatorMode,
			ticker: calc.activeTicker || undefined,
			strategyLegs: calc.strategyLegs.length > 0 ? [...calc.strategyLegs] : undefined,
			tags: tags.trim()
				? tags
						.split(',')
						.map((t) => t.trim())
						.filter(Boolean)
				: undefined
		});

		calc.addToast('success', `Configuration "${trimmedName}" saved!`);
		onSaved();
		calc.showSaveModal = false;
	}

	function handleClose(): void {
		calc.showSaveModal = false;
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (!calc.showSaveModal) return;
		if (e.key === 'Escape') handleClose();
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSave();
		}
	}

	let canSave = $derived(name.trim().length > 0);
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showSaveModal}
	<div
		class="fixed inset-0 z-[9996] flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-label="Save Configuration"
	>
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
			onclick={handleClose}
			aria-label="Close"
			tabindex={-1}
		></button>

		<div
			bind:this={modalEl}
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
					<Save size={16} style="color: var(--calc-accent);" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Save Configuration
					</h3>
				</div>
				<button
					onclick={handleClose}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<X size={16} />
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
				<span>\u03c3={(calc.volatility * 100).toFixed(1)}%</span>
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
					bind:this={nameInputEl}
					type="text"
					bind:value={name}
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
					onclick={handleClose}
					class="flex-1 text-xs py-2.5 rounded-lg cursor-pointer"
					style="color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					>Cancel</button
				>
				<button
					onclick={handleSave}
					disabled={!canSave}
					class="flex-1 flex items-center justify-center gap-1.5 text-xs font-semibold py-2.5 rounded-lg cursor-pointer transition-all duration-150"
					style="background: var(--calc-accent); color: white; opacity: {canSave ? 1 : 0.5};"
				>
					<Save size={12} />
					Save
				</button>
			</div>
		</div>
	</div>
{/if}
