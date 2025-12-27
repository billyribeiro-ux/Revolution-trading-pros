<script lang="ts">
	/**
	 * Account Layout - Pass through to children
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * The secondary navigation for account pages is handled by the parent
	 * dashboard/+layout.svelte when isOnAccountPage is true.
	 *
	 * This layout simply renders children - no duplicate nav needed.
	 *
	 * @version 2.1.0 - Removed duplicate secondary nav
	 */
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<!-- Page Content - Secondary nav is handled by parent dashboard layout -->
{@render children()}
