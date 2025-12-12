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
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { LoginLayout, TradingHeroBackground, LoginForm } from '$lib/components/auth';
</script>

<SEOHead
	title="Login to Your Trading Account | Revolution Trading Pros"
	description="Access your Revolution Trading Pros account. Log in to your live trading rooms, real-time alerts, and expert courses dashboard."
	canonical="/login"
	ogType="website"
	keywords={[
		'trading login',
		'revolution trading pros login',
		'member login',
		'trading dashboard',
		'live trading room'
	]}
/>

<LoginLayout>
	{#snippet heroContent()}
		<TradingHeroBackground showTickers={true} animated={true} />
	{/snippet}

	<LoginForm />
</LoginLayout>

<style>
	/* Page-level overrides - handled by components */
</style>
