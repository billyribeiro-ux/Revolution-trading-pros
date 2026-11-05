<script lang="ts">
	import { page } from '$app/state';
	import MemberProfileView from '#lib/components/admin/MemberProfileView.svelte';
</script>

<div class="member-detail-page"><MemberProfileView memberId={page.params.id ?? ''} /></div>

<style>
	.member-detail-page {
		max-width: 1440px;
		margin: 0 auto;
		padding: 1.5rem;
	}
	@media (max-width: 600px) {
		.member-detail-page {
			padding: 1rem;
		}
	}
</style>
