# RULES-ANIMATIONS.md
## Netflix-Grade UI/UX & Animations

## 🎬 GSAP SETUP
```svelte
<script lang="ts">
  import { gsap } from 'gsap';
  import { ScrollTrigger } from 'gsap/ScrollTrigger';
  gsap.registerPlugin(ScrollTrigger);
  
  let container: HTMLElement;
  
  $effect(() => {
    if (!container) return;
    const ctx = gsap.context(() => {
      // Animations here
    }, container);
    return () => ctx.revert(); // CLEANUP!
  });
</script>
```

## 🎴 NETFLIX CARD HOVER
```svelte
<script lang="ts">
  import { gsap } from 'gsap';
  let card: HTMLElement;
  
  function onEnter() {
    gsap.to(card, {
      scale: 1.05, y: -10,
      boxShadow: '0 25px 50px -12px rgba(0,0,0,0.5)',
      duration: 0.3, ease: 'power2.out'
    });
  }
  
  function onLeave() {
    gsap.to(card, {
      scale: 1, y: 0,
      boxShadow: '0 4px 6px -1px rgba(0,0,0,0.1)',
      duration: 0.3, ease: 'power2.out'
    });
  }
</script>

<div bind:this={card} class="transform-gpu" onmouseenter={onEnter} onmouseleave={onLeave}>
  <img src={thumb} class="aspect-video object-cover" />
</div>
```

## 🔄 STAGGERED ENTRANCE
```svelte
<script lang="ts">
  let cards: HTMLElement[] = $state([]);
  
  $effect(() => {
    if (cards.length === 0) return;
    gsap.from(cards, {
      y: 100, opacity: 0,
      duration: 0.8, stagger: 0.1,
      ease: 'power3.out',
      scrollTrigger: { trigger: cards[0], start: 'top 80%' }
    });
  });
</script>

{#each items as item, i}
  <div bind:this={cards[i]}></div>
{/each}
```

## 💀 SKELETON LOADING
```svelte
{#if isLoading}
  <div class="animate-pulse space-y-4">
    <div class="h-48 bg-surface-elevated rounded-xl"></div>
    <div class="h-4 bg-surface-elevated rounded w-3/4"></div>
    <div class="h-4 bg-surface-elevated rounded w-1/2"></div>
  </div>
{:else}
  {@render children()}
{/if}
```

## ✨ MICRO-INTERACTIONS
```svelte
<!-- Button press -->
<button class="active:scale-95 hover:scale-[1.02] transition-transform">

<!-- Input focus -->
<input class="focus:ring-4 focus:ring-brand-primary/20 transition-all">

<!-- Toggle -->
<button 
  class="w-12 h-6 rounded-full {checked ? 'bg-brand-primary' : 'bg-surface-elevated'}"
>
  <span class="w-4 h-4 rounded-full bg-white transition-transform {checked ? 'translate-x-6' : ''}"></span>
</button>
```

## 🎯 PERFORMANCE
```svelte
<!-- GPU acceleration -->
<div class="transform-gpu will-change-transform">

<!-- Reduced motion -->
<script>
  let reducedMotion = $state(false);
  $effect(() => {
    reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  });
</script>
```

## 📋 CHECKLIST
```
□ 60fps animations
□ transform-gpu on animated elements
□ GSAP contexts cleaned up
□ Skeleton states for loading
□ Hover/focus states
□ Reduced motion respected
```
