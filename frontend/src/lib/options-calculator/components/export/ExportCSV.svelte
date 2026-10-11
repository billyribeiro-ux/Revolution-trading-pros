<script lang="ts">
	import { allGreeks } from '../../engine/greeks.js';
	import { generateGreeksCSV, downloadCSV } from '../../utils/export-utils.js';
	import { getCalculator } from '../../state/context.js';
	import type { AllGreeks } from '../../engine/types.js';

	const calc = getCalculator();

	/** Number of strikes in the export, spread evenly across ±30% of spot. */
	const STRIKE_COUNT = 51;
	const STRIKE_SPAN = 0.3;

	/**
	 * Generate and download a CSV file containing Greeks
	 * across 51 strikes centered on the current spot price.
	 * This component renders nothing: the parent calls this through `bind:this`.
	 */
	export function exportCSV(): void {
		const spot = calc.spotPrice;
		const range = spot * STRIKE_SPAN;
		const step = (range * 2) / (STRIKE_COUNT - 1);

		const greeksData: { strike: number; greeks: AllGreeks }[] = Array.from(
			{ length: STRIKE_COUNT },
			(_, i) => {
				const strike = spot - range + i * step;
				return {
					strike,
					greeks: allGreeks({ ...calc.inputs, strikePrice: strike }, calc.optionType)
				};
			}
		);

		const csv = generateGreeksCSV(calc.inputs, calc.optionType, greeksData);
		const ticker = calc.activeTicker || 'custom';
		downloadCSV(csv, `greeks-${ticker}-${calc.optionType}`);
		calc.addToast('success', 'Greeks CSV exported!');
	}
</script>
