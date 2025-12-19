<script lang="ts">
	/**
	 * Email Verification Page
	 * Handles email verification link callbacks with beautiful UI.
	 * ICT 11+ Principal Engineer Grade
	 *
	 * @version 2.0.0
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
	import { verifyEmail, resendVerificationEmail } from '$lib/api/auth';

	let status: 'loading' | 'success' | 'error' | 'expired' | 'no-token' = 'loading';
	let errorMessage = '';
	let resending = false;
	let resendSuccess = false;
	let userEmail = '';

	async function verify() {
		const token = $page.url.searchParams.get('token');

		if (!token) {
			status = 'no-token';
			errorMessage = 'No verification token provided. Please check your email for the correct link.';
			return;
		}

		try {
			await verifyEmail(token);
			status = 'success';
		} catch (error: any) {
			if (error.message?.includes('expired')) {
				status = 'expired';
				errorMessage = 'This verification link has expired. Please request a new one.';
			} else {
				status = 'error';
				errorMessage = error.message || 'Verification failed. Please try again.';
			}
		}
	}

	async function handleResend() {
		if (!userEmail) {
			errorMessage = 'Please enter your email address to resend verification.';
			return;
		}
		
		resending = true;
		try {
			await resendVerificationEmail(userEmail);
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
	class="flex min-h-screen items-center justify-center bg-gradient-to-br from-slate-900 via-emerald-950 to-slate-900 p-4"
>
	<div class="w-full max-w-md">
		<!-- Logo -->
		<div class="mb-8 text-center">
			<div class="inline-block rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 p-3 mb-4">
				<IconMail class="h-8 w-8 text-white" />
			</div>
			<h1 class="text-3xl font-bold text-white">Revolution Trading Pros</h1>
		</div>

		<!-- Card -->
		<div class="rounded-2xl bg-white/10 backdrop-blur-xl border border-white/20 p-8 shadow-2xl">
			{#if status === 'loading'}
				<div class="text-center">
					<div
						class="mx-auto flex h-20 w-20 items-center justify-center rounded-full bg-emerald-500/20"
					>
						<IconLoader class="h-10 w-10 animate-spin text-emerald-400" />
					</div>
					<h2 class="mt-6 text-2xl font-bold text-white">Verifying Your Email</h2>
					<p class="mt-2 text-slate-300">Please wait while we verify your email address...</p>
				</div>
			{:else if status === 'success'}
				<div class="text-center">
					<div
						class="mx-auto flex h-20 w-20 items-center justify-center rounded-full bg-emerald-500/20"
					>
						<IconCheck class="h-10 w-10 text-emerald-400" />
					</div>
					<h2 class="mt-6 text-2xl font-bold text-white">Email Verified!</h2>
					<p class="mt-2 text-slate-300">
						Your email has been successfully verified. You now have full access to your account.
					</p>

					<div class="mt-8">
						<a
							href="/login"
							class="inline-flex w-full items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 px-6 py-3 font-semibold text-white transition hover:from-emerald-600 hover:to-teal-600"
						>
							<IconLogin class="h-5 w-5" />
							Login to Your Account
						</a>
					</div>

					<div class="mt-6 rounded-xl bg-emerald-500/10 border border-emerald-500/30 p-4">
						<h3 class="font-semibold text-emerald-300">🚀 What's Next?</h3>
						<ul class="mt-2 space-y-1 text-left text-sm text-emerald-200">
							<li>• Complete your profile</li>
							<li>• Explore our trading courses</li>
							<li>• Join the Discord community</li>
							<li>• Set up trading alerts</li>
						</ul>
					</div>
				</div>
			{:else if status === 'expired' || status === 'error' || status === 'no-token'}
				<div class="text-center">
					<div
						class="mx-auto flex h-20 w-20 items-center justify-center rounded-full"
						class:bg-yellow-500/20={status === 'expired'}
						class:bg-red-500/20={status === 'error' || status === 'no-token'}
					>
						{#if status === 'expired'}
							<IconMail class="h-10 w-10 text-yellow-400" />
						{:else}
							<IconX class="h-10 w-10 text-red-400" />
						{/if}
					</div>
					<h2 class="mt-6 text-2xl font-bold text-white">
						{status === 'expired' ? 'Link Expired' : 'Verification Failed'}
					</h2>
					<p class="mt-2 text-slate-300">{errorMessage}</p>

					{#if resendSuccess}
						<div class="mt-6 rounded-xl bg-emerald-500/10 border border-emerald-500/30 p-4">
							<p class="text-emerald-300 flex items-center justify-center gap-2">
								<IconCheck class="h-5 w-5" />
								A new verification email has been sent! Check your inbox.
							</p>
						</div>
					{:else}
						<div class="mt-6 space-y-4">
							<div>
								<label for="email" class="sr-only">Email address</label>
								<input
									id="email"
									type="email"
									bind:value={userEmail}
									placeholder="Enter your email address"
									class="w-full rounded-xl bg-white/5 border border-white/10 px-4 py-3 text-white placeholder-slate-400 focus:border-emerald-500 focus:ring-2 focus:ring-emerald-500/20 focus:outline-none"
								/>
							</div>
							<button
								onclick={handleResend}
								disabled={resending}
								class="inline-flex w-full items-center justify-center gap-2 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 px-6 py-3 font-semibold text-white transition hover:from-emerald-600 hover:to-teal-600 disabled:opacity-50"
							>
								{#if resending}
									<IconLoader class="h-5 w-5 animate-spin" />
									Sending...
								{:else}
									<IconRefresh class="h-5 w-5" />
									Resend Verification Email
								{/if}
							</button>
							<a
								href="/login"
								class="inline-flex w-full items-center justify-center gap-2 rounded-xl border border-white/20 px-6 py-3 font-semibold text-white transition hover:bg-white/5"
							>
								Back to Login
							</a>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Footer -->
		<p class="mt-6 text-center text-sm text-slate-400">
			Need help?
			<a href="/support" class="text-emerald-400 hover:text-emerald-300">Contact Support</a>
		</p>
	</div>
</div>
