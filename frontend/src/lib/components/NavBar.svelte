<script lang="ts">
    /**
     * NavBar Component - Google L7 Production Standard
     * Svelte 5 Runes | Zero-CLS | Accessible
     */
    import { onMount, onDestroy } from 'svelte';
    import { page } from '$app/stores';
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
    import { logout as logoutApi } from '$lib/api/auth';

    // ─────────────────────────────────────────────────────────────────────────────
    // Types & Config
    // ─────────────────────────────────────────────────────────────────────────────
    interface NavItem {
        id: string;
        label: string;
        href?: string;
        submenu?: Array<{ href: string; label: string; }>;
    }

    interface UserMenuItem {
        href: string;
        label: string;
        icon: any;
    }

    const navItems: NavItem[] = [
        {
            id: 'live',
            label: 'Live Trading',
            submenu: [
                { href: '/day-trading', label: 'Day Trading Room' },
                { href: '/swing-trading', label: 'Swing Trading Room' },
                { href: '/small-accounts', label: 'Small Accounts Room' }
            ]
        },
        {
            id: 'alerts',
            label: 'Alerts',
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
        { id: 'analytics', label: 'Analytics', href: '/analytics' },
        { id: 'media', label: 'Media', href: '/media' },
        { id: 'email', label: 'Email', href: '/email' },
        { id: 'about', label: 'About', href: '/about' },
        { id: 'blog', label: 'Blog', href: '/blog' }
    ];

    const userMenuItems: UserMenuItem[] = [
        { href: '/dashboard/courses', label: 'My Courses', icon: IconBook },
        { href: '/dashboard/indicators', label: 'My Indicators', icon: IconChartLine },
        { href: '/dashboard/account', label: 'My Account', icon: IconSettings }
    ];

    // ─────────────────────────────────────────────────────────────────────────────
    // State (Svelte 5 Runes)
    // ─────────────────────────────────────────────────────────────────────────────
    let isMobileMenuOpen = $state(false);
    let activeDropdown = $state<string | null>(null);
    let activeMobileSubmenu = $state<string | null>(null);
    let isUserMenuOpen = $state(false);
    let windowWidth = $state(0);
    let scrollY = $state(0);
    
    // DOM References
    let headerRef = $state<HTMLElement>();
    let mobileNavRef = $state<HTMLElement>();

    // ─────────────────────────────────────────────────────────────────────────────
    // Derived State
    // ─────────────────────────────────────────────────────────────────────────────
    // 1024px matches standard Tablet/Desktop boundary
    const isDesktop = $derived(windowWidth >= 1024);
    const isScrolled = $derived(scrollY > 20);

    // ─────────────────────────────────────────────────────────────────────────────
    // Effects & Logic
    // ─────────────────────────────────────────────────────────────────────────────
    
    // Auto-close menu on resize to desktop
    $effect(() => {
        if (isDesktop && isMobileMenuOpen) {
            closeMobileMenu();
        }
    });

    // Handle Scroll Locking properly (CSS Class based)
    $effect(() => {
        if (!browser) return;
        if (isMobileMenuOpen) {
            document.body.classList.add('nav-scroll-lock');
        } else {
            document.body.classList.remove('nav-scroll-lock');
        }
        return () => {
            document.body.classList.remove('nav-scroll-lock');
        };
    });

    // Close user menu when clicking outside
    function handleDocumentClick(event: MouseEvent) {
        if (!browser) return;
        const target = event.target as HTMLElement;
        
        // Close dropdowns if clicking outside
        if (!target.closest('.nav-dropdown')) {
            activeDropdown = null;
        }
        // Close user menu if clicking outside
        if (!target.closest('.nav-user')) {
            isUserMenuOpen = false;
        }
    }

    function toggleMobileMenu() {
        isMobileMenuOpen = !isMobileMenuOpen;
        activeMobileSubmenu = null; // Reset submenus when toggling
    }

    function closeMobileMenu() {
        isMobileMenuOpen = false;
        activeMobileSubmenu = null;
    }

    async function handleLogout() {
        closeMobileMenu();
        await logoutApi();
        authStore.clearAuth();
        goto('/login');
    }

    // Lifecycle
    onMount(() => {
        windowWidth = window.innerWidth;
        const handleResize = () => windowWidth = window.innerWidth;
        const handleScroll = () => scrollY = window.scrollY;
        
        window.addEventListener('resize', handleResize, { passive: true });
        window.addEventListener('scroll', handleScroll, { passive: true });
        document.addEventListener('click', handleDocumentClick);

        return () => {
            window.removeEventListener('resize', handleResize);
            window.removeEventListener('scroll', handleScroll);
            document.removeEventListener('click', handleDocumentClick);
        };
    });
</script>

<svelte:head>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
    <link href="https://fonts.googleapis.com/css2?family=Montserrat:wght@400;500;600;700&display=swap" rel="stylesheet" />
</svelte:head>

<header
    class="nav-header"
    class:nav-header--scrolled={isScrolled}
    bind:this={headerRef}
    aria-label="Main navigation"
>
    <div class="nav-container">
        <a href="/" class="nav-logo" onclick={closeMobileMenu}>
            <img
                src="/revolution-trading-pros.png"
                alt="Revolution Trading Pros"
                width="260"
                height="88"
                class="nav-logo__img"
            />
        </a>

        {#if isDesktop}
            <nav class="nav-desktop">
                {#each navItems as item}
                    {#if item.submenu}
                        <div class="nav-dropdown">
                            <button
                                class="nav-link nav-link--btn"
                                aria-expanded={activeDropdown === item.id}
                                aria-haspopup="true"
                                onclick={(e) => {
                                    e.stopPropagation();
                                    activeDropdown = activeDropdown === item.id ? null : item.id;
                                }}
                            >
                                {item.label}
                                <IconChevronDown size={16} class="nav-icon-chevron {activeDropdown === item.id ? 'rotate-180' : ''}" />
                            </button>

                            <div class="nav-dropdown__menu" class:open={activeDropdown === item.id}>
                                {#each item.submenu as subItem}
                                    <a
                                        href={subItem.href}
                                        class="nav-dropdown__item"
                                        class:active={$page.url.pathname === subItem.href}
                                        onclick={() => activeDropdown = null}
                                    >
                                        {subItem.label}
                                    </a>
                                {/each}
                            </div>
                        </div>
                    {:else}
                        <a
                            href={item.href}
                            class="nav-link"
                            class:active={$page.url.pathname === item.href}
                        >
                            {item.label}
                        </a>
                    {/if}
                {/each}
            </nav>
        {/if}

        <div class="nav-actions">
            {#if !isMobileMenuOpen}
                {#if $hasCartItems}
                    <a href="/cart" class="nav-action-btn" aria-label="Cart">
                        <IconShoppingCart size={22} />
                        {#if $cartItemCount > 0}
                            <span class="nav-badge">{$cartItemCount}</span>
                        {/if}
                    </a>
                {/if}

                {#if isDesktop}
                    {#if $isAuthenticated}
                        <div class="nav-user">
                            <button
                                class="nav-user__trigger"
                                onclick={(e) => {
                                    e.stopPropagation();
                                    isUserMenuOpen = !isUserMenuOpen;
                                }}
                            >
                                <IconUser size={20} />
                                <span class="truncate max-w-[100px]">{$user?.name || 'Account'}</span>
                                <IconChevronDown size={14} />
                            </button>

                            <div class="nav-user__dropdown" class:open={isUserMenuOpen}>
                                {#each userMenuItems as menuItem}
                                    <a href={menuItem.href} class="nav-user__item" onclick={() => isUserMenuOpen = false}>
                                        <menuItem.icon size={16} />
                                        {menuItem.label}
                                    </a>
                                {/each}
                                <button class="nav-user__item text-red-400" onclick={handleLogout}>
                                    <IconLogout size={16} />
                                    Logout
                                </button>
                            </div>
                        </div>
                    {:else}
                        <a href="/login" class="nav-btn-primary">
                            <IconUser size={18} />
                            <span>Login</span>
                        </a>
                    {/if}
                {/if}
            {/if}

            {#if !isDesktop}
                <button
                    class="nav-hamburger"
                    aria-label={isMobileMenuOpen ? "Close Menu" : "Open Menu"}
                    aria-expanded={isMobileMenuOpen}
                    onclick={toggleMobileMenu}
                >
                    {#if isMobileMenuOpen}
                        <IconX size={28} />
                    {:else}
                        <IconMenu2 size={28} />
                    {/if}
                </button>
            {/if}
        </div>
    </div>
</header>

{#if !isDesktop && isMobileMenuOpen}
    <div 
        class="nav-mobile-backdrop" 
        onclick={closeMobileMenu}
        role="presentation"
        aria-hidden="true"
    ></div>

    <nav 
        class="nav-mobile-panel" 
        bind:this={mobileNavRef}
        aria-label="Mobile Navigation"
    >
        <div class="nav-mobile-content">
            {#each navItems as item}
                {#if item.submenu}
                    <div class="nav-mobile-group">
                        <button
                            class="nav-mobile-link justify-between"
                            onclick={() => activeMobileSubmenu = activeMobileSubmenu === item.id ? null : item.id}
                        >
                            {item.label}
                            <IconChevronRight 
                                size={20} 
                                class="transition-transform duration-200 {activeMobileSubmenu === item.id ? 'rotate-90' : ''}" 
                            />
                        </button>
                        
                        {#if activeMobileSubmenu === item.id}
                            <div class="nav-mobile-submenu">
                                {#each item.submenu as subItem}
                                    <a 
                                        href={subItem.href} 
                                        class="nav-mobile-sublink"
                                        class:active={$page.url.pathname === subItem.href}
                                        onclick={closeMobileMenu}
                                    >
                                        {subItem.label}
                                    </a>
                                {/each}
                            </div>
                        {/if}
                    </div>
                {:else}
                    <a 
                        href={item.href} 
                        class="nav-mobile-link"
                        class:active={$page.url.pathname === item.href}
                        onclick={closeMobileMenu}
                    >
                        {item.label}
                    </a>
                {/if}
            {/each}

            <div class="nav-mobile-footer">
                {#if $isAuthenticated}
                    <div class="p-4 bg-white/5 rounded-lg mb-4">
                        <div class="font-medium text-yellow-400 mb-2">{$user?.name}</div>
                        {#each userMenuItems as item}
                            <a href={item.href} class="nav-mobile-sublink pl-0" onclick={closeMobileMenu}>
                                {item.label}
                            </a>
                        {/each}
                        <button class="nav-mobile-sublink pl-0 text-red-400 w-full text-left" onclick={handleLogout}>
                            Logout
                        </button>
                    </div>
                {:else}
                    <a href="/login" class="nav-btn-primary w-full justify-center py-3" onclick={closeMobileMenu}>
                        Login
                    </a>
                {/if}
            </div>
        </div>
    </nav>
{/if}

<style>
    /* ═══════════════════════════════════════════════════════════════════════════
     * CORE VARIABLES & LAYOUT (Google L7 Standards)
     * ═══════════════════════════════════════════════════════════════════════════ */
    :root {
        /* Semantic Heights */
        --nav-height: 120px;
        --nav-bg: #05142b;
        --nav-text: #e5e7eb;
        --nav-accent: #facc15;
        --nav-border: rgba(148, 163, 253, 0.16);
        --z-nav: 1000;
        --z-mobile-menu: 1001;
    }

    @media (max-width: 1024px) {
        :root { --nav-height: 90px; }
    }
    @media (max-width: 768px) {
        :root { --nav-height: 72px; }
    }

    /* GLOBAL FIX: Ensure Anchor links scroll to correct position */
    :global(html) {
        scroll-padding-top: var(--nav-height);
    }
    
    /* GLOBAL FIX: Lock scroll safely */
    :global(body.nav-scroll-lock) {
        overflow: hidden;
        height: 100vh;
        touch-action: none;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
     * HEADER STRUCTURE
     * ═══════════════════════════════════════════════════════════════════════════ */
    .nav-header {
        position: sticky;
        top: 0;
        z-index: var(--z-nav);
        width: 100%;
        background: rgba(5, 20, 43, 0.8);
        backdrop-filter: blur(12px);
        -webkit-backdrop-filter: blur(12px);
        border-bottom: 1px solid var(--nav-border);
        height: var(--nav-height);
        transition: background 0.3s ease, height 0.3s ease;
    }

    /* Optimization: Less transparency when scrolled */
    .nav-header--scrolled {
        background: rgba(5, 20, 43, 0.98);
        box-shadow: 0 4px 20px rgba(0,0,0,0.2);
    }

    .nav-container {
        max-width: 1920px;
        margin: 0 auto;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 1.5rem;
    }

    /* ═══════════════════════════════════════════════════════════════════════════
     * LOGO & LINKS
     * ═══════════════════════════════════════════════════════════════════════════ */
    .nav-logo {
        display: flex;
        align-items: center;
        height: 100%;
        z-index: 20; /* Keep above mobile menu if needed */
    }

    .nav-logo__img {
        height: 70%; 
        width: auto;
        object-fit: contain;
    }

    .nav-desktop {
        display: flex;
        gap: 0.5rem;
        height: 100%;
        align-items: center;
    }

    .nav-link {
        color: var(--nav-text);
        font-family: 'Montserrat', sans-serif;
        font-weight: 600;
        font-size: 0.9rem;
        padding: 0.5rem 1rem;
        border-radius: 99px;
        text-decoration: none;
        transition: all 0.2s;
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
        border: 1px solid transparent;
        cursor: pointer;
        background: transparent;
    }

    .nav-link--btn {
        appearance: none;
    }

    .nav-link:hover, .nav-link:focus-visible, .nav-link.active {
        color: var(--nav-accent);
        background: rgba(255, 255, 255, 0.05);
        border-color: rgba(250, 204, 21, 0.2);
    }

    :global(.nav-icon-chevron) {
        transition: transform 0.2s;
    }
    :global(.rotate-180) { transform: rotate(180deg); }

    /* ═══════════════════════════════════════════════════════════════════════════
     * DROPDOWNS (Desktop)
     * ═══════════════════════════════════════════════════════════════════════════ */
    .nav-dropdown {
        position: relative;
        height: 100%;
        display: flex;
        align-items: center;
    }

    .nav-dropdown__menu {
        position: absolute;
        top: calc(100% - 10px);
        left: 50%;
        transform: translateX(-50%) translateY(10px);
        background: #05142b;
        border: 1px solid var(--nav-border);
        border-radius: 12px;
        padding: 0.5rem;
        min-width: 220px;
        opacity: 0;
        visibility: hidden;
        transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
        box-shadow: 0 10px 30px rgba(0,0,0,0.4);
    }

    .nav-dropdown__menu.open {
        opacity: 1;
        visibility: visible;
        transform: translateX(-50%) translateY(0);
    }

    .nav-dropdown__item {
        display: block;
        padding: 0.75rem 1rem;
        color: #cbd5e1;
        text-decoration: none;
        border-radius: 8px;
        font-size: 0.9rem;
        transition: 0.15s;
    }

    .nav-dropdown__item:hover {
        background: rgba(250, 204, 21, 0.1);
        color: var(--nav-accent);
    }

    /* ═══════════════════════════════════════════════════════════════════════════
     * ACTIONS & USER MENU
     * ═══════════════════════════════════════════════════════════════════════════ */
    .nav-actions {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .nav-action-btn {
        position: relative;
        color: #a78bfa;
        padding: 0.5rem;
        border-radius: 8px;
        background: rgba(139, 92, 246, 0.1);
        transition: 0.2s;
    }
    .nav-action-btn:hover { background: rgba(139, 92, 246, 0.2); }

    .nav-badge {
        position: absolute;
        top: -5px;
        right: -5px;
        background: var(--nav-accent);
        color: #000;
        font-size: 0.7rem;
        font-weight: bold;
        width: 18px;
        height: 18px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .nav-btn-primary {
        background: linear-gradient(135deg, #2563eb, #1d4ed8);
        color: white;
        padding: 0.6rem 1.25rem;
        border-radius: 12px;
        font-weight: 600;
        text-decoration: none;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        transition: 0.2s;
    }
    .nav-btn-primary:hover { box-shadow: 0 0 15px rgba(37, 99, 235, 0.5); }

    .nav-user { position: relative; }
    
    .nav-user__trigger {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: rgba(250, 204, 21, 0.1);
        border: 1px solid rgba(250, 204, 21, 0.3);
        color: var(--nav-accent);
        padding: 0.5rem 1rem;
        border-radius: 8px;
        cursor: pointer;
    }

    .nav-user__dropdown {
        position: absolute;
        top: 120%;
        right: 0;
        width: 200px;
        background: #05142b;
        border: 1px solid var(--nav-border);
        border-radius: 12px;
        padding: 0.5rem;
        opacity: 0;
        visibility: hidden;
        transition: 0.2s;
    }
    .nav-user__dropdown.open { opacity: 1; visibility: visible; top: 110%; }

    .nav-user__item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        width: 100%;
        padding: 0.75rem;
        color: #e2e8f0;
        text-decoration: none;
        border-radius: 8px;
        font-size: 0.9rem;
        transition: 0.15s;
        background: transparent;
        border: none;
        cursor: pointer;
        text-align: left;
    }
    .nav-user__item:hover { background: rgba(255,255,255,0.05); }

    /* ═══════════════════════════════════════════════════════════════════════════
     * MOBILE MENU
     * ═══════════════════════════════════════════════════════════════════════════ */
    .nav-hamburger {
        background: transparent;
        border: none;
        color: white;
        padding: 0.5rem;
        cursor: pointer;
    }

    .nav-mobile-backdrop {
        position: fixed;
        inset: 0;
        background: rgba(0,0,0,0.6);
        z-index: 1000;
        backdrop-filter: blur(4px);
    }

    .nav-mobile-panel {
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        width: 85%;
        max-width: 400px;
        background: #05142b;
        z-index: 1001;
        box-shadow: -10px 0 30px rgba(0,0,0,0.5);
        display: flex;
        flex-direction: column;
        animation: slideIn 0.3s cubic-bezier(0.16, 1, 0.3, 1);
        border-left: 1px solid var(--nav-border);
    }

    @keyframes slideIn {
        from { transform: translateX(100%); }
        to { transform: translateX(0); }
    }

    .nav-mobile-content {
        padding: 6rem 1.5rem 2rem 1.5rem; /* Top padding clears close button area */
        overflow-y: auto;
        height: 100%;
    }

    .nav-mobile-link {
        display: flex;
        width: 100%;
        padding: 1rem 0;
        border-bottom: 1px solid rgba(255,255,255,0.1);
        color: #e5e7eb;
        font-size: 1.1rem;
        font-weight: 600;
        text-decoration: none;
        background: transparent;
        border-left: none; border-right: none; border-top: none;
        cursor: pointer;
        text-align: left;
    }
    .nav-mobile-link.active { color: var(--nav-accent); }

    .nav-mobile-submenu {
        background: rgba(0,0,0,0.2);
        margin: 0 -1.5rem;
        padding: 0.5rem 2rem;
        border-bottom: 1px solid rgba(255,255,255,0.1);
    }

    .nav-mobile-sublink {
        display: block;
        padding: 0.8rem 0;
        color: #94a3b8;
        text-decoration: none;
        font-size: 1rem;
    }
    .nav-mobile-sublink.active, .nav-mobile-sublink:active { color: var(--nav-accent); }

    .nav-mobile-footer {
        margin-top: 2rem;
        border-top: 1px solid var(--nav-border);
        padding-top: 2rem;
    }
</style>