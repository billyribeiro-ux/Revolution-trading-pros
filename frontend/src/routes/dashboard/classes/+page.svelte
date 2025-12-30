<script lang="ts">
	/**
	 * My Classes - Member Dashboard
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
			console.error('Failed to load classes:', err);
		} finally {
			loading = false;
		}
	});

	// Derived classes list
	const classes = $derived(membershipsData?.courses || []);

	// Format date helper
	function formatDate(dateString?: string): string {
		if (!dateString) return 'Enrolled';
		return new Date(dateString).toLocaleDateString('en-US', { month: 'long', year: 'numeric' });
	}
</script>

<svelte:head>
	<title>My Classes | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<SectionTitle title="My Classes" />

			{#if loading}
				<LoadingState message="Loading your classes..." />
			{:else if classes.length > 0}
				<CardGrid>
					{#each classes as cls (cls.id)}
						<ContentCard
							title={cls.name}
							href="/classes/{cls.slug}"
							meta={formatDate(cls.startDate)}
							buttonText="Watch Now"
						/>
					{/each}
				</CardGrid>
			{:else}
				<EmptyState
					title="You don't have any Classes."
					buttonText="Browse Classes"
					buttonHref="/store/classes"
				/>
			{/if}
		</section>
	</div>
</div>
