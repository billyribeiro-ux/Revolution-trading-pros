<script lang="ts">
	/**
	 * Connection Gate - Apple Principal Engineer ICT11+ Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * A wrapper component that conditionally renders content based on service
	 * connection status. Shows connected content or a beautiful connection prompt.
	 *
	 * Usage:
	 * <ConnectionGate feature="analytics">
	 *   {#snippet connected()}
	 *     <AnalyticsWidget />
	 *   {/snippet}
	 * </ConnectionGate>
	 *
	 * @level ICT11+ Principal Engineer - Premium Enterprise UX
	 */

	import { type Snippet } from 'svelte';
	import ServiceConnectionStatus from './ServiceConnectionStatus.svelte';
	import {
		connections,
		isAnalyticsConnected,
		isSeoConnected,
		isEmailConnected,
		isPaymentConnected,
		isCrmConnected,
		isFluentConnected,
		isFormsConnected,
		isBehaviorConnected,
		FEATURE_SERVICES,
		type ServiceKey
	} from '$lib/stores/connections.svelte';

	interface Props {
		/** Feature category to check */
		feature?: string;
		/** Specific service key to check */
		serviceKey?: ServiceKey;
		/** Variant for the disconnected state display */
		variant?: 'card' | 'inline' | 'badge' | 'banner' | 'minimal';
		/** Show feature list in disconnected state */
		showFeatures?: boolean;
		/** Content to show when connected */
		connected?: Snippet;
		/** Optional: Content to show when disconnected (overrides default) */
		disconnected?: Snippet;
		/** Children (alternative to connected snippet) */
		children?: Snippet;
	}

	let {
		feature,
		serviceKey,
		variant = 'card',
		showFeatures = true,
		connected,
		disconnected,
		children
	}: Props = $props();

	// Check connection status
	let isConnected = $derived(() => {
		if (serviceKey) {
			return connections.isConnected(serviceKey);
		}
		if (feature) {
			switch (feature) {
				case 'analytics':
					return $isAnalyticsConnected;
				case 'seo':
					return $isSeoConnected;
				case 'email':
					return $isEmailConnected;
				case 'payment':
					return $isPaymentConnected;
				case 'crm':
					return $isCrmConnected;
				case 'fluent':
					return $isFluentConnected;
				case 'forms':
					return $isFormsConnected;
				case 'behavior':
					return $isBehaviorConnected;
				default:
					return connections.isFeatureConnected(feature as keyof typeof FEATURE_SERVICES);
			}
		}
		return false;
	});
</script>

{#if isConnected()}
	<!-- Connected: Show content -->
	{#if connected}
		{@render connected()}
	{:else if children}
		{@render children()}
	{/if}
{:else}
	<!-- Disconnected: Show connection prompt or custom disconnected content -->
	{#if disconnected}
		{@render disconnected()}
	{:else}
		<ServiceConnectionStatus {feature} {serviceKey} {variant} {showFeatures} />
	{/if}
{/if}
