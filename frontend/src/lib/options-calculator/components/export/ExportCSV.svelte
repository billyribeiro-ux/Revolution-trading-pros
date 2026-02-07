<script lang="ts">
	import { allGreeks } from '../../engine/greeks.js';
	import { generateGreeksCSV, downloadCSV } from '../../utils/export-utils.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	/**
	 * Generate and download a CSV file containing Greeks
	 * across 51 strikes centered on the current spot price.
	 */
	export function exportCSV(): void {
		const spot = calc.spotPrice;
		const numStrikes = 51;
		const range = spot * 0.3;
		const step = (range * 2) / (numStrikes - 1);

		const greeksData: { strike: number; greeks: ReturnType<typeof allGreeks> }[] = [];
		for (let i = 0; i < numStrikes; i++) {
			const strike = spot - range + i * step;
			const strikeInputs = { ...calc.inputs, strikePrice: strike };
			const greeks = allGreeks(strikeInputs, calc.optionType);
			greeksData.push({ strike, greeks });
		}

		const csv = generateGreeksCSV(calc.inputs, calc.optionType, greeksData);
		const ticker = calc.activeTicker || 'custom';
		downloadCSV(csv, `greeks-${ticker}-${calc.optionType}`);
		calc.addToast('success', 'Greeks CSV exported!');
	}
</script>
