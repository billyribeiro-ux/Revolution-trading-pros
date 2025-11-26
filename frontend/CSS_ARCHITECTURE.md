# CSS Architecture Guide

## Core Principle: Complete Design Freedom Per Page/Section

Each page and section has **complete independence** for styling. No global styles should interfere with component-specific designs.

## File Structure

### 1. `src/app.css` - Global Tokens ONLY
**Purpose:** Design system tokens, resets, and utilities  
**Contains:**
- ✅ CSS custom properties (colors, spacing, shadows)
- ✅ CSS resets (box-sizing, margins, padding)
- ✅ Utility classes (`.sr-only`, `.transition-fast`, etc.)
- ✅ Performance optimizations (font rendering, scroll behavior)

**Does NOT contain:**
- ❌ Component-specific styles
- ❌ Layout styles
- ❌ Section-specific animations
- ❌ Page-specific designs

### 2. Component `<style>` Tags - Scoped Styles
**Purpose:** All component-specific styling  
**Scope:** Automatically scoped to the component (Svelte magic!)

```svelte
<style>
  /* These styles ONLY apply to this component */
  .my-button {
    background: blue;
  }
</style>
```

## Best Practices

### ✅ DO:
1. **Use scoped styles in components**
   ```svelte
   <style>
     .hero-section {
       background: linear-gradient(...);
     }
   </style>
   ```

2. **Use Tailwind classes for rapid development**
   ```svelte
   <div class="bg-slate-900 p-8 rounded-xl">
   ```

3. **Use CSS custom properties from app.css**
   ```svelte
   <style>
     .card {
       border-radius: var(--radius-lg);
       transition: var(--transition-base);
     }
   </style>
   ```

4. **Combine Tailwind + scoped CSS**
   ```svelte
   <div class="relative p-8">
     <div class="custom-gradient"></div>
   </div>
   
   <style>
     .custom-gradient {
       background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
     }
   </style>
   ```

### ❌ DON'T:
1. **Don't add component styles to app.css**
   ```css
   /* ❌ WRONG - in app.css */
   .hero-title {
     font-size: 4rem;
   }
   ```

2. **Don't use `:global()` unless absolutely necessary**
   ```svelte
   <style>
     /* ❌ Avoid this - breaks scoping */
     :global(.some-class) {
       color: red;
     }
   </style>
   ```

3. **Don't rely on parent component styles**
   - Each component should be self-contained
   - Don't assume parent will provide certain styles

## Component Style Template

```svelte
<script lang="ts">
  // Component logic
</script>

<!-- Markup with Tailwind classes -->
<div class="relative p-8 bg-slate-900">
  <h1 class="custom-heading">Title</h1>
</div>

<!-- Scoped styles for custom designs -->
<style>
  /* Component-specific styles */
  .custom-heading {
    font-size: clamp(2rem, 5vw, 4rem);
    background: linear-gradient(to right, #667eea, #764ba2);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  /* Animations */
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .custom-heading {
    animation: fadeIn 0.5s ease-out;
  }
</style>
```

## CSS Scoping in Svelte

### How It Works:
Svelte automatically adds unique class names to your styles:

**Your code:**
```svelte
<style>
  .button { color: blue; }
</style>
```

**Compiled output:**
```css
.button.svelte-xyz123 { color: blue; }
```

This ensures styles ONLY apply to that component!

## Debugging Style Conflicts

If styles aren't working:

1. **Check specificity**
   - Tailwind classes have high specificity
   - Use `!important` sparingly in scoped styles if needed

2. **Inspect in DevTools**
   - Look for `.svelte-xxxxx` classes
   - Check which styles are being applied

3. **Verify scoping**
   - Styles in `<style>` tags are automatically scoped
   - Styles in `app.css` are global

4. **Check for `:global()`**
   - This breaks scoping - avoid unless necessary

## Migration Checklist

When creating a new page/section:

- [ ] Create component file in appropriate directory
- [ ] Use Tailwind classes for layout/spacing
- [ ] Add `<style>` tag for custom designs
- [ ] Test in isolation (component should work standalone)
- [ ] Verify no global styles are interfering
- [ ] Document any design tokens used

## Performance Notes

- ✅ Scoped styles are tree-shaken (unused styles removed)
- ✅ Tailwind purges unused classes in production
- ✅ CSS custom properties have minimal performance impact
- ✅ Component styles are code-split automatically

## Examples

### Good Example: Hero Section
```svelte
<script>
  // Logic
</script>

<section class="relative min-h-screen flex items-center justify-center">
  <div class="hero-content">
    <h1 class="hero-title">Welcome</h1>
  </div>
</section>

<style>
  .hero-content {
    max-width: 1200px;
    padding: 2rem;
  }

  .hero-title {
    font-size: clamp(2.5rem, 8vw, 5rem);
    font-weight: 800;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }
</style>
```

### Bad Example: Styles in app.css
```css
/* ❌ DON'T DO THIS in app.css */
.hero-title {
  font-size: 5rem;
  color: blue;
}
```

## Questions?

If you're unsure where styles should go:
- **Design tokens** → `app.css`
- **Component-specific** → Component `<style>` tag
- **Reusable utilities** → `app.css` utilities layer
- **Page layouts** → Page component `<style>` tag

---

**Remember:** Each page and section should have complete design freedom!
