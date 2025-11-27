<script lang="ts">
	/**
	 * Social Media Section - Google L11 Enterprise Standard
	 * Mobile-first, brand-accurate colors, proportional icons
	 */

	// Brand colors - official platform colors
	const socials = [
		{
			name: 'YouTube',
			href: 'https://youtube.com/@revolutiontradingpros',
			color: '#FF0000',
			hoverBg: 'rgba(255, 0, 0, 0.1)',
			icon: 'M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z'
		},
		{
			name: 'X',
			href: 'https://x.com/revolutiontrading',
			color: '#000000',
			hoverBg: 'rgba(0, 0, 0, 0.1)',
			icon: 'M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z'
		},
		{
			name: 'Facebook',
			href: 'https://facebook.com/revolutiontradingpros',
			color: '#1877F2',
			hoverBg: 'rgba(24, 119, 242, 0.1)',
			icon: 'M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z'
		},
		{
			name: 'Instagram',
			href: 'https://instagram.com/revolutiontradingpros',
			color: '#E4405F',
			hoverBg: 'rgba(228, 64, 95, 0.1)',
			icon: 'M12 0C8.74 0 8.333.015 7.053.072 5.775.132 4.905.333 4.14.63c-.789.306-1.459.717-2.126 1.384S.935 3.35.63 4.14C.333 4.905.131 5.775.072 7.053.012 8.333 0 8.74 0 12s.015 3.667.072 4.947c.06 1.277.261 2.148.558 2.913.306.788.717 1.459 1.384 2.126.667.666 1.336 1.079 2.126 1.384.766.296 1.636.499 2.913.558C8.333 23.988 8.74 24 12 24s3.667-.015 4.947-.072c1.277-.06 2.148-.262 2.913-.558.788-.306 1.459-.718 2.126-1.384.666-.667 1.079-1.335 1.384-2.126.296-.765.499-1.636.558-2.913.06-1.28.072-1.687.072-4.947s-.015-3.667-.072-4.947c-.06-1.277-.262-2.149-.558-2.913-.306-.789-.718-1.459-1.384-2.126C21.319 1.347 20.651.935 19.86.63c-.765-.297-1.636-.499-2.913-.558C15.667.012 15.26 0 12 0zm0 2.16c3.203 0 3.585.016 4.85.071 1.17.055 1.805.249 2.227.415.562.217.96.477 1.382.896.419.42.679.819.896 1.381.164.422.36 1.057.413 2.227.057 1.266.07 1.646.07 4.85s-.015 3.585-.074 4.85c-.061 1.17-.256 1.805-.421 2.227-.224.562-.479.96-.899 1.382-.419.419-.824.679-1.38.896-.42.164-1.065.36-2.235.413-1.274.057-1.649.07-4.859.07-3.211 0-3.586-.015-4.859-.074-1.171-.061-1.816-.256-2.236-.421-.569-.224-.96-.479-1.379-.899-.421-.419-.69-.824-.9-1.38-.165-.42-.359-1.065-.42-2.235-.045-1.26-.061-1.649-.061-4.844 0-3.196.016-3.586.061-4.861.061-1.17.255-1.814.42-2.234.21-.57.479-.96.9-1.381.419-.419.81-.689 1.379-.898.42-.166 1.051-.361 2.221-.421 1.275-.045 1.65-.06 4.859-.06l.045.03zm0 3.678c-3.405 0-6.162 2.76-6.162 6.162 0 3.405 2.76 6.162 6.162 6.162 3.405 0 6.162-2.76 6.162-6.162 0-3.405-2.76-6.162-6.162-6.162zM12 16c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4zm7.846-10.405c0 .795-.646 1.44-1.44 1.44-.795 0-1.44-.646-1.44-1.44 0-.794.646-1.439 1.44-1.439.793-.001 1.44.645 1.44 1.439z'
		},
		{
			name: 'Discord',
			href: 'https://discord.gg/revolutiontrading',
			color: '#5865F2',
			hoverBg: 'rgba(88, 101, 242, 0.1)',
			icon: 'M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9555 2.4189-2.1569 2.4189zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189Z'
		}
	];
</script>

<section class="social-section" aria-label="Follow us on social media">
	<div class="social-container">
		<p class="social-label">Connect with us</p>
		
		<div class="social-links">
			{#each socials as social (social.name)}
				<a
					href={social.href}
					target="_blank"
					rel="noopener noreferrer"
					class="social-link"
					style="--brand-color: {social.color}; --hover-bg: {social.hoverBg}"
					aria-label="Follow us on {social.name}"
				>
					<svg 
						class="social-icon" 
						viewBox="0 0 24 24" 
						xmlns="http://www.w3.org/2000/svg"
						aria-hidden="true"
					>
						<path d={social.icon}/>
					</svg>
					<span class="social-name">{social.name}</span>
				</a>
			{/each}
		</div>
	</div>
</section>

<style>
	/* Mobile-first base styles */
	.social-section {
		background: #0a0a0a;
		border-top: 1px solid rgba(255, 255, 255, 0.06);
		border-bottom: 1px solid rgba(255, 255, 255, 0.06);
		padding: 2rem 1rem;
	}

	.social-container {
		max-width: 1200px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
	}

	.social-label {
		color: rgba(255, 255, 255, 0.5);
		font-size: 0.75rem;
		font-weight: 500;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		margin: 0;
	}

	.social-links {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 0.5rem;
	}

	.social-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 8px;
		color: rgba(255, 255, 255, 0.7);
		text-decoration: none;
		font-size: 0.8125rem;
		font-weight: 500;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.social-link:hover {
		background: var(--hover-bg);
		border-color: var(--brand-color);
		color: #fff;
		transform: translateY(-1px);
	}

	.social-link:hover .social-icon {
		fill: var(--brand-color);
	}

	.social-link:focus-visible {
		outline: 2px solid var(--brand-color);
		outline-offset: 2px;
	}

	.social-link:active {
		transform: translateY(0);
	}

	.social-icon {
		width: 18px;
		height: 18px;
		fill: currentColor;
		flex-shrink: 0;
		transition: fill 0.2s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.social-name {
		line-height: 1;
	}

	/* Tablet: horizontal layout */
	@media (min-width: 640px) {
		.social-section {
			padding: 2.5rem 1.5rem;
		}

		.social-container {
			flex-direction: row;
			justify-content: center;
			gap: 2rem;
		}

		.social-label {
			margin: 0;
			white-space: nowrap;
		}

		.social-links {
			gap: 0.75rem;
		}
	}

	/* Desktop: refined spacing */
	@media (min-width: 1024px) {
		.social-section {
			padding: 3rem 2rem;
		}

		.social-container {
			gap: 3rem;
		}

		.social-link {
			padding: 0.75rem 1.25rem;
			font-size: 0.875rem;
		}

		.social-icon {
			width: 20px;
			height: 20px;
		}
	}
</style>
