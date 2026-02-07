<script lang="ts">
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
		class?: string;
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
		class: className = '',
	}: Props = $props();

	let isEditing = $state(false);
	let editValue = $state('');

	let displayValue = $derived(() => {
		const v = value * displayMultiplier;
		return v.toFixed(displayDecimals);
	});

	let fillPercent = $derived(((value - min) / (max - min)) * 100);

	function handleSliderInput(e: Event) {
		const target = e.currentTarget as HTMLInputElement;
		onchange(parseFloat(target.value));
	}

	function startEdit() {
		editValue = displayValue();
		isEditing = true;
	}

	function commitEdit() {
		const parsed = parseFloat(editValue);
		if (!isNaN(parsed)) {
			const actual = parsed / displayMultiplier;
			const clamped = Math.max(min, Math.min(max, actual));
			onchange(clamped);
		}
		isEditing = false;
	}

	function handleEditKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') commitEdit();
		if (e.key === 'Escape') isEditing = false;
	}

	function increment() {
		const next = Math.min(max, value + step);
		onchange(next);
	}

	function decrement() {
		const prev = Math.max(min, value - step);
		onchange(prev);
	}
</script>

<div class="flex flex-col gap-1.5 {className}">
	<!-- Label row -->
	<div class="flex items-center justify-between">
		<span class="flex items-center gap-1.5 text-xs font-medium" style="color: var(--calc-text-secondary); font-family: var(--calc-font-body);">
			{#if symbol}
				<span class="inline-flex items-center justify-center w-5 h-5 rounded text-[10px] font-bold" style="background: var(--calc-accent-glow); color: var(--calc-accent); font-family: var(--calc-font-mono);">
					{symbol}
				</span>
			{/if}
			{label}
		</span>

		<!-- Value display / edit -->
		<div class="flex items-center gap-1">
			<button
				onclick={decrement}
				class="w-5 h-5 flex items-center justify-center rounded text-xs transition-colors cursor-pointer"
				style="background: var(--calc-surface-hover); color: var(--calc-text-secondary);"
				aria-label="Decrease {label}"
			>−</button>

			{#if isEditing}
				<!-- svelte-ignore a11y_autofocus -->
				<input
					type="text"
					bind:value={editValue}
					onblur={commitEdit}
					onkeydown={handleEditKeydown}
					class="w-16 text-right text-xs px-1 py-0.5 rounded outline-none"
					style="background: var(--calc-surface-active); color: var(--calc-text); font-family: var(--calc-font-mono); border: 1px solid var(--calc-accent);"
					autofocus
				/>
			{:else}
				<button
					onclick={startEdit}
					class="text-xs text-right px-1 py-0.5 rounded cursor-text transition-colors min-w-[3.5rem]"
					style="color: var(--calc-text); font-family: var(--calc-font-mono);"
					aria-label="Edit {label} value"
				>
					{displayValue()}{unit === '%' ? '%' : unit === '$' ? '' : ` ${unit}`}
				</button>
			{/if}

			<button
				onclick={increment}
				class="w-5 h-5 flex items-center justify-center rounded text-xs transition-colors cursor-pointer"
				style="background: var(--calc-surface-hover); color: var(--calc-text-secondary);"
				aria-label="Increase {label}"
			>+</button>
		</div>
	</div>

	<!-- Slider -->
	<div class="relative">
		<input
			type="range"
			{min}
			{max}
			{step}
			{value}
			oninput={handleSliderInput}
			class="calc-slider w-full"
			style="background: linear-gradient(to right, var(--calc-accent) 0%, var(--calc-accent) {fillPercent}%, var(--calc-border) {fillPercent}%, var(--calc-border) 100%);"
			aria-label={label}
		/>
	</div>

	<!-- Min/Max labels -->
	<div class="flex justify-between text-[10px]" style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);">
		<span>{(min * displayMultiplier).toFixed(displayDecimals > 2 ? 1 : 0)}{unit === '%' ? '%' : ''}</span>
		<span>{(max * displayMultiplier).toFixed(displayDecimals > 2 ? 1 : 0)}{unit === '%' ? '%' : ''}</span>
	</div>
</div>
