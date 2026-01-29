<!--
	ThumbnailSelector.svelte
	═══════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT Level 7 - January 2026
	
	Thumbnail selection grid for video uploads.
	Displays multiple thumbnail options with selection indicator.
	
	@version 1.0.0
-->
<script lang="ts">
	interface Props {
		thumbnails: string[];
		selected: number;
		onselect: (index: number) => void;
		label?: string;
	}

	const { thumbnails, selected, onselect, label = 'Select Thumbnail' }: Props = $props();
</script>

{#if thumbnails.length > 0}
	<div class="thumbnail-section">
		<span class="section-label">{label}</span>
		<div class="thumbnail-grid">
			{#each thumbnails as thumb, i}
				<button
					type="button"
					class="thumbnail-option"
					class:selected={selected === i}
					onclick={() => onselect(i)}
					aria-label="Select thumbnail {i + 1}"
					aria-pressed={selected === i}
				>
					<img src={thumb} alt="Thumbnail option {i + 1}" loading="lazy" />
					{#if selected === i}
						<div class="thumb-check">
							<svg
								viewBox="0 0 24 24"
								fill="currentColor"
								width="16"
								height="16"
								aria-hidden="true"
							>
								<path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z" />
							</svg>
						</div>
					{/if}
				</button>
			{/each}
		</div>
	</div>
{/if}

<style>
	.thumbnail-section {
		margin-bottom: 20px;
	}

	.section-label {
		display: block;
		font-size: 12px;
		font-weight: 600;
		color: #475569;
		margin-bottom: 10px;
	}

	.thumbnail-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 10px;
	}

	.thumbnail-option {
		position: relative;
		aspect-ratio: 16/9;
		border: 3px solid transparent;
		border-radius: 8px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.15s ease;
		padding: 0;
		background: #f1f5f9;
	}

	.thumbnail-option img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-option:hover {
		border-color: #cbd5e1;
	}

	.thumbnail-option.selected {
		border-color: #143e59;
		box-shadow: 0 0 0 2px rgba(20, 62, 89, 0.2);
	}

	.thumb-check {
		position: absolute;
		top: 4px;
		right: 4px;
		width: 24px;
		height: 24px;
		background: #143e59;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	@media (max-width: 480px) {
		.thumbnail-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}
</style>
