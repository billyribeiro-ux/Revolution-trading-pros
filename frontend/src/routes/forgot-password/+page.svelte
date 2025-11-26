<script lang="ts">
	import { forgotPassword } from '$lib/api/auth';
	import { IconMail, IconAlertCircle, IconCheck, IconSend } from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';

	let email = '';
	let errors: Record<string, string[]> = {};
	let generalError = '';
	let successMessage = '';
	let isLoading = false;
	let isVisible = false;

	onMount(() => {
		isVisible = true;
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};
		generalError = '';
		successMessage = '';
		isLoading = true;

		try {
			const message = await forgotPassword({ email });
			successMessage = message;
			email = ''; // Clear email field after success
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
	title="Forgot Password - Reset Your Account"
	description="Forgot your password? Reset your Revolution Trading Pros account password securely."
	canonical="/forgot-password"
	ogType="website"
	noIndex={true}
/>

<div
	class="forgot-password-page min-h-screen flex items-center justify-center px-4 py-12 overflow-hidden relative"
>
	<!-- Animated gradient background -->
	<div
		class="absolute inset-0 bg-gradient-to-br from-slate-950 via-amber-950/30 to-slate-950"
	></div>

	<!-- Floating orbs -->
	<div
		class="absolute top-20 left-10 w-96 h-96 bg-amber-500/10 rounded-full blur-3xl animate-float"
	></div>
	<div
		class="absolute bottom-20 right-10 w-96 h-96 bg-yellow-500/10 rounded-full blur-3xl animate-float-delayed"
	></div>

	<!-- Forgot password card -->
	<div
		class="forgot-password-card relative w-full max-w-md z-10"
		class:animate-card-reveal={isVisible}
	>
		<!-- Glow effect -->
		<div
			class="absolute -inset-1 bg-gradient-to-r from-amber-500 via-yellow-500 to-orange-500 rounded-3xl opacity-20 blur-xl"
		></div>

		<div
			class="relative bg-slate-900/90 backdrop-blur-xl border border-amber-500/20 rounded-3xl p-10 shadow-2xl"
		>
			<!-- Header -->
			<div class="text-center mb-8">
				<div
					class="inline-flex items-center justify-center w-20 h-20 rounded-2xl bg-gradient-to-br from-amber-500 to-orange-500 p-0.5 mb-6"
				>
					<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
						<IconMail size={40} class="text-amber-400" />
					</div>
				</div>
				<h1
					class="text-4xl font-heading font-bold mb-3 bg-gradient-to-r from-amber-300 via-yellow-300 to-orange-300 bg-clip-text text-transparent"
				>
					Forgot Password?
				</h1>
				<p class="text-slate-400">No worries, we'll send you reset instructions</p>
			</div>

			<!-- Success message -->
			{#if successMessage}
				<div
					class="mb-6 p-4 bg-emerald-500/10 border border-emerald-500/30 rounded-xl flex items-start gap-3 animate-fade-in"
				>
					<IconCheck size={20} class="text-emerald-400 flex-shrink-0 mt-0.5" />
					<p class="text-sm text-emerald-300">{successMessage}</p>
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

			<!-- Forgot password form -->
			<form on:submit={handleSubmit} class="space-y-6">
				<!-- Email field -->
				<div class="form-group">
					<label for="email" class="block text-sm font-semibold text-slate-300 mb-2">
						Email Address
					</label>
					<div class="relative">
						<div class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500">
							<IconMail size={20} />
						</div>
						<input
							id="email"
							type="email"
							bind:value={email}
							required
							class="w-full pl-12 pr-4 py-3.5 bg-slate-800/50 border border-slate-700 rounded-xl text-white placeholder-slate-500 focus:border-amber-500 focus:ring-2 focus:ring-amber-500/20 focus:outline-none transition-all duration-300"
							class:border-red-500={errors.email}
							placeholder="you@example.com"
						/>
					</div>
					{#if errors.email}
						<p class="mt-2 text-sm text-red-400">{errors.email[0]}</p>
					{/if}
				</div>

				<!-- Submit button -->
				<button
					type="submit"
					disabled={isLoading}
					class="w-full relative px-6 py-4 bg-gradient-to-r from-amber-500 via-yellow-500 to-orange-500 text-slate-900 font-heading font-bold rounded-xl overflow-hidden group transition-all duration-300 hover:shadow-lg hover:shadow-amber-500/50 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					<span class="relative z-10 flex items-center justify-center gap-2">
						{#if isLoading}
							<div
								class="w-5 h-5 border-2 border-slate-900/30 border-t-slate-900 rounded-full animate-spin"
							></div>
							<span>Sending...</span>
						{:else}
							<IconSend size={20} />
							<span>Send Reset Link</span>
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
					class="text-amber-400 hover:text-amber-300 font-semibold transition-colors duration-300 flex items-center justify-center gap-2"
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
		box-shadow: 0 0 0 4px rgba(251, 191, 36, 0.1);
	}
</style>
