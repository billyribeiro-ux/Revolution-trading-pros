<script lang="ts">
	/**
	 * Login Page - Premium Trading-Themed Authentication
	 * Netflix L11+ Principal Engineer Grade
	 *
	 * Features:
	 * - Cinematic split-panel layout
	 * - Animated candlestick chart background
	 * - Premium form with micro-animations
	 * - Full light/dark theme support
	 * - Mobile-optimized experience
	 *
	 * @version 2.0.0
	 */
	import { LoginLayout, TradingHeroBackground, LoginForm } from '$lib/components/auth';
</script>

<LoginLayout>
	{#snippet heroContent()}
		<TradingHeroBackground showTickers={true} animated={true} />
	{/snippet}

	<LoginForm />
</LoginLayout>

<style>
	/* Page-level overrides - handled by components */
</style>
