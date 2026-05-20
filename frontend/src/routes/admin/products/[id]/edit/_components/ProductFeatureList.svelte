<script lang="ts">
	/**
	 * R20-C extraction (2026-05-20): feature-bullet add/remove list.
	 * Receives the parent's `features` array as a bindable prop so the parent
	 * can keep using `validFeatures = formData.features.filter(...)` derived.
	 */
	import { IconPlus, IconTag, IconX } from '$lib/icons';

	interface Props {
		features: string[];
		onAdd: () => void;
		onRemove: (index: number) => void;
		onUpdate: (index: number, value: string) => void;
	}

	let { features, onAdd, onRemove, onUpdate }: Props = $props();
</script>

<div class="features-section">
	<div class="section-header">
		<h4>
			<IconTag size={18} />
			Features
		</h4>
		<button type="button" class="add-feature-btn" onclick={onAdd}>
			<IconPlus size={16} />
			Add Feature
		</button>
	</div>

	{#each features as feature, index (index)}
		<div class="feature-row">
			<input
				id="edit-feature-{index}"
				name="edit-feature-{index}"
				type="text"
				value={feature}
				oninput={(e) => onUpdate(index, e.currentTarget.value)}
				placeholder="Feature description"
			/>
			{#if features.length > 1}
				<button type="button" class="remove-feature-btn" onclick={() => onRemove(index)}>
					<IconX size={16} />
				</button>
			{/if}
		</div>
	{/each}
</div>

<style>
	.features-section {
		margin: 1.5rem 0;
		padding: 1.5rem;
		background: rgba(59, 130, 246, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(59, 130, 246, 0.15);
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.section-header h4 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.add-feature-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(59, 130, 246, 0.1);
		border: 1px solid rgba(59, 130, 246, 0.3);
		border-radius: 6px;
		color: #60a5fa;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-feature-btn:hover {
		background: rgba(59, 130, 246, 0.2);
	}

	.feature-row {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.feature-row input {
		flex: 1;
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.feature-row input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.remove-feature-btn {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		width: 36px;
		height: 36px;
		border-radius: 6px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: all 0.2s;
		flex-shrink: 0;
	}

	.remove-feature-btn:hover {
		background: rgba(239, 68, 68, 0.2);
	}
</style>
