# RULES-SVELTE5.md
## Svelte 5 Mandatory Patterns | January 2026

## 🔥 RUNES - ALWAYS USE
```svelte
<script lang="ts">
  // STATE
  let count = $state(0);
  let items = $state<Item[]>([]);
  
  // DERIVED
  let doubled = $derived(count * 2);
  let filtered = $derived.by(() => items.filter(i => i.active));
  
  // PROPS
  let { title, onClose, variant = 'primary' }: Props = $props();
  let { value = $bindable() }: { value: string } = $props();
  
  // EFFECTS
  $effect(() => {
    console.log('Count:', count);
    return () => console.log('Cleanup');
  });
</script>
```

## ❌ BANNED - SVELTE 4 LEGACY
```svelte
export let prop;              // Use $props()
$: derived = x * 2;           // Use $derived()
<slot />                      // Use {@render children()}
on:click={handler}            // Use onclick={handler}
createEventDispatcher()       // Use callback props
```

## 🧩 SNIPPETS - REPLACE SLOTS
```svelte
<script lang="ts">
  import type { Snippet } from 'svelte';
  let { children, header }: { 
    children: Snippet; 
    header?: Snippet 
  } = $props();
</script>

{@render header?.()}
{@render children()}
```

Usage:
```svelte
<Component>
  {#snippet header()}
    <h1>Title</h1>
  {/snippet}
  <p>Content</p>
</Component>
```

## 🖱️ EVENT HANDLERS
```svelte
<button onclick={handleClick}>Click</button>
<input oninput={(e) => value = e.currentTarget.value} />
<form onsubmit={(e) => { e.preventDefault(); submit(); }}>
```

## 📤 CALLBACK PROPS (Not Dispatch)
```svelte
<script lang="ts">
  let { onSelect, onClose }: { 
    onSelect?: (item: Item) => void;
    onClose?: () => void;
  } = $props();
</script>

<button onclick={() => onClose?.()}>Close</button>
```

## 🗄️ STORES - .svelte.ts
```typescript
// stores/user.svelte.ts
class UserStore {
  user = $state<User | null>(null);
  isLoading = $state(false);
  isAuthenticated = $derived(this.user !== null);
  
  async login(email: string, password: string) {
    this.isLoading = true;
    try {
      const res = await fetch('/api/auth/login', {
        method: 'POST',
        body: JSON.stringify({ email, password })
      });
      this.user = await res.json();
    } finally {
      this.isLoading = false;
    }
  }
}
export const userStore = new UserStore();
```

## 📋 CHECKLIST
```
□ Using $state() for reactive variables
□ Using $derived() for computed values  
□ Using $props() for component props
□ Using $effect() for side effects
□ Using {@render} for snippets
□ Using onclick not on:click
□ Using callback props not dispatch
□ No legacy Svelte 4 syntax
```
