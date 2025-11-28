# SvelteKit 5 Consent & Tracking System

A production-grade, GDPR/CCPA-compliant consent management system for SvelteKit 5 applications with built-in support for Google Analytics 4 (GA4) and Meta Pixel.

## Quick Start

### 1. Set Environment Variables

Create or update your `.env` file:

```bash
# Google Analytics 4
PUBLIC_GA4_MEASUREMENT_ID=G-XXXXXXXXXX

# Meta Pixel
PUBLIC_META_PIXEL_ID=1234567890
```

Both variables are optional. If not set, the respective vendor will be silently skipped with a debug message.

### 2. That's It!

The consent system is already integrated into the root layout. When a user visits the site:

1. A consent banner appears at the bottom of the screen
2. Users can "Accept All", "Reject All", or "Manage Preferences"
3. Consent preferences are stored in cookies (with localStorage backup)
4. GA4 and Meta Pixel only load when appropriate consent is granted
5. Google Consent Mode v2 is automatically configured

## Features

- **Privacy-First Defaults**: All non-essential tracking is disabled until consent is granted
- **Google Consent Mode v2**: Full compliance with Google's consent framework
- **Cookie + localStorage Storage**: Reliable persistence with fallback
- **Server-Side Rendering Support**: Consent state is available during SSR
- **Accessible UI**: WCAG 2.1 AA compliant banner and modal
- **SPA Navigation Tracking**: Automatic page view tracking on route changes
- **Vendor Registry Pattern**: Easy to add new tracking vendors

## Architecture

```
src/lib/consent/
├── index.ts              # Main exports and initialization
├── types.ts              # TypeScript type definitions
├── store.ts              # Svelte store for consent state
├── storage.ts            # Cookie/localStorage persistence
├── google-consent-mode.ts # Google Consent Mode v2 integration
├── vendor-loader.ts      # Consent-aware script loading
├── components/
│   ├── ConsentBanner.svelte
│   └── ConsentPreferencesModal.svelte
└── vendors/
    ├── index.ts          # Vendor registry
    ├── ga4.ts            # Google Analytics 4
    └── meta-pixel.ts     # Meta (Facebook) Pixel
```

## Consent Categories

| Category | Description | Required |
|----------|-------------|----------|
| `necessary` | Essential for site functionality | Yes (always on) |
| `analytics` | Performance measurement (GA4) | No |
| `marketing` | Advertising & retargeting (Meta Pixel) | No |
| `preferences` | Personalization features | No |

## API Reference

### Initialization

```typescript
import { initializeConsent } from '$lib/consent';
import { onMount } from 'svelte';

onMount(() => {
  const unsubscribe = initializeConsent();
  return () => unsubscribe();
});
```

### Consent Store

```typescript
import { consentStore, hasAnalyticsConsent, hasMarketingConsent } from '$lib/consent';

// Read current state
const state = consentStore.getState();

// Check specific consent
const hasAnalytics = consentStore.hasConsent('analytics');

// Subscribe to changes
consentStore.subscribe((consent) => {
  console.log('Consent updated:', consent);
});

// Update consent
consentStore.acceptAll();
consentStore.rejectAll();
consentStore.setCategory('analytics', true);
consentStore.updateCategories({ analytics: true, marketing: false });
```

### Tracking Events

```typescript
// GA4 Events
import { trackGA4Event, trackGA4PageView, setGA4UserId } from '$lib/consent';

trackGA4PageView('/custom-path');
trackGA4Event('purchase', { value: 99.99, currency: 'USD' });
setGA4UserId('user-123');

// Meta Pixel Events
import { trackPixelEvent, trackCustomPixelEvent } from '$lib/consent';

trackPixelEvent('Purchase', { value: 99.99, currency: 'USD' });
trackCustomPixelEvent('TrialStarted', { plan: 'pro' });
```

### UI Components

```svelte
<script>
  import { ConsentBanner, ConsentPreferencesModal, openPreferencesModal } from '$lib/consent';
</script>

<!-- Automatic banner (shows when no consent choice made) -->
<ConsentBanner position="bottom" />

<!-- Preferences modal (shows when triggered) -->
<ConsentPreferencesModal />

<!-- Manual trigger for preferences -->
<button on:click={openPreferencesModal}>Cookie Settings</button>
```

## Adding a New Vendor

To add a new tracking vendor (e.g., TikTok Pixel):

### 1. Create the Vendor File

```typescript
// src/lib/consent/vendors/tiktok.ts
import { browser } from '$app/environment';
import { PUBLIC_TIKTOK_PIXEL_ID } from '$env/static/public';
import type { VendorConfig } from '../types';
import { injectScript } from '../vendor-loader';

export const tiktokVendor: VendorConfig = {
  id: 'tiktok',
  name: 'TikTok Pixel',
  description: 'Advertising and analytics for TikTok campaigns.',
  requiredCategories: ['marketing'],
  privacyPolicyUrl: 'https://www.tiktok.com/legal/privacy-policy',

  async load(): Promise<void> {
    if (!PUBLIC_TIKTOK_PIXEL_ID) {
      console.debug('[TikTok] PUBLIC_TIKTOK_PIXEL_ID not set');
      return;
    }

    if (!browser) return;

    // Initialize TikTok Pixel
    // ... implementation
  },

  onConsentRevoked(): void {
    // Handle consent revocation
  },
};

export default tiktokVendor;
```

### 2. Register the Vendor

```typescript
// src/lib/consent/vendors/index.ts
import { tiktokVendor } from './tiktok';

export const vendors: VendorConfig[] = [
  ga4Vendor,
  metaPixelVendor,
  tiktokVendor, // Add here
];
```

### 3. Add Environment Variable

```bash
PUBLIC_TIKTOK_PIXEL_ID=your-pixel-id
```

## Verification

### Google Tag Assistant

1. Install [Google Tag Assistant](https://tagassistant.google.com/)
2. Navigate to your site
3. Verify:
   - Consent Mode is detected
   - Default consent state is "denied" before user interaction
   - Consent updates correctly after user choice
   - GA4 tag fires only after analytics consent

### Meta Pixel Helper

1. Install [Meta Pixel Helper](https://www.facebook.com/business/help/198406866570498)
2. Navigate to your site
3. Verify:
   - Pixel doesn't fire before consent
   - PageView fires after marketing consent
   - Events include proper consent signals

## Server-Side Rendering

The consent system works with SSR:

1. `hooks.server.ts` reads consent from cookies
2. `+layout.server.ts` passes consent to the client
3. Google Consent Mode defaults are injected into the HTML
4. Client hydrates and takes over consent management

Access consent in server load functions:

```typescript
// +page.server.ts
export const load: PageServerLoad = async ({ locals }) => {
  const hasAnalytics = locals.consent.analytics;
  // ...
};
```

## Troubleshooting

### Scripts Not Loading

1. Check browser console for debug messages (`[Consent]`, `[GA4]`, `[MetaPixel]`)
2. Verify environment variables are set
3. Confirm consent has been granted (check `consentStore.getState()`)
4. Check for CSP violations in console

### Consent Not Persisting

1. Check if cookies are enabled in browser
2. Verify domain configuration for cross-subdomain consent
3. Check localStorage availability

### Google Consent Mode Not Working

1. Use Tag Assistant to verify consent signals
2. Ensure `applyConsentMode` is called before gtag.js loads
3. Check that consent is set before any events fire

## Security Considerations

- Environment variables with `PUBLIC_` prefix are exposed to clients
- Never include API secrets in public environment variables
- The consent cookie is `HttpOnly: false` (required for client-side access)
- Consider adding CSP headers for tracking domains

## License

Internal use only. Contact the engineering team for questions.
