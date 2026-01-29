<script lang="ts">
	/**
	 * Calculated Field - Dynamic value computation
	 *
	 * Features:
	 * - Formula-based calculations
	 * - Real-time updates
	 * - Multiple output formats (number, currency, percentage)
	 * - Field references
	 * - Math functions support
	 *
	 * @version 2.0.0
	 */

	import type { FormField } from '$lib/api/forms';
	import { safeMathEval, SafeMathError } from '$lib/utils/safe-math-parser';

	interface Props {
		field: FormField;
		formData: Record<string, any>;
		onchange?: (value: any) => void;
	}

	let { field, formData, onchange }: Props = $props();

	// Configuration from field attributes
	const formula = $derived(field.attributes?.['formula'] ?? '');
	const format = $derived(field.attributes?.['format'] ?? 'number');
	const decimals = $derived(field.attributes?.['decimals'] ?? 2);
	const prefix = $derived(field.attributes?.['prefix'] ?? '');
	const suffix = $derived(field.attributes?.['suffix'] ?? '');
	const currency = $derived(field.attributes?.['currency'] ?? 'USD');
	const showFormula = $derived(field.attributes?.['show_formula'] ?? false);

	let calculatedValue = $state<number | null>(null);
	let displayValue = $state<string>('—');
	let error = $state<string | null>(null);

	// Currency symbols
	const currencySymbols: Record<string, string> = {
		USD: '$',
		EUR: '€',
		GBP: '£',
		JPY: '¥',
		BRL: 'R$',
		CAD: 'C$',
		AUD: 'A$'
	};

	// Calculate when form data changes
	$effect(() => {
		calculate();
	});

	// Perform calculation
	function calculate() {
		if (!formula) {
			error = 'No formula defined';
			return;
		}

		try {
			error = null;
			let expression = formula;

			// Replace field references {field_name} with actual values
			expression = expression.replace(
				/\{([a-zA-Z_][a-zA-Z0-9_]*)\}/g,
				(_: string, fieldName: string) => {
					const value = formData[fieldName];

					if (value === undefined || value === null || value === '') {
						return '0';
					}

					if (typeof value === 'number') {
						return String(value);
					}

					const parsed = parseFloat(value);
					return isNaN(parsed) ? '0' : String(parsed);
				}
			);

			// Process built-in functions
			expression = processFunctions(expression);

			// Evaluate the expression safely
			const result = safeEval(expression);

			if (typeof result === 'number' && !isNaN(result)) {
				calculatedValue = result;
				displayValue = formatValue(result);
				onchange?.(result);
			} else {
				error = 'Invalid calculation result';
				displayValue = '—';
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Calculation error';
			displayValue = '—';
		}
	}

	// Process mathematical functions
	function processFunctions(expr: string): string {
		// SUM
		expr = expr.replace(/SUM\(([^)]+)\)/gi, (_, args) => {
			const values = args.split(',').map((v: string) => parseFloat(v.trim()) || 0);
			return String(values.reduce((a: number, b: number) => a + b, 0));
		});

		// AVG
		expr = expr.replace(/AVG\(([^)]+)\)/gi, (_, args) => {
			const values = args.split(',').map((v: string) => parseFloat(v.trim()) || 0);
			return String(values.reduce((a: number, b: number) => a + b, 0) / values.length);
		});

		// MIN
		expr = expr.replace(/MIN\(([^)]+)\)/gi, (_, args) => {
			const values = args.split(',').map((v: string) => parseFloat(v.trim()) || 0);
			return String(Math.min(...values));
		});

		// MAX
		expr = expr.replace(/MAX\(([^)]+)\)/gi, (_, args) => {
			const values = args.split(',').map((v: string) => parseFloat(v.trim()) || 0);
			return String(Math.max(...values));
		});

		// ROUND
		expr = expr.replace(/ROUND\(([^,]+),?\s*(\d*)\)/gi, (_, value, precision) => {
			const num = parseFloat(value) || 0;
			const prec = parseInt(precision) || 0;
			return String(Number(num.toFixed(prec)));
		});

		// ABS
		expr = expr.replace(/ABS\(([^)]+)\)/gi, (_, value) => {
			return String(Math.abs(parseFloat(value) || 0));
		});

		// CEIL
		expr = expr.replace(/CEIL\(([^)]+)\)/gi, (_, value) => {
			return String(Math.ceil(parseFloat(value) || 0));
		});

		// FLOOR
		expr = expr.replace(/FLOOR\(([^)]+)\)/gi, (_, value) => {
			return String(Math.floor(parseFloat(value) || 0));
		});

		// IF(condition, true_value, false_value)
		expr = expr.replace(/IF\(([^,]+),([^,]+),([^)]+)\)/gi, (_, condition, trueVal, falseVal) => {
			const result = evaluateCondition(condition.trim());
			return result ? trueVal.trim() : falseVal.trim();
		});

		return expr;
	}

	// Evaluate a condition
	function evaluateCondition(condition: string): boolean {
		// Simple comparison operators
		const operators = ['>=', '<=', '!=', '==', '>', '<'];

		for (const op of operators) {
			if (condition.includes(op)) {
				const [left, right] = condition.split(op).map((s) => s.trim());
				const leftVal = parseFloat(left ?? '') || 0;
				const rightVal = parseFloat(right ?? '') || 0;

				switch (op) {
					case '>=':
						return leftVal >= rightVal;
					case '<=':
						return leftVal <= rightVal;
					case '!=':
						return leftVal !== rightVal;
					case '==':
						return leftVal === rightVal;
					case '>':
						return leftVal > rightVal;
					case '<':
						return leftVal < rightVal;
				}
			}
		}

		return parseFloat(condition) !== 0;
	}

	// Safe math expression evaluation using recursive descent parser
	// This replaces dangerous new Function() with a secure whitelist-based approach
	function safeEval(expression: string): number {
		if (!expression.trim()) {
			return 0;
		}

		// Use the safe recursive descent parser (no code execution possible)
		// Only allows: numbers, +, -, *, /, %, parentheses
		try {
			return safeMathEval(expression);
		} catch (err) {
			if (err instanceof SafeMathError) {
				throw new Error(`Invalid expression: ${err.message}`);
			}
			throw new Error('Invalid expression');
		}
	}

	// Format the calculated value
	function formatValue(value: number): string {
		switch (format) {
			case 'currency': {
				const symbol = currencySymbols[currency] || currency + ' ';
				return (
					symbol +
					value.toLocaleString(undefined, {
						minimumFractionDigits: decimals,
						maximumFractionDigits: decimals
					})
				);
			}

			case 'percentage':
				return (
					value.toLocaleString(undefined, {
						minimumFractionDigits: decimals,
						maximumFractionDigits: decimals
					}) + '%'
				);

			case 'integer':
				return Math.round(value).toLocaleString();

			default:
				return (
					prefix +
					value.toLocaleString(undefined, {
						minimumFractionDigits: decimals,
						maximumFractionDigits: decimals
					}) +
					suffix
				);
		}
	}
</script>

<div class="calculated-field">
	<label class="field-label" for={field.name}>
		{field.label}
		{#if field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if field.help_text}
		<p class="field-help">{field.help_text}</p>
	{/if}

	<div
		class="calculated-display"
		class:has-error={!!error}
		id={field.name}
		role="status"
		aria-live="polite"
	>
		<span class="calculated-value">{displayValue}</span>
		{#if calculatedValue !== null}
			<span class="raw-value">({calculatedValue})</span>
		{/if}
	</div>

	{#if showFormula}
		<div class="formula-display">
			<span class="formula-label">Formula:</span>
			<code class="formula-code">{formula}</code>
		</div>
	{/if}

	{#if error}
		<div class="field-error">{error}</div>
	{/if}

	<!-- Hidden input for form submission -->
	<input type="hidden" name={field.name} value={calculatedValue ?? ''} />
</div>

<style>
	.calculated-field {
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

	.calculated-display {
		padding: 1rem 1.25rem;
		background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
		border: 2px solid #0ea5e9;
		border-radius: 0.5rem;
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
	}

	.calculated-display.has-error {
		background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
		border-color: #f87171;
	}

	.calculated-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #0369a1;
		font-family: 'SF Mono', Monaco, 'Courier New', monospace;
	}

	.calculated-display.has-error .calculated-value {
		color: #dc2626;
	}

	.raw-value {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.formula-display {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background-color: #f3f4f6;
		border-radius: 0.25rem;
	}

	.formula-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #6b7280;
	}

	.formula-code {
		font-size: 0.75rem;
		font-family: 'SF Mono', Monaco, 'Courier New', monospace;
		color: #374151;
		background-color: transparent;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
		padding: 0.25rem 0.5rem;
		background-color: #fee2e2;
		border-radius: 0.25rem;
	}
</style>
