# Dashboard Design Specifications
## Source: WordPress Simpler Trading (dashboard.8f78208b.css)

---

## PAGE STRUCTURE (Line 2729+ of DashboardHome)

```
1. <header id="masthead" class="site-header">     - Logo
2. <nav id="site-navigation" class="main-navigation">  - Main Menu (70px line-height)
3. <nav id="breadcrumbs" class="breadcrumbs">     - Breadcrumbs
4. <div class="dashboard">                         - Dashboard Container
   ├── <aside class="dashboard__sidebar">          - Sidebar
   │   ├── <nav class="dashboard__nav-primary">    - Navigation
   │   ├── <footer class="dashboard__toggle">      - Mobile Toggle (hidden desktop)
   │   └── <div class="dashboard__overlay">        - Mobile Overlay
   └── <main class="dashboard__main">              - Main Content
       ├── <header class="dashboard__header">      - Page Header
       └── <div class="dashboard__content">        - Content Area
```

---

## COLORS

| Element | Color |
|---------|-------|
| Sidebar Background | `#0f2d41` |
| Toggle Footer Background | `#0d2532` |
| Main Content Background | `#f4f4f4` |
| Dashboard Header Background | `#ffffff` |
| Text White | `#ffffff` |
| Text Muted (links) | `hsla(0, 0%, 100%, 0.5)` |
| Accent Blue (hover/active) | `#0984ae` |
| Border Color | `#dbdbdb` |
| Body Background | `#efefef` |
| Link Blue | `#1e73be` |

---

## TYPOGRAPHY

**RTP uses Montserrat (not Open Sans like WordPress)**

| Element | Font | Size | Weight |
|---------|------|------|--------|
| Body/Nav | Montserrat | 14px | 300 |
| Category Headers | Montserrat | 16px | 700, uppercase |
| Profile Name | Montserrat | 14px | 600 |
| Nav Item Text | Montserrat | 14px | 300 (bold items: 700) |
| Breadcrumbs | Montserrat | 13px | 400 |

---

## SIDEBAR (.dashboard__sidebar)

### Desktop (1280px+)
```css
.dashboard__sidebar {
  display: flex;
  flex: 0 0 auto;
  flex-direction: column;
  width: 280px;
  background-color: #0f2d41;
  position: static;  /* Part of flex layout */
  min-height: 100vh;
}
```

### Collapsed State (desktop)
```css
.dashboard__sidebar.is-collapsed {
  width: 80px;
  padding-top: 30px;
}
/* Hide text, show only icons */
.is-collapsed .dashboard__profile-name,
.is-collapsed .dashboard__nav-item-text,
.is-collapsed .dashboard__nav-category {
  opacity: 0;
  visibility: hidden;
  width: 0;
}
```

### Mobile (<1280px)
```css
.dashboard__sidebar {
  position: fixed;
  top: 0;
  left: 0;
  bottom: 50px;  /* Space for toggle footer */
  width: 280px;
  opacity: 0;
  visibility: hidden;
  z-index: 100010;
  transform: translateX(-100%);
  transition: all 0.3s ease-in-out;
}

.dashboard__sidebar.is-mobile-open {
  opacity: 1;
  visibility: visible;
  transform: translateX(0);
}
```

---

## NAV PRIMARY (.dashboard__nav-primary)

```css
.dashboard__nav-primary {
  width: 280px;
  padding-bottom: 30px;
  font-size: 16px;
  background-color: #0f2d41;
}
```

---

## PROFILE SECTION

```css
.dashboard__profile-nav-item {
  height: auto;
  line-height: 1.4;
  padding: 32px 30px 28px 80px;
  position: relative;
}

.dashboard__profile-photo {
  position: absolute;
  top: 50%;
  left: 30px;
  margin-top: -17px;
  width: 34px;
  height: 34px;
  border: 2px solid #fff;
  border-radius: 50%;
  background-size: 32px;
  background-position: center;
  transition: all 0.15s ease-in-out;
}

.dashboard__profile-nav-item:hover .dashboard__profile-photo {
  border-color: #0984ae;
}

.dashboard__profile-name {
  color: #fff;
  font-size: 14px;
  font-weight: 600;
}
```

---

## CATEGORY HEADERS (.dashboard__nav-category)

```css
.dashboard__nav-category {
  padding: 30px 30px 0;
  font-size: 16px;
  font-weight: 700;
  text-transform: uppercase;
  color: #fff;
}
```

---

## NAV LINKS (.dash_main_links)

```css
.dash_main_links a {
  display: flex;
  align-items: center;
  height: 50px;
  padding: 0 20px 0 80px;
  color: hsla(0, 0%, 100%, 0.5);
  font-size: 14px;
  font-weight: 300;
  transition: all 0.15s ease-in-out;
  position: relative;
}

.dash_main_links a:hover {
  color: #fff;
  background-color: rgba(255, 255, 255, 0.05);
}
```

### Active State
```css
.dash_main_links li.is-active a {
  color: #fff;
  background-color: rgba(255, 255, 255, 0.08);
}

/* RIGHT border indicator (user confirmed) */
.dash_main_links li.is-active a::after {
  content: '';
  position: absolute;
  right: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background-color: #0984ae;
}
```

---

## NAV ICONS (.dashboard__nav-item-icon)

```css
.dashboard__nav-item-icon {
  position: absolute;
  top: 50%;
  left: 30px;
  margin-top: -16px;
  width: 32px;
  height: 32px;
  font-size: 32px;
  line-height: 32px;
  color: hsla(0, 0%, 100%, 0.5);
  transition: all 0.15s ease-in-out;
}

.dash_main_links a:hover .dashboard__nav-item-icon {
  color: #fff;
}
```

---

## TOGGLE FOOTER (.dashboard__toggle)

```css
/* Hidden on desktop (1280px+) */
.dashboard__toggle {
  display: none;
}

/* Mobile only */
@media (max-width: 1279px) {
  .dashboard__toggle {
    display: block;
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: 50px;
    line-height: 50px;
    background-color: #0d2532;
    z-index: 100010;
  }
}
```

---

## MAIN CONTENT (.dashboard__main)

```css
.dashboard__main {
  flex: 1 1 auto;
  min-width: 0;
  background-color: #f4f4f4;
}

/* Mobile */
@media (max-width: 1279px) {
  .dashboard__main {
    width: 100%;
    padding-bottom: 50px;  /* Space for toggle footer */
  }
}
```

---

## DASHBOARD HEADER (.dashboard__header)

```css
.dashboard__header {
  background-color: #fff;
  border-bottom: 1px solid #dbdbdb;
  border-right: 1px solid #dbdbdb;
  max-width: 1700px;
  padding: 20px;
}

@media (min-width: 820px) {
  .dashboard__header {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
  }
}

@media (min-width: 1280px) {
  .dashboard__header {
    padding: 30px;
  }
}

@media (min-width: 1440px) {
  .dashboard__header {
    padding: 30px 40px;
  }
}
```

---

## BREADCRUMBS (#breadcrumbs)

```css
.breadcrumbs {
  z-index: 1;
  background-color: #efefef;
  border-bottom: 1px solid #dbdbdb;
  font-size: 13px;
  padding: 10px 20px;
}

.breadcrumb-link {
  color: #1e73be;
}

.breadcrumb-link:hover {
  color: #0984ae;
}

.breadcrumb-current {
  color: #666;
  font-weight: 600;
}

.separator {
  color: #999;
  padding: 0 8px;
}
```

---

## BREAKPOINTS

| Name | Width | Sidebar Behavior |
|------|-------|------------------|
| Mobile | < 1280px | Fixed, hidden by default |
| Desktop | >= 1280px | Static, visible |
| Desktop Large | >= 1440px | Increased padding |

---

## TRANSITIONS

```css
/* Standard transition */
transition: all 0.15s ease-in-out;

/* Sidebar transition */
transition: all 0.3s ease-in-out;

/* Hover tooltip (collapsed state) */
transition: all 0.15s ease-in-out;
transform: translate(5px);
```

---

## Z-INDEX HIERARCHY

| Element | z-index |
|---------|---------|
| Sidebar (mobile) | 100010 |
| Toggle Footer | 100010 |
| Overlay | 99 |
| Secondary Sidebar | 99 |
| Sidebar (desktop) | auto |

---

## HOVER EFFECTS

### Profile
- Photo border: `#fff` → `#0984ae`
- Name color: `#fff` → `#0984ae`

### Nav Links
- Text: `hsla(0,0%,100%,0.5)` → `#fff`
- Icon: `hsla(0,0%,100%,0.5)` → `#fff`
- Background: transparent → `rgba(255,255,255,0.05)`

### Active State
- Left border: 4px `#0984ae`
- Background: `rgba(255,255,255,0.08)`

---

## RESPONSIVE PADDING (1440px+)

```css
.dash_main_links a {
  padding: 0 40px 0 80px;
}

.dashboard__profile-nav-item {
  padding: 32px 40px 28px 80px;
}

.dashboard__nav-category {
  padding: 30px 40px 0;
}
```

---

## DASHBOARD CONTENT AREA

### Content Container (.dashboard__content)
```css
.dashboard__content {
  display: flex;
  flex-flow: row nowrap;
}
```

### Content Main (.dashboard__content-main)
```css
.dashboard__content-main {
  flex: 1 1 auto;
  min-width: 0;
  border-right: 1px solid #dbdbdb;
}
```

### Content Section (.dashboard__content-section)
```css
.dashboard__content-section {
  padding: 30px 20px;
  overflow-x: auto;
  overflow-y: hidden;
}

@media (min-width: 1280px) {
  padding: 30px;
}

@media (min-width: 1440px) {
  padding: 40px;
}
```

### Content Sidebar (.dashboard__content-sidebar)
```css
.dashboard__content-sidebar {
  display: none;
  width: 260px;
  flex: 0 0 auto;
  margin-top: -1px;
  border-right: 1px solid #dbdbdb;
  border-top: 1px solid #dbdbdb;
}

@media (min-width: 1080px) {
  display: block;
}
```

### Page Title (.dashboard__page-title)
```css
h1.dashboard__page-title {
  margin: 0;
  color: #333;
  font-size: 36px;
  font-weight: 300;
}
```

### Section Title (.section-title)
```css
h2.section-title {
  color: #333;
  font-weight: 700;
  font-size: 20px;
  margin-bottom: 30px;
}
```

### Section Title Alt (.section-title-alt)
```css
h2.section-title-alt {
  color: #0984ae;
  font-weight: 700;
  font-size: 14px;
  letter-spacing: 20%;
  margin-bottom: 30px;
  text-transform: uppercase;
}

/* With --underline modifier */
h2.section-title-alt--underline {
  padding-bottom: 30px;
  position: relative;
}

h2.section-title-alt--underline::after {
  content: " ";
  display: block;
  position: absolute;
  bottom: 2px;
  left: 0;
  width: 50px;
  height: 2px;
  background-color: #e8e8e8;
}
```

---

## MEMBERSHIP CARDS

### Card Container (.membership-cards)
```css
.membership-cards {
  margin-top: -30px;
}
```

### Card Base (.membership-card)
```css
.membership-card {
  margin-top: 30px;
  background: #fff;
  border-radius: 5px;
  box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
}
```

### Card Header (.membership-card__header)
```css
.membership-card__header {
  display: block;
  padding: 20px;
  color: #333;
  font-weight: 700;
  white-space: nowrap;
  transition: all 0.15s ease-in-out;
}

.membership-card__header:hover {
  color: #0984ae;
}
```

### Card Icon (.membership-card__icon)
```css
.membership-card__icon {
  display: inline-block;
  width: 50px;
  height: 50px;
  margin-right: 9px;
  line-height: 50px;
  color: #fff;
  text-align: center;
  border-radius: 50%;
  transition: all 0.15s ease-in-out;
}
```

### Card Actions (.membership-card__actions)
```css
.membership-card__actions {
  display: flex;
  font-size: 14px;
  border-top: 1px solid #ededed;
  justify-content: center;
}

.membership-card__actions a {
  display: block;
  flex: 0 0 auto;
  flex-basis: 50%;
  width: 50%;
  height: 100%;
  padding: 15px;
  text-align: center;
}

.membership-card__actions a + a {
  border-left: 1px solid #ededed;
}
```

---

## MEMBERSHIP CARD VARIANTS (Icon Colors)

| Variant | Background | Box-Shadow | Hover |
|---------|------------|------------|-------|
| `--options` | `#0984ae` | `rgba(9,132,174,.25)` | `#076787` |
| `--foundation` | `#00abaf` | `rgba(0,171,175,.25)` | `#008386` |
| `--consistent-growth` | `#005695` | `rgba(0,86,149,.25)` | - |
| `--tr3ndy-spx-alerts` | `#fe8900` | `rgba(254,137,0,.25)` | `#d57300` |
| `--ww` (Weekly Watchlist) | `#0c2434` | `rgba(12,36,52,.25)` | `#040d13` |
| `--training-room` | `#3c22f1` | `rgba(60,34,241,.25)` | `#280edc` |

---

## DROPDOWN MENU

```css
.dropdown-menu {
  padding: 20px;
  min-width: 260px;
  max-width: 280px;
  margin: 5px 0 0;
  font-size: 14px;
  border: none;
  border-radius: 5px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
}

.dropdown-menu__heading {
  margin: -20px -20px 20px;
  padding: 15px 20px;
  color: #fff;
  background: #0984ae;
  font-size: 17px;
  font-weight: 700;
}

.dropdown-menu__menu > li > a {
  padding: 10px 15px;
  font-size: 14px;
  color: #666;
}

.dropdown-menu__menu > li > a:hover {
  background-color: #f4f4f4;
}
```

---

## ICON SIZES

```css
.icon--md {
  font-size: 24px;
  line-height: 0;
}

.icon--lg {
  font-size: 32px;
  line-height: 0;
}
```
