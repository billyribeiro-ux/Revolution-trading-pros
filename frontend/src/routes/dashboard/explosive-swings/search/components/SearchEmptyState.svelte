<script lang="ts">
	/**
	 * SearchEmptyState - No results found state
	 * @standards Svelte 5 January 2026 | Apple Principal Engineer ICT 7+
	 */

	interface Props {
		query: string;
		onClearSearch: () => void;
	}

	let { query, onClearSearch }: Props = $props();
</script>

<div class="empty-state">
	<div class="empty-icon">
		<svg
			width="64"
			height="64"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
		>
			<circle cx="11" cy="11" r="8" />
			<path d="M21 21l-4.35-4.35" />
			<path d="M8 8l6 6" stroke-dasharray="2 2" />
		</svg>
	</div>

	<h3 class="empty-title">No results found</h3>

	<p class="empty-message">
		No matches found for "<strong>{query}</strong>".
		<br />
		Try adjusting your search or filters.
	</p>

	<div class="empty-suggestions">
		<span class="suggestions-label">Suggestions:</span>
		<ul class="suggestions-list">
			<li>Check the spelling of your search term</li>
			<li>Try searching for a different ticker symbol</li>
			<li>Remove some filters to broaden your search</li>
			<li>Try using fewer or different keywords</li>
		</ul>
	</div>

	<button class="clear-btn" onclick={onClearSearch}> Clear Search </button>
</div>

<style>
	.empty-state {
		text-align: center;
		padding: 64px 24px;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
	}

	.empty-icon {
		color: var(--color-text-tertiary);
		margin-bottom: 16px;
	}

	.empty-title {
		font-size: 20px;
		font-weight: 600;
		color: var(--color-text-primary);
		margin: 0 0 8px 0;
	}

	.empty-message {
		font-size: 14px;
		color: var(--color-text-secondary);
		margin: 0 0 24px 0;
		line-height: 1.5;
	}

	.empty-message strong {
		color: var(--color-text-primary);
	}

	.empty-suggestions {
		text-align: left;
		max-width: 320px;
		margin: 0 auto 24px;
		padding: 16px;
		background: var(--color-bg-subtle);
		border-radius: 8px;
	}

	.suggestions-label {
		display: block;
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 8px;
	}

	.suggestions-list {
		margin: 0;
		padding: 0 0 0 20px;
		font-size: 13px;
		color: var(--color-text-secondary);
		line-height: 1.7;
	}

	.suggestions-list li {
		margin-bottom: 4px;
	}

	.clear-btn {
		padding: 10px 24px;
		background: var(--color-brand-primary);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.15s;
	}

	.clear-btn:hover {
		background: var(--color-brand-primary-hover);
	}

	/* Mobile */
	@media (max-width: 768px) {
		.empty-state {
			padding: 48px 16px;
		}
	}
</style>
