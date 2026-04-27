<!--
	ORPHAN COMPONENT — DO NOT USE.

	TODO(2026-04-26-audit): This file is the legacy `admin/Sidebar.svelte`,
	flagged in audits/admin-2026-04-26/01-shell-and-dashboard.md §P1-2.

	  - The canonical sidebar is `$lib/components/layout/AdminSidebar.svelte`
	    (6 sections, 26 items, rune-based auth, calls the API logout endpoint).
	  - This file ships a 3-item nav (Dashboard / Forms / SEO), uses the
	    legacy `$authStore` autosubscribe (P1-1 hazard), and bypasses the
	    server-side logout endpoint (P3-7).
	  - A repo-wide grep at audit time found ZERO importers, so commenting
	    the body is safe per the CREATE-not-DELETE rule.

	If you find yourself wanting to use this file, import
	`AdminSidebar` from `$lib/components/layout` instead.

	Original implementation kept below in a Svelte HTML comment so the file
	still parses as a no-op component. To restore, lift the comment block.
-->

<script lang="ts">
	// Intentionally empty — this component renders nothing.
	// See header note above.
</script>

<!--
	BEGIN ARCHIVED IMPLEMENTATION (2026-04-26)

	<script lang="ts">
		import { page } from '$app/state';
		import {
			IconDashboard,
			IconForms,
			IconUsers,
			IconSeo,
			IconLink,
			IconError404,
			IconSettings,
			IconLogout
		} from '$lib/icons';
		import { authStore } from '$lib/stores/auth.svelte';
		import { goto } from '$app/navigation';

		interface Props {
			isOpen: boolean;
			onclose: () => void;
		}
		let props: Props = $props();
		const isOpen = $derived(props.isOpen ?? false);
		const onclose = $derived(props.onclose);

		interface NavItem {
			label: string;
			href: string;
			icon: any;
			children?: NavItem[];
		}

		const navigation: NavItem[] = [
			{ label: 'Dashboard', href: '/admin', icon: IconDashboard },
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
				return page.url.pathname === '/admin';
			}
			return page.url.pathname.startsWith(href);
		}

		function handleNavClick() {
			if (window.innerWidth < 1024) {
				onclose();
			}
		}
	</script>

	... template + styles archived; see git history before commit 2026-04-26.

	END ARCHIVED IMPLEMENTATION
-->
