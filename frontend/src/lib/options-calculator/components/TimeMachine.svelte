<script lang="ts">
	import { Play, Pause } from '@lucide/svelte';
	import { formatCurrency } from '../utils/formatters.js';
	import type { CalculatorState } from '../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let isPlaying = $state(false);
	let speed = $state(1);
	let intervalId: ReturnType<typeof setInterval> | null = $state(null);

	const SPEEDS = [1, 2, 5, 10];

	function togglePlay() {
		if (isPlaying) {
			stopPlayback();
		} else {
			startPlayback();
		}
	}

	function startPlayback() {
		isPlaying = true;
		if (calc.timeMachineDay >= calc.timeMachineMaxDays) {
			calc.timeMachineDay = 0;
		}
		intervalId = setInterval(() => {
			if (calc.timeMachineDay >= calc.timeMachineMaxDays) {
				stopPlayback();
				return;
			}
			calc.timeMachineDay = Math.min(calc.timeMachineDay + speed, calc.timeMachineMaxDays);
		}, 50);
	}

	function stopPlayback() {
		isPlaying = false;
		if (intervalId !== null) {
			clearInterval(intervalId);
			intervalId = null;
		}
	}

	function handleSliderInput(e: Event) {
		const val = parseInt((e.currentTarget as HTMLInputElement).value, 10);
		calc.timeMachineDay = val;
		if (isPlaying) stopPlayback();
	}

	function cycleSpeed() {
		const idx = SPEEDS.indexOf(speed);
		speed = SPEEDS[(idx + 1) % SPEEDS.length];
	}

	let remainingDTE = $derived(Math.max(0, calc.timeMachineMaxDays - calc.timeMachineDay));

	let snapshot = $derived(calc.currentTimeMachineSnapshot);

	let sliderPercent = $derived(
		calc.timeMachineMaxDays > 0 ? (calc.timeMachineDay / calc.timeMachineMaxDays) * 100 : 0
	);

	// Key DTE markers
	let markers = $derived.by(() => {
		const max = calc.timeMachineMaxDays;
		const keyDTEs = [30, 14, 7, 1];
		return keyDTEs
			.filter((dte) => dte < max)
			.map((dte) => ({
				dte,
				position: ((max - dte) / max) * 100
			}));
	});

	$effect(() => {
		return () => {
			if (intervalId !== null) clearInterval(intervalId);
		};
	});
</script>

<div class="flex flex-col gap-2">
	<!-- Header row -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-2">
			<button
				onclick={togglePlay}
				class="flex items-center justify-center w-7 h-7 rounded-lg transition-colors cursor-pointer"
				style="background: var(--calc-accent); color: white;"
				aria-label={isPlaying ? 'Pause' : 'Play'}
			>
				{#if isPlaying}
					<Pause size={14} />
				{:else}
					<Play size={14} />
				{/if}
			</button>

			<button
				onclick={cycleSpeed}
				class="text-[10px] font-bold px-2 py-1 rounded-lg cursor-pointer"
				style="background: var(--calc-surface); color: var(--calc-accent); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
			>
				{speed}x
			</button>

			<span
				class="text-xs font-semibold"
				style="color: var(--calc-text); font-family: var(--calc-font-mono);"
			>
				Day {calc.timeMachineDay}
			</span>
			<span class="text-[10px]" style="color: var(--calc-text-muted);">
				({remainingDTE} DTE remaining)
			</span>
		</div>

		{#if snapshot}
			<div class="flex items-center gap-3">
				<div class="flex items-center gap-1">
					<span class="text-[10px]" style="color: var(--calc-text-muted);">Call:</span>
					<span
						class="text-xs font-semibold"
						style="color: var(--calc-call); font-family: var(--calc-font-mono);"
					>
						{formatCurrency(snapshot.callPrice)}
					</span>
				</div>
				<div class="flex items-center gap-1">
					<span class="text-[10px]" style="color: var(--calc-text-muted);">Put:</span>
					<span
						class="text-xs font-semibold"
						style="color: var(--calc-put); font-family: var(--calc-font-mono);"
					>
						{formatCurrency(snapshot.putPrice)}
					</span>
				</div>
				<div class="flex items-center gap-1">
					<span class="text-[10px]" style="color: var(--calc-text-muted);">Δ:</span>
					<span
						class="text-xs"
						style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
					>
						{snapshot.greeks.first.delta.toFixed(3)}
					</span>
				</div>
			</div>
		{/if}
	</div>

	<!-- Slider -->
	<div class="relative w-full h-8">
		<!-- Track background -->
		<div
			class="absolute top-3 left-0 right-0 h-2 rounded-full"
			style="background: var(--calc-surface-hover);"
		>
			<!-- Filled track -->
			<div
				class="absolute top-0 left-0 h-full rounded-full transition-all"
				style="width: {sliderPercent}%; background: linear-gradient(90deg, var(--calc-accent), var(--calc-warning));"
			></div>
		</div>

		<!-- Key DTE markers -->
		{#each markers as marker (marker.dte)}
			<div
				class="absolute top-0 flex flex-col items-center"
				style="left: {marker.position}%; transform: translateX(-50%);"
			>
				<span
					class="text-[7px] font-medium"
					style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
				>
					{marker.dte}d
				</span>
				<div class="w-px h-2" style="background: var(--calc-border);"></div>
			</div>
		{/each}

		<!-- Native range input (invisible, on top for interaction) -->
		<input
			type="range"
			min="0"
			max={calc.timeMachineMaxDays}
			value={calc.timeMachineDay}
			oninput={handleSliderInput}
			class="absolute top-1.5 left-0 w-full h-5 opacity-0 cursor-pointer z-10"
		/>

		<!-- Custom thumb -->
		<div
			class="absolute top-1.5 w-4 h-4 rounded-full pointer-events-none"
			style="left: calc({sliderPercent}% - 8px); background: var(--calc-accent); box-shadow: 0 0 8px var(--calc-accent-glow); border: 2px solid var(--calc-bg);"
		></div>
	</div>

	<!-- Timeline labels -->
	<div class="flex items-center justify-between">
		<span
			class="text-[9px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);">Today</span
		>
		<span
			class="text-[9px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
			>Expiration ({calc.timeMachineMaxDays}d)</span
		>
	</div>
</div>
