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

	interface PlatformOption {
		value: string;
		label: string;
	}

	interface Props {
		indicator: Indicator;
		platformOptions: ReadonlyArray<PlatformOption>;
	}

	let { indicator = $bindable(), platformOptions }: Props = $props();
</script>

<div class="form-section">
	<h2>Basic Information</h2>
	<div class="form-grid">
		<div class="form-group">
			<label for="name">Name *</label>
			<input type="text" id="name" name="name" bind:value={indicator.name} />
		</div>
		<div class="form-group">
			<label for="slug">Slug</label>
			<input type="text" id="slug" name="slug" bind:value={indicator.slug} />
		</div>
		<div class="form-group">
			<label for="price">Price (USD)</label>
			<input type="number" id="price" name="price" step="0.01" bind:value={indicator.price} />
		</div>
		<div class="form-group">
			<label for="platform">Platform</label>
			<select id="platform" bind:value={indicator.platform}>
				{#each platformOptions as opt (opt.value)}
					<option value={opt.value}>{opt.label}</option>
				{/each}
			</select>
		</div>
		<div class="form-group">
			<label for="version">Version</label>
			<input
				type="text"
				id="version"
				name="version"
				bind:value={indicator.version}
				placeholder="1.0"
			/>
		</div>
		<div class="form-group">
			<label for="is_active">Status</label>
			<select id="is_active" bind:value={indicator.is_active}>
				<option value={true}>Active</option>
				<option value={false}>Inactive</option>
			</select>
		</div>
	</div>
</div>

<div class="form-section">
	<h2>Images & URLs</h2>
	<div class="form-grid">
		<div class="form-group">
			<label for="thumbnail">Thumbnail URL</label>
			<input
				type="url"
				id="thumbnail"
				name="thumbnail"
				bind:value={indicator.thumbnail}
				placeholder="https://..."
			/>
		</div>
		<div class="form-group">
			<label for="download_url">Download URL</label>
			<input
				type="url"
				id="download_url"
				name="download_url"
				bind:value={indicator.download_url}
				placeholder="https://..."
			/>
		</div>
		<div class="form-group full-width">
			<label for="documentation_url">Documentation URL</label>
			<input
				type="url"
				id="documentation_url"
				name="documentation_url"
				bind:value={indicator.documentation_url}
				placeholder="https://..."
			/>
		</div>
	</div>
</div>

<div class="form-section">
	<h2>Description</h2>
	<div class="form-group full-width">
		<label for="description">Short Description</label>
		<textarea id="description" rows="3" bind:value={indicator.description}></textarea>
	</div>
	<div class="form-group full-width">
		<label for="long_description">Long Description</label>
		<textarea id="long_description" rows="8" bind:value={indicator.long_description}></textarea>
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

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}
	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
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
	select,
	textarea {
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	input:focus,
	select:focus,
	textarea:focus {
		outline: none;
		border-color: #143e59;
	}
	textarea {
		resize: vertical;
	}

	@media (max-width: 639px) {
		.form-grid {
			grid-template-columns: 1fr;
		}

		.form-section {
			padding: 16px;
		}
	}

	@media (min-width: 768px) {
		.form-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (hover: none) and (pointer: coarse) {
		input,
		select,
		textarea {
			font-size: 16px;
			min-height: 44px;
		}
	}
</style>
