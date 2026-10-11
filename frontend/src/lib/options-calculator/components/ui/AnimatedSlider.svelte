<script lang="ts">
	import type { Attachment } from 'svelte/attachments';
	import type { ClassValue } from 'svelte/elements';

	interface Props {
		value: number;
		min: number;
		max: number;
		step: number;
		label: string;
		unit?: string;
		displayMultiplier?: number;
		displayDecimals?: number;
		symbol?: string;
		onchange: (value: number) => void;
		class?: ClassValue;
	}

	let {
		value,
		min,
		max,
		step,
		label,
		unit = '',
		displayMultiplier = 1,
		displayDecimals = 2,
		symbol = '',
		onchange,
		class: className
	}: Props = $props();

	let isEditing = $state(false);
	let editValue = $state('');
	/** Set when the editor closes from the keyboard, so focus returns to the value button. */
	let refocusValue = false;

	const displayValue = $derived((value * displayMultiplier).toFixed(displayDecimals));
	const unitSuffix = $derived(unit === '%' ? '%' : unit && unit !== '$' ? ` ${unit}` : '');
	const fillPercent = $derived(((value - min) / (max - min)) * 100);
	const sliderBackground = $derived(
		`linear-gradient(to right, var(--calc-accent) 0%, var(--calc-accent) ${fillPercent}%, var(--calc-border) ${fillPercent}%, var(--calc-border) 100%)`
	);
	const rangeDecimals = $derived(displayDecimals > 2 ? 1 : 0);
	const rangeUnit = $derived(unit === '%' ? '%' : '');

	function startEdit() {
		editValue = displayValue;
		isEditing = true;
	}

	function commitEdit() {
		// Enter/Escape already closed the editor; ignore the blur that follows its removal.
		if (!isEditing) return;
		const parsed = parseFloat(editValue);
		if (!Number.isNaN(parsed)) {
			onchange(Math.max(min, Math.min(max, parsed / displayMultiplier)));
		}
		isEditing = false;
	}

	function handleEditKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			refocusValue = true;
			commitEdit();
		} else if (event.key === 'Escape') {
			refocusValue = true;
			isEditing = false;
		}
	}

	function increment() {
		onchange(Math.min(max, value + step));
	}

	function decrement() {
		onchange(Math.max(min, value - step));
	}

	const focusEditor: Attachment<HTMLInputElement> = (input) => {
		input.focus();
		input.select();
	};

	const restoreFocus: Attachment<HTMLButtonElement> = (button) => {
		if (!refocusValue) return;
		refocusValue = false;
		button.focus();
	};
</script>

<div class={['flex flex-col gap-1.5', className]}>
	<!-- Label row -->
	<div class="flex items-center justify-between">
		<span
			class="flex items-center gap-1.5 text-xs font-medium"
			style="color: var(--calc-text-secondary); font-family: var(--calc-font-body);"
		>
			{#if symbol}
				<span
					class="inline-flex items-center justify-center w-5 h-5 rounded text-[10px] font-bold"
					style="background: var(--calc-accent-glow); color: var(--calc-accent); font-family: var(--calc-font-mono);"
				>
					{symbol}
				</span>
			{/if}
			{label}
		</span>

		<!-- Value display / edit -->
		<div class="flex items-center gap-1">
			<button
				type="button"
				onclick={decrement}
				class="w-5 h-5 flex items-center justify-center rounded text-xs transition-colors cursor-pointer"
				style="background: var(--calc-surface-hover); color: var(--calc-text-secondary);"
				aria-label="Decrease {label}">−</button
			>

			{#if isEditing}
				<input
					{@attach focusEditor}
					type="text"
					inputmode="decimal"
					bind:value={editValue}
					onblur={commitEdit}
					onkeydown={handleEditKeydown}
					aria-label="{label} value"
					class="w-16 text-right text-xs px-1 py-0.5 rounded outline-none"
					style="background: var(--calc-surface-active); color: var(--calc-text); font-family: var(--calc-font-mono); border: 1px solid var(--calc-accent);"
				/>
			{:else}
				<button
					{@attach restoreFocus}
					type="button"
					onclick={startEdit}
					class="text-xs text-right px-1 py-0.5 rounded cursor-text transition-colors min-w-[3.5rem]"
					style="color: var(--calc-text); font-family: var(--calc-font-mono);"
				>
					<span class="sr-only">Edit {label}:</span>
					{displayValue}{unitSuffix}
				</button>
			{/if}

			<button
				type="button"
				onclick={increment}
				class="w-5 h-5 flex items-center justify-center rounded text-xs transition-colors cursor-pointer"
				style="background: var(--calc-surface-hover); color: var(--calc-text-secondary);"
				aria-label="Increase {label}">+</button
			>
		</div>
	</div>

	<!-- Slider. One-way `value` + `oninput`, not `bind:value`: a value off the step grid (100
	     with min 0.01, step 0.5) is snapped by the browser, and a binding writes that snapped
	     value back into the calculator when it hydrates the prerendered page. -->
	<div class="relative">
		<input
			type="range"
			{min}
			{max}
			{step}
			{value}
			oninput={(event) => onchange(event.currentTarget.valueAsNumber)}
			class="calc-slider w-full"
			style:background={sliderBackground}
			aria-label={label}
			aria-valuetext="{displayValue}{unitSuffix}"
		/>
	</div>

	<!-- Min/Max labels -->
	<div
		class="flex justify-between text-[10px]"
		style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
	>
		<span>{(min * displayMultiplier).toFixed(rangeDecimals)}{rangeUnit}</span>
		<span>{(max * displayMultiplier).toFixed(rangeDecimals)}{rangeUnit}</span>
	</div>
</div>
