<script lang="ts">
	import type { FormField } from '$lib/api/forms';

	interface Props {
		field: FormField;
		value?: number | [number, number];
		error?: string[];
		onchange?: (value: number | [number, number]) => void;
	}

	let props: Props = $props();

	const min = $derived(props.field.validation?.min ?? 0);
	const max = $derived(props.field.validation?.max ?? 100);
	const step = $derived(props.field.validation?.step ?? 1);
	const isRange = $derived(props.field.attributes?.range_type === 'range');
	const showValue = $derived(props.field.attributes?.show_value !== false);
	const showTicks = $derived(props.field.attributes?.show_ticks || false);
	const prefix = $derived(props.field.attributes?.prefix || '');
	const suffix = $derived(props.field.attributes?.suffix || '');
	const tickCount = $derived(props.field.attributes?.tick_count || 5);

	let singleValue = $state<number>(0);
	let rangeStart = $state<number>(0);
	let rangeEnd = $state<number>(100);

	$effect(() => {
		singleValue = min;
	});

	$effect(() => {
		rangeStart = min;
	});

	$effect(() => {
		rangeEnd = max;
	});

	// Sync state values with value prop changes
	$effect(() => {
		if (typeof props.value === 'number') {
			singleValue = props.value as number;
		} else if (Array.isArray(props.value)) {
			rangeStart = (props.value as [number, number])[0];
			rangeEnd = (props.value as [number, number])[1];
		} else {
			// Initialize from defaults if no value provided
			singleValue = min;
			rangeStart = min;
			rangeEnd = max;
		}
	});

	function formatValue(val: number): string {
		return `${prefix}${val}${suffix}`;
	}

	function handleSingleChange(event: Event) {
		const target = event.target as HTMLInputElement;
		singleValue = parseFloat(target.value);
		props.onchange?.(singleValue);
	}

	function handleRangeStartChange(event: Event) {
		const target = event.target as HTMLInputElement;
		rangeStart = Math.min(parseFloat(target.value), rangeEnd - step);
		props.onchange?.([rangeStart, rangeEnd]);
	}

	function handleRangeEndChange(event: Event) {
		const target = event.target as HTMLInputElement;
		rangeEnd = Math.max(parseFloat(target.value), rangeStart + step);
		props.onchange?.([rangeStart, rangeEnd]);
	}

	function generateTicks(): number[] {
		const ticks: number[] = [];
		const interval = (max - min) / (tickCount - 1);
		for (let i = 0; i < tickCount; i++) {
			ticks.push(min + interval * i);
		}
		return ticks;
	}

	function getFilledPercentage(): string {
		if (isRange) {
			const startPercent = ((rangeStart - min) / (max - min)) * 100;
			const endPercent = ((rangeEnd - min) / (max - min)) * 100;
			return `linear-gradient(to right, #e5e7eb ${startPercent}%, #2563eb ${startPercent}%, #2563eb ${endPercent}%, #e5e7eb ${endPercent}%)`;
		}
		const percent = ((singleValue - min) / (max - min)) * 100;
		return `linear-gradient(to right, #2563eb ${percent}%, #e5e7eb ${percent}%)`;
	}
</script>

<div class="range-slider-field">
	<label class="field-label" for={`field-${props.field.name}`}>
		{props.field.label}
		{#if props.field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="field-help">{props.field.help_text}</p>
	{/if}

	<div class="slider-container">
		{#if showValue}
			<div class="value-display">
				{#if isRange}
					<span class="value-badge start">{formatValue(rangeStart)}</span>
					<span class="value-separator">to</span>
					<span class="value-badge end">{formatValue(rangeEnd)}</span>
				{:else}
					<span class="value-badge">{formatValue(singleValue)}</span>
				{/if}
			</div>
		{/if}

		<div class="slider-wrapper">
			{#if isRange}
				<div class="range-track" style={`background: ${getFilledPercentage()}`}></div>
				<input
					type="range"
					id={`${props.field.name}-start`}
					name={`${props.field.name}[start]`}
					{min}
					{max}
					{step}
					value={rangeStart}
					class="range-input range-start"
					oninput={handleRangeStartChange}
					aria-label={`${props.field.label} start value`}
				/>
				<input
					type="range"
					id={`${props.field.name}-end`}
					name={`${props.field.name}[end]`}
					{min}
					{max}
					{step}
					value={rangeEnd}
					class="range-input range-end"
					oninput={handleRangeEndChange}
					aria-label={`${props.field.label} end value`}
				/>
			{:else}
				<input
					type="range"
					id={`field-${props.field.name}`}
					name={props.field.name}
					{min}
					{max}
					{step}
					value={singleValue}
					class="range-input single"
					style={`background: ${getFilledPercentage()}`}
					oninput={handleSingleChange}
				/>
			{/if}
		</div>

		{#if showTicks}
			<div class="tick-marks">
				{#each generateTicks() as tick}
					<div class="tick">
						<span class="tick-mark"></span>
						<span class="tick-label">{formatValue(tick)}</span>
					</div>
				{/each}
			</div>
		{:else}
			<div class="min-max-labels">
				<span>{formatValue(min)}</span>
				<span>{formatValue(max)}</span>
			</div>
		{/if}
	</div>

	{#if error && error.length > 0}
		<div class="field-error">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.range-slider-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.field-label {
		font-weight: 500;
		font-size: 0.875rem;
		color: #374151;
	}

	.required {
		color: #dc2626;
		margin-left: 0.25rem;
	}

	.field-help {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.slider-container {
		padding: 0.5rem 0;
	}

	.value-display {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.value-badge {
		display: inline-flex;
		align-items: center;
		padding: 0.375rem 0.75rem;
		background-color: #2563eb;
		color: white;
		border-radius: 9999px;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.value-separator {
		color: #6b7280;
		font-size: 0.75rem;
	}

	/* 2026 Mobile-First: Increased height for larger thumb */
	.slider-wrapper {
		position: relative;
		height: 32px; /* Increased for larger touch targets */
		display: flex;
		align-items: center;
		touch-action: manipulation;
	}

	.range-track {
		position: absolute;
		width: 100%;
		height: 8px;
		background: #e5e7eb;
		border-radius: 4px;
		pointer-events: none;
	}

	.range-input {
		width: 100%;
		height: 8px;
		-webkit-appearance: none;
		appearance: none;
		background: transparent;
		border-radius: 4px;
		cursor: pointer;
	}

	.range-input.single {
		background: #e5e7eb;
	}

	.range-input.range-start,
	.range-input.range-end {
		position: absolute;
		pointer-events: none;
		background: transparent;
	}

	/* 2026 Mobile-First: Enhanced slider thumb touch targets */
	.range-input::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 28px; /* Enhanced for touch */
		height: 28px; /* Enhanced for touch */
		background: #2563eb;
		border-radius: 50%;
		cursor: pointer;
		border: 3px solid white;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
		transition: transform 0.2s;
		pointer-events: auto;
		touch-action: manipulation;
	}

	.range-input::-webkit-slider-thumb:hover {
		transform: scale(1.1);
	}

	.range-input::-moz-range-thumb {
		width: 28px; /* Enhanced for touch */
		height: 28px; /* Enhanced for touch */
		background: #2563eb;
		border-radius: 50%;
		cursor: pointer;
		border: 3px solid white;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
		transition: transform 0.2s;
		pointer-events: auto;
	}

	.range-input::-moz-range-thumb:hover {
		transform: scale(1.1);
	}

	.tick-marks {
		display: flex;
		justify-content: space-between;
		margin-top: 0.5rem;
		padding: 0 10px;
	}

	.tick {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.25rem;
	}

	.tick-mark {
		width: 1px;
		height: 8px;
		background-color: #9ca3af;
	}

	.tick-label {
		font-size: 0.625rem;
		color: #6b7280;
	}

	.min-max-labels {
		display: flex;
		justify-content: space-between;
		margin-top: 0.25rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>
