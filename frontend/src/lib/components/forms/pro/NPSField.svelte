<script lang="ts">
	import type { FormField } from '$lib/api/forms';

	interface Props {
		field: FormField;
		value?: number | null;
		error?: string[];
		onchange?: (value: number) => void;
	}

	let { field, value = null, error, onchange }: Props = $props();

	const minLabel = $derived(field.attributes?.min_label || 'Not at all likely');
	const maxLabel = $derived(field.attributes?.max_label || 'Extremely likely');
	const showLabels = $derived(field.attributes?.show_labels !== false);

	function getScoreColor(score: number): string {
		if (score <= 6) return '#dc2626'; // Detractor - Red
		if (score <= 8) return '#f59e0b'; // Passive - Yellow
		return '#059669'; // Promoter - Green
	}

	function getScoreCategory(score: number): string {
		if (score <= 6) return 'Detractor';
		if (score <= 8) return 'Passive';
		return 'Promoter';
	}
</script>

<div class="nps-field">
	<label class="field-label" for="nps-field-{field.name}">
		{field.label}
		{#if field.required}
			<span class="required">*</span>
		{/if}
	</label>

	{#if field.help_text}
		<p class="field-help">{field.help_text}</p>
	{/if}

	<div class="nps-scale">
		{#if showLabels}
			<span class="scale-label left">{minLabel}</span>
		{/if}

		<div class="score-buttons">
			{#each Array(11) as _, i}
				<button
					type="button"
					class="score-btn"
					class:selected={value === i}
					style={value === i ? `background-color: ${getScoreColor(i)}; color: white; border-color: ${getScoreColor(i)};` : ''}
					onclick={() => onchange?.(i)}
					aria-label={`Score ${i}`}
				>
					{i}
				</button>
			{/each}
		</div>

		{#if showLabels}
			<span class="scale-label right">{maxLabel}</span>
		{/if}
	</div>

	{#if value !== null}
		<div class="score-feedback" style={`color: ${getScoreColor(value)}`}>
			<span class="score-category">{getScoreCategory(value)}</span>
			<span class="score-value">Score: {value}/10</span>
		</div>
	{/if}

	<div class="nps-legend">
		<div class="legend-item detractor">
			<span class="legend-color"></span>
			<span>0-6: Detractors</span>
		</div>
		<div class="legend-item passive">
			<span class="legend-color"></span>
			<span>7-8: Passives</span>
		</div>
		<div class="legend-item promoter">
			<span class="legend-color"></span>
			<span>9-10: Promoters</span>
		</div>
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
	.nps-field {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
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

	.nps-scale {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.scale-label {
		font-size: 0.75rem;
		color: #6b7280;
		min-width: 80px;
	}

	.scale-label.left {
		text-align: right;
	}

	.scale-label.right {
		text-align: left;
	}

	.score-buttons {
		display: flex;
		gap: 0.25rem;
		flex-wrap: wrap;
	}

	.score-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 2px solid #e5e7eb;
		border-radius: 0.375rem;
		background-color: white;
		cursor: pointer;
		font-size: 0.875rem;
		font-weight: 500;
		transition: all 0.2s;
	}

	.score-btn:hover:not(.selected) {
		border-color: #9ca3af;
		background-color: #f3f4f6;
	}

	.score-btn.selected {
		transform: scale(1.1);
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
	}

	.score-feedback {
		display: flex;
		align-items: center;
		gap: 1rem;
		font-weight: 500;
	}

	.score-category {
		font-size: 1rem;
	}

	.score-value {
		font-size: 0.875rem;
		opacity: 0.8;
	}

	.nps-legend {
		display: flex;
		gap: 1.5rem;
		flex-wrap: wrap;
		padding-top: 0.5rem;
		border-top: 1px solid #e5e7eb;
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.legend-color {
		width: 12px;
		height: 12px;
		border-radius: 2px;
	}

	.legend-item.detractor .legend-color {
		background-color: #dc2626;
	}

	.legend-item.passive .legend-color {
		background-color: #f59e0b;
	}

	.legend-item.promoter .legend-color {
		background-color: #059669;
	}

	.field-error {
		font-size: 0.75rem;
		color: #dc2626;
	}

	.field-error p {
		margin: 0;
	}

	@media (max-width: 640px) {
		.nps-scale {
			flex-direction: column;
			align-items: flex-start;
		}

		.scale-label {
			text-align: left !important;
		}

		.score-btn {
			width: 32px;
			height: 32px;
			font-size: 0.75rem;
		}
	}
</style>
