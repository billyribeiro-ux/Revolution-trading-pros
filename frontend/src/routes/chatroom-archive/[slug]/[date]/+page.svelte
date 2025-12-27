<script lang="ts">
	/**
	 * Chatroom Archive - Dynamic Route
	 * Handles all chatroom archive content
	 */
	import { page } from '$app/stores';
	
	const slug = $derived($page.params.slug!);
	// @ts-ignore - date param exists in route
	const date = $derived($page.params.date);
</script>

<svelte:head>
	<title>Chatroom Archive | Day Trading Room</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Chatroom Archive</h1>
		<a href="/dashboard/day-trading-room" class="btn btn-xs btn-default">
			← Back to Dashboard
		</a>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2>Trading Room Recording</h2>
			<p>Room: <strong>{slug}</strong></p>
			<p>Date: <strong>{date}</strong></p>
			<p>This is a placeholder page. Chatroom archive content will be added here.</p>
			
			<div class="video-placeholder">
				<p>📹 Chatroom recording will be embedded here</p>
			</div>
			
			<a href="/dashboard/day-trading-room" class="btn btn-default">Return to Dashboard</a>
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		gap: 15px;
	}

	.dashboard__header-left {
		display: flex;
		align-items: center;
		gap: 15px;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.btn {
		padding: 8px 16px;
		border-radius: 3px;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e0e0e0;
	}

	.dashboard__content {
		padding: 30px;
	}

	.dashboard__content-section {
		background: #fff;
		padding: 30px;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0,0,0,0.1);
	}

	h2 {
		font-size: 24px;
		margin-bottom: 20px;
		color: #333;
	}

	.video-placeholder {
		background: #f5f5f5;
		border: 2px dashed #dbdbdb;
		padding: 60px;
		text-align: center;
		margin: 30px 0;
		border-radius: 4px;
	}

	.video-placeholder p {
		font-size: 18px;
		color: #666;
	}
</style>
