<script lang="ts">
	/**
	 * Signup Page - Svelte 5 Runes Implementation
	 * @version 2.0.0 - November 2025
	 */
	import { authStore } from '$lib/stores/auth';
	import { registerAndLogin } from '$lib/api/auth';
	import { goto } from '$app/navigation';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Svelte 5 state runes
	let name = $state('');
	let email = $state('');
	let password = $state('');
	let passwordConfirmation = $state('');
	let isLoading = $state(false);
	let errorMessage = $state('');
	let validationErrors = $state<Record<string, string[]>>({});

	// Redirect if already logged in - Svelte 5 effect
	$effect(() => {
		if ($authStore.user) {
			goto('/account');
		}
	});

	async function handleSignup(e: Event) {
		e.preventDefault();
		errorMessage = '';
		validationErrors = {};
		isLoading = true;

		// Client-side validation
		if (password !== passwordConfirmation) {
			errorMessage = 'Passwords do not match';
			isLoading = false;
			return;
		}

		if (password.length < 8) {
			errorMessage = 'Password must be at least 8 characters';
			isLoading = false;
			return;
		}

		try {
			await registerAndLogin({
				name,
				email,
				password,
				password_confirmation: passwordConfirmation
			});
			goto('/account');
		} catch (error: any) {
			errorMessage = error.message || 'Registration failed. Please try again.';
			if (error.errors) {
				validationErrors = error.errors;
			}
		} finally {
			isLoading = false;
		}
	}
</script>

<SEOHead
	title="Sign Up - Create Your Trading Account"
	description="Join Revolution Trading Pros. Create your free account to access live trading rooms, professional alerts, courses, and our trading community."
	canonical="/signup"
	ogType="website"
	keywords={['trading signup', 'create trading account', 'join revolution trading pros', 'trading membership']}
/>

<div class="min-h-[calc(100vh-120px)] flex items-center justify-center bg-rtp-bg px-4 sm:px-6 py-8 sm:py-12">
	<div class="w-full max-w-md">
		<!-- Card -->
		<div class="bg-rtp-surface rounded-2xl shadow-xl p-6 sm:p-8 border border-rtp-border">
			<!-- Logo/Header -->
			<div class="text-center mb-6 sm:mb-8">
				<h1 class="text-2xl sm:text-3xl font-heading font-bold text-rtp-text mb-2">Join Revolution Trading</h1>
				<p class="text-rtp-muted">Start your trading journey today</p>
			</div>

			<!-- Error Message -->
			{#if errorMessage}
				<div class="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
					<p class="text-red-800 text-sm">{errorMessage}</p>
					{#if Object.keys(validationErrors).length > 0}
						<ul class="mt-2 list-disc list-inside text-red-700 text-sm">
							{#each Object.entries(validationErrors) as [, errors]}
								{#each errors as error}
									<li>{error}</li>
								{/each}
							{/each}
						</ul>
					{/if}
				</div>
			{/if}

			<!-- Signup Form -->
			<form onsubmit={handleSignup} class="space-y-6">
				<!-- Name -->
				<div>
					<label for="name" class="block text-sm font-semibold text-rtp-text mb-2">
						Full Name
					</label>
					<input
						type="text"
						id="name"
						bind:value={name}
						required
						disabled={isLoading}
						class="w-full px-4 py-3.5 border border-rtp-border rounded-lg focus:ring-2 focus:ring-rtp-primary focus:border-transparent transition-all disabled:opacity-50 disabled:cursor-not-allowed text-base min-h-[44px]"
						placeholder="John Doe"
					/>
					{#if validationErrors.name}
						<p class="mt-1 text-sm text-red-600">{validationErrors.name[0]}</p>
					{/if}
				</div>

				<!-- Email -->
				<div>
					<label for="email" class="block text-sm font-semibold text-rtp-text mb-2">
						Email Address
					</label>
					<input
						type="email"
						id="email"
						bind:value={email}
						required
						disabled={isLoading}
						class="w-full px-4 py-3.5 border border-rtp-border rounded-lg focus:ring-2 focus:ring-rtp-primary focus:border-transparent transition-all disabled:opacity-50 disabled:cursor-not-allowed text-base min-h-[44px]"
						placeholder="you@example.com"
					/>
					{#if validationErrors.email}
						<p class="mt-1 text-sm text-red-600">{validationErrors.email[0]}</p>
					{/if}
				</div>

				<!-- Password -->
				<div>
					<label for="password" class="block text-sm font-semibold text-rtp-text mb-2">
						Password
					</label>
					<input
						type="password"
						id="password"
						bind:value={password}
						required
						minlength="8"
						disabled={isLoading}
						class="w-full px-4 py-3.5 border border-rtp-border rounded-lg focus:ring-2 focus:ring-rtp-primary focus:border-transparent transition-all disabled:opacity-50 disabled:cursor-not-allowed text-base min-h-[44px]"
						placeholder="••••••••"
					/>
					<p class="mt-1 text-xs text-rtp-muted">Minimum 8 characters</p>
					{#if validationErrors.password}
						<p class="mt-1 text-sm text-red-600">{validationErrors.password[0]}</p>
					{/if}
				</div>

				<!-- Confirm Password -->
				<div>
					<label for="password_confirmation" class="block text-sm font-semibold text-rtp-text mb-2">
						Confirm Password
					</label>
					<input
						type="password"
						id="password_confirmation"
						bind:value={passwordConfirmation}
						required
						minlength="8"
						disabled={isLoading}
						class="w-full px-4 py-3.5 border border-rtp-border rounded-lg focus:ring-2 focus:ring-rtp-primary focus:border-transparent transition-all disabled:opacity-50 disabled:cursor-not-allowed text-base min-h-[44px]"
						placeholder="••••••••"
					/>
				</div>

				<!-- Terms Checkbox -->
				<div>
					<label class="flex items-start cursor-pointer p-2 -m-2 rounded-lg hover:bg-rtp-border/10">
						<input
							type="checkbox"
							required
							class="w-5 h-5 mt-0.5 text-rtp-primary border-rtp-border rounded focus:ring-rtp-primary min-w-[20px]"
						/>
						<span class="ml-3 text-sm text-rtp-muted">
							I agree to the
							<a href="/terms" class="text-rtp-primary hover:underline">Terms of Service</a>
							and
							<a href="/privacy" class="text-rtp-primary hover:underline">Privacy Policy</a>
						</span>
					</label>
				</div>

				<!-- Submit Button -->
				<button
					type="submit"
					disabled={isLoading}
					class="w-full py-4 px-4 bg-gradient-to-r from-rtp-primary to-rtp-blue text-white font-bold rounded-lg hover:shadow-lg transform hover:scale-[1.02] transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none min-h-[48px] text-base"
				>
					{#if isLoading}
						<span class="inline-flex items-center">
							<svg
								class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
								xmlns="http://www.w3.org/2000/svg"
								fill="none"
								viewBox="0 0 24 24"
							>
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
								></path>
							</svg>
							Creating account...
						</span>
					{:else}
						Create Account
					{/if}
				</button>
			</form>

			<!-- Divider -->
			<div class="relative my-6">
				<div class="absolute inset-0 flex items-center">
					<div class="w-full border-t border-rtp-border"></div>
				</div>
				<div class="relative flex justify-center text-sm">
					<span class="px-2 bg-rtp-surface text-rtp-muted">Or</span>
				</div>
			</div>

			<!-- Sign In Link -->
			<div class="text-center">
				<p class="text-rtp-muted">
					Already have an account?
					<a href="/login" class="text-rtp-primary font-semibold hover:underline"> Sign in </a>
				</p>
			</div>
		</div>

		<!-- Benefits -->
		<div class="mt-8 grid grid-cols-1 sm:grid-cols-3 gap-4 text-center">
			<div>
				<div class="text-2xl font-bold text-rtp-primary">10K+</div>
				<div class="text-xs text-rtp-muted mt-1">Active Traders</div>
			</div>
			<div>
				<div class="text-2xl font-bold text-rtp-emerald">78%</div>
				<div class="text-xs text-rtp-muted mt-1">Win Rate</div>
			</div>
			<div>
				<div class="text-2xl font-bold text-rtp-indigo">$23M+</div>
				<div class="text-xs text-rtp-muted mt-1">Total Profits</div>
			</div>
		</div>
	</div>
</div>
