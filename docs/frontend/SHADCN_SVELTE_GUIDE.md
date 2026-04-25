# shadcn-svelte Component Library Guide

**Apple ICT 7 Principal Engineer Grade - January 2026**

Complete guide for using shadcn-svelte and bits-ui in the Revolution Trading Pros project.

---

## 📦 Installed Components

### Core Components
- ✅ **Button** - Primary, secondary, outline, ghost variants
- ✅ **Dialog** - Modal dialogs with header/footer
- ✅ **Dropdown Menu** - Context menus and dropdowns
- ✅ **Card** - Content containers with header/footer
- ✅ **Table** - Data tables with sorting
- ✅ **Badge** - Status indicators and labels
- ✅ **Input** - Text input fields
- ✅ **Label** - Form labels
- ✅ **Select** - Dropdown selects
- ✅ **Textarea** - Multi-line text input
- ✅ **Separator** - Visual dividers

### Location
All components are in: `src/lib/components/ui/`

---

## 🎯 Quick Start Examples

### Button Component

```svelte
<script>
  import { Button } from '$lib/components/ui/button';
</script>

<!-- Variants -->
<Button>Default</Button>
<Button variant="secondary">Secondary</Button>
<Button variant="outline">Outline</Button>
<Button variant="ghost">Ghost</Button>
<Button variant="destructive">Delete</Button>

<!-- Sizes -->
<Button size="sm">Small</Button>
<Button size="default">Default</Button>
<Button size="lg">Large</Button>
<Button size="icon">🔍</Button>

<!-- With onclick -->
<Button onclick={() => console.log('clicked')}>
  Click Me
</Button>
```

---

### Dialog Component

```svelte
<script>
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  
  let open = $state(false);
</script>

<Dialog.Root bind:open>
  <Dialog.Trigger asChild let:builder>
    <Button builders={[builder]}>Open Dialog</Button>
  </Dialog.Trigger>
  
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Edit Profile</Dialog.Title>
      <Dialog.Description>
        Make changes to your profile here. Click save when you're done.
      </Dialog.Description>
    </Dialog.Header>
    
    <!-- Your form content here -->
    <div class="grid gap-4 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <label for="name" class="text-right">Name</label>
        <input id="name" class="col-span-3" />
      </div>
    </div>
    
    <Dialog.Footer>
      <Button type="submit">Save changes</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
```

---

### Dropdown Menu

```svelte
<script>
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
  import { Button } from '$lib/components/ui/button';
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger asChild let:builder>
    <Button builders={[builder]} variant="outline">Open Menu</Button>
  </DropdownMenu.Trigger>
  
  <DropdownMenu.Content class="w-56">
    <DropdownMenu.Label>My Account</DropdownMenu.Label>
    <DropdownMenu.Separator />
    
    <DropdownMenu.Group>
      <DropdownMenu.Item>
        Profile
      </DropdownMenu.Item>
      <DropdownMenu.Item>
        Settings
      </DropdownMenu.Item>
    </DropdownMenu.Group>
    
    <DropdownMenu.Separator />
    
    <DropdownMenu.Item class="text-red-600">
      Log out
    </DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
```

---

### Card Component

```svelte
<script>
  import * as Card from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>Card Title</Card.Title>
    <Card.Description>Card description goes here</Card.Description>
  </Card.Header>
  
  <Card.Content>
    <p>Card content goes here</p>
  </Card.Content>
  
  <Card.Footer class="flex justify-between">
    <Button variant="outline">Cancel</Button>
    <Button>Save</Button>
  </Card.Footer>
</Card.Root>
```

---

### Table Component

```svelte
<script>
  import * as Table from '$lib/components/ui/table';
  
  const trades = [
    { ticker: 'NVDA', entry: 142.50, exit: 156.00, profit: '+9.5%' },
    { ticker: 'TSLA', entry: 248.00, exit: 265.00, profit: '+6.9%' }
  ];
</script>

<Table.Root>
  <Table.Caption>Recent Trades</Table.Caption>
  <Table.Header>
    <Table.Row>
      <Table.Head>Ticker</Table.Head>
      <Table.Head>Entry</Table.Head>
      <Table.Head>Exit</Table.Head>
      <Table.Head class="text-right">Profit</Table.Head>
    </Table.Row>
  </Table.Header>
  <Table.Body>
    {#each trades as trade}
      <Table.Row>
        <Table.Cell class="font-medium">{trade.ticker}</Table.Cell>
        <Table.Cell>${trade.entry}</Table.Cell>
        <Table.Cell>${trade.exit}</Table.Cell>
        <Table.Cell class="text-right">{trade.profit}</Table.Cell>
      </Table.Row>
    {/each}
  </Table.Body>
</Table.Root>
```

---

### Form Components

```svelte
<script>
  import { Button } from '$lib/components/ui/button';
  import { Input } from '$lib/components/ui/input';
  import { Label } from '$lib/components/ui/label';
  import * as Select from '$lib/components/ui/select';
  import { Textarea } from '$lib/components/ui/textarea';
  
  let ticker = $state('');
  let notes = $state('');
</script>

<form class="space-y-4">
  <div class="space-y-2">
    <Label for="ticker">Ticker Symbol</Label>
    <Input id="ticker" bind:value={ticker} placeholder="NVDA" />
  </div>
  
  <div class="space-y-2">
    <Label for="type">Alert Type</Label>
    <Select.Root>
      <Select.Trigger id="type">
        <Select.Value placeholder="Select type" />
      </Select.Trigger>
      <Select.Content>
        <Select.Item value="entry">Entry</Select.Item>
        <Select.Item value="exit">Exit</Select.Item>
        <Select.Item value="update">Update</Select.Item>
      </Select.Content>
    </Select.Root>
  </div>
  
  <div class="space-y-2">
    <Label for="notes">Notes</Label>
    <Textarea id="notes" bind:value={notes} placeholder="Trade notes..." />
  </div>
  
  <Button type="submit">Create Alert</Button>
</form>
```

---

### Badge Component

```svelte
<script>
  import { Badge } from '$lib/components/ui/badge';
</script>

<Badge>Default</Badge>
<Badge variant="secondary">Secondary</Badge>
<Badge variant="outline">Outline</Badge>
<Badge variant="destructive">Error</Badge>

<!-- Custom colors -->
<Badge class="bg-green-500">Profit</Badge>
<Badge class="bg-red-500">Loss</Badge>
```

---

## 🎨 Customization

### Modifying Components
All components are **source code in your project**. Edit them directly:

```bash
# Edit button styles
code src/lib/components/ui/button/button.svelte

# Edit dialog styles
code src/lib/components/ui/dialog/dialog.svelte
```

### Theme Customization
Edit `src/app.css` to change colors:

```css
@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;
    --primary: 222.2 47.4% 11.2%;
    /* ... more variables */
  }
}
```

---

## 🔄 Integration with Existing Code

### Replace Custom Modals

**Before (Custom Modal):**
```svelte
<div class="modal-overlay" class:open={isOpen}>
  <div class="modal-content">
    <h2>Title</h2>
    <p>Content</p>
    <button onclick={close}>Close</button>
  </div>
</div>
```

**After (shadcn Dialog):**
```svelte
<Dialog.Root bind:open={isOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Title</Dialog.Title>
    </Dialog.Header>
    <p>Content</p>
    <Dialog.Footer>
      <Button onclick={() => isOpen = false}>Close</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
```

### Benefits
- ✅ Automatic accessibility (ARIA, focus trap, keyboard nav)
- ✅ Consistent styling across app
- ✅ Less custom CSS to maintain
- ✅ Mobile-responsive by default

---

## 📚 Adding More Components

```bash
# See all available components
npx shadcn-svelte@latest add

# Add specific components
npx shadcn-svelte@latest add alert
npx shadcn-svelte@latest add toast
npx shadcn-svelte@latest add tabs
npx shadcn-svelte@latest add accordion
npx shadcn-svelte@latest add calendar
npx shadcn-svelte@latest add date-picker
npx shadcn-svelte@latest add command
npx shadcn-svelte@latest add popover
npx shadcn-svelte@latest add sheet
npx shadcn-svelte@latest add tooltip
```

---

## 🎯 Best Practices

### 1. Use for New Features
Start with shadcn components for new pages/features.

### 2. Gradual Migration
Don't refactor existing working code. Migrate opportunistically when:
- Adding new features
- Fixing bugs in modal/form code
- Building new dashboards

### 3. Customize as Needed
Edit component source to match your design system:
```svelte
<!-- src/lib/components/ui/button/button.svelte -->
<script>
  // Add custom variants
  const variants = {
    default: "bg-primary text-primary-foreground",
    brand: "bg-[var(--color-brand-primary)] text-white", // Your custom color
    // ...
  };
</script>
```

### 4. Combine with Existing Patterns
```svelte
<script>
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  
  // Use with your existing state management
  const ps = createPageState();
</script>

<Dialog.Root bind:open={ps.isAlertModalOpen}>
  <!-- Your existing alert form logic -->
</Dialog.Root>
```

---

## 🚨 Important Notes

### TypeScript
All components are fully typed. Use `$props()` for type safety:

```svelte
<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import type { Button as ButtonPrimitive } from 'bits-ui';
  
  type Props = ButtonPrimitive.Props & {
    customProp?: string;
  };
  
  const { customProp, ...rest }: Props = $props();
</script>
```

### Accessibility
Components are WCAG 2.1 AA compliant by default:
- ✅ Keyboard navigation
- ✅ Screen reader support
- ✅ Focus management
- ✅ ARIA attributes

### Mobile-First
All components are responsive. Customize with Tailwind breakpoints:
```svelte
<Dialog.Content class="w-full sm:max-w-[425px] md:max-w-[600px]">
  <!-- Responsive width -->
</Dialog.Content>
```

---

## 🔗 Resources

- **shadcn-svelte Docs:** https://www.shadcn-svelte.com
- **bits-ui Docs:** https://www.bits-ui.com
- **Component Examples:** https://www.shadcn-svelte.com/examples
- **GitHub:** https://github.com/huntabyte/shadcn-svelte

---

## 🎬 Next Steps

1. **Try a component** - Replace one modal with Dialog
2. **Customize theme** - Edit `app.css` colors to match brand
3. **Build new feature** - Use shadcn for next dashboard section
4. **Add more components** - Install as needed with CLI

---

**Last Updated:** January 31, 2026  
**Maintained By:** Development Team
