<script lang="ts">
	import {
		IconUser,
		IconMail,
		IconCalendar,
		IconShieldCheck,
		IconChartLine,
		IconBell,
		IconBook
	} from '@tabler/icons-svelte';
	import { onMount } from 'svelte';
	import { gsap } from 'gsap';
	import type { PageData } from './$types';

	export let data: PageData;

	let isVisible = false;
	let dashboardRef: HTMLDivElement;
	let particlesRef: HTMLDivElement;
	let headerRef: HTMLDivElement;
	let cardsRef: HTMLDivElement;
	let actionsRef: HTMLDivElement;

	onMount(() => {
		isVisible = true;

		// Initialize GSAP animations
		initAnimations();
		createFloatingParticles();

		return () => {
			// Cleanup particles
			if (particlesRef) {
				particlesRef.innerHTML = '';
			}
		};
	});

	function initAnimations() {
		const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

		// Animate header
		tl.from(headerRef, {
			opacity: 0,
			y: -50,
			duration: 1,
			ease: 'back.out(1.7)'
		})
			// Animate cards with stagger
			.from(
				'.info-card',
				{
					opacity: 0,
					y: 80,
					scale: 0.9,
					duration: 1,
					stagger: 0.2,
					ease: 'power4.out'
				},
				'-=0.6'
			)
			// Animate quick actions
			.from(
				actionsRef,
				{
					opacity: 0,
					y: 60,
					duration: 0.8
				},
				'-=0.4'
			)
			.from(
				'.action-card',
				{
					opacity: 0,
					scale: 0.8,
					duration: 0.6,
					stagger: 0.15,
					ease: 'back.out(1.4)'
				},
				'-=0.4'
			);

		// Animate icons with pulse
		gsap.to('.icon-pulse', {
			scale: 1.1,
			opacity: 0.8,
			duration: 2,
			repeat: -1,
			yoyo: true,
			ease: 'sine.inOut'
		});

		// Floating card effect on hover
		const cards = document.querySelectorAll('.hover-lift');
		cards.forEach((card) => {
			card.addEventListener('mouseenter', () => {
				gsap.to(card, {
					y: -10,
					duration: 0.3,
					ease: 'power2.out'
				});
			});

			card.addEventListener('mouseleave', () => {
				gsap.to(card, {
					y: 0,
					duration: 0.3,
					ease: 'power2.out'
				});
			});
		});
	}

	function createFloatingParticles() {
		if (!particlesRef) return;

		// Create 50 particles with varying colors
		const colors = ['#6366f1', '#8b5cf6', '#ec4899', '#06b6d4', '#10b981'];

		for (let i = 0; i < 50; i++) {
			const particle = document.createElement('div');
			particle.className = 'floating-particle';

			const size = Math.random() * 5 + 2;
			const color = colors[Math.floor(Math.random() * colors.length)];
			const startX = Math.random() * 100;
			const startY = Math.random() * 100;
			const duration = Math.random() * 25 + 20;
			const delay = Math.random() * 5;

			particle.style.width = `${size}px`;
			particle.style.height = `${size}px`;
			particle.style.backgroundColor = color;
			particle.style.left = `${startX}%`;
			particle.style.top = `${startY}%`;
			particle.style.borderRadius = '50%';
			particle.style.position = 'absolute';
			particle.style.pointerEvents = 'none';
			particle.style.opacity = '0.6';
			particle.style.filter = 'blur(1px)';

			particlesRef.appendChild(particle);

			// Animate particles
			gsap.to(particle, {
				y: -150,
				x: `+=${Math.random() * 150 - 75}`,
				opacity: 0,
				duration: duration,
				delay: delay,
				repeat: -1,
				ease: 'none'
			});

			// Add rotation
			gsap.to(particle, {
				rotation: 360,
				duration: duration / 2,
				delay: delay,
				repeat: -1,
				ease: 'none'
			});
		}
	}

	function formatDate(dateString: string | null | undefined): string {
		if (!dateString) return 'Not verified';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>Dashboard | Revolution Trading Pros</title>
</svelte:head>

<div
	class="dashboard-page min-h-screen px-4 py-12 overflow-hidden relative"
	bind:this={dashboardRef}
>
	<!-- Animated gradient background -->
	<div class="gradient-bg absolute inset-0"></div>

	<!-- Grid overlay -->
	<div class="grid-overlay absolute inset-0"></div>

	<!-- Floating particles container -->
	<div
		class="particles-container absolute inset-0 pointer-events-none overflow-hidden"
		bind:this={particlesRef}
	></div>

	<!-- Floating orbs -->
	<div class="glow-orb glow-orb-1"></div>
	<div class="glow-orb glow-orb-2"></div>
	<div class="glow-orb glow-orb-3"></div>

	<!-- Dashboard content -->
	<div class="relative max-w-6xl mx-auto z-10">
		<!-- Header -->
		<div class="mb-12" bind:this={headerRef}>
			<h1
				class="text-5xl md:text-6xl font-heading font-bold mb-4 bg-gradient-to-r from-indigo-300 via-purple-300 to-pink-300 bg-clip-text text-transparent"
			>
				Dashboard
			</h1>
			<p class="text-xl md:text-2xl text-slate-300">
				Welcome back, <span class="text-indigo-400 font-semibold glow-text">{data.user?.name}</span>
			</p>
		</div>

		<!-- User info cards grid -->
		<div class="grid md:grid-cols-2 gap-6 mb-12" bind:this={cardsRef}>
			<!-- Account info card -->
			<div class="info-card card-wrapper hover-lift">
				<div
					class="absolute -inset-1 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 rounded-3xl opacity-30 blur-2xl"
				></div>
				<div
					class="relative bg-slate-900/95 backdrop-blur-xl border border-indigo-500/30 rounded-3xl p-8 shadow-2xl"
				>
					<div class="flex items-center gap-4 mb-8">
						<div
							class="w-20 h-20 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-600 p-0.5 icon-pulse shadow-lg shadow-indigo-500/50"
						>
							<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
								<IconUser size={36} class="text-indigo-400" />
							</div>
						</div>
						<div>
							<h2 class="text-2xl font-heading font-bold text-white mb-1">Account Information</h2>
							<p class="text-slate-400 text-sm">Your profile details</p>
						</div>
					</div>

					<div class="space-y-3">
						<div
							class="info-item flex items-center gap-3 p-4 bg-gradient-to-r from-slate-800/60 to-slate-800/40 rounded-xl border border-slate-700/50 hover:border-indigo-500/30 transition-all duration-300"
						>
							<div class="w-10 h-10 rounded-lg bg-indigo-500/10 flex items-center justify-center">
								<IconUser size={20} class="text-indigo-400" />
							</div>
							<div>
								<p class="text-xs text-slate-500 uppercase tracking-wide font-semibold">Name</p>
								<p class="text-white font-medium text-lg">{data.user?.name}</p>
							</div>
						</div>

						<div
							class="info-item flex items-center gap-3 p-4 bg-gradient-to-r from-slate-800/60 to-slate-800/40 rounded-xl border border-slate-700/50 hover:border-indigo-500/30 transition-all duration-300"
						>
							<div class="w-10 h-10 rounded-lg bg-indigo-500/10 flex items-center justify-center">
								<IconMail size={20} class="text-indigo-400" />
							</div>
							<div>
								<p class="text-xs text-slate-500 uppercase tracking-wide font-semibold">Email</p>
								<p class="text-white font-medium text-lg">{data.user?.email}</p>
							</div>
						</div>

						<div
							class="info-item flex items-center gap-3 p-4 bg-gradient-to-r from-slate-800/60 to-slate-800/40 rounded-xl border border-slate-700/50 hover:border-indigo-500/30 transition-all duration-300"
						>
							<div class="w-10 h-10 rounded-lg bg-indigo-500/10 flex items-center justify-center">
								<IconCalendar size={20} class="text-indigo-400" />
							</div>
							<div>
								<p class="text-xs text-slate-500 uppercase tracking-wide font-semibold">
									Member Since
								</p>
								<p class="text-white font-medium text-lg">{formatDate(data.user?.created_at)}</p>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Email verification card -->
			<div class="info-card card-wrapper hover-lift">
				<div
					class="absolute -inset-1 bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 rounded-3xl opacity-30 blur-2xl"
				></div>
				<div
					class="relative bg-slate-900/95 backdrop-blur-xl border border-emerald-500/30 rounded-3xl p-8 shadow-2xl"
				>
					<div class="flex items-center gap-4 mb-8">
						<div
							class="w-20 h-20 rounded-2xl bg-gradient-to-br from-emerald-500 to-teal-600 p-0.5 icon-pulse shadow-lg shadow-emerald-500/50"
						>
							<div class="w-full h-full bg-slate-900 rounded-2xl flex items-center justify-center">
								<IconShieldCheck size={36} class="text-emerald-400" />
							</div>
						</div>
						<div>
							<h2 class="text-2xl font-heading font-bold text-white mb-1">Verification Status</h2>
							<p class="text-slate-400 text-sm">Email verification</p>
						</div>
					</div>

					{#if data.user?.email_verified_at}
						<div
							class="p-6 bg-gradient-to-br from-emerald-500/15 to-teal-500/10 border border-emerald-500/40 rounded-2xl shadow-lg shadow-emerald-500/10"
						>
							<div class="flex items-center gap-3 mb-3">
								<div
									class="w-12 h-12 rounded-full bg-emerald-500/20 flex items-center justify-center"
								>
									<IconShieldCheck size={28} class="text-emerald-400" />
								</div>
								<p class="text-xl font-bold text-emerald-300">Email Verified</p>
							</div>
							<p class="text-sm text-emerald-400/80 ml-15">
								Verified on {formatDate(data.user?.email_verified_at)}
							</p>
						</div>
					{:else}
						<div
							class="p-6 bg-gradient-to-br from-amber-500/15 to-orange-500/10 border border-amber-500/40 rounded-2xl shadow-lg shadow-amber-500/10"
						>
							<div class="flex items-center gap-3 mb-4">
								<div
									class="w-12 h-12 rounded-full bg-amber-500/20 flex items-center justify-center"
								>
									<IconMail size={28} class="text-amber-400" />
								</div>
								<p class="text-xl font-bold text-amber-300">Email Not Verified</p>
							</div>
							<p class="text-sm text-amber-400/80 mb-5">
								Please check your email and click the verification link.
							</p>
							<button
								class="w-full px-6 py-3.5 bg-gradient-to-r from-amber-500 to-orange-500 text-white font-bold rounded-xl hover:shadow-lg hover:shadow-amber-500/50 transition-all duration-300 transform hover:scale-105"
							>
								Resend Verification Email
							</button>
						</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- Quick actions -->
		<div class="mb-12" bind:this={actionsRef}>
			<h2
				class="text-3xl font-heading font-bold text-white mb-8 bg-gradient-to-r from-white via-slate-200 to-slate-400 bg-clip-text text-transparent"
			>
				Quick Actions
			</h2>
			<div class="grid sm:grid-cols-3 gap-6">
				<a
					href="/trading-rooms"
					class="action-card group relative p-8 bg-slate-900/80 backdrop-blur-xl border border-purple-500/30 rounded-2xl overflow-hidden transition-all duration-500 hover:transform hover:scale-105 hover:shadow-2xl hover:shadow-purple-500/20"
				>
					<div
						class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"
					></div>
					<div class="relative z-10">
						<div
							class="w-14 h-14 rounded-xl bg-gradient-to-br from-purple-500 to-pink-500 p-0.5 mb-5 shadow-lg shadow-purple-500/30"
						>
							<div class="w-full h-full bg-slate-900 rounded-xl flex items-center justify-center">
								<IconChartLine size={28} class="text-purple-400" />
							</div>
						</div>
						<h3
							class="text-xl font-bold text-white mb-2 group-hover:text-purple-300 transition-colors"
						>
							Trading Rooms
						</h3>
						<p class="text-sm text-slate-400 group-hover:text-slate-300 transition-colors">
							Join live trading sessions
						</p>
					</div>
					<div
						class="absolute bottom-0 left-0 right-0 h-1 bg-gradient-to-r from-purple-500 to-pink-500 transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 origin-left"
					></div>
				</a>

				<a
					href="/alert-services"
					class="action-card group relative p-8 bg-slate-900/80 backdrop-blur-xl border border-amber-500/30 rounded-2xl overflow-hidden transition-all duration-500 hover:transform hover:scale-105 hover:shadow-2xl hover:shadow-amber-500/20"
				>
					<div
						class="absolute inset-0 bg-gradient-to-br from-amber-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"
					></div>
					<div class="relative z-10">
						<div
							class="w-14 h-14 rounded-xl bg-gradient-to-br from-amber-500 to-orange-500 p-0.5 mb-5 shadow-lg shadow-amber-500/30"
						>
							<div class="w-full h-full bg-slate-900 rounded-xl flex items-center justify-center">
								<IconBell size={28} class="text-amber-400" />
							</div>
						</div>
						<h3
							class="text-xl font-bold text-white mb-2 group-hover:text-amber-300 transition-colors"
						>
							Alert Services
						</h3>
						<p class="text-sm text-slate-400 group-hover:text-slate-300 transition-colors">
							Manage your subscriptions
						</p>
					</div>
					<div
						class="absolute bottom-0 left-0 right-0 h-1 bg-gradient-to-r from-amber-500 to-orange-500 transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 origin-left"
					></div>
				</a>

				<a
					href="/courses"
					class="action-card group relative p-8 bg-slate-900/80 backdrop-blur-xl border border-cyan-500/30 rounded-2xl overflow-hidden transition-all duration-500 hover:transform hover:scale-105 hover:shadow-2xl hover:shadow-cyan-500/20"
				>
					<div
						class="absolute inset-0 bg-gradient-to-br from-cyan-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"
					></div>
					<div class="relative z-10">
						<div
							class="w-14 h-14 rounded-xl bg-gradient-to-br from-cyan-500 to-blue-500 p-0.5 mb-5 shadow-lg shadow-cyan-500/30"
						>
							<div class="w-full h-full bg-slate-900 rounded-xl flex items-center justify-center">
								<IconBook size={28} class="text-cyan-400" />
							</div>
						</div>
						<h3
							class="text-xl font-bold text-white mb-2 group-hover:text-cyan-300 transition-colors"
						>
							Courses
						</h3>
						<p class="text-sm text-slate-400 group-hover:text-slate-300 transition-colors">
							Continue your education
						</p>
					</div>
					<div
						class="absolute bottom-0 left-0 right-0 h-1 bg-gradient-to-r from-cyan-500 to-blue-500 transform scale-x-0 group-hover:scale-x-100 transition-transform duration-500 origin-left"
					></div>
				</a>
			</div>
		</div>
	</div>
</div>

<style>
	/* Animated gradient background */
	.gradient-bg {
		background: linear-gradient(
			135deg,
			#0f172a 0%,
			#1e1b4b 20%,
			#312e81 40%,
			#1e1b4b 60%,
			#0f172a 80%,
			#1e1b4b 100%
		);
		background-size: 400% 400%;
		animation: gradientShift 25s ease infinite;
	}

	@keyframes gradientShift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	/* Grid overlay */
	.grid-overlay {
		background-image:
			linear-gradient(rgba(99, 102, 241, 0.03) 1px, transparent 1px),
			linear-gradient(90deg, rgba(99, 102, 241, 0.03) 1px, transparent 1px);
		background-size: 50px 50px;
		animation: gridMove 30s linear infinite;
		pointer-events: none;
	}

	@keyframes gridMove {
		0% {
			transform: translate(0, 0);
		}
		100% {
			transform: translate(50px, 50px);
		}
	}

	/* Glowing orbs */
	.glow-orb {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.4;
		pointer-events: none;
	}

	.glow-orb-1 {
		width: 600px;
		height: 600px;
		top: -100px;
		left: -100px;
		background: radial-gradient(circle, #6366f1, transparent 70%);
		animation: float1 25s ease-in-out infinite;
	}

	.glow-orb-2 {
		width: 500px;
		height: 500px;
		bottom: -100px;
		right: -100px;
		background: radial-gradient(circle, #8b5cf6, transparent 70%);
		animation: float2 30s ease-in-out infinite;
	}

	.glow-orb-3 {
		width: 450px;
		height: 450px;
		top: 50%;
		left: 50%;
		background: radial-gradient(circle, #ec4899, transparent 70%);
		animation: float3 35s ease-in-out infinite;
	}

	@keyframes float1 {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(100px, 50px) scale(1.1);
		}
		66% {
			transform: translate(-50px, 100px) scale(0.9);
		}
	}

	@keyframes float2 {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(-100px, -50px) scale(1.1);
		}
		66% {
			transform: translate(50px, -100px) scale(0.9);
		}
	}

	@keyframes float3 {
		0%,
		100% {
			transform: translate(-50%, -50%) scale(1);
		}
		50% {
			transform: translate(-50%, -50%) scale(1.2);
		}
	}

	/* Text glow effect */
	.glow-text {
		text-shadow:
			0 0 20px rgba(99, 102, 241, 0.5),
			0 0 40px rgba(99, 102, 241, 0.3);
		animation: textGlow 3s ease-in-out infinite;
	}

	@keyframes textGlow {
		0%,
		100% {
			text-shadow:
				0 0 20px rgba(99, 102, 241, 0.5),
				0 0 40px rgba(99, 102, 241, 0.3);
		}
		50% {
			text-shadow:
				0 0 30px rgba(99, 102, 241, 0.7),
				0 0 60px rgba(99, 102, 241, 0.5);
		}
	}

	/* Card wrapper */
	.card-wrapper {
		position: relative;
	}

	/* Info item hover effects */
	.info-item {
		transition: all 0.3s ease;
	}

	.info-item:hover {
		transform: translateX(5px);
		background: linear-gradient(to right, rgba(99, 102, 241, 0.1), rgba(99, 102, 241, 0.05));
	}

	/* Responsive adjustments */
	@media (max-width: 640px) {
		.glow-orb {
			filter: blur(60px);
			opacity: 0.3;
		}

		.glow-orb-1 {
			width: 400px;
			height: 400px;
		}

		.glow-orb-2 {
			width: 350px;
			height: 350px;
		}

		.glow-orb-3 {
			width: 300px;
			height: 300px;
		}
	}

	/* Particle base styles */
	:global(.floating-particle) {
		will-change: transform, opacity;
	}
</style>
