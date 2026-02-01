# RULES-SSR-HYDRATION-SEO.md
## SSR, Hydration, CLS, Core Web Vitals & SEO

## 🌐 RENDERING MODES
```typescript
// +page.ts
export const prerender = true;  // Static (marketing, docs)
export const ssr = true;        // Server render (default)
export const ssr = false;       // Client only (heavy dashboards)
```

## 🚨 HYDRATION ERRORS
Server HTML must match client HTML exactly.

### ❌ CAUSES HYDRATION ERROR
```svelte
let now = new Date();              // Different server/client
let id = Math.random();            // Different each render
let width = window.innerWidth;     // window undefined on server
{#if browser}<Thing />{/if}        // Mismatch
```

### ✅ CORRECT
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  
  let mounted = $state(false);
  let width = $state(0);
  
  onMount(() => {
    mounted = true;
    width = window.innerWidth;
  });
</script>

{#if mounted}
  <ClientOnlyComponent />
{:else}
  <div class="h-[400px] animate-pulse bg-surface-elevated"></div>
{/if}
```

## 📊 CORE WEB VITALS
| Metric | Target | Measures |
|--------|--------|----------|
| LCP | < 2.5s | Main content load |
| INP | < 200ms | Interaction speed |
| CLS | < 0.1 | Layout stability |

## 📐 CLS (Cumulative Layout Shift)
### Images - MUST have dimensions
```svelte
<!-- ❌ BAD -->
<img src={url} alt={title} />

<!-- ✅ GOOD -->
<img src={url} alt={title} width={800} height={450} class="w-full h-auto" />

<!-- ✅ GOOD - Aspect ratio -->
<div class="aspect-video">
  <img src={url} alt={title} class="w-full h-full object-cover" />
</div>
```

### Fonts
```css
@font-face {
  font-family: 'Inter';
  src: url('/fonts/Inter.woff2') format('woff2');
  font-display: swap;
}
```

### Dynamic Content - Reserve space
```svelte
<div class="min-h-[200px]">
  {#if isLoading}
    <div class="animate-pulse space-y-4">
      <div class="h-8 bg-surface-elevated rounded w-3/4"></div>
      <div class="h-4 bg-surface-elevated rounded"></div>
    </div>
  {:else}
    <h1>{data.title}</h1>
  {/if}
</div>
```

### Embeds
```svelte
<div class="aspect-video min-h-[315px]">
  <iframe src={url} class="w-full h-full"></iframe>
</div>
```

### Fixed Header
```svelte
<header class="fixed top-0 h-16"></header>
<main class="pt-16"><!-- Compensate --></main>
```

## ⚡ LCP - Preload Critical
```svelte
<svelte:head>
  <link rel="preload" href="/hero.webp" as="image" fetchpriority="high" />
  <link rel="preload" href="/fonts/Inter.woff2" as="font" crossorigin />
</svelte:head>

<img src="/hero.webp" fetchpriority="high" width={1600} height={900} />
```

## 👆 INP - Debounce
```svelte
<script lang="ts">
  let query = $state('');
  let debounced = $state('');
  let timer: ReturnType<typeof setTimeout>;
  
  $effect(() => {
    clearTimeout(timer);
    timer = setTimeout(() => debounced = query, 300);
    return () => clearTimeout(timer);
  });
</script>
```

## 🔍 SEO
```svelte
<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  <link rel="canonical" href={url} />
  
  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />
  <meta property="og:image" content={image} />
  
  <meta name="twitter:card" content="summary_large_image" />
  
  {@html `<script type="application/ld+json">${JSON.stringify(schema)}</script>`}
</svelte:head>
```

## 📋 CHECKLIST
```
CLS:
□ Images have width/height
□ Fonts use font-display: swap
□ Dynamic content has min-height
□ Embeds have reserved space

LCP:
□ Hero preloaded with fetchpriority="high"
□ Fonts preloaded

Hydration:
□ No Date/Math.random without onMount
□ No window/document without browser check

SEO:
□ Unique title/description per page
□ Open Graph tags
□ JSON-LD structured data
□ Sitemap.xml
```
