# Storybook Guide - Revolution Trading Pros

## 🎨 What is Storybook?

Storybook is a **component development workshop** that lets you:
- Build and test UI components in isolation
- Document components with interactive examples
- Test different states and props
- Share component library with team
- Catch UI bugs before they reach production

## 🚀 Quick Start

### Run Storybook

```bash
npm run storybook
```

This starts Storybook at `http://localhost:6006`

### Build Storybook (for deployment)

```bash
npm run build-storybook
```

## 📝 Writing Stories (Svelte CSF)

We use **Svelte CSF** (Component Story Format) - write stories in `.stories.svelte` files using native Svelte syntax.

### Basic Story Structure

```svelte
<!-- Component.stories.svelte -->
<script context="module">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import Component from './Component.svelte';
  
  const { Story } = defineMeta({
    component: Component,
    title: 'Category/Component',
    tags: ['autodocs']
  });
</script>

<!-- Default story -->
<Story name="Default" />

<!-- Story with props -->
<Story name="Custom" args={{ prop: 'value' }} />
```

### Interactive Stories with Svelte 5 Runes

```svelte
<Story name="Interactive">
  <script>
    let count = $state(0);
    let doubled = $derived(count * 2);
  </script>
  
  <button onclick={() => count++}>
    Count: {count} (doubled: {doubled})
  </button>
  <Component {count} />
</Story>
```

### Testing Different States

```svelte
<!-- Test all variants -->
<Story name="Loading" args={{ loading: true }} />
<Story name="Error" args={{ error: 'Something went wrong' }} />
<Story name="Success" args={{ data: mockData }} />
```

## 🎯 Best Practices

### 1. **One Story File Per Component**

```
src/lib/components/admin/
├── ServiceConnectionStatus.svelte
├── ServiceConnectionStatus.stories.svelte  ← Stories here
├── ConnectionGate.svelte
└── ConnectionGate.stories.svelte           ← Stories here
```

### 2. **Use Descriptive Story Names**

```svelte
<Story name="Payment - Card Variant" />        ✅ Good
<Story name="Test1" />                          ❌ Bad
```

### 3. **Group Related Stories**

```svelte
const { Story } = defineMeta({
  title: 'Admin/Connections/ServiceConnectionStatus',  // Creates hierarchy
  component: ServiceConnectionStatus
});
```

### 4. **Document Props with ArgTypes**

```svelte
const { Story } = defineMeta({
  component: Component,
  argTypes: {
    variant: {
      control: 'select',
      options: ['card', 'banner', 'inline'],
      description: 'Display variant of the component'
    },
    showFeatures: {
      control: 'boolean',
      description: 'Show feature list'
    }
  }
});
```

### 5. **Test Edge Cases**

```svelte
<Story name="Empty State" args={{ items: [] }} />
<Story name="Long Text" args={{ title: 'Very long title that might wrap...' }} />
<Story name="Disabled" args={{ disabled: true }} />
```

## 🧪 Testing Components

### Visual Testing

Stories automatically serve as visual tests. Any UI change is visible in Storybook.

### Interaction Testing

```svelte
<Story name="Click Interaction">
  <script>
    let clicked = $state(false);
  </script>
  
  <button onclick={() => clicked = true}>
    Click me
  </button>
  {#if clicked}
    <p>Button was clicked!</p>
  {/if}
</Story>
```

### Accessibility Testing

Storybook includes the **a11y addon** - check the "Accessibility" tab in Storybook UI.

## 📚 Example Stories

### Admin Components

We've created example stories for:

1. **ServiceConnectionStatus** - All 5 variants (card, banner, inline, badge, minimal)
2. **ConnectionGate** - Conditional rendering wrapper
3. More coming soon...

### Creating Your Own Stories

```svelte
<!-- YourComponent.stories.svelte -->
<script context="module">
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import YourComponent from './YourComponent.svelte';
  
  const { Story } = defineMeta({
    component: YourComponent,
    title: 'Admin/YourComponent',
    tags: ['autodocs']
  });
</script>

<Story name="Default" />

<Story name="With Props" args={{
  title: 'Custom Title',
  variant: 'primary'
}} />

<Story name="Interactive">
  <script>
    let value = $state('');
  </script>
  
  <input bind:value placeholder="Type something..." />
  <YourComponent {value} />
</Story>
```

## 🎨 Design System Integration

### Using Admin Design Tokens

```svelte
<Story name="With Design Tokens">
  <div style="
    padding: 2rem;
    background: var(--bg-base, #0d1117);
    color: var(--text-primary, #f0f6fc);
  ">
    <YourComponent />
  </div>
</Story>
```

### Testing Dark Theme

All admin components use dark theme by default. Wrap stories in dark background:

```svelte
<Story name="Dark Theme">
  <div style="padding: 2rem; background: #0d1117;">
    <YourComponent />
  </div>
</Story>
```

## 🔧 Configuration

### Main Config (`.storybook/main.ts`)

```typescript
const config: StorybookConfig = {
  stories: [
    "../src/**/*.mdx",
    "../src/**/*.stories.@(js|ts|svelte)"
  ],
  addons: [
    "@storybook/addon-svelte-csf",      // Svelte CSF support
    "@chromatic-com/storybook",         // Visual testing
    "@storybook/addon-a11y",            // Accessibility
    "@storybook/addon-docs"             // Auto-docs
  ],
  framework: "@storybook/sveltekit"
};
```

### Preview Config (`.storybook/preview.ts`)

Global decorators and parameters for all stories.

## 📖 Addons Included

| Addon | Purpose |
|-------|---------|
| `addon-svelte-csf` | Write stories in `.svelte` files |
| `addon-a11y` | Accessibility testing |
| `addon-docs` | Auto-generate documentation |
| `chromatic` | Visual regression testing |

## 🚢 Deployment

### Build for Production

```bash
npm run build-storybook
```

Output: `storybook-static/` directory

### Deploy to Cloudflare Pages

```bash
# Build
npm run build-storybook

# Deploy
wrangler pages deploy storybook-static --project-name=rtp-storybook
```

## 💡 Tips & Tricks

### 1. **Use Controls Panel**

Storybook's Controls panel lets you change props interactively without editing code.

### 2. **Keyboard Shortcuts**

- `S` - Toggle sidebar
- `A` - Toggle addons panel
- `F` - Toggle fullscreen
- `/` - Search stories

### 3. **Mock SvelteKit Modules**

Storybook automatically mocks:
- `$app/navigation` (goto, invalidate, etc.)
- `$app/stores` (page, navigating, etc.)
- `$app/environment` (browser, dev, etc.)

### 4. **Test Responsive**

Use the viewport addon to test mobile/tablet/desktop views.

### 5. **Share Stories**

Share a link to a specific story:
```
http://localhost:6006/?path=/story/admin-connections-serviceconnectionstatus--payment-card
```

## 🐛 Troubleshooting

### Story Not Showing Up

1. Check file name ends with `.stories.svelte`
2. Verify `defineMeta` is in `<script context="module">`
3. Restart Storybook dev server

### Component Not Rendering

1. Check imports are correct
2. Verify component works in your app first
3. Check browser console for errors

### Styles Not Applied

1. Import global styles in `.storybook/preview.ts`
2. Use inline styles or component-scoped styles
3. Check CSS variables are defined

## 📚 Resources

- [Storybook Docs](https://storybook.js.org/docs)
- [Svelte CSF Addon](https://github.com/storybookjs/addon-svelte-csf)
- [SvelteKit Integration](https://storybook.js.org/docs/get-started/frameworks/sveltekit)
- [Svelte 5 Runes](https://svelte.dev/docs/svelte/what-are-runes)

## 🎯 Next Steps

1. ✅ Run `npm run storybook` to see example stories
2. ✅ Browse existing stories for ServiceConnectionStatus and ConnectionGate
3. ✅ Create stories for your own components
4. ✅ Share Storybook URL with team
5. ✅ Integrate into CI/CD for visual regression testing

---

**Happy Story Writing! 🎨**
