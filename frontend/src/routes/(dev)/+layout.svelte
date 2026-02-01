<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Dev Routes Layout - Development Navigation Bar
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Provides dev-only navigation and visual indicator
	 * @version 1.0.0 - January 2026
	 * @standards Apple Principal Engineer ICT Level 7+
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<div class="dev-layout">
	<!-- Dev Mode Banner -->
	<header class="dev-header">
		<div class="dev-banner">
			<span class="dev-icon">🛠️</span>
			<span class="dev-label">Development Mode</span>
		</div>
		<nav class="dev-nav">
			<a href="/dev/workbench" class="dev-nav-link">
				<span class="nav-icon">📦</span>
				Component Workbench
			</a>
			<a href="/" class="dev-nav-link dev-nav-link--secondary">
				← Back to App
			</a>
		</nav>
	</header>

	<!-- Page Content -->
	<main class="dev-content">
		{@render children()}
	</main>
</div>

<style>
	.dev-layout {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
		background: #0a0a0a;
		color: #fafafa;
	}

	.dev-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.dev-banner {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: rgba(139, 92, 246, 0.2);
		border: 1px solid rgba(139, 92, 246, 0.4);
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #c4b5fd;
	}

	.dev-icon {
		font-size: 0.875rem;
	}

	.dev-nav {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.dev-nav-link {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: #fafafa;
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 500;
		transition: all 0.15s ease;
	}

	.dev-nav-link:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.dev-nav-link--secondary {
		background: transparent;
		border-color: transparent;
		color: #a1a1aa;
	}

	.dev-nav-link--secondary:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #fafafa;
	}

	.nav-icon {
		font-size: 1rem;
	}

	.dev-content {
		flex: 1;
		display: flex;
		flex-direction: column;
	}
</style>
