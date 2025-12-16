<script lang="ts">
	/**
	 * Email Verification Page
	 * Handles email verification link callbacks with beautiful UI.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import {
		IconCheck,
		IconX,
		IconLoader,
		IconMail,
		IconRefresh,
		IconLogin
	} from '$lib/icons';
	import { verifyEmail, sendEmailVerification } from '$lib/api/auth';

	let status: 'loading' | 'success' | 'error' | 'expired' = 'loading';
	let errorMessage = '';
	let resending = false;
	let resendSuccess = false;

	async function verify() {
		const id = $page.url.searchParams.get('id');
		const hash = $page.url.searchParams.get('hash');
		const expires = $page.url.searchParams.get('expires');
		const signature = $page.url.searchParams.get('signature');

		if (!id || !hash || !signature) {
			status = 'error';
			errorMessage = 'Invalid verification link. Please request a new one.';
			return;
		}

		// Check if link has expired
		if (expires) {
			const expiresTimestamp = parseInt(expires);
			if (Date.now() / 1000 > expiresTimestamp) {
				status = 'expired';
				errorMessage = 'This verification link has expired. Please request a new one.';
				return;
			}
		}

		try {
			await verifyEmail(id, hash, signature);
			status = 'success';
		} catch (error: any) {
			status = 'error';
			errorMessage = error.message || 'Verification failed. Please try again.';
		}
	}

	async function handleResend() {
		resending = true;
		try {
			await sendEmailVerification();
			resendSuccess = true;
		} catch (error: any) {
			errorMessage = error.message || 'Failed to resend verification email.';
		} finally {
			resending = false;
		}
	}

	onMount(() => {
		verify();
	});
</script>

<svelte:head>
	<title>Verify Email - Revolution Trading Pros</title>
</svelte:head>

<div
	class="flex min-h-screen items-center justify-center bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900 p-4"
>
	<div class="w-full max-w-md">
		<!-- Logo -->
		<div class="mb-8 text-center">
			<h1 class="text-3xl font-bold text-white">Revolution Trading Pros</h1>
		</div>

		<!-- Card -->
		<div class="rounded-2xl bg-white p-8 shadow-2xl">
			{#if status === 'loading'}
				<div class="text-center">
					<div
						class="mx-auto flex h-20 w-20 items-center justify-center rounded-full bg-blue-100"
					>
						<IconLoader class="h-10 w-10 animate-spin text-blue-600" />
					</div>
					<h2 class="mt-6 text-2xl font-bold text-gray-900">Verifying Your Email</h2>
					<p class="mt-2 text-gray-600">Please wait while we verify your email address...</p>
				</div>
			{:else if status === 'success'}
				<div class="text-center">
					<div
						class="mx-auto flex h-20 w-20 items-center justify-center rounded-full bg-green-100"
					>
						<IconCheck class="h-10 w-10 text-green-600" />
					</div>
					<h2 class="mt-6 text-2xl font-bold text-gray-900">Email Verified!</h2>
					<p class="mt-2 text-gray-600">
						Your email has been successfully verified. You now have full access to your account.
					</p>

					<div class="mt-8">
						<a
							href="/dashboard"
							class="inline-flex w-full items-center justify-center gap-2 rounded-lg bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-3 font-semibold text-white transition hover:from-blue-700 hover:to-blue-800"
						>
							<IconLogin class="h-5 w-5" />
							Go to Dashboard
						</a>
					</div>

					<div class="mt-6 rounded-lg bg-green-50 p-4">
						<h3 class="font-semibold text-green-800">What's Next?</h3>
						<ul class="mt-2 space-y-1 text-left text-sm text-green-700">
							<li>Complete your profile</li>
							<li>Explore our trading courses</li>
							<li>Join the Discord community</li>
							<li>Set up trading alerts</li>
						</ul>
					</div>
				</div>
			{:else if status === 'expired'}
				<div class="text-center">
					<div
						class="mx-auto flex h-20 w-20 items-center justify-center rounded-full bg-yellow-100"
					>
						<IconMail class="h-10 w-10 text-yellow-600" />
					</div>
					<h2 class="mt-6 text-2xl font-bold text-gray-900">Link Expired</h2>
					<p class="mt-2 text-gray-600">
						This verification link has expired. Don't worry, you can request a new one!
					</p>

					{#if resendSuccess}
						<div class="mt-6 rounded-lg bg-green-50 p-4">
							<p class="text-green-700">
								<IconCheck class="mr-2 inline h-5 w-5" />
								A new verification email has been sent! Check your inbox.
							</p>
						</div>
					{:else}
						<div class="mt-8">
							<button
								onclick={handleResend}
								disabled={resending}
								class="inline-flex w-full items-center justify-center gap-2 rounded-lg bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-3 font-semibold text-white transition hover:from-blue-700 hover:to-blue-800 disabled:opacity-50"
							>
								{#if resending}
									<IconLoader class="h-5 w-5 animate-spin" />
									Sending...
								{:else}
									<IconRefresh class="h-5 w-5" />
									Resend Verification Email
								{/if}
							</button>
						</div>
					{/if}
				</div>
			{:else}
				<div class="text-center">
					<div class="mx-auto flex h-20 w-20 items-center justify-center rounded-full bg-red-100">
						<IconX class="h-10 w-10 text-red-600" />
					</div>
					<h2 class="mt-6 text-2xl font-bold text-gray-900">Verification Failed</h2>
					<p class="mt-2 text-gray-600">{errorMessage}</p>

					{#if resendSuccess}
						<div class="mt-6 rounded-lg bg-green-50 p-4">
							<p class="text-green-700">
								<IconCheck class="mr-2 inline h-5 w-5" />
								A new verification email has been sent! Check your inbox.
							</p>
						</div>
					{:else}
						<div class="mt-8 space-y-3">
							<button
								onclick={handleResend}
								disabled={resending}
								class="inline-flex w-full items-center justify-center gap-2 rounded-lg bg-gradient-to-r from-blue-600 to-blue-700 px-6 py-3 font-semibold text-white transition hover:from-blue-700 hover:to-blue-800 disabled:opacity-50"
							>
								{#if resending}
									<IconLoader class="h-5 w-5 animate-spin" />
									Sending...
								{:else}
									<IconRefresh class="h-5 w-5" />
									Request New Verification Email
								{/if}
							</button>
							<a
								href="/login"
								class="inline-flex w-full items-center justify-center gap-2 rounded-lg border border-gray-300 px-6 py-3 font-semibold text-gray-700 transition hover:bg-gray-50"
							>
								Back to Login
							</a>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Footer -->
		<p class="mt-6 text-center text-sm text-gray-400">
			Need help?
			<a href="/support" class="text-blue-400 hover:text-blue-300">Contact Support</a>
		</p>
	</div>
</div>
