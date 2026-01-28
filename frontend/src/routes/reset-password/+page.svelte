<script lang="ts">
	import { goto } from '$app/navigation';
	import { resetPassword } from '$lib/api/auth';
	import {
		IconLock,
		IconAlertCircle,
		IconCheck,
		IconShieldCheck,
		IconEye,
		IconEyeOff
	} from '$lib/icons';
	import { onMount } from 'svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';

	let email = '';
	let password = '';
	let password_confirmation = '';
	let token = '';
	let showPassword = false;
	let showConfirmPassword = false;
	let errors: Record<string, string[]> = {};
	let generalError = '';
	let successMessage = '';
	let isLoading = false;
	let isVisible = false;

	onMount(() => {
		isVisible = true;
		// Get token and email from URL parameters
		const urlParams = new URLSearchParams(window.location.search);
		token = urlParams.get('token') || '';
		email = urlParams.get('email') || '';
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};
		generalError = '';
		successMessage = '';
		isLoading = true;

		try {
			const message = await resetPassword({ token, email, password, password_confirmation });
			successMessage = message;

			// Redirect to login after 2 seconds
			setTimeout(() => {
				goto('/login');
			}, 2000);
		} catch (error) {
			if (error && typeof error === 'object' && 'errors' in error) {
				errors = (error as { errors: Record<string, string[]> }).errors;
			} else if (error instanceof Error) {
				generalError = error.message;
			} else {
				generalError = 'An unexpected error occurred. Please try again.';
			}
		} finally {
			isLoading = false;
		}
	}
</script>

<SEOHead
	title="Reset Password - Secure Account Recovery"
	description="Reset your Revolution Trading Pros account password securely. Create a new password for your trading account."
	canonical="/reset-password"
	ogType="website"
	noindex={true}
/>

<div
	class="reset-password-page flex items-center justify-center px-4 py-12 overflow-hidden relative"
>
	<!-- Animated gradient background -->
	<div
		class="absolute inset-0 bg-gradient-to-br from-slate-950 via-purple-950/30 to-slate-950"
	></div>

	<!-- Floating orbs -->
	<div
		class="absolute top-20 right-10 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-float"
	></div>
	<div
		class="absolute bottom-20 left-10 w-96 h-96 bg-pink-500/10 rounded-full blur-3xl animate-float-delayed"
	></div>

	<!-- Reset password card -->
	<div
		class="reset-password-card relative w-full max-w-md z-10"
		class:animate-card-reveal={isVisible}
	>
		<!-- Glow effect -->
		<div
			class="absolute -inset-1 bg-gradient-to-r from-purple-500 via-pink-500 to-rose-500 rounded-3xl opacity-20 blur-xl"
		></div>

		<div
			class="relative bg-slate-900/90 backdrop-blur-xl border border-purple-500/20 rounded-3xl p-10 shadow-2xl"
		>
			<!-- Header -->
			<div class="text-center mb-8">
				<div
					class="inline-flex items-center justify-center w-20 h-20 rounded-2xl bg-gradient-to-br from-purple-500 to-pink-500 p-0.5 mb-6"
				>
					<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
						<IconShieldCheck size={40} class="text-purple-400" />
					</div>
				</div>
				<h1
					class="text-4xl font-heading font-bold mb-3 bg-gradient-to-r from-purple-300 via-pink-300 to-rose-300 bg-clip-text text-transparent"
				>
					Reset Password
				</h1>
				<p class="text-slate-400">Enter your new password below</p>
			</div>

			<!-- Success message -->
			{#if successMessage}
				<div
					class="mb-6 p-4 bg-emerald-500/10 border border-emerald-500/30 rounded-xl flex items-start gap-3 animate-fade-in"
				>
					<IconCheck size={20} class="text-emerald-400 flex-shrink-0 mt-0.5" />
					<div class="text-sm text-emerald-300">
						<p>{successMessage}</p>
						<p class="mt-1 text-emerald-400/70">Redirecting to login...</p>
					</div>
				</div>
			{/if}

			<!-- General error -->
			{#if generalError}
				<div
					class="mb-6 p-4 bg-red-500/10 border border-red-500/30 rounded-xl flex items-start gap-3 animate-fade-in"
				>
					<IconAlertCircle size={20} class="text-red-400 flex-shrink-0 mt-0.5" />
					<p class="text-sm text-red-300">{generalError}</p>
				</div>
			{/if}

			<!-- Reset password form -->
			<form onsubmit={handleSubmit} class="space-y-5">
				<!-- Email field (hidden, auto-filled from URL) -->
				<input type="hidden" name="email" bind:value={email} />
				<input type="hidden" name="token" bind:value={token} />

				<!-- Email display (read-only) -->
				<div class="form-group">
					<div class="block text-sm font-semibold text-slate-300 mb-2">Email Address</div>
					<div
						class="px-4 py-3.5 bg-slate-800/30 border border-slate-700/50 rounded-xl text-slate-400"
					>
						{email}
					</div>
				</div>

				<!-- New password field -->
				<div class="form-group">
					<label for="password" class="block text-sm font-semibold text-slate-300 mb-2">
						New Password
					</label>
					<div class="relative">
						<div class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500">
							<IconLock size={20} />
						</div>
						<input
							id="password"
							type={showPassword ? 'text' : 'password'}
							bind:value={password}
							required
							minlength="8"
							class="w-full pl-12 pr-12 py-3.5 bg-slate-800/50 border border-slate-700 rounded-xl text-white placeholder-slate-500 focus:border-purple-500 focus:ring-2 focus:ring-purple-500/20 focus:outline-none transition-all duration-300"
							class:border-red-500={errors.password}
							placeholder="••••••••"
						/>
						<button
							type="button"
							class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-500 hover:text-purple-400 transition-colors"
							onclick={() => (showPassword = !showPassword)}
							aria-label={showPassword ? 'Hide password' : 'Show password'}
							tabindex={-1}
						>
							{#if showPassword}
								<IconEyeOff size={20} />
							{:else}
								<IconEye size={20} />
							{/if}
						</button>
					</div>
					{#if errors.password}
						<p class="mt-2 text-sm text-red-400">{errors.password[0]}</p>
					{:else}
						<p class="mt-2 text-xs text-slate-500">Minimum 8 characters</p>
					{/if}
				</div>

				<!-- Confirm password field -->
				<div class="form-group">
					<label
						for="password_confirmation"
						class="block text-sm font-semibold text-slate-300 mb-2"
					>
						Confirm New Password
					</label>
					<div class="relative">
						<div class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500">
							<IconLock size={20} />
						</div>
						<input
							id="password_confirmation"
							type={showConfirmPassword ? 'text' : 'password'}
							bind:value={password_confirmation}
							required
							class="w-full pl-12 pr-12 py-3.5 bg-slate-800/50 border border-slate-700 rounded-xl text-white placeholder-slate-500 focus:border-purple-500 focus:ring-2 focus:ring-purple-500/20 focus:outline-none transition-all duration-300"
							placeholder="••••••••"
						/>
						<button
							type="button"
							class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-500 hover:text-purple-400 transition-colors"
							onclick={() => (showConfirmPassword = !showConfirmPassword)}
							aria-label={showConfirmPassword ? 'Hide password' : 'Show password'}
							tabindex={-1}
						>
							{#if showConfirmPassword}
								<IconEyeOff size={20} />
							{:else}
								<IconEye size={20} />
							{/if}
						</button>
					</div>
				</div>

				<!-- Submit button -->
				<button
					type="submit"
					disabled={isLoading || !!successMessage}
					class="w-full relative px-6 py-4 bg-gradient-to-r from-purple-500 via-pink-500 to-rose-500 text-white font-heading font-bold rounded-xl overflow-hidden group transition-all duration-300 hover:shadow-lg hover:shadow-purple-500/50 disabled:opacity-50 disabled:cursor-not-allowed mt-6"
				>
					<span class="relative z-10 flex items-center justify-center gap-2">
						{#if isLoading}
							<div
								class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"
							></div>
							<span>Resetting Password...</span>
						{:else if successMessage}
							<IconCheck size={20} />
							<span>Password Reset!</span>
						{:else}
							<IconShieldCheck size={20} />
							<span>Reset Password</span>
						{/if}
					</span>
					<div
						class="absolute inset-0 bg-white/20 transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 origin-left"
					></div>
				</button>
			</form>

			<!-- Back to login link -->
			<div class="mt-8 text-center">
				<a
					href="/login"
					class="text-purple-400 hover:text-purple-300 font-semibold transition-colors duration-300 flex items-center justify-center gap-2"
				>
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M10 19l-7-7m0 0l7-7m-7 7h18"
						/>
					</svg>
					<span>Back to login</span>
				</a>
			</div>
		</div>
	</div>
</div>

<style>
	/* Floating animations */
	@keyframes float {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(-20px) scale(1.05);
		}
	}

	@keyframes float-delayed {
		0%,
		100% {
			transform: translateY(0) scale(1);
		}
		50% {
			transform: translateY(30px) scale(0.95);
		}
	}

	.animate-float {
		animation: float 8s ease-in-out infinite;
	}

	.animate-float-delayed {
		animation: float-delayed 10s ease-in-out infinite;
	}

	/* Card reveal animation */
	@keyframes card-reveal {
		from {
			opacity: 0;
			transform: translateY(40px) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.animate-card-reveal {
		animation: card-reveal 0.6s ease-out forwards;
		opacity: 0;
	}

	/* Fade in animation */
	@keyframes fade-in {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.animate-fade-in {
		animation: fade-in 0.3s ease-out;
	}

	/* Input focus glow */
	input:focus {
		box-shadow: 0 0 0 4px rgba(168, 85, 247, 0.1);
	}
</style>
