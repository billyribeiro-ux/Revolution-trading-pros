<script lang="ts">
	/**
	 * NavBar Component - Google L7+ Principal Engineer Ultimate Fix
	 * ═══════════════════════════════════════════════════════════════════════════
	 * 
	 * CRITICAL BUG FIXES IDENTIFIED & RESOLVED:
	 * 
	 * 1. HAMBURGER MENU NOT SHOWING (PRIMARY ISSUE):
	 *    - Fixed: Svelte 5 class directive syntax (class:name={condition})
	 *    - Fixed: Event handler syntax (onclick not onClick)
	 *    - Fixed: Component instantiation with {@const Icon = component}
	 *    - Fixed: Proper inert attribute handling
	 * 
	 * 2. SVELTE 5 RUNE COMPLIANCE:
	 *    - Fixed: Proper $state() initialization
	 *    - Fixed: $derived usage for reactive computations
	 *    - Fixed: $effect with proper cleanup returns
	 * 
	 * 3. EVENT HANDLER FIXES:
	 *    - Fixed: onload handler in svelte:head (invalid syntax)
	 *    - Fixed: All mouse events to lowercase (onmouseenter, onmouseleave)
	 *    - Fixed: Proper event delegation with correct target handling
	 * 
	 * 4. COMPONENT RENDERING:
	 *    - Fixed: Dynamic component rendering with proper syntax
	 *    - Fixed: Class binding with proper Svelte 5 syntax
	 *    - Fixed: Conditional rendering with proper boolean handling
	 * 
	 * @version 4.0.0 (Google L7+ Ultimate)
	 * @license MIT
	 */

	import { onMount, onDestroy, tick } from 'svelte';
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import {
		IconMenu2,
		IconX,
		IconUser,
		IconLogout,
		IconShoppingCart,
		IconBook,
		IconChartLine,
		IconStar,
		IconSettings,
		IconChevronDown,
		IconChevronRight
	} from '@tabler/icons-svelte';
	import { authStore, user, isAuthenticated } from '$lib/stores/auth';
	import { cartItemCount, hasCartItems } from '$lib/stores/cart';
	import { logout as logoutApi, getUser } from '$lib/api/auth';

	// ─────────────────────────────────────────────────────────────────────────────
	// Type Definitions (Google L7+ Type Safety)
	// ─────────────────────────────────────────────────────────────────────────────
	interface NavItem {
		id: string;
		label: string;
		href?: string;
		submenu?: Array<{
			href: string;
			label: string;
		}>;
	}

	interface UserMenuItem {
		href: string;
		label: string;
		icon: typeof IconBook; // Proper type for Tabler icons
	}

	interface TouchPoint {
		x: number;
		y: number;
		time: number;
	}

	interface RenderMetrics {
		fps: number;
		jank: number;
		longTasks: number;
	}

	// ─────────────────────────────────────────────────────────────────────────────
	// Constants
	// ─────────────────────────────────────────────────────────────────────────────
	const BREAKPOINTS = Object.freeze({
		mobile: 768,
		tablet: 1024,
		desktop: 1025
	});

	const TIMING = Object.freeze({
		ANIMATION_DURATION: 250,
		DEBOUNCE_THRESHOLD: 100,
		TOUCH_THRESHOLD: 50,
		HEALTH_CHECK_INTERVAL: 10000,
		FOCUS_DELAY: 50,
		STATE_SYNC_DELAY: 16
	});

	const ERROR_RECOVERY = Object.freeze({
		MAX_ATTEMPTS: 3,
		BACKOFF_BASE: 1000,
		TELEMETRY_ENDPOINT: '/api/telemetry/navbar'
	});

	// ─────────────────────────────────────────────────────────────────────────────
	// State Management (Svelte 5 Runes)
	// ─────────────────────────────────────────────────────────────────────────────
	let isMobileMenuOpen = $state(false);
	let activeDropdown = $state<string | null>(null);
	let activeMobileSubmenu = $state<string | null>(null);
	let isUserMenuOpen = $state(false);
	let windowWidth = $state(0);
	let isAnimating = $state(false);
	let prefersReducedMotion = $state(false);
	let isOnline = $state(true);
	let hasHydrated = $state(false);

	// Performance monitoring
	let performanceObserver: PerformanceObserver | null = null;
	let renderMetrics = $state<RenderMetrics>({
		fps: 60,
		jank: 0,
		longTasks: 0
	});

	// DOM References
	let headerRef = $state<HTMLElement>();
	let mobileNavRef = $state<HTMLElement>();
	let hamburgerRef = $state<HTMLButtonElement>();
	
	// Focus management
	let focusableElements = new Map<string, HTMLElement[]>();
	let currentFocusIndex = 0;
	let escapeKeyStack: string[] = [];

	// Touch handling
	let touchStartPoint: TouchPoint = { x: 0, y: 0, time: 0 };
	let touchVelocity = { x: 0, y: 0 };

	// Cleanup
	const abortController = new AbortController();
	const { signal } = abortController;
	const animationFrames = new Set<number>();
	
	// Telemetry
	const telemetryQueue: Array<{
		error: Error;
		context: Record<string, any>;
		timestamp: number;
	}> = [];

	// ─────────────────────────────────────────────────────────────────────────────
	// Derived State (Svelte 5 $derived)
	// ─────────────────────────────────────────────────────────────────────────────
	const isMobile = $derived(windowWidth > 0 && windowWidth < BREAKPOINTS.mobile);
	const isTablet = $derived(windowWidth >= BREAKPOINTS.mobile && windowWidth < BREAKPOINTS.tablet);
	const isDesktop = $derived(windowWidth >= BREAKPOINTS.tablet);
	const showMobileNav = $derived(isMobile || isTablet);
	const shouldReduceMotion = $derived(prefersReducedMotion || renderMetrics.fps < 30);
	const isLowEndDevice = $derived(
		(browser && navigator.hardwareConcurrency <= 2) || renderMetrics.longTasks > 5
	);

	// ─────────────────────────────────────────────────────────────────────────────
	// Effects (Svelte 5 $effect)
	// ─────────────────────────────────────────────────────────────────────────────

	// Auto-close mobile menu on desktop
	$effect(() => {
		if (isDesktop && isMobileMenuOpen) {
			requestAnimationFrame(() => closeMobileMenu());
		}
	});

	// Reset dropdown on mobile
	$effect(() => {
		if (showMobileNav && activeDropdown) {
			activeDropdown = null;
		}
	});

	// Body scroll lock with proper cleanup
	$effect(() => {
		if (!browser || !hasHydrated) return;

		const body = document.body;
		
		if (isMobileMenuOpen) {
			// Save scroll position
			const scrollY = window.scrollY;
			body.dataset.scrollY = String(scrollY);
			
			// Lock scroll
			body.style.position = 'fixed';
			body.style.top = `-${scrollY}px`;
			body.style.width = '100%';
			body.style.overflow = 'hidden';
			body.style.touchAction = 'none';
			
			// Accessibility
			body.setAttribute('aria-hidden', 'true');
			headerRef?.setAttribute('aria-expanded', 'true');
		} else {
			// Restore scroll position
			const storedScrollY = parseInt(body.dataset.scrollY || '0', 10);
			
			// Unlock scroll
			body.style.position = '';
			body.style.top = '';
			body.style.width = '';
			body.style.overflow = '';
			body.style.touchAction = '';
			
			// Accessibility
			body.removeAttribute('aria-hidden');
			headerRef?.setAttribute('aria-expanded', 'false');
			
			// Restore position
			if (storedScrollY) {
				window.scrollTo(0, storedScrollY);
				delete body.dataset.scrollY;
			}
		}

		// Cleanup function
		return () => {
			body.style.cssText = '';
			body.removeAttribute('aria-hidden');
			delete body.dataset.scrollY;
		};
	});

	// ─────────────────────────────────────────────────────────────────────────────
	// Focus Management
	// ─────────────────────────────────────────────────────────────────────────────
	function getFocusableElements(container: HTMLElement): HTMLElement[] {
		const selectors = [
			'button:not([disabled]):not([aria-hidden="true"])',
			'a[href]:not([aria-hidden="true"])',
			'input:not([disabled]):not([type="hidden"]):not([aria-hidden="true"])',
			'select:not([disabled]):not([aria-hidden="true"])',
			'textarea:not([disabled]):not([aria-hidden="true"])',
			'[tabindex]:not([tabindex="-1"]):not([aria-hidden="true"])'
		];

		const elements = Array.from(
			container.querySelectorAll<HTMLElement>(selectors.join(', '))
		);

		return elements.filter(el => {
			const rect = el.getBoundingClientRect();
			const style = window.getComputedStyle(el);
			return (
				rect.width > 0 &&
				rect.height > 0 &&
				style.visibility !== 'hidden' &&
				style.display !== 'none'
			);
		});
	}

	async function updateFocusableElements(): Promise<void> {
		if (!mobileNavRef || !browser) return;

		try {
			await tick();
			const elements = getFocusableElements(mobileNavRef);
			focusableElements.set('mobile-nav', elements);
			
			elements.forEach((el, index) => {
				el.tabIndex = index === currentFocusIndex ? 0 : -1;
			});
		} catch (error) {
			logError('Focus management failed', error as Error, { component: 'navbar' });
		}
	}

	function handleFocusNavigation(event: KeyboardEvent): void {
		if (!isMobileMenuOpen) return;

		const elements = focusableElements.get('mobile-nav') || [];
		if (elements.length === 0) return;

		let handled = false;

		switch (event.key) {
			case 'ArrowDown':
			case 'ArrowRight':
				currentFocusIndex = (currentFocusIndex + 1) % elements.length;
				handled = true;
				break;
			case 'ArrowUp':
			case 'ArrowLeft':
				currentFocusIndex = (currentFocusIndex - 1 + elements.length) % elements.length;
				handled = true;
				break;
			case 'Home':
				currentFocusIndex = 0;
				handled = true;
				break;
			case 'End':
				currentFocusIndex = elements.length - 1;
				handled = true;
				break;
		}

		if (handled) {
			event.preventDefault();
			elements.forEach((el, index) => {
				el.tabIndex = index === currentFocusIndex ? 0 : -1;
			});
			elements[currentFocusIndex]?.focus();
		}
	}

	// ─────────────────────────────────────────────────────────────────────────────
	// Event Handlers
	// ─────────────────────────────────────────────────────────────────────────────
	async function handleMobileMenuToggle(): Promise<void> {
		performance.mark('menu-toggle-start');

		try {
			// Debounce
			const now = performance.now();
			const lastToggle = parseFloat(hamburgerRef?.dataset.lastToggle || '0');
			if (now - lastToggle < TIMING.DEBOUNCE_THRESHOLD) {
				return;
			}
			if (hamburgerRef) {
				hamburgerRef.dataset.lastToggle = String(now);
			}

			// Animation lock
			if (isAnimating) {
				return;
			}

			isAnimating = true;
			const targetState = !isMobileMenuOpen;

			const rafId = requestAnimationFrame(async () => {
				isMobileMenuOpen = targetState;

				if (isMobileMenuOpen) {
					escapeKeyStack.push('mobile-menu');
					await tick();
					await updateFocusableElements();
					
					announceToScreenReader('Mobile menu opened');
					
					const focusRafId = requestAnimationFrame(() => {
						const elements = focusableElements.get('mobile-nav');
						elements?.[0]?.focus();
						animationFrames.delete(focusRafId);
					});
					animationFrames.add(focusRafId);
				} else {
					escapeKeyStack = escapeKeyStack.filter(id => id !== 'mobile-menu');
					activeMobileSubmenu = null;
					
					announceToScreenReader('Mobile menu closed');
					hamburgerRef?.focus();
				}

				setTimeout(() => {
					isAnimating = false;
					performance.mark('menu-toggle-end');
					performance.measure('menu-toggle', 'menu-toggle-start', 'menu-toggle-end');
				}, TIMING.ANIMATION_DURATION);
			});
			animationFrames.add(rafId);

		} catch (error) {
			logError('Menu toggle failed', error as Error, { state: isMobileMenuOpen });
			await recoverMenuState();
		}
	}

	function closeMobileMenu(): void {
		if (!isMobileMenuOpen) return;

		try {
			isMobileMenuOpen = false;
			activeMobileSubmenu = null;
			escapeKeyStack = escapeKeyStack.filter(id => id !== 'mobile-menu');
			currentFocusIndex = 0;
			focusableElements.delete('mobile-nav');
			hamburgerRef?.focus();
		} catch (error) {
			logError('Close menu failed', error as Error);
			isMobileMenuOpen = false;
			activeMobileSubmenu = null;
		}
	}

	async function handleLogout(): Promise<void> {
		try {
			closeMobileMenu();
			const button = document.activeElement as HTMLElement;
			button?.setAttribute('aria-busy', 'true');
			
			await logoutApi();
			await goto('/login');
		} catch (error) {
			logError('Logout failed', error as Error);
			authStore.clearAuth();
			await goto('/login');
		} finally {
			const button = document.activeElement as HTMLElement;
			button?.setAttribute('aria-busy', 'false');
		}
	}

	function handleDocumentClick(event: MouseEvent): void {
		if (!browser) return;

		const target = event.target as HTMLElement;
		
		if (hamburgerRef?.contains(target)) return;

		const clickHandlers: Array<[string, () => void]> = [
			['.nav-dropdown', () => { activeDropdown = null; }],
			['.nav-user', () => { isUserMenuOpen = false; }],
			['.nav-mobile', () => { if (!mobileNavRef?.contains(target)) closeMobileMenu(); }]
		];

		for (const [selector, handler] of clickHandlers) {
			const element = document.querySelector(selector);
			if (element && !element.contains(target)) {
				handler();
			}
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
		try {
			if (event.key === 'Escape') {
				event.preventDefault();
				const current = escapeKeyStack.pop();
				
				switch (current) {
					case 'mobile-menu':
						closeMobileMenu();
						break;
					case 'dropdown':
						activeDropdown = null;
						break;
					case 'user-menu':
						isUserMenuOpen = false;
						break;
				}
			}

			if (isMobileMenuOpen) {
				handleFocusNavigation(event);
			}
		} catch (error) {
			logError('Keyboard handler failed', error as Error);
		}
	}

	function handleTouchStart(event: TouchEvent): void {
		if (!isMobileMenuOpen) return;

		const touch = event.touches[0];
		touchStartPoint = {
			x: touch.clientX,
			y: touch.clientY,
			time: Date.now()
		};
	}

	function handleTouchEnd(event: TouchEvent): void {
		if (!isMobileMenuOpen) return;

		const touch = event.changedTouches[0];
		const deltaX = touch.clientX - touchStartPoint.x;
		const deltaY = touch.clientY - touchStartPoint.y;
		const deltaTime = Date.now() - touchStartPoint.time;

		touchVelocity = {
			x: deltaX / deltaTime,
			y: deltaY / deltaTime
		};

		const isHorizontalSwipe = Math.abs(deltaX) > Math.abs(deltaY);
		const isLeftSwipe = deltaX < -TIMING.TOUCH_THRESHOLD;
		const isFastSwipe = Math.abs(touchVelocity.x) > 0.5;

		if (isHorizontalSwipe && isLeftSwipe && isFastSwipe) {
			closeMobileMenu();
		}
	}

	// ─────────────────────────────────────────────────────────────────────────────
	// Error Handling & Telemetry
	// ─────────────────────────────────────────────────────────────────────────────
	function logError(message: string, error: Error, context: Record<string, any> = {}): void {
		console.error(`[NavBar] ${message}:`, error, context);

		telemetryQueue.push({
			error,
			context: {
				...context,
				message,
				userAgent: navigator.userAgent,
				viewport: { width: windowWidth, height: window.innerHeight },
				timestamp: Date.now()
			},
			timestamp: Date.now()
		});

		if ('requestIdleCallback' in window) {
			requestIdleCallback(() => sendTelemetry());
		}
	}

	async function sendTelemetry(): Promise<void> {
		if (telemetryQueue.length === 0 || !isOnline) return;

		const batch = telemetryQueue.splice(0, 10);

		try {
			await fetch(ERROR_RECOVERY.TELEMETRY_ENDPOINT, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ errors: batch })
			});
		} catch {
			telemetryQueue.unshift(...batch);
		}
	}

	async function recoverMenuState(attempt = 1): Promise<void> {
		if (attempt > ERROR_RECOVERY.MAX_ATTEMPTS) {
			console.error('[NavBar] Max recovery attempts reached');
			isMobileMenuOpen = false;
			activeMobileSubmenu = null;
			activeDropdown = null;
			isUserMenuOpen = false;
			isAnimating = false;
			escapeKeyStack = [];
			currentFocusIndex = 0;
			
			animationFrames.forEach(id => cancelAnimationFrame(id));
			animationFrames.clear();
			
			if (browser) {
				document.body.style.cssText = '';
				document.body.removeAttribute('aria-hidden');
			}
			return;
		}

		const backoff = ERROR_RECOVERY.BACKOFF_BASE * Math.pow(2, attempt - 1);
		console.warn(`[NavBar] Recovery attempt ${attempt}/${ERROR_RECOVERY.MAX_ATTEMPTS}`);

		await new Promise(resolve => setTimeout(resolve, backoff));

		try {
			if (isAnimating && animationFrames.size === 0) {
				isAnimating = false;
			}

			const bodyLocked = document.body.style.overflow === 'hidden';
			if (bodyLocked !== isMobileMenuOpen) {
				if (isMobileMenuOpen) {
					document.body.style.overflow = 'hidden';
				} else {
					document.body.style.cssText = '';
				}
			}
		} catch (error) {
			await recoverMenuState(attempt + 1);
		}
	}

	function announceToScreenReader(message: string): void {
		if (!browser) return;

		const announcement = document.createElement('div');
		announcement.setAttribute('role', 'status');
		announcement.setAttribute('aria-live', 'polite');
		announcement.className = 'sr-only';
		announcement.textContent = message;
		
		document.body.appendChild(announcement);
		setTimeout(() => announcement.remove(), 1000);
	}

	// ─────────────────────────────────────────────────────────────────────────────
	// Performance Monitoring
	// ─────────────────────────────────────────────────────────────────────────────
	function initPerformanceMonitoring(): void {
		if (!browser || !('PerformanceObserver' in window)) return;

		try {
			performanceObserver = new PerformanceObserver((list) => {
				for (const entry of list.getEntries()) {
					if (entry.duration > 50) {
						renderMetrics.longTasks++;
					}
				}
			});

			if ('longtask' in PerformanceObserver.supportedEntryTypes) {
				performanceObserver.observe({ entryTypes: ['longtask'] });
			}

			let lastTime = performance.now();
			let frames = 0;

			function measureFPS(): void {
				frames++;
				const currentTime = performance.now();

				if (currentTime >= lastTime + 1000) {
					renderMetrics.fps = Math.round((frames * 1000) / (currentTime - lastTime));
					frames = 0;
					lastTime = currentTime;
				}

				if (browser) {
					requestAnimationFrame(measureFPS);
				}
			}

			requestAnimationFrame(measureFPS);
		} catch (error) {
			console.warn('[NavBar] Performance monitoring failed:', error);
		}
	}

	// ─────────────────────────────────────────────────────────────────────────────
	// Lifecycle
	// ─────────────────────────────────────────────────────────────────────────────
	let healthCheckInterval: number | null = null;

	onMount(async () => {
		if (!browser) return;

		console.debug('[NavBar] Mounting with Google L7+ standards...');

		try {
			windowWidth = window.innerWidth;
			isOnline = navigator.onLine;

			const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
			prefersReducedMotion = mediaQuery.matches;
			
			mediaQuery.addEventListener('change', (e) => {
				prefersReducedMotion = e.matches;
			}, { signal });

			const viewportQuery = window.matchMedia(`(min-width: ${BREAKPOINTS.tablet}px)`);
			const handleViewportChange = () => {
				windowWidth = window.innerWidth;
			};
			viewportQuery.addEventListener('change', handleViewportChange, { signal });
			window.addEventListener('resize', handleViewportChange, { signal, passive: true });

			window.addEventListener('online', () => { isOnline = true; }, { signal });
			window.addEventListener('offline', () => { isOnline = false; }, { signal });

			const token = authStore.getToken();
			if (token && !$user) {
				try {
					await getUser();
				} catch (error) {
					console.error('User fetch failed:', error);
					authStore.clearAuth();
				}
			}

			document.addEventListener('click', handleDocumentClick, { passive: true, signal });
			document.addEventListener('keydown', handleKeydown, { signal });
			
			if ('ontouchstart' in window) {
				document.addEventListener('touchstart', handleTouchStart, { passive: true, signal });
				document.addEventListener('touchend', handleTouchEnd, { passive: true, signal });
			}

			initPerformanceMonitoring();

			healthCheckInterval = window.setInterval(() => {
				if (renderMetrics.fps < 30 || renderMetrics.longTasks > 10) {
					console.warn('[NavBar] Performance degradation detected', renderMetrics);
				}
			}, TIMING.HEALTH_CHECK_INTERVAL);

			hasHydrated = true;
			
			console.debug('[NavBar] Mount complete');
		} catch (error) {
			logError('Mount failed', error as Error);
			await recoverMenuState();
		}
	});

	onDestroy(() => {
		if (!browser) return;

		console.debug('[NavBar] Unmounting...');

		try {
			abortController.abort();

			if (healthCheckInterval !== null) {
				clearInterval(healthCheckInterval);
			}

			animationFrames.forEach(id => cancelAnimationFrame(id));
			animationFrames.clear();

			performanceObserver?.disconnect();

			focusableElements.clear();

			document.body.style.cssText = '';
			document.body.removeAttribute('aria-hidden');

			if (telemetryQueue.length > 0 && navigator.sendBeacon) {
				navigator.sendBeacon(ERROR_RECOVERY.TELEMETRY_ENDPOINT, JSON.stringify({
					errors: telemetryQueue
				}));
			}

			console.debug('[NavBar] Unmount complete');
		} catch (error) {
			console.error('[NavBar] Unmount error:', error);
		}
	});

	// ─────────────────────────────────────────────────────────────────────────────
	// Navigation Configuration
	// ─────────────────────────────────────────────────────────────────────────────
	const navItems: NavItem[] = [
		{
			id: 'live',
			label: 'Live Trading Rooms',
			submenu: [
				{ href: '/day-trading', label: 'Day Trading Room' },
				{ href: '/swing-trading', label: 'Swing Trading Room' },
				{ href: '/small-accounts', label: 'Small Accounts Room' }
			]
		},
		{
			id: 'alerts',
			label: 'Alert Services',
			submenu: [
				{ href: '/spx-profit-pulse', label: 'SPX Profit Pulse' },
				{ href: '/explosive-swings', label: 'Explosive Swings' }
			]
		},
		{ id: 'mentorship', label: 'Mentorship', href: '/mentorship' },
		{
			id: 'store',
			label: 'Store',
			submenu: [
				{ href: '/courses', label: 'Courses' },
				{ href: '/indicators', label: 'Indicators' }
			]
		},
		{ id: 'mission', label: 'Our Mission', href: '/our-mission' },
		{ id: 'about', label: 'About', href: '/about' },
		{
			id: 'resources',
			label: 'Resources',
			submenu: [
				{ href: '/resources/etf-stocks-list', label: 'ETF Stocks List' },
				{ href: '/resources/stock-indexes-list', label: 'Stock Indexes List' }
			]
		},
		{ id: 'blog', label: 'Blog', href: '/blog' }
	];

	const userMenuItems: UserMenuItem[] = [
		{ href: '/dashboard/courses', label: 'My Courses', icon: IconBook },
		{ href: '/dashboard/indicators', label: 'My Indicators', icon: IconChartLine },
		{ href: '/dashboard/memberships', label: 'My Memberships', icon: IconStar },
		{ href: '/dashboard/account', label: 'My Account', icon: IconSettings }
	];
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Montserrat:wght@400;500;600;700&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<header
	class="nav-header"
	class:nav-header--menu-open={isMobileMenuOpen}
	class:nav-header--reduced-motion={shouldReduceMotion}
	class:nav-header--low-end={isLowEndDevice}
	bind:this={headerRef}
	aria-label="Main navigation"
>
	<div class="nav-container">
		<div class="nav-inner">
			<!-- Logo -->
			<a 
				href="/" 
				class="nav-logo" 
				aria-label="Revolution Trading Pros - Home"
			>
				<img
					src="/revolution-trading-pros.png"
					alt="Revolution Trading Pros Logo"
					class="nav-logo__image"
					width="260"
					height="88"
					loading="eager"
					fetchpriority="high"
					decoding="async"
				/>
			</a>

			<!-- Desktop Navigation -->
			{#if !showMobileNav}
				<nav class="nav-desktop" aria-label="Main navigation">
					{#each navItems as item}
						{#if item.submenu}
							<div
								class="nav-dropdown"
								role="none"
								onmouseenter={() => !prefersReducedMotion && (activeDropdown = item.id)}
								onmouseleave={() => !prefersReducedMotion && (activeDropdown = null)}
							>
								<button
									class="nav-link nav-link--dropdown"
									type="button"
									aria-expanded={activeDropdown === item.id}
									aria-haspopup="true"
									aria-controls={`dropdown-${item.id}`}
									onclick={(e) => {
										e.stopPropagation();
										activeDropdown = activeDropdown === item.id ? null : item.id;
										if (activeDropdown) escapeKeyStack.push('dropdown');
									}}
								>
									<span>{item.label}</span>
									<IconChevronDown
										size={16}
										class={`nav-link__icon${activeDropdown === item.id ? ' nav-link__icon--rotated' : ''}`}
										aria-hidden="true"
									/>
								</button>
								<div
									id={`dropdown-${item.id}`}
									class="nav-dropdown__menu"
									class:nav-dropdown__menu--open={activeDropdown === item.id}
									role="menu"
									aria-labelledby={`dropdown-trigger-${item.id}`}
								>
									{#each item.submenu as subItem}
										<a
											href={subItem.href}
											class="nav-dropdown__item"
											role="menuitem"
											tabindex={activeDropdown === item.id ? 0 : -1}
											onclick={() => {
												activeDropdown = null;
												escapeKeyStack = escapeKeyStack.filter(id => id !== 'dropdown');
											}}
										>
											{subItem.label}
										</a>
									{/each}
								</div>
							</div>
						{:else}
							<a href={item.href} class="nav-link">
								{item.label}
							</a>
						{/if}
					{/each}
				</nav>
			{/if}

			<!-- Right Actions -->
			<div class="nav-actions">
				{#if !showMobileNav}
					<!-- Cart (Desktop) -->
					{#if $hasCartItems}
						<a 
							href="/cart" 
							class="nav-cart" 
							aria-label="Shopping cart with {$cartItemCount} {$cartItemCount === 1 ? 'item' : 'items'}"
						>
							<IconShoppingCart size={22} aria-hidden="true" />
							{#if $cartItemCount > 0}
								<span class="nav-cart__badge" aria-label="{$cartItemCount} items">
									{$cartItemCount}
								</span>
							{/if}
						</a>
					{/if}

					<!-- User Menu (Desktop) -->
					{#if $isAuthenticated}
						<div class="nav-user" role="navigation" aria-label="User menu">
							<button
								class="nav-user__trigger"
								type="button"
								aria-haspopup="true"
								aria-expanded={isUserMenuOpen}
								aria-controls="user-menu"
								onclick={(e) => {
									e.stopPropagation();
									isUserMenuOpen = !isUserMenuOpen;
									if (isUserMenuOpen) {
										activeDropdown = null;
										escapeKeyStack.push('user-menu');
									} else {
										escapeKeyStack = escapeKeyStack.filter(id => id !== 'user-menu');
									}
								}}
							>
								<IconUser size={20} aria-hidden="true" />
								<span class="nav-user__name">{$user?.name || 'Account'}</span>
								<IconChevronDown
									size={16}
									class={isUserMenuOpen ? 'nav-user__icon--rotated' : ''}
									aria-hidden="true"
								/>
							</button>
							<div
								id="user-menu"
								class="nav-user__menu"
								class:nav-user__menu--open={isUserMenuOpen}
								role="menu"
								aria-labelledby="user-menu-trigger"
							>
								{#each userMenuItems as menuItem}
									{@const Icon = menuItem.icon}
									<a 
										href={menuItem.href} 
										class="nav-user__item" 
										role="menuitem"
										tabindex={isUserMenuOpen ? 0 : -1}
										onclick={() => {
											isUserMenuOpen = false;
											escapeKeyStack = escapeKeyStack.filter(id => id !== 'user-menu');
										}}
									>
										<Icon size={16} aria-hidden="true" />
										<span>{menuItem.label}</span>
									</a>
								{/each}
								<button 
									onclick={handleLogout} 
									class="nav-user__item nav-user__item--logout"
									role="menuitem"
									tabindex={isUserMenuOpen ? 0 : -1}
									type="button"
								>
									<IconLogout size={16} aria-hidden="true" />
									<span>Logout</span>
								</button>
							</div>
						</div>
					{:else}
						<a href="/login" class="nav-login" aria-label="Login to your account">
							<span class="nav-login__inner">
								<IconUser size={24} aria-hidden="true" />
								<span>Login</span>
							</span>
						</a>
					{/if}
				{/if}

				<!-- Hamburger Button (Mobile/Tablet) -->
				{#if showMobileNav}
					<button
						class="nav-hamburger"
						class:nav-hamburger--active={isMobileMenuOpen}
						type="button"
						aria-label={isMobileMenuOpen ? 'Close navigation menu' : 'Open navigation menu'}
						aria-expanded={isMobileMenuOpen}
						aria-controls="mobile-navigation"
						bind:this={hamburgerRef}
						onclick={handleMobileMenuToggle}
						disabled={isAnimating}
						data-last-toggle="0"
					>
						<span class="nav-hamburger__icon" aria-hidden="true">
							{#if isMobileMenuOpen}
								<IconX size={28} strokeWidth={2.5} />
							{:else}
								<IconMenu2 size={28} strokeWidth={2.5} />
							{/if}
						</span>
					</button>
				{/if}
			</div>
		</div>

		<!-- Mobile Navigation Panel -->
		{#if showMobileNav}
			<nav
				id="mobile-navigation"
				class="nav-mobile"
				class:nav-mobile--open={isMobileMenuOpen}
				bind:this={mobileNavRef}
				aria-label="Mobile navigation menu"
				aria-hidden={!isMobileMenuOpen}
				inert={!isMobileMenuOpen ? true : null}
			>
				<!-- Backdrop -->
				<div
					class="nav-mobile__backdrop"
					class:nav-mobile__backdrop--visible={isMobileMenuOpen}
					onclick={closeMobileMenu}
					role="presentation"
					aria-hidden="true"
				></div>

				<!-- Panel Content -->
				<div
					class="nav-mobile__panel"
					class:nav-mobile__panel--open={isMobileMenuOpen}
				>
					<div class="nav-mobile__content" role="navigation">
						{#each navItems as item}
							{#if item.submenu}
								<div class="nav-mobile__group">
									<button
										class="nav-mobile__button"
										type="button"
										aria-expanded={activeMobileSubmenu === item.id}
										aria-controls={`mobile-submenu-${item.id}`}
										onclick={() => {
											activeMobileSubmenu = activeMobileSubmenu === item.id ? null : item.id;
										}}
									>
										<span>{item.label}</span>
										<IconChevronRight
											size={20}
											class={`nav-mobile__chevron${activeMobileSubmenu === item.id ? ' nav-mobile__chevron--rotated' : ''}`}
											aria-hidden="true"
										/>
									</button>
									<div
										id={`mobile-submenu-${item.id}`}
										class="nav-mobile__submenu"
										class:nav-mobile__submenu--open={activeMobileSubmenu === item.id}
										role="group"
									>
										{#each item.submenu as subItem}
											<a 
												href={subItem.href} 
												class="nav-mobile__sublink" 
												onclick={closeMobileMenu}
												tabindex={activeMobileSubmenu === item.id ? 0 : -1}
											>
												{subItem.label}
											</a>
										{/each}
									</div>
								</div>
							{:else}
								<a href={item.href} class="nav-mobile__link" onclick={closeMobileMenu}>
									{item.label}
								</a>
							{/if}
						{/each}

						<!-- User Section -->
						<div class="nav-mobile__user">
							{#if $isAuthenticated}
								<div class="nav-mobile__user-info">
									<IconUser size={20} aria-hidden="true" />
									<span>{$user?.name || 'Account'}</span>
								</div>
								{#each userMenuItems as menuItem}
									{@const Icon = menuItem.icon}
									<a href={menuItem.href} class="nav-mobile__user-link" onclick={closeMobileMenu}>
										<Icon size={18} aria-hidden="true" />
										<span>{menuItem.label}</span>
									</a>
								{/each}
								<button onclick={handleLogout} class="nav-mobile__logout" type="button">
									<IconLogout size={20} aria-hidden="true" />
									<span>Logout</span>
								</button>
							{:else}
								<a href="/login" class="nav-mobile__login" onclick={closeMobileMenu}>
									<span class="nav-mobile__login-inner">
										<IconUser size={24} aria-hidden="true" />
										<span>Login</span>
									</span>
								</a>
							{/if}
						</div>
					</div>
				</div>
			</nav>
		{/if}
	</div>
</header>

<!-- Screen reader announcements -->
<div class="sr-only" aria-live="polite" aria-atomic="true"></div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * CSS Custom Properties
	 * ═══════════════════════════════════════════════════════════════════════════ */
	:root {
		--nav-height-desktop: 120px;
		--nav-height-tablet: 90px;
		--nav-height-mobile: 72px;
		--nav-bg: #05142b;
		--nav-bg-transparent: rgba(5, 20, 43, 0.98);
		--nav-border: rgba(148, 163, 253, 0.16);
		--nav-text: #e5e7eb;
		--nav-text-hover: #facc15;
		--nav-accent: #facc15;
		--nav-accent-bg: rgba(250, 204, 21, 0.1);
		--nav-dropdown-bg: rgba(5, 14, 31, 0.98);
		--nav-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
		--nav-transition: 250ms cubic-bezier(0.4, 0, 0.2, 1);
		--nav-transition-fast: 150ms cubic-bezier(0.4, 0, 0.2, 1);
		--z-dropdown: 100;
		--z-mobile-backdrop: 998;
		--z-mobile-panel: 999;
		--z-header: 1000;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Base Header
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.nav-header {
		position: sticky;
		top: 0;
		left: 0;
		right: 0;
		z-index: var(--z-header);
		background: var(--nav-bg);
		border-bottom: 1px solid var(--nav-border);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		will-change: transform;
		contain: layout style paint;
	}

	.nav-header--reduced-motion *,
	.nav-header--reduced-motion *::before,
	.nav-header--reduced-motion *::after {
		animation-duration: 0.01ms !important;
		animation-iteration-count: 1 !important;
		transition-duration: 0.01ms !important;
	}

	.nav-header--low-end {
		backdrop-filter: none;
		-webkit-backdrop-filter: none;
	}

	.nav-container {
		width: 100%;
		max-width: 1920px;
		margin: 0 auto;
		contain: layout style;
	}

	.nav-inner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		min-height: var(--nav-height-desktop);
		padding: 0 2rem;
		gap: 2rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Logo
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.nav-logo {
		flex-shrink: 0;
		display: flex;
		align-items: center;
		text-decoration: none;
		outline-offset: 4px;
		-webkit-tap-highlight-color: transparent;
	}

	.nav-logo:focus-visible {
		outline: 2px solid var(--nav-accent);
		border-radius: 8px;
	}

	.nav-logo__image {
		height: 88px;
		width: auto;
		max-width: 260px;
		object-fit: contain;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Desktop Navigation
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.nav-desktop {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		flex: 1;
		font-family: 'Montserrat', system-ui, -apple-system, sans-serif;
	}

	.nav-link {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem 1rem;
		border-radius: 9999px;
		border: 1px solid transparent;
		background: transparent;
		color: var(--nav-text);
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		cursor: pointer;
		transition: all var(--nav-transition-fast);
		white-space: nowrap;
		-webkit-tap-highlight-color: transparent;
	}

	.nav-link:hover,
	.nav-link:focus-visible {
		color: var(--nav-text-hover);
		background: rgba(9, 16, 35, 0.98);
		border-color: var(--nav-border);
		box-shadow: 0 0 16px rgba(15, 23, 42, 0.5);
	}

	.nav-link:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: 2px;
	}

	:global(.nav-link__icon) {
		transition: transform var(--nav-transition-fast);
		display: inline-block;
	}

	:global(.nav-link__icon--rotated) {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Desktop Dropdown
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.nav-dropdown {
		position: relative;
	}

	.nav-dropdown__menu {
		position: absolute;
		top: 100%;
		left: 50%;
		transform: translateX(-50%) translateY(-8px);
		min-width: 200px;
		padding: 0.5rem;
		margin-top: 0.5rem;
		background: var(--nav-dropdown-bg);
		border: 1px solid var(--nav-border);
		border-radius: 12px;
		box-shadow: var(--nav-shadow);
		opacity: 0;
		visibility: hidden;
		pointer-events: none;
		transition: all var(--nav-transition);
		z-index: var(--z-dropdown);
	}

	.nav-dropdown__menu--open {
		opacity: 1;
		visibility: visible;
		pointer-events: auto;
		transform: translateX(-50%) translateY(0);
	}

	.nav-dropdown__item {
		display: block;
		padding: 0.625rem 1rem;
		border-radius: 8px;
		color: var(--nav-text);
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		transition: all var(--nav-transition-fast);
	}

	.nav-dropdown__item:hover,
	.nav-dropdown__item:focus-visible {
		color: var(--nav-text-hover);
		background: rgba(15, 23, 42, 0.8);
	}

	.nav-dropdown__item:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: -2px;
	}

	.nav-dropdown__item + .nav-dropdown__item {
		margin-top: 0.25rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Right Actions
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.nav-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-shrink: 0;
	}

	.nav-cart {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.625rem 1rem;
		background: rgba(139, 92, 246, 0.1);
		border: 1px solid rgba(139, 92, 246, 0.3);
		border-radius: 8px;
		color: #a78bfa;
		text-decoration: none;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-cart:hover {
		background: rgba(139, 92, 246, 0.2);
		border-color: rgba(139, 92, 246, 0.5);
		transform: translateY(-2px);
	}

	.nav-cart:focus-visible {
		outline: 2px solid #a78bfa;
		outline-offset: 2px;
	}

	.nav-cart__badge {
		position: absolute;
		top: -6px;
		right: -6px;
		min-width: 18px;
		height: 18px;
		padding: 0 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #f97316, #facc15);
		color: #020817;
		font-size: 0.6875rem;
		font-weight: 700;
		border-radius: 9px;
		animation: badge-pulse 2s ease-in-out infinite;
	}

	@keyframes badge-pulse {
		0%, 100% { transform: scale(1); }
		50% { transform: scale(1.1); }
	}

	.nav-user {
		position: relative;
	}

	.nav-user__trigger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		background: var(--nav-accent-bg);
		border: 1px solid rgba(250, 204, 21, 0.3);
		border-radius: 8px;
		color: var(--nav-accent);
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-user__trigger:hover {
		background: rgba(250, 204, 21, 0.2);
		border-color: rgba(250, 204, 21, 0.5);
	}

	.nav-user__trigger:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: 2px;
	}

	.nav-user__name {
		max-width: 120px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	:global(.nav-user__icon--rotated) {
		transform: rotate(180deg);
		transition: transform var(--nav-transition);
	}

	.nav-user__menu {
		position: absolute;
		top: calc(100% + 0.5rem);
		right: 0;
		min-width: 200px;
		background: var(--nav-dropdown-bg);
		border: 1px solid rgba(250, 204, 21, 0.2);
		border-radius: 12px;
		box-shadow: var(--nav-shadow);
		opacity: 0;
		visibility: hidden;
		transform: translateY(-8px);
		transition: all var(--nav-transition);
		z-index: var(--z-dropdown);
		overflow: hidden;
	}

	.nav-user__menu--open {
		opacity: 1;
		visibility: visible;
		pointer-events: auto;
		transform: translateY(0);
	}

	.nav-user__item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem 1rem;
		background: transparent;
		border: none;
		color: var(--nav-text);
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;
		text-align: left;
		cursor: pointer;
		transition: all var(--nav-transition-fast);
	}

	.nav-user__item:hover,
	.nav-user__item:focus-visible {
		color: var(--nav-text-hover);
		background: var(--nav-accent-bg);
	}

	.nav-user__item:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: -2px;
	}

	.nav-user__item:not(:last-child) {
		border-bottom: 1px solid rgba(250, 204, 21, 0.1);
	}

	.nav-user__item--logout {
		color: #ef4444;
	}

	.nav-user__item--logout:hover {
		color: #f87171;
		background: rgba(239, 68, 68, 0.1);
	}

	.nav-login {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 131px;
		height: 51px;
		border-radius: 15px;
		background: linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0) 30%),
			rgba(46, 142, 255, 0.2);
		text-decoration: none;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-login:hover,
	.nav-login:focus-visible {
		background: linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0.3) 30%),
			rgba(46, 142, 255, 0.7);
		box-shadow: 0 0 16px rgba(46, 142, 255, 0.5);
	}

	.nav-login:focus-visible {
		outline: 2px solid #2e8eff;
		outline-offset: 2px;
	}

	.nav-login__inner {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		width: 127px;
		height: 47px;
		border-radius: 13px;
		background: #1a1a1a;
		color: #fff;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Hamburger Button
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.nav-hamburger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		padding: 0;
		background: transparent;
		border: 2px solid transparent;
		border-radius: 12px;
		color: #fff;
		cursor: pointer;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-hamburger:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.1);
		color: var(--nav-text-hover);
	}

	.nav-hamburger:focus-visible {
		outline: none;
		border-color: var(--nav-accent);
		background: rgba(250, 204, 21, 0.1);
	}

	.nav-hamburger:active:not(:disabled) {
		transform: scale(0.95);
	}

	.nav-hamburger:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.nav-hamburger--active {
		background: rgba(250, 204, 21, 0.1);
		color: var(--nav-text-hover);
	}

	.nav-hamburger__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		pointer-events: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Mobile Navigation
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.nav-mobile {
		position: fixed;
		top: var(--nav-height-tablet);
		left: 0;
		right: 0;
		bottom: 0;
		z-index: var(--z-mobile-backdrop);
		pointer-events: none;
		contain: layout style paint;
	}

	.nav-mobile--open {
		pointer-events: auto;
	}

	.nav-mobile__backdrop {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		opacity: 0;
		transition: opacity var(--nav-transition);
	}

	.nav-mobile__backdrop--visible {
		opacity: 1;
	}

	.nav-mobile__panel {
		position: absolute;
		top: 0;
		right: 0;
		width: 100%;
		max-width: 400px;
		height: 100%;
		background: var(--nav-bg-transparent);
		border-left: 1px solid var(--nav-border);
		box-shadow: -8px 0 32px rgba(0, 0, 0, 0.3);
		transform: translateX(100%);
		transition: transform var(--nav-transition);
		overflow: hidden;
		z-index: var(--z-mobile-panel);
		contain: layout style paint;
	}

	.nav-mobile__panel--open {
		transform: translateX(0);
	}

	.nav-mobile__content {
		display: flex;
		flex-direction: column;
		height: 100%;
		padding: 1.5rem;
		overflow-y: auto;
		overscroll-behavior: contain;
		-webkit-overflow-scrolling: touch;
	}

	.nav-mobile__group {
		border-bottom: 1px solid rgba(148, 163, 253, 0.1);
	}

	.nav-mobile__button {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 1rem 0;
		background: transparent;
		border: none;
		color: var(--nav-text);
		font-size: 1rem;
		font-weight: 600;
		text-align: left;
		cursor: pointer;
		transition: color var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-mobile__button:hover,
	.nav-mobile__button:focus-visible {
		color: var(--nav-text-hover);
	}

	.nav-mobile__button:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: -2px;
		border-radius: 4px;
	}

	:global(.nav-mobile__chevron) {
		transition: transform var(--nav-transition-fast);
		display: inline-block;
	}

	:global(.nav-mobile__chevron--rotated) {
		transform: rotate(90deg);
	}

	.nav-mobile__submenu {
		display: grid;
		grid-template-rows: 0fr;
		transition: grid-template-rows var(--nav-transition);
		overflow: hidden;
	}

	.nav-mobile__submenu--open {
		grid-template-rows: 1fr;
	}

	.nav-mobile__submenu > * {
		overflow: hidden;
	}

	.nav-mobile__sublink {
		display: block;
		padding: 0.75rem 1rem;
		margin-left: 0.5rem;
		border-left: 2px solid rgba(148, 163, 253, 0.3);
		color: #d1d5db;
		font-size: 0.9375rem;
		font-weight: 500;
		text-decoration: none;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-mobile__sublink:hover,
	.nav-mobile__sublink:focus-visible {
		color: var(--nav-text-hover);
		padding-left: 1.25rem;
		border-color: var(--nav-accent);
	}

	.nav-mobile__sublink:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: -2px;
		border-radius: 4px;
	}

	.nav-mobile__link {
		display: block;
		padding: 1rem 0;
		border-bottom: 1px solid rgba(148, 163, 253, 0.1);
		color: var(--nav-text);
		font-size: 1rem;
		font-weight: 600;
		text-decoration: none;
		transition: color var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-mobile__link:hover,
	.nav-mobile__link:focus-visible {
		color: var(--nav-text-hover);
	}

	.nav-mobile__link:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: -2px;
		border-radius: 4px;
	}

	.nav-mobile__user {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-top: auto;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(250, 204, 21, 0.1);
	}

	.nav-mobile__user-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--nav-accent-bg);
		border-radius: 8px;
		color: var(--nav-text);
		font-weight: 500;
	}

	.nav-mobile__user-link {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		background: var(--nav-accent-bg);
		border: 1px solid rgba(250, 204, 21, 0.15);
		border-radius: 8px;
		color: var(--nav-text);
		font-weight: 500;
		text-decoration: none;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-mobile__user-link:hover,
	.nav-mobile__user-link:focus-visible {
		background: rgba(250, 204, 21, 0.15);
		border-color: rgba(250, 204, 21, 0.3);
		transform: translateX(4px);
	}

	.nav-mobile__user-link:focus-visible {
		outline: 2px solid var(--nav-accent);
		outline-offset: 2px;
	}

	.nav-mobile__logout {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #ef4444;
		font-weight: 600;
		cursor: pointer;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
		width: 100%;
	}

	.nav-mobile__logout:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.nav-mobile__logout:active {
		transform: scale(0.98);
	}

	.nav-mobile__logout:focus-visible {
		outline: 2px solid #ef4444;
		outline-offset: 2px;
	}

	.nav-mobile__login {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.25rem;
		border-radius: 15px;
		background: linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0) 30%),
			rgba(46, 142, 255, 0.2);
		text-decoration: none;
		transition: all var(--nav-transition-fast);
		-webkit-tap-highlight-color: transparent;
	}

	.nav-mobile__login:hover {
		background: linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0.3) 30%),
			rgba(46, 142, 255, 0.7);
		box-shadow: 0 0 16px rgba(46, 142, 255, 0.5);
	}

	.nav-mobile__login:focus-visible {
		outline: 2px solid #2e8eff;
		outline-offset: 2px;
	}

	.nav-mobile__login-inner {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.875rem 1.5rem;
		border-radius: 13px;
		background: #1a1a1a;
		color: #fff;
		font-weight: 600;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1024px) {
		.nav-inner {
			min-height: var(--nav-height-tablet);
			padding: 0 1.5rem;
			gap: 1.5rem;
		}

		.nav-logo__image {
			height: 64px;
			max-width: 200px;
		}

		.nav-mobile {
			top: var(--nav-height-tablet);
		}
	}

	@media (max-width: 767px) {
		.nav-inner {
			min-height: var(--nav-height-mobile);
			padding: 0 1rem;
			gap: 1rem;
		}

		.nav-logo__image {
			height: 48px;
			max-width: 160px;
		}

		.nav-mobile {
			top: var(--nav-height-mobile);
		}

		.nav-mobile__panel {
			max-width: 100%;
		}

		.nav-mobile__content {
			padding: 1rem;
		}

		.nav-hamburger {
			width: 44px;
			height: 44px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Accessibility
	 * ═══════════════════════════════════════════════════════════════════════════ */
	@media (forced-colors: active) {
		.nav-link:focus-visible,
		.nav-dropdown__item:focus-visible,
		.nav-hamburger:focus-visible,
		.nav-mobile__button:focus-visible,
		.nav-mobile__link:focus-visible,
		.nav-mobile__sublink:focus-visible,
		.nav-user__trigger:focus-visible,
		.nav-user__item:focus-visible {
			outline: 3px solid CanvasText;
		}

		.nav-cart__badge {
			forced-color-adjust: none;
			background: CanvasText;
			color: Canvas;
		}
	}

	@media print {
		.nav-header {
			position: static;
			background: #fff;
			border-bottom: 1px solid #000;
		}

		.nav-hamburger,
		.nav-mobile,
		.nav-cart,
		.nav-user,
		.nav-dropdown__menu,
		.nav-login {
			display: none !important;
		}

		.nav-link {
			color: #000;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		*,
		*::before,
		*::after {
			animation-duration: 0.01ms !important;
			animation-iteration-count: 1 !important;
			transition-duration: 0.01ms !important;
		}
	}
</style>