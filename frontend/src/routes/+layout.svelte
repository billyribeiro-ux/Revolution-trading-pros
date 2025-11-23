<script>
	import '../app.css';
	import AdminToolbar from '$lib/components/AdminToolbar.svelte';
	import NavBar from '$lib/components/NavBar.svelte';
	import PopupModal from '$lib/components/PopupModal.svelte';
</script>

<svelte:head>
	<title>Revolution Trading Pros | Live Trading Rooms, Alerts & Pro Tools</title>
	<meta
		name="description"
		content="Professional trading education and tools. Live trading rooms, SPX alerts, courses, and indicators built for disciplined traders."
	/>
</svelte:head>

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
