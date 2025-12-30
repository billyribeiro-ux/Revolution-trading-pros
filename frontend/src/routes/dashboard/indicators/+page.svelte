<script lang="ts">
	/**
	 * My Indicators - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Svelte 5 - Uses shared components, NO duplicate styles
	 * @version 3.0.0
	 */
	import { onMount } from 'svelte';
	import { getUserMemberships, type UserMembershipsResponse } from '$lib/api/user-memberships';
	import {
		SectionTitle,
		ContentCard,
		CardGrid,
		LoadingState,
		EmptyState
	} from '$lib/components/dashboard';

	// State - Svelte 5 runes
	let loading = $state(true);
	let membershipsData = $state<UserMembershipsResponse | null>(null);

	onMount(async () => {
		try {
			membershipsData = await getUserMemberships();
		} catch (err) {
			console.error('Failed to load indicators:', err);
		} finally {
			loading = false;
		}
	});

	// Derived indicators list
	const indicators = $derived(membershipsData?.indicators || []);
</script>

<svelte:head>
	<title>My Indicators | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<SectionTitle title="My Indicators" />

			{#if loading}
				<LoadingState message="Loading your indicators..." />
			{:else if indicators.length > 0}
				<CardGrid>
					{#each indicators as indicator (indicator.id)}
						<ContentCard
							title={indicator.name}
							href="/dashboard/indicators/{indicator.slug}"
							buttonText="Download"
						/>
					{/each}
				</CardGrid>
			{:else}
				<EmptyState
					title="You don't have any Indicators."
					buttonText="See All Indicators"
					buttonHref="/product/product-category/Indicators/"
				/>
			{/if}
		</section>
	</div>
</div>
