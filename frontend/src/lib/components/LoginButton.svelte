<script lang="ts">
	export let isLoggedIn: boolean = false;

	import { IconUserCircle, IconLogin } from '@tabler/icons-svelte';

	$: label = isLoggedIn ? 'Account' : 'Login / Signup';
	$: href = isLoggedIn ? '/account' : '/login';
</script>

<a {href} class="login-button-outer group">
	<div class="login-button-inner">
		{#if isLoggedIn}
			<IconUserCircle size={27} class="text-white" />
		{:else}
			<IconLogin size={27} class="text-white" />
		{/if}
		<span class="font-heading font-semibold text-white text-base">
			{label}
		</span>
	</div>
</a>

<style>
	.login-button-outer {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 131px;
		height: 51px;
		border-radius: 15px;
		background: linear-gradient(135deg, rgba(46, 142, 255, 0.2) 0%, rgba(46, 142, 255, 0.1) 100%);
		transition: all 0.3s ease;
	}

	.login-button-outer:hover {
		background: rgba(46, 142, 255, 0.7);
		box-shadow: 0 0 20px rgba(46, 142, 255, 0.4);
	}

	.login-button-inner {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 127px;
		height: 47px;
		border-radius: 13px;
		background: #1a1a1a;
	}
</style>
