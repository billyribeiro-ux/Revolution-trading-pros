<script lang="ts">
	interface Indicator {
		id: number;
		name: string;
		slug: string;
		description?: string;
		long_description?: string;
		price?: number;
		is_active?: boolean;
		thumbnail?: string;
		platform?: string;
		version?: string;
		download_url?: string;
		documentation_url?: string;
		features?: unknown;
		requirements?: unknown;
		screenshots?: unknown;
		meta_title?: string;
		meta_description?: string;
		created_at?: string;
		updated_at?: string;
	}

	interface Props {
		indicator: Indicator;
	}

	let { indicator = $bindable() }: Props = $props();
</script>

<div class="form-section">
	<h2>SEO Settings</h2>
	<div class="form-group full-width">
		<label for="meta_title">Meta Title</label>
		<input
			type="text"
			id="meta_title"
			name="meta_title"
			bind:value={indicator.meta_title}
			placeholder="Page title for search engines"
		/>
	</div>
	<div class="form-group full-width">
		<label for="meta_description">Meta Description</label>
		<textarea
			id="meta_description"
			rows="3"
			bind:value={indicator.meta_description}
			placeholder="Description for search engines"
		></textarea>
	</div>
</div>

<style>
	.form-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 24px;
	}
	.form-section h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 20px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
		margin-bottom: 16px;
	}
	.form-group.full-width {
		grid-column: 1 / -1;
	}
	label {
		font-size: 13px;
		font-weight: 500;
		color: #374151;
	}
	input,
	textarea {
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	input:focus,
	textarea:focus {
		outline: none;
		border-color: #143e59;
	}
	textarea {
		resize: vertical;
	}

	@media (max-width: 639px) {
		.form-section {
			padding: 16px;
		}
	}

	@media (hover: none) and (pointer: coarse) {
		input,
		textarea {
			font-size: 16px;
			min-height: 44px;
		}
	}
</style>
