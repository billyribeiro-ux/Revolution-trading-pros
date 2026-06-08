<script lang="ts">
	/**
	 * R24-C extraction (2026-05-20): product-type selector grid (course /
	 * indicator / membership / bundle). The list of options is owned by the
	 * parent (parent provides `productTypes` + `selected`); this component
	 * just renders + emits the click via `onSelect`.
	 */
	import type { ProductType, ProductTypeOption } from './types';

	interface Props {
		productTypes: readonly ProductTypeOption[];
		selected: ProductType;
		onSelect: (value: ProductType) => void;
	}

	let { productTypes, selected, onSelect }: Props = $props();
</script>

<div class="form-group">
	<span id="product-type-label" class="group-label">Product Type *</span>
	<div class="type-selector" role="group" aria-labelledby="product-type-label">
		{#each productTypes as type (type.value)}
			{const Icon = type.icon}
			<button
				type="button"
				class={['type-option', { selected: selected === type.value }]}
				onclick={() => onSelect(type.value)}
				style:--type-color={type.color}
			>
				<Icon size={24} />
				<span>{type.label}</span>
			</button>
		{/each}
	</div>
</div>

<style>
	.form-group {
		margin-bottom: 1.5rem;
	}

	.group-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.type-selector {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
	}

	.type-option {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 2px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.type-option:hover {
		border-color: rgba(148, 163, 184, 0.4);
		color: #f1f5f9;
	}

	.type-option.selected {
		border-color: var(--type-color);
		background: color-mix(in srgb, var(--type-color) 10%, transparent);
		color: #f1f5f9;
	}

	.type-option span {
		font-size: 0.875rem;
		font-weight: 500;
	}

	@media (max-width: 1023.98px) {
		.type-selector {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
