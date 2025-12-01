<script lang="ts">
	import { page } from '$app/stores';
	import {
		IconDashboard,
		IconForms,
		IconUsers,
		IconSeo,
		IconLink,
		IconError404,
		IconSettings,
		IconLogout
	} from '@tabler/icons-svelte';
	import { authStore } from '$lib/stores/auth';
	import { goto } from '$app/navigation';

	interface NavItem {
		label: string;
		href: string;
		icon: any;
		children?: NavItem[];
	}

	const navigation: NavItem[] = [
		{
			label: 'Dashboard',
			href: '/admin',
			icon: IconDashboard
		},
		{
			label: 'Forms',
			href: '/admin/forms',
			icon: IconForms,
			children: [
				{ label: 'All Forms', href: '/admin/forms', icon: IconForms },
				{ label: 'Entries', href: '/admin/forms/entries', icon: IconForms },
				{ label: 'Contacts', href: '/admin/contacts', icon: IconUsers }
			]
		},
		{
			label: 'SEO',
			href: '/admin/seo',
			icon: IconSeo,
			children: [
				{ label: 'Analysis', href: '/admin/seo/analysis', icon: IconSeo },
				{ label: 'Redirects', href: '/admin/seo/redirects', icon: IconLink },
				{ label: '404 Errors', href: '/admin/seo/404s', icon: IconError404 },
				{ label: 'Settings', href: '/admin/seo/settings', icon: IconSettings }
			]
		}
	];

	function handleLogout() {
		authStore.logout();
		goto('/login');
	}

	function isActive(href: string): boolean {
		if (href === '/admin') {
			return $page.url.pathname === '/admin';
		}
		return $page.url.pathname.startsWith(href);
	}
</script>

<aside class="w-64 bg-gray-900 text-white h-screen flex flex-col">
	<!-- Logo -->
	<div class="p-6 border-b border-gray-800">
		<h1 class="text-xl font-bold">Revolution Admin</h1>
		<p class="text-xs text-gray-400 mt-1">Fluent Revo & SEO Pro</p>
	</div>

	<!-- Navigation -->
	<nav class="flex-1 overflow-y-auto p-4 space-y-1">
		{#each navigation as item}
			<div>
				<a
					href={item.href}
					class="flex items-center gap-3 px-4 py-2.5 rounded-lg transition-colors
            {isActive(item.href) ? 'bg-blue-600 text-white' : 'text-gray-300 hover:bg-gray-800'}"
				>
					<svelte:component this={item.icon} size={20} />
					<span class="font-medium">{item.label}</span>
				</a>

				{#if item.children && isActive(item.href)}
					<div class="ml-8 mt-1 space-y-1">
						{#each item.children as child}
							<a
								href={child.href}
								class="flex items-center gap-2 px-3 py-2 rounded-md text-sm transition-colors
                  {$page.url.pathname === child.href
									? 'text-blue-400'
									: 'text-gray-400 hover:text-white'}"
							>
								{child.label}
							</a>
						{/each}
					</div>
				{/if}
			</div>
		{/each}
	</nav>

	<!-- User Section -->
	<div class="p-4 border-t border-gray-800">
		<div class="flex items-center gap-3 mb-3">
			<div class="w-10 h-10 rounded-full bg-blue-600 flex items-center justify-center">
				<span class="text-sm font-semibold">
					{$authStore.user?.name?.[0]?.toUpperCase() || 'A'}
				</span>
			</div>
			<div class="flex-1 min-w-0">
				<p class="text-sm font-medium truncate">{$authStore.user?.name || 'Admin'}</p>
				<p class="text-xs text-gray-400 truncate">{$authStore.user?.email || ''}</p>
			</div>
		</div>
		<button
			onclick={handleLogout}
			class="w-full flex items-center gap-2 px-4 py-2 text-sm text-gray-400 hover:text-white hover:bg-gray-800 rounded-lg transition-colors"
		>
			<IconLogout size={18} />
			<span>Logout</span>
		</button>
	</div>
</aside>
