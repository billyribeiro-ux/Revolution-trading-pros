<script lang="ts">
	import ConsentBanner from '#lib/consent/components/ConsentBanner.svelte';
	import ConsentPreferencesModal from '#lib/consent/components/ConsentPreferencesModal.svelte';
	import ConsentSettingsButton from '#lib/consent/components/ConsentSettingsButton.svelte';
	import { consentStore } from '#lib/consent/store.svelte.js';
	import { clearABTestData } from '#lib/consent/ab-testing.js';
	import PublicPopupRuntime from '#lib/components/popups/PublicPopupRuntime.svelte';
	import { onMount } from 'svelte';
	let { pathname }: { pathname: string } = $props();
	let popupSuppressed = $derived(
		['/account', '/auth', '/checkout', '/login', '/register', '/my'].some(
			(prefix) => pathname === prefix || pathname.startsWith(`${prefix}/`)
		)
	);

	onMount(() => {
		return consentStore.subscribe((state) => {
			if (!state.analytics) clearABTestData();
			if (state.marketing) return;
			for (const storage of [localStorage, sessionStorage]) {
				for (let index = storage.length - 1; index >= 0; index -= 1) {
					const key = storage.key(index);
					if (key?.startsWith('popup-frequency-')) storage.removeItem(key);
				}
			}
		});
	});
</script>

<ConsentBanner />
<ConsentPreferencesModal />
<ConsentSettingsButton />
{#key pathname}
	{#if $consentStore.marketing && !popupSuppressed}
		<PublicPopupRuntime {pathname} />
	{/if}
{/key}
