# ClassDownloadsSection Component - Usage Guide

**Apple ICT 7 Principal Engineer - Svelte 5 Component**

---

## Overview

The `ClassDownloadsSection` component is a reusable, standardized component for displaying Box.com file downloads across all class pages. Each class has its own unique Box.com folder URL.

## Component Props

```typescript
interface Props {
  boxUrl: string;      // Required: Box.com embed URL for the class materials
  title?: string;      // Optional: Section heading (default: "Class Downloads")
}
```

## Basic Usage

```svelte
<script>
  import ClassDownloadsSection from '$lib/components/classes/ClassDownloadsSection.svelte';
</script>

<ClassDownloadsSection 
  boxUrl="https://simplertrading.app.box.com/embed/s/YOUR_UNIQUE_FOLDER_ID?sortColumn=date&view=list"
  title="Class Downloads"
/>
```

---

## Examples for Different Classes

### 1. Quickstart To Precision Trading
```svelte
<ClassDownloadsSection 
  boxUrl="https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list"
  title="Class Downloads"
/>
```

### 2. Advanced Options Trading (Example)
```svelte
<ClassDownloadsSection 
  boxUrl="https://simplertrading.app.box.com/embed/s/abc123def456ghi789jkl?sortColumn=date&view=list"
  title="Class Downloads"
/>
```

### 3. Technical Analysis Masterclass (Example)
```svelte
<ClassDownloadsSection 
  boxUrl="https://simplertrading.app.box.com/embed/s/xyz789uvw456rst123opq?sortColumn=date&view=list"
  title="Technical Analysis Materials"
/>
```

### 4. Custom Title Example
```svelte
<ClassDownloadsSection 
  boxUrl="https://simplertrading.app.box.com/embed/s/custom123folder456?sortColumn=date&view=list"
  title="Course Resources & Downloads"
/>
```

---

## How to Get Box.com Embed URL

1. Go to Box.com and navigate to the class folder
2. Click "Share" button
3. Select "Create Shared Link"
4. Copy the shared link (e.g., `https://simplertrading.app.box.com/s/FOLDER_ID`)
5. Convert to embed URL: `https://simplertrading.app.box.com/embed/s/FOLDER_ID?sortColumn=date&view=list`

**URL Structure:**
- Base: `https://simplertrading.app.box.com/embed/s/`
- Folder ID: Unique identifier for the class folder
- Query params: `?sortColumn=date&view=list` (sorts by date, shows list view)

---

## Implementation in Class Pages

### File Structure
```
/routes/classes/
  ├── quickstart-precision-trading-c/
  │   └── +page.svelte
  ├── advanced-options-trading/
  │   └── +page.svelte
  └── technical-analysis-masterclass/
      └── +page.svelte
```

### Page Template
```svelte
<script lang="ts">
  import ClassDownloadsSection from '$lib/components/classes/ClassDownloadsSection.svelte';
  import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
  import HaveQuestionsSection from '$lib/components/sections/HaveQuestionsSection.svelte';
</script>

<DashboardBreadcrumbs />

<div id="page" class="hfeed site grid-parent">
  <div id="content" class="site-content">
    
    <!-- Class Info Section -->
    <section class="class-section cpost-section" id="class-info">
      <div class="section-inner">
        <h1>Your Class Title</h1>
      </div>
    </section>

    <!-- Video Section -->
    <section class="class-section cpost-section" id="class-recordings">
      <!-- Video player here -->
    </section>

    <!-- Class Downloads - UNIQUE URL PER CLASS -->
    <ClassDownloadsSection 
      boxUrl="https://simplertrading.app.box.com/embed/s/YOUR_CLASS_FOLDER_ID?sortColumn=date&view=list"
      title="Class Downloads"
    />

  </div>
</div>

<HaveQuestionsSection 
  email="support@revolutiontradingpros.com" 
  phone="8002668659" 
  phoneDisplay="(800) 266-8659" 
/>
```

---

## Component Features

### ✅ Exact Specifications
- **Desktop Container**: 1080px × 512px
- **Desktop Iframe**: 518px height
- **Padding**: 25px all sides
- **Background**: #FFFFFF (white)
- **Heading**: 1.75rem (28px), weight 400, #4a4a4a

### ✅ Responsive Breakpoints
- **Mobile Small** (< 428px): 300px iframe height
- **Mobile** (428-743px): 350px iframe height
- **Tablet** (744-1023px): 400px iframe height, 720px max-width
- **Desktop** (1024-1365px): 518px iframe height, 1080px max-width
- **Desktop Large** (1366px+): Fixed 1080px width, 518px iframe

### ✅ Accessibility Features
- Loading="lazy" for performance
- Sandbox permissions for security
- Noscript fallback with direct link
- Touch target compliance (44px minimum)
- Proper ARIA attributes

### ✅ Security
- `referrerpolicy="strict-origin-when-cross-origin"`
- Sandbox with specific permissions:
  - `allow-scripts`
  - `allow-same-origin`
  - `allow-forms`
  - `allow-popups`
  - `allow-popups-to-escape-sandbox`

---

## Maintenance

### Adding a New Class
1. Create new class page: `/routes/classes/new-class-name/+page.svelte`
2. Import the component
3. Pass the unique Box.com URL for that class
4. Customize the title if needed

### Updating Component Styles
All styling is centralized in the component. Changes to:
- Dimensions
- Colors
- Responsive breakpoints
- Typography

...will automatically apply to ALL class pages using the component.

---

## Best Practices

### ✅ DO
- Use unique Box.com URLs for each class
- Keep the `?sortColumn=date&view=list` query params
- Use descriptive titles when needed
- Test the Box.com link before deploying

### ❌ DON'T
- Hardcode styles in the page (use the component)
- Remove security attributes from iframe
- Use non-embed Box.com URLs
- Duplicate the component code

---

## Troubleshooting

### Issue: Iframe not displaying
- Check if Box.com URL is correct and accessible
- Verify the folder has proper sharing permissions
- Ensure URL uses `/embed/` not just `/s/`

### Issue: Wrong dimensions on mobile
- Component handles all responsive breakpoints automatically
- Check if there are conflicting global styles

### Issue: Files not showing in Box.com embed
- Verify folder permissions in Box.com
- Check if files are uploaded to the correct folder
- Ensure shared link is active

---

## Version History

- **v2.0.0** - Apple ICT 7 Principal Engineer refactor with exact specifications
- **v1.0.0** - Initial component creation

---

**Component Location**: `/src/lib/components/classes/ClassDownloadsSection.svelte`  
**Documentation**: This file  
**Specification**: `CLASS_DOWNLOADS_SPEC.md`
