<script lang="ts">
	/**
	 * My Courses - Member Dashboard
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
			console.error('Failed to load courses:', err);
		} finally {
			loading = false;
		}
	});

	// Derived courses list
	const courses = $derived(membershipsData?.courses || []);
</script>

<svelte:head>
	<title>My Courses | Revolution Trading Pros</title>
</svelte:head>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<SectionTitle title="My Courses" />

			{#if loading}
				<LoadingState message="Loading your courses..." />
			{:else if courses.length > 0}
				<CardGrid>
					{#each courses as course (course.id)}
						<ContentCard
							title={course.name}
							href="/dashboard/{course.slug}"
							meta="Active • Enrolled"
							buttonText="Continue Learning"
						/>
					{/each}
				</CardGrid>
			{:else}
				<EmptyState
					title="You don't have any Courses."
					buttonText="Browse Courses"
					buttonHref="/pricing"
				/>
			{/if}
		</section>
	</div>
</div>
