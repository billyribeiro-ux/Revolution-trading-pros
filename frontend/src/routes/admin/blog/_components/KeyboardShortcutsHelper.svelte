<div class="keyboard-shortcuts">
	<span>⌘N New</span>
	<span>⌘F Search</span>
	<span>⌘A Select All</span>
	<span>V Toggle View</span>
	<span>R Refresh</span>
</div>

<style>
	.keyboard-shortcuts {
		position: fixed;
		bottom: 1rem;
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		gap: 1rem;
		padding: 0.75rem 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
		z-index: 100;
	}

	.keyboard-shortcuts span {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	@media (max-width: 767.98px) {
		.keyboard-shortcuts {
			display: none;
		}
	}

	@media print {
		.keyboard-shortcuts {
			display: none;
		}
	}
</style>
