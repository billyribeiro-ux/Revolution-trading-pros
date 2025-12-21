<script lang="ts">
	/**
	 * Membership Dashboard Layout - Pass-through
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * This layout is a simple pass-through. The parent /dashboard/+layout.svelte
	 * handles all navigation:
	 * - Detects membership routes via `isMembershipSection`
	 * - Collapses DashboardSidebar to 60px (icons only)
	 * - Shows SecondaryNav with 220px membership navigation
	 *
	 * DO NOT add navigation here - it would duplicate the parent's navigation.
	 *
	 * @version 3.0.0 (December 2025)
	 */

	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<!-- Simple pass-through - parent layout handles all navigation -->
{@render children()}
