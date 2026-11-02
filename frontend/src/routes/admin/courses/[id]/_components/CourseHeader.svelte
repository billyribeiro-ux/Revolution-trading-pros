<script lang="ts">
	/**
	 * R19-C extraction (2026-05-20): course editor header — back link, title,
	 * status badge, action buttons (Page Builder / Preview / Publish / Save).
	 * Mirrors R18-C IndicatorHeader.svelte pattern.
	 */
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconLayoutGrid from '@tabler/icons-svelte-runes/icons/layout-grid';
	import IconExternalLink from '@tabler/icons-svelte-runes/icons/external-link';

	interface Props {
		courseId: string;
		courseTitle: string;
		courseSlug: string;
		isPublished: boolean;
		saving: boolean;
		onPublishToggle: () => void;
		onSave: () => void;
	}

	let { courseId, courseTitle, courseSlug, isPublished, saving, onPublishToggle, onSave }: Props =
		$props();
</script>

<header class="editor-header">
	<div class="header-left">
		<a href="/admin/courses" class="back-link">
			<IconChevronLeft size={20} aria-hidden="true" />
			Back to Courses
		</a>
		<h1>{courseTitle}</h1>
		<span class={['status', { published: isPublished }]}>
			{isPublished ? 'Published' : 'Draft'}
		</span>
	</div>
	<div class="header-actions">
		<a href="/admin/page-builder?course={courseId}" class="btn-builder">
			<IconLayoutGrid size={16} aria-hidden="true" />
			Page Builder
		</a>
		<a href="/classes/{courseSlug}" target="_blank" class="btn-secondary">
			<IconExternalLink size={16} aria-hidden="true" />
			Preview
		</a>
		<button class="btn-secondary" onclick={onPublishToggle}>
			{isPublished ? 'Unpublish' : 'Publish'}
		</button>
		<button class="btn-primary" onclick={onSave} disabled={saving}>
			{saving ? 'Saving...' : 'Save Changes'}
		</button>
	</div>
</header>

<style>
	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 24px;
		padding-bottom: 24px;
		border-bottom: 1px solid var(--border-default, #30363d);
	}
	.header-left {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: var(--text-tertiary, #6e7681);
		text-decoration: none;
		font-size: 14px;
	}
	.back-link:hover {
		color: var(--primary-500, #e6b800);
	}
	.header-left h1 {
		font-size: 24px;
		margin: 0;
		color: var(--text-primary, #f0f6fc);
	}
	.status {
		display: inline-block;
		padding: 4px 10px;
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--warning-emphasis, #d29922);
		border-radius: 4px;
		font-size: 12px;
		font-weight: 500;
	}
	.status.published {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.header-actions {
		display: flex;
		gap: 8px;
	}
	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		border: none;
	}
	.btn-primary {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}
	.btn-primary:hover {
		background: var(--primary-500, #e6b800);
	}
	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.btn-secondary {
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
	}
	.btn-secondary:hover {
		background: var(--bg-surface, #1c2128);
	}
	.btn-builder {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}
	.btn-builder:hover {
		background: var(--primary-500, #e6b800);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
	}

	@media (max-width: 767.98px) {
		.editor-header {
			flex-direction: column;
			gap: 16px;
		}
		.header-actions {
			width: 100%;
			justify-content: flex-end;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.btn-primary,
		.btn-secondary,
		.btn-builder {
			transition: none;
		}
		.btn-builder:hover {
			transform: none;
		}
	}
</style>
