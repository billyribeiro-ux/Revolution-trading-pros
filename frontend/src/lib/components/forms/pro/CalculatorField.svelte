<script lang="ts">
	import type { FormField } from '$lib/api/forms';
	import { tryMathEval } from '$lib/utils/safe-math-parser';

	interface CalculationVariable {
		name: string;
		label: string;
		value: number;
	}

	interface Props {
		field: FormField;
		value?: number;
		formData?: Record<string, unknown>;
		error?: string[];
		onchange?: (value: number) => void;
	}

	let props: Props = $props();

	const formula = $derived(props.field.attributes?.formula || '');
	const prefix = $derived(props.field.attributes?.prefix || '');
	const suffix = $derived(props.field.attributes?.suffix || '');
	const decimals = $derived(props.field.attributes?.decimals ?? 2);
	const showFormula = $derived(props.field.attributes?.show_formula || false);
	const variables = $derived<CalculationVariable[]>(props.field.attributes?.variables || []);

	const calculatedValue = $derived.by(() => {
		if (!formula) return 0;

		try {
			// Replace variable placeholders with actual values
			let expression = formula;

			// Replace field references like {field_name} with values from formData
			expression = expression.replace(/\{([^}]+)\}/g, (_: string, fieldName: string) => {
				const fieldValue = (props.formData ?? {})[fieldName];
				if (typeof fieldValue === 'number') return String(fieldValue);
				if (typeof fieldValue === 'string') {
					const parsed = parseFloat(fieldValue);
					return isNaN(parsed) ? '0' : String(parsed);
				}
				return '0';
			});

			// Safe evaluation using recursive descent parser (no code execution)
			// Only allows: numbers, +, -, *, /, %, parentheses
			const numericResult = tryMathEval(expression);

			if (isNaN(numericResult) || !isFinite(numericResult)) {
				return 0;
			}

			return numericResult;
		} catch {
			console.error('Calculator evaluation error');
			return 0;
		}
	});

	function formatValue(val: number): string {
		const formatted = val.toFixed(decimals);
		return `${prefix}${formatted}${suffix}`;
	}

	$effect(() => {
		const newValue = calculatedValue;
		if (newValue !== value) {
			props.onchange?.(newValue);
		}
	});
</script>

<div class="calculator-field">
	<label for="calculator-{props.field.name}" class="field-label">
		{props.field.label}
		{#if props.field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if props.field.help_text}
		<p class="field-help">{props.field.help_text}</p>
	{/if}

	<div class="calculator-display">
		<span class="calculator-value">{formatValue(calculatedValue)}</span>
		<span class="calculator-badge">Auto-calculated</span>
	</div>

	{#if showFormula && variables.length > 0}
		<div class="formula-breakdown">
			<div class="formula-header">
				<svg
					width="14"
					height="14"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<polyline points="4 17 10 11 4 5"></polyline>
					<line x1="12" y1="19" x2="20" y2="19"></line>
				</svg>
				<span>Calculation breakdown</span>
			</div>
			<div class="variables-list">
				{#each variables as variable}
					<div class="variable-row">
						<span class="variable-label">{variable.label}:</span>
						<span class="variable-value">{formatValue(variable.value)}</span>
					</div>
				{/each}
			</div>
			{#if formula}
				<div class="formula-row">
					<span class="formula-label">Formula:</span>
					<code class="formula-expression">{formula}</code>
				</div>
			{/if}
		</div>
	{/if}

	<input id="calculator-{props.field.name}" type="hidden" name={props.field.name} value={calculatedValue} />

	{#if error && error.length > 0}
		<div class="field-error">
			{#each error as err}
				<p>{err}</p>
			{/each}
		</div>
	{/if}
</div>

<style>
	.calculator-field {
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

	.calculator-display {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		background: linear-gradient(135deg, #1e3a8a 0%, #2563eb 100%);
		border-radius: 0.5rem;
		color: white;
	}

	.calculator-value {
		font-size: 1.75rem;
		font-weight: 700;
		font-family: 'Courier New', monospace;
	}

	.calculator-badge {
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		padding: 0.25rem 0.5rem;
		background-color: rgba(255, 255, 255, 0.2);
		border-radius: 0.25rem;
	}

	.formula-breakdown {
		padding: 0.75rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.375rem;
		font-size: 0.75rem;
	}

	.formula-header {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: #6b7280;
		margin-bottom: 0.5rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px dashed #e5e7eb;
	}

	.variables-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.variable-row {
		display: flex;
		justify-content: space-between;
		color: #374151;
	}

	.variable-label {
		color: #6b7280;
	}

	.variable-value {
		font-family: 'Courier New', monospace;
		font-weight: 500;
	}

	.formula-row {
		margin-top: 0.5rem;
		padding-top: 0.5rem;
		border-top: 1px dashed #e5e7eb;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.formula-label {
		color: #6b7280;
	}

	.formula-expression {
		font-family: 'Courier New', monospace;
		background-color: #e5e7eb;
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.6875rem;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}
</style>
