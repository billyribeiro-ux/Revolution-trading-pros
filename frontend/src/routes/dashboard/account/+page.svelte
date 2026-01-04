<script lang="ts">
	import type { AccountPageData } from './+page.d';

	interface Props {
		data: AccountPageData;
	}

	let { data }: Props = $props();

	// Derived user data
	let firstName = $derived(data.profile?.firstName || '');
	let lastName = $derived(data.profile?.lastName || '');
	let displayName = $derived(`${firstName} ${lastName}`.trim() || 'Member');
</script>

<svelte:head>
	<title>My Account - Revolution Trading Pros</title>
</svelte:head>

<div class="woocommerce">
	<div class="woocommerce-MyAccount-content">
		<div class="woocommerce-notices-wrapper"></div>
		<div class="content-box content-box--centered">
			<div class="content-box__section">
				<p>
					Hello <strong>{displayName}</strong> (not <strong>{displayName}</strong>? <a href="/logout">Log out</a>)
				</p>
				
				<p class="u--margin-bottom-0">
					From your account dashboard you can view your <a href="/dashboard/account/orders">recent orders</a>, manage your <a href="/dashboard/account/edit-address">billing address</a>, and <a href="/dashboard/account/edit-account">edit your password and account details</a>.
				</p>
			</div>
		</div>
	</div>
</div>

<style>
	.woocommerce {
		background: #fff;
		padding: 0;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.content-box {
		background: #fff;
		border: 1px solid #e5e5e5;
		margin-bottom: 20px;
	}

	.content-box--centered {
		max-width: 100%;
	}

	.content-box__section {
		padding: 24px;
	}

	.woocommerce-MyAccount-content p {
		font-size: 14px;
		line-height: 1.6;
		color: #515151;
		margin-bottom: 16px;
	}

	.woocommerce-MyAccount-content p:first-of-type {
		margin-top: 0;
	}

	.u--margin-bottom-0 {
		margin-bottom: 0 !important;
	}

	.woocommerce-MyAccount-content a {
		color: #0073aa;
		text-decoration: none;
	}

	.woocommerce-MyAccount-content a:hover {
		color: #005177;
		text-decoration: underline;
	}

	.woocommerce-MyAccount-content strong {
		font-weight: 600;
		color: #333;
	}
</style>
