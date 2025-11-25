<script>
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import PopupModal from '$lib/components/PopupModal.svelte';
</script>

<!--
	SEO Note: Page-specific SEO is handled by individual +page.svelte files using SEOHead component.
	Global meta tags are set in app.html for fallback only.
	Each page must define its own title, description, canonical, and structured data.
-->

<div class="min-h-screen bg-rtp-bg text-rtp-text">
	<!-- Admin Toolbar (only shows for logged-in admins) -->
	<AdminToolbar />

	<NavBar />

	<main>
		<slot />
	</main>

	<!-- Global Popup Modal -->
	<PopupModal />
</div>
