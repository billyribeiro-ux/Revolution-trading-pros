<script lang="ts">
	/**
	 * Template Preview Card Component - Svelte 5
	 *
	 * Shows a miniature preview of a banner template with actions.
	 *
	 * Updated: December 2025 - Migrated to Svelte 5 runes ($props)
	 */
	import type { BannerTemplate } from './types';

	// Svelte 5: Props with callback events using $props() rune
	interface Props {
		template: BannerTemplate;
		isActive?: boolean;
		showActions?: boolean;
		onSelect?: (template: BannerTemplate) => void;
		onPreview?: (template: BannerTemplate) => void;
		onEdit?: (template: BannerTemplate) => void;
		onDelete?: (template: BannerTemplate) => void;
	}

	let { template, isActive = false, showActions = true, onSelect, onPreview, onEdit, onDelete }: Props = $props();

	// Generate mini preview styles
	function getMiniPreviewStyles(): string {
		const styles: string[] = [];
		styles.push(`background: ${template.colors.background}`);
		styles.push(`color: ${template.colors.text}`);
		if (template.border) styles.push(`border: ${template.border}`);
		return styles.join('; ');
	}

	// Get button preview color
	function getButtonColor(): string {
		return template.colors.acceptButton.startsWith('linear')
			? template.colors.acceptButtonText
			: template.colors.acceptButton;
	}

	// Category badge colors
	const categoryColors: Record<string, string> = {
		minimal: 'bg-gray-500/20 text-gray-300',
		modern: 'bg-purple-500/20 text-purple-300',
		corporate: 'bg-blue-500/20 text-blue-300',
		playful: 'bg-yellow-500/20 text-yellow-300',
		dark: 'bg-zinc-500/20 text-zinc-300',
		light: 'bg-white/20 text-white',
		custom: 'bg-green-500/20 text-green-300',
	};
</script>

<div
	class="template-card"
	class:active={isActive}
	role="button"
	tabindex="0"
	onclick={() => onSelect?.(template)}
	onkeypress={(e: KeyboardEvent) => e.key === 'Enter' && onSelect?.(template)}
>
	<!-- Mini Preview -->
	<div class="preview-container">
		<div class="mini-preview" style={getMiniPreviewStyles()}>
			<!-- Mini banner structure -->
			<div class="mini-content">
				{#if template.showIcon}
					<div class="mini-icon" style="color: {template.colors.accent}">●</div>
				{/if}
				<div class="mini-text">
					<div class="mini-title" style="background: {template.colors.text}"></div>
					<div class="mini-desc" style="background: {template.colors.textMuted}"></div>
				</div>
				<div class="mini-buttons">
					<div class="mini-btn" style="background: {getButtonColor()}"></div>
				</div>
			</div>
		</div>

		<!-- Active indicator -->
		{#if isActive}
			<div class="active-badge">
				<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
					<path d="m9 12 2 2 4-4"/>
				</svg>
				Active
			</div>
		{/if}

		<!-- Position indicator -->
		<div class="position-indicator position-{template.position}">
			<div class="position-dot"></div>
		</div>
	</div>

	<!-- Card Info -->
	<div class="card-info">
		<div class="card-header">
			<h3 class="card-title">{template.name}</h3>
			<span class="category-badge {categoryColors[template.category] || categoryColors.custom}">
				{template.category}
			</span>
		</div>
		<p class="card-description">{template.description}</p>

		<!-- Features -->
		<div class="features">
			{#if template.showIcon}
				<span class="feature">Icon</span>
			{/if}
			{#if template.showRejectButton}
				<span class="feature">Reject</span>
			{/if}
			{#if template.backdropBlur}
				<span class="feature">Blur</span>
			{/if}
			{#if template.animation !== 'none'}
				<span class="feature">{template.animation}</span>
			{/if}
		</div>

		<!-- Actions -->
		{#if showActions}
			<div class="card-actions">
				<button
					class="action-btn preview"
					onclick={(e: MouseEvent) => { e.stopPropagation(); onPreview?.(template); }}
				>
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/>
						<circle cx="12" cy="12" r="3"/>
					</svg>
					Preview
				</button>
				{#if template.isEditable}
					<button
						class="action-btn edit"
						onclick={(e: MouseEvent) => { e.stopPropagation(); onEdit?.(template); }}
					>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/>
						</svg>
						Edit
					</button>
				{/if}
				{#if template.category === 'custom'}
					<button
						class="action-btn delete"
						aria-label="Delete template"
						title="Delete template"
						onclick={(e: MouseEvent) => { e.stopPropagation(); onDelete?.(template); }}
					>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
						</svg>
					</button>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.template-card {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.template-card:hover {
		border-color: rgba(255, 255, 255, 0.2);
		transform: translateY(-2px);
	}

	.template-card.active {
		border-color: #0ea5e9;
		box-shadow: 0 0 0 2px rgba(14, 165, 233, 0.2);
	}

	.preview-container {
		position: relative;
		height: 120px;
		background: #18181b;
		overflow: hidden;
	}

	.mini-preview {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0.5rem;
		font-size: 0.5rem;
	}

	.mini-content {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.mini-icon {
		font-size: 1rem;
	}

	.mini-text {
		flex: 1;
	}

	.mini-title {
		height: 6px;
		width: 60%;
		border-radius: 2px;
		margin-bottom: 4px;
		opacity: 0.9;
	}

	.mini-desc {
		height: 4px;
		width: 80%;
		border-radius: 2px;
		opacity: 0.5;
	}

	.mini-buttons {
		display: flex;
		gap: 4px;
	}

	.mini-btn {
		height: 12px;
		width: 40px;
		border-radius: 3px;
	}

	.active-badge {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: #22c55e;
		color: white;
		font-size: 0.625rem;
		font-weight: 600;
		border-radius: 9999px;
	}

	.position-indicator {
		position: absolute;
		top: 0.5rem;
		left: 0.5rem;
		width: 40px;
		height: 30px;
		background: rgba(0, 0, 0, 0.5);
		border-radius: 4px;
		border: 1px solid rgba(255, 255, 255, 0.2);
	}

	.position-dot {
		position: absolute;
		width: 8px;
		height: 4px;
		background: #0ea5e9;
		border-radius: 2px;
	}

	.position-bottom .position-dot {
		bottom: 2px;
		left: 50%;
		transform: translateX(-50%);
		width: 80%;
	}

	.position-top .position-dot {
		top: 2px;
		left: 50%;
		transform: translateX(-50%);
		width: 80%;
	}

	.position-bottom-left .position-dot {
		bottom: 2px;
		left: 2px;
		width: 12px;
		height: 8px;
	}

	.position-bottom-right .position-dot {
		bottom: 2px;
		right: 2px;
		width: 12px;
		height: 8px;
	}

	.position-center .position-dot,
	.position-fullscreen .position-dot {
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 16px;
		height: 10px;
	}

	.card-info {
		padding: 1rem;
	}

	.card-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.card-title {
		margin: 0;
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.category-badge {
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.card-description {
		margin: 0 0 0.75rem 0;
		font-size: 0.75rem;
		color: #94a3b8;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.features {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
		margin-bottom: 0.75rem;
	}

	.feature {
		padding: 0.125rem 0.375rem;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		font-size: 0.625rem;
		color: #94a3b8;
	}

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem 0.625rem;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 6px;
		color: #e2e8f0;
		font-size: 0.6875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	.action-btn.preview:hover {
		background: rgba(14, 165, 233, 0.2);
		color: #38bdf8;
	}

	.action-btn.edit:hover {
		background: rgba(168, 85, 247, 0.2);
		color: #c084fc;
	}

	.action-btn.delete {
		padding: 0.375rem;
	}

	.action-btn.delete:hover {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}
</style>
