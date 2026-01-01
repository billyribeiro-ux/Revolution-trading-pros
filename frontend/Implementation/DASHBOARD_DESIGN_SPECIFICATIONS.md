# Dashboard Design Specifications - Complete Visual Guide
**Apple ICT 11 Principal Engineer Grade Documentation**

**Source File:** `frontend/Implementation/DashboardHome`  
**Date Created:** January 1, 2026  
**Purpose:** Comprehensive design specifications for all dashboard components including typography, hover effects, animations, colors, and spacing

---

## Table of Contents
1. [Typography System](#typography-system)
2. [Color Palette](#color-palette)
3. [Hover Effects & Transitions](#hover-effects--transitions)
4. [Animations](#animations)
5. [Dashboard Components](#dashboard-components)
6. [Navigation System](#navigation-system)
7. [Button Specifications](#button-specifications)
8. [Icon System](#icon-system)
9. [Responsive Breakpoints](#responsive-breakpoints)

---

## Typography System

### Font Families
```css
/* Primary Font Stack */
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, 
                 Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif;
}

/* Google Fonts Integration */
@import url('https://fonts.googleapis.com/css?family=Open+Sans:400,700');
```

### Font Sizes

#### Headings
| Element | Font Size | Font Weight | Line Height | Usage |
|---------|-----------|-------------|-------------|-------|
| `h1` | Not specified | bold | Default | Page titles |
| `h2` | 32px | Default | Default | Section headers |
| `h3` | 26px | Default | Default | Subsection headers |
| `.dashboard__page-title` | Inherited | 700 | Default | Dashboard page titles |

#### Body Text
| Element | Font Size | Font Weight | Line Height | Usage |
|---------|-----------|-------------|-------------|-------|
| Body | 16px (default) | 400 | 1.5 | Standard content |
| `.fl-photo-caption` | 13px | normal | 18px | Image captions |
| `.fl-form-error-message` | 12px | lighter | Default | Form errors |

#### Navigation
| Element | Font Size | Font Weight | Usage |
|---------|-----------|-------------|-------|
| `.dashboard__nav-item-text` | Inherited | normal / bold | Navigation labels |
| `.dashboard__nav-category` | Inherited | normal | Category labels |
| Desktop nav links | 16px (implied) | normal | Main navigation |
| Mobile nav links (≤950px) | 14px | normal | Mobile navigation |

#### Icons
| Element | Font Size | Usage |
|---------|-----------|-------|
| `.st-icon-this-week` | 28px | Feature icons |
| `.dashboard__nav-item-icon.st-icon-training-room` (primary) | 26px | Primary nav training room |
| `.dashboard__nav-secondary .dashboard__nav-item-icon.st-icon-training-room` | 20px | Secondary nav training room |
| `.dashboard__nav-item-icon.st-icon-stacked-profits` | 40px | Stacked profits feature |
| `.fl-icon i` | 28px | Beaver Builder icons |
| `.fl-button i` | 1.3em | Button icons (relative) |
| `.mfp-preloader.fa` | 30px | Loading spinner |
| `.product-sidebar-inner .video-thumb .video-play-icon:after` | 17px | Video play button |

---

## Color Palette

### Primary Colors
```css
/* Brand Colors */
--primary-orange: #ff8c00;
--primary-orange-hover: #dc7309;
--primary-blue: #0984ae;
--dark-blue: #0a2335;
--dark-blue-hover: #0a2436;

/* Neutral Colors */
--white: #ffffff;
--black: #000000;
--gray-dark: #666666;
--gray-medium: #5D5D5D;
--gray-light: #f5f5f5;
--gray-lighter: #F0F0F0;
--gray-border: #e6e6e6;
--gray-hover: #e9ebed;

/* Semantic Colors */
--error-color: #DD6420;
--success-color: (not defined)
--warning-color: (not defined)
```

### Background Colors
| Element | Background | Hover State |
|---------|------------|-------------|
| `.primary-btn` | `#ff8c00` | `#dc7309` |
| `.menu-item-cart` | Default | `#0a2436` |
| `.sub-menu li` | Default | `#e9ebed` |
| `.fl-photo-caption-hover` | `rgba(0,0,0,0.7)` | N/A |
| `.fl-builder-pagination li span.current` | `#f5f5f5` | N/A |

### Text Colors
| Element | Color | Hover State |
|---------|-------|-------------|
| Body text | Default (black) | N/A |
| `.fl-photo-caption-hover` | `#fff` | N/A |
| `.fl-form-error-message` | `#DD6420` | N/A |
| `.dashboard__nav-primary.is-collapsed .dash_main_links .dashboard__nav-item-text` | `#0984ae` | N/A |

---

## Hover Effects & Transitions

### Navigation Hover Effects
```css
/* Menu Item Cart Hover */
li.menu-item-cart:hover,
.menu-item-cart a:hover {
    background: #0a2436 !important;
}

/* Sub-menu Hover */
.sub-menu li:hover,
.sub-menu li:active {
    background: #e9ebed;
}

/* Icon Hover */
.fl-node-5b72f8ed22a5c .fl-icon i:hover,
.fl-node-5b72f8ed22a5c .fl-icon i:hover:before,
.fl-node-5b72f8ed22a5c .fl-icon a:hover i,
.fl-node-5b72f8ed22a5c .fl-icon a:hover i:before {
    color: #666666;
}
```

### Button Hover Effects
```css
/* Primary Button Hover */
.primary-btn {
    background-color: #ff8c00;
    transition: all .2s ease-in-out;
}

.primary-btn:hover {
    background-color: #dc7309;
}

/* Squared Button Hover */
.squared-btn {
    transition: all .2s ease-in-out;
}

/* Beaver Builder Button Hover */
.fl-builder-content .fl-button:hover {
    text-decoration: none;
}
```

### Photo Caption Hover
```css
.fl-photo-caption-hover {
    background: rgba(0,0,0,0.7);
    bottom: 0;
    color: #fff;
    left: 0;
    opacity: 0;
    filter: alpha(opacity = 0);
    padding: 10px 15px;
    position: absolute;
    right: 0;
    -webkit-transition: opacity 0.3s ease-in;
    -moz-transition: opacity 0.3s ease-in;
    transition: opacity 0.3s ease-in;
}

.fl-photo-content:hover .fl-photo-caption-hover {
    opacity: 100;
    filter: alpha(opacity = 100);
}
```

### Pagination Hover
```css
.fl-builder-pagination li a.page-numbers:hover,
.fl-builder-pagination li span.current {
    background: #f5f5f5;
    text-decoration: none;
}
```

---

## Animations

### Button Icon Animation
```css
/* Icon Animation on Button Hover */
.fl-button.fl-button-icon-animation i {
    width: 0 !important;
    opacity: 0;
    -ms-filter: "alpha(opacity=0)";
    transition: all 0.2s ease-out;
    -webkit-transition: all 0.2s ease-out;
}

.fl-button.fl-button-icon-animation:hover i {
    opacity: 1 !important;
    -ms-filter: "alpha(opacity=100)";
}

/* Icon After (Right Side) */
.fl-button.fl-button-icon-animation i.fl-button-icon-after {
    margin-left: 0px !important;
}

.fl-button.fl-button-icon-animation:hover i.fl-button-icon-after {
    margin-left: 10px !important;
}

/* Icon Before (Left Side) */
.fl-button.fl-button-icon-animation i.fl-button-icon-before {
    margin-right: 0 !important;
}

.fl-button.fl-button-icon-animation:hover i.fl-button-icon-before {
    margin-right: 20px !important;
    margin-left: -10px;
}
```

### Animation Base Classes
```css
/* Animation Setup */
.fl-animation {
    opacity: 0;
}

.fl-builder-preview .fl-animation,
.fl-builder-edit .fl-animation,
.fl-animated {
    opacity: 1;
}

.fl-animated {
    animation-fill-mode: both;
    -webkit-animation-fill-mode: both;
}
```

---

## Dashboard Components

### Dashboard Structure
```css
/* Main Dashboard Container */
.dashboard {
    /* Container for entire dashboard */
}

/* Sidebar */
.dashboard__sidebar {
    /* Left sidebar navigation */
}

/* Header */
.dashboard__header {
    justify-content: space-between;
}

/* Content Areas */
.dashboard__content-sidebar {
    /* Right sidebar content */
}

.dashboard__content-main {
    /* Main content area */
}

.dashboard__content-section {
    /* Content sections */
}
```

### Dashboard Navigation
```css
/* Primary Navigation */
.dashboard__nav-primary {
    /* Main navigation container */
}

.dashboard__nav-primary.is-collapsed .dash_main_links .dashboard__nav-item-text {
    color: #0984ae !important;
}

/* Secondary Navigation */
.dashboard__nav-secondary {
    /* Secondary navigation container */
}

.dashboard__nav-secondary .dashboard__nav-submenu {
    z-index: 110 !important;
}

/* Navigation Items */
.dashboard__nav-item-icon {
    /* Icon container */
}

.dashboard__nav-item-text {
    /* Text label */
}

/* Navigation Categories */
.dashboard__nav-category {
    /* Category labels (e.g., "memberships", "mastery") */
}
```

### Profile Navigation
```css
.dashboard__profile-nav-item {
    /* Profile link in navigation */
}

.dashboard__profile-photo {
    /* Profile photo (background-image) */
}

.dashboard__profile-name {
    /* Profile name display */
}
```

### Page-Specific Styles
```css
/* Hide Main Dashboard Sidebar */
.page-id-401190 .dashboard__content-sidebar {
    display: none;
}

/* Member Dashboard Page Title */
h1.dashboard__page-title {
    font-weight: 700;
}

/* Foundation Dashboard Minimum Height */
.page-id-1390136 .dashboard__content-section,
.parent-pageid-1390136 .dashboard__content-section {
    min-height: 825px !important;
}

/* Hide Header on Specific Pages */
.page-id-1390136 .dashboard__header,
.parent-pageid-1390136 .dashboard__header {
    display: none;
}
```

---

## Navigation System

### Navigation Icon Specifications

#### Icon Dimensions & Positioning
```css
/* Standard Navigation Icon Pattern */
.{nav-item}-nav-item a:before {
    content: url(https://cloud-streaming.s3.amazonaws.com/inc/SVG/{icon-name}.svg);
    width: 18px;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

#### Specific Navigation Icons

**Foundation Icon**
```css
.foundation-nav-item a:before {
    content: url(https://cloud-streaming.s3.amazonaws.com/inc/SVG/foundation-icon.svg);
    width: 18px;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

**Mastery Icon**
```css
.mastery-nav-item a:before {
    content: url(https://cloud-streaming.s3.amazonaws.com/inc/SVG/mastery-icon.svg);
    width: 18px;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

**Options Icon**
```css
.options-nav-item a:before {
    content: url(https://cloud-streaming.s3.amazonaws.com/inc/SVG/options-icon.svg);
    width: 18px;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

**Futures Icon**
```css
.futures-nav-item a:before {
    content: url(https://cloud-streaming.s3.amazonaws.com/inc/SVG/futures-icon.svg);
    width: 18px;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

**Fibonacci Icon**
```css
.fibonacci-nav-item a:before {
    content: url(https://cloud-streaming.s3.amazonaws.com/inc/SVG/fibonacci-icon.svg);
    width: 18px;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

**Premium Newsletter Icon** (Background Image)
```css
.premium-newsletter-nav-item a:before {
    content: '';
    background-image: url(https://cdn.simplertrading.com/images/icons/premium-newsletter-icon.svg);
    background-size: contain;
    background-position: center;
    width: 18px;
    height: 50px;
    background-repeat: no-repeat;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

**Day Trading Room Icon** (Background Image)
```css
.day-nav-item a:before {
    content: '';
    background-image: url(https://cdn.simplertrading.com/images/icons/simpler-day-trading-room-icon.svg);
    background-size: 100%;
    background-position: center;
    width: 18px;
    height: 50px;
    background-repeat: no-repeat;
    position: relative;
    float: left;
    line-height: 54px;
    vertical-align: bottom;
    margin-right: 5px;
}
```

### Sub-menu Styling
```css
.sub-menu li > a {
    padding: 0px 0px !important;
    text-align: left;
}

.sub-menu li:hover,
.sub-menu li:active {
    background: #e9ebed;
}
```

### Menu Item Cart
```css
.menu-item-cart {
    min-width: auto !important;
    max-width: 50px !important;
}

li.menu-item-cart:hover,
.menu-item-cart a:hover {
    background: #0a2436 !important;
}
```

---

## Button Specifications

### Primary Button (Rounded)
```css
.primary-btn {
    display: block;
    border-radius: 25px;
    width: 100%;
    font-weight: 800;
    font-size: 18px;
    text-transform: uppercase;
    padding: 10px 20px;
    letter-spacing: 1.125px;
    transition: all .2s ease-in-out;
    background-color: #ff8c00;
    color: #fff;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
}

.primary-btn:hover {
    background-color: #dc7309;
}
```

**Visual Specifications:**
- **Border Radius:** 25px (fully rounded)
- **Font Weight:** 800 (Extra Bold)
- **Font Size:** 18px
- **Text Transform:** UPPERCASE
- **Padding:** 10px vertical, 20px horizontal
- **Letter Spacing:** 1.125px
- **Transition:** all .2s ease-in-out
- **Box Shadow:** 0 2px 5px rgba(0, 0, 0, 0.16)

### Secondary Button (Squared)
```css
.squared-btn {
    display: block;
    border-radius: 4px;
    width: 100%;
    font-weight: 800;
    font-size: 14px;
    text-transform: uppercase;
    padding: 10px 20px;
    letter-spacing: 1.125px;
    transition: all .2s ease-in-out;
}
```

**Visual Specifications:**
- **Border Radius:** 4px (slightly rounded corners)
- **Font Weight:** 800 (Extra Bold)
- **Font Size:** 14px (22% smaller than primary)
- **Text Transform:** UPPERCASE
- **Padding:** 10px vertical, 20px horizontal
- **Letter Spacing:** 1.125px
- **Transition:** all .2s ease-in-out

### Beaver Builder Button
```css
.fl-button {
    border: none;
    -moz-border-radius: 4px;
    -webkit-border-radius: 4px;
    display: inline-block;
    font-size: 16px;
    font-weight: normal;
    line-height: 18px;
    padding: 12px 24px;
    text-decoration: none;
    text-shadow: none;
}

.fl-builder-content .fl-button:hover {
    text-decoration: none;
}
```

**Visual Specifications:**
- **Border Radius:** 4px
- **Font Size:** 16px
- **Font Weight:** normal (400)
- **Line Height:** 18px
- **Padding:** 12px vertical, 24px horizontal
- **Display:** inline-block

### WordPress Block Button
```css
.wp-block-button__link {
    color: #fff;
    background-color: #32373c;
    border-radius: 9999px;
    box-shadow: none;
    text-decoration: none;
    padding: calc(.667em + 2px) calc(1.333em + 2px);
    font-size: 1.125em;
}
```

**Visual Specifications:**
- **Border Radius:** 9999px (fully rounded)
- **Font Size:** 1.125em (18px if base is 16px)
- **Padding:** calc(.667em + 2px) vertical, calc(1.333em + 2px) horizontal
- **Background:** #32373c (dark gray)
- **Color:** #fff (white text)

---

## Icon System

### Dashboard Icon Sizes
| Icon Class | Font Size | Context |
|------------|-----------|---------|
| `.st-icon-home` | Inherited | Home/Dashboard |
| `.st-icon-courses` | Inherited | My Classes |
| `.st-icon-indicators` | Inherited | My Indicators |
| `.st-icon-mastering-the-trade` | Inherited | Mastering the Trade |
| `.st-icon-simpler-showcase` | Inherited | Simpler Showcase |
| `.st-icon-tr3ndy-spx-alerts-circle` | Inherited | Tr3ndy SPX Alerts |
| `.st-icon-consistent-growth` | Inherited | Compounding Growth Mastery |
| `.st-icon-trade-of-the-week` | Inherited | Weekly Watchlist |
| `.st-icon-support` | Inherited | Support |
| `.st-icon-training-room` (primary) | 26px | Training Room (primary nav) |
| `.st-icon-training-room` (secondary) | 20px | Training Room (secondary nav) |
| `.st-icon-stacked-profits` | 40px | Stacked Profits feature |
| `.st-icon-this-week` | 28px | This Week feature |

### Beaver Builder Icons
```css
.fl-node-5b72f8ed22a5c .fl-icon i,
.fl-node-5b72f8ed22a5c .fl-icon i:before {
    font-size: 28px;
}

/* Responsive - Same size across all breakpoints */
@media(max-width: 992px) {
    .fl-node-5b72f8ed22a5c .fl-icon i,
    .fl-node-5b72f8ed22a5c .fl-icon i:before {
        font-size: 28px;
    }
}

@media(max-width: 768px) {
    .fl-node-5b72f8ed22a5c .fl-icon i,
    .fl-node-5b72f8ed22a5c .fl-icon i:before {
        font-size: 28px;
    }
}
```

### Button Icons
```css
.fl-builder-content .fl-button i {
    font-size: 1.3em; /* Relative to button font size */
    height: auto;
    margin-right: 8px;
    vertical-align: middle;
}
```

---

## Responsive Breakpoints

### Complete Breakpoint System

The dashboard uses a comprehensive 7-breakpoint responsive system optimized for all device sizes from mobile to large desktop displays.

| Breakpoint | Width | Device Target | Primary Changes |
|------------|-------|---------------|-----------------|
| **XXL Desktop** | Default | Large monitors (>1300px) | Full desktop experience |
| **XL Desktop** | ≤1300px | Standard desktop | Submenu positioning |
| **Large Desktop** | ≤1170px | Small desktop | Logo margin adjustments |
| **Desktop** | ≤1120px | Laptop | Menu width adjustments |
| **Tablet Landscape** | ≤992px | iPad landscape | Column widths, Beaver Builder layouts |
| **Tablet Portrait** | ≤950px | iPad portrait | **Navigation font size reduction** |
| **Mobile** | ≤768px | Phones | **Mobile-optimized layouts** |
| **Small Mobile** | ≤641px | Small phones | Navigation color adjustments |
| **Extra Small** | ≤600px | Compact phones | Hero image hiding |
| **Tiny Mobile** | ≤567px | Very small screens | Footer logo centering |

---

### Detailed Breakpoint Specifications

#### 1. Desktop Breakpoint (≤1300px)
**Purpose:** Submenu positioning optimization for standard desktop screens

```css
/*
===============================================================================
LARGE DESKTOP BREAKPOINT - 1300px
===============================================================================
ARCHITECTURAL OVERVIEW:
Intermediate responsive breakpoint system optimizing layout for
desktop and large tablet devices with adaptive menu systems.

BREAKPOINT HIERARCHY:
- Large Desktop: 1300px maximum width
- Progressive Enhancement: Feature adaptation based on available space
- Layout Optimization: Content-driven sizing strategies

MENU SYSTEM ADAPTATIONS:
- Submenu Positioning: Right-aligned for last menu items
- Layout Flexibility: Responsive to content length variations
- Visual Consistency: Maintained design integrity across sizes
===============================================================================
*/

@media (max-width: 1300px) {
    .main-navigation li:last-child .sub-menu {
        right: 0 !important;
        left: auto !important;
    }
}
```

**Changes:**
- Sub-menu positioning adjusted to prevent overflow
- Last child menu items align right instead of left

---

#### 2. Large Desktop Breakpoint (≤1170px)
**Purpose:** Logo and navigation spacing optimization

```css
@media (max-width: 1170px) {
    .main-navigation .navigation-logo.site-logo {
        margin-left: 0;
    }
    
    body.sticky-menu-logo.nav-float-left .main-navigation .site-logo.navigation-logo {
        margin-right: 0;
    }
}
```

**Changes:**
- Logo margin removed for better spacing
- Sticky menu logo margins reset

---

#### 3. Desktop Breakpoint (≤1120px)
**Purpose:** Menu item width flexibility

```css
/*
===============================================================================
INTERMEDIATE BREAKPOINT SYSTEM - DESKTOP OPTIMIZATION
===============================================================================
ARCHITECTURAL OVERVIEW:
Intermediate responsive breakpoint system optimizing layout for
desktop and large tablet devices with adaptive menu systems.

BREAKPOINT HIERARCHY:
- Small Desktop: 1120px maximum width
- Progressive Enhancement: Feature adaptation based on available space
- Layout Optimization: Content-driven sizing strategies

MENU SYSTEM ADAPTATIONS:
- Homepage Menu 2020: Flexible width removal for content adaptation
- Menu Items: Auto-width based on content requirements
- Layout Flexibility: Responsive to content length variations
- Visual Consistency: Maintained design integrity across sizes
===============================================================================
*/

@media (max-width: 1120px) {
    #menu-homepage-2020 li {
        min-width: 0 !important;
    }
}
```

**Changes:**
- Menu items no longer have minimum width constraints
- Allows content-based sizing for better fit

---

#### 4. Tablet Landscape Breakpoint (≤992px)
**Purpose:** Beaver Builder layout optimization and column adjustments

```css
/*
===============================================================================
TABLET LANDSCAPE BREAKPOINT - 992px
===============================================================================
RESPONSIVE UTILITIES:
- Column width adjustments for tablet landscape
- Beaver Builder module spacing
- Icon sizing consistency
- Separator adjustments
===============================================================================
*/

@media (max-width: 992px) {
    /* Column Width Adjustments */
    .fl-builder-content .fl-node-5b6c661a8e13a {
        width: 26% !important;
        max-width: none;
        clear: none;
        float: left;
    }
    
    .fl-builder-content .fl-node-5b6c661a8e176 {
        width: 25% !important;
        max-width: none;
        clear: none;
        float: left;
    }
    
    .fl-builder-content .fl-node-5b6c661a8e1b2 {
        width: 25% !important;
        max-width: none;
        clear: none;
        float: left;
    }
    
    .fl-builder-content .fl-node-5b6c661a8e1ec {
        width: 24% !important;
        max-width: none;
        clear: none;
        float: left;
    }
    
    /* Icon Sizing - Consistent 28px */
    .fl-node-5b72f8ed22a5c .fl-icon i,
    .fl-node-5b72f8ed22a5c .fl-icon i:before {
        font-size: 28px;
    }
    
    .fl-node-5b72f8ed22a5c .fl-icon-wrap .fl-icon-text {
        height: 49px;
    }
    
    /* Module Margin Adjustments */
    .fl-node-59adb48fc0c6a.fl-module > .fl-module-content {
        margin-right: 0px;
        margin-left: 0px;
    }
    
    .fl-node-5b6c66514654a.fl-module > .fl-module-content {
        margin-top: 0px;
    }
    
    .fl-node-5b6c693409ba7.fl-module > .fl-module-content {
        margin-top: 0px;
    }
}
```

**Changes:**
- 4-column grid system (26%, 25%, 25%, 24%)
- Icon sizes maintained at 28px
- Module margins reset for tighter spacing
- Separator adjustments for tablet layout

---

#### 5. Tablet Portrait Breakpoint (≤950px) 🔴 **CRITICAL MOBILE TRANSITION**
**Purpose:** Primary mobile navigation optimization

```css
/*
===============================================================================
MOBILE RESPONSIVE DESIGN SYSTEM - TABLET AND MOBILE OPTIMIZATION
===============================================================================
ARCHITECTURAL OVERVIEW:
Comprehensive mobile responsive design system optimizing user experience
for tablet and mobile devices with adaptive layouts and touch interactions.

MOBILE BREAKPOINT STRATEGY:
- Tablet Breakpoint: 950px maximum width
- Mobile Optimization: Touch-friendly interface elements
- Progressive Enhancement: Feature adaptation based on screen size
- Performance Optimization: Reduced resource usage for mobile devices

CART SYSTEM OPTIMIZATION:
- Cart Item: Max-width removal for flexible layout
- Touch Targets: Optimized for finger interaction
- Layout Adaptation: Content-based sizing
- Visual Hierarchy: Prioritized mobile experience

NAVIGATION ADAPTATIONS:
- Font Size: 14px for improved mobile readability
- Padding: 19px horizontal padding for touch targets
- Menu Items: Optimized spacing for mobile interaction
- Sub-menu: Left-aligned text for mobile consistency

USER EXPERIENCE FEATURES:
- Touch-Friendly: 44px minimum touch targets
- Readability: Optimized font sizes and spacing
- Navigation: Streamlined mobile menu structure
- Performance: Optimized rendering for mobile processors
===============================================================================
*/

@media (max-width: 950px) {
    /* Navigation Title Size Reduction */
    .navigation-branding .main-title {
        font-size: 30px; /* Down from 50px - 40% reduction */
    }
    
    /* Cart Item Flexibility */
    .menu-item-cart {
        max-width: none !important;
    }
    
    /* Navigation Link Optimization */
    .main-navigation ul > li > a {
        font-size: 14px; /* Down from 16px - 12.5% reduction */
        padding: 0 19px !important; /* Increased horizontal padding for touch */
    }
    
    /* Sub-menu Alignment */
    .sub-menu li a {
        text-align: left;
    }
}
```

**Changes:**
- **Navigation title:** 50px → 30px (40% reduction)
- **Navigation links:** 16px → 14px (12.5% reduction)
- **Touch targets:** 19px horizontal padding for easier tapping
- **Cart max-width:** Removed for flexible layout
- **Sub-menu:** Left-aligned for mobile consistency

---

#### 6. Mobile Breakpoint (≤768px) 🔴 **FULL MOBILE OPTIMIZATION**
**Purpose:** Complete mobile layout transformation

```css
/*
===============================================================================
MOBILE BREAKPOINT - 768px
===============================================================================
RESPONSIVE UTILITIES:
- Full mobile layout transformation
- Single column layouts
- 50% width for paired elements
- Text alignment adjustments
- Module spacing optimization
===============================================================================
*/

@media (max-width: 768px) {
    /* Column Layout - Full Width */
    .fl-builder-content .fl-node-5b6c661a8e13a {
        width: 100% !important; /* Full width on mobile */
        max-width: none;
        clear: none;
        float: left;
    }
    
    /* Column Layout - 50% Width (Paired) */
    .fl-builder-content .fl-node-5b6c661a8e176 {
        width: 50% !important; /* Half width for side-by-side */
        max-width: none;
        clear: none;
        float: left;
    }
    
    .fl-builder-content .fl-node-5b6c661a8e1b2 {
        width: 50% !important; /* Half width for side-by-side */
        max-width: none;
        clear: none;
        float: left;
    }
    
    /* Column Layout - Full Width */
    .fl-builder-content .fl-node-5b6c661a8e1ec {
        width: 100% !important; /* Full width on mobile */
        max-width: none;
        clear: none;
        float: left;
    }
    
    /* Icon Sizing - Maintained */
    .fl-node-5b72f8ed22a5c .fl-icon i,
    .fl-node-5b72f8ed22a5c .fl-icon i:before {
        font-size: 28px; /* Consistent with tablet */
    }
    
    .fl-node-5b72f8ed22a5c .fl-icon-wrap .fl-icon-text {
        height: 49px;
    }
    
    /* Text Alignment - Center */
    .fl-builder-content .fl-node-5b6c66888faf0 .fl-rich-text,
    .fl-builder-content .fl-node-5b6c66888faf0 .fl-rich-text *:not(b, strong) {
        text-align: center;
    }
    
    .fl-builder-content .fl-node-5b6c66514654a .fl-rich-text,
    .fl-builder-content .fl-node-5b6c66514654a .fl-rich-text *:not(b, strong) {
        text-align: center;
    }
    
    .fl-builder-content .fl-node-5b6c693409ba7 .fl-rich-text,
    .fl-builder-content .fl-node-5b6c693409ba7 .fl-rich-text *:not(b, strong) {
        text-align: center;
    }
    
    .fl-builder-content .fl-node-5b6c6953e8ba2 .fl-rich-text,
    .fl-builder-content .fl-node-5b6c6953e8ba2 .fl-rich-text *:not(b, strong) {
        text-align: center;
    }
    
    /* Typography Reduction */
    .fl-builder-content .fl-node-59adb3c86fc1f .fl-rich-text,
    .fl-builder-content .fl-node-59adb3c86fc1f .fl-rich-text *:not(b, strong) {
        font-size: 12px; /* Down from 13px */
        line-height: 1.3; /* Down from 21px */
    }
    
    /* Module Margin Adjustments */
    .fl-node-5b6c66514654a > .fl-module-content {
        margin-left: 20px;
    }
    
    .fl-node-5b6c693409ba7.fl-module > .fl-module-content {
        margin-top: 0px;
    }
    
    .fl-node-5b6c6953e8ba2.fl-module > .fl-module-content {
        margin-top: 20px;
    }
    
    .fl-node-5b96b44e7af4e.fl-module > .fl-module-content {
        margin-right: 20px;
        margin-left: 20px;
    }
}
```

**Changes:**
- **Layout:** Mixed full-width (100%) and paired (50%) columns
- **Typography:** 13px → 12px for rich text
- **Line height:** 21px → 1.3 (relative)
- **Text alignment:** Centered for mobile readability
- **Module margins:** Adjusted for mobile spacing (20px standard)
- **Icon sizes:** Maintained at 28px for consistency

---

#### 7. Small Mobile Breakpoint (≤641px)
**Purpose:** Navigation color adjustments for compact phones

```css
@media (max-width: 641px) {
    .main-navigation .main-nav ul li[class*="current-menu-"] > a,
    .main-navigation .main-nav ul li > a {
        color: #191717 !important;
    }
}
```

**Changes:**
- Navigation link color changed to dark gray (#191717)
- Applies to current menu items and all links

---

#### 8. Extra Small Mobile Breakpoint (≤600px)
**Purpose:** Hero image optimization for very small screens

```css
@media only screen and (max-width: 600px) {
    .courses-hero--img.col-lg-6 {
        display: none; /* Hide hero images to save space */
    }
}
```

**Changes:**
- Hero images hidden to prioritize content
- Improves load time and screen real estate

---

#### 9. Tiny Mobile Breakpoint (≤567px)
**Purpose:** Footer logo centering for smallest screens

```css
@media (max-width: 567px) {
    #footer-logo a {
        text-align: center;
    }
}
```

**Changes:**
- Footer logo centered for better visual balance

---

### WooCommerce Mobile Styles

**Small Screen Stylesheet (≤768px)**
```html
<link rel='stylesheet' 
      href='woocommerce-smallscreen.css' 
      media='only screen and (max-width: 768px)' />
```

**Purpose:** WooCommerce-specific mobile optimizations for cart, checkout, and product pages

---

### Mobile-Specific Features

#### Touch Target Optimization
```css
/* Minimum 44px touch targets for mobile accessibility */
.main-navigation ul > li > a {
    padding: 0 19px !important; /* Horizontal padding for touch */
    min-height: 44px; /* WCAG AAA compliant */
}
```

#### Mobile Navigation Color
```css
/* High contrast for mobile readability */
.main-navigation .main-nav ul li > a {
    color: #191717 !important; /* Dark gray for readability */
}
```

#### Mobile Performance Optimizations
- **Image hiding:** Hero images hidden on small screens
- **Layout simplification:** Single column layouts
- **Font size reduction:** Smaller text for mobile screens
- **Margin optimization:** Tighter spacing for compact displays

---

## Specific Dashboard Sections

### Member Dashboard
- **Page ID:** 401190
- **Sidebar:** Hidden (`.dashboard__content-sidebar { display: none; }`)
- **Page Title:** Font weight 700

### Foundation Dashboard
- **Page ID:** 1390136
- **Content Section Min-Height:** 825px
- **Header:** Hidden
- **Sidebar:** Hidden

### Password Reset
```css
.woocommerce-lost-password .dashboard__main--gradient .dashboard__content-main .woocommerce p {
    color: #fff !important;
}
```

---

## Form Elements

### Form Error Messages
```css
.fl-form-error-message {
    color: #DD6420;
    display: none;
    padding-top: 8px;
    font-size: 12px;
    font-weight: lighter;
}
```

### Form Button States
```css
.fl-form-button-disabled {
    opacity: 0.5;
}
```

---

## Photo & Media Elements

### Photo Captions
```css
.fl-photo-caption {
    font-size: 13px;
    line-height: 18px;
    overflow: hidden;
    text-overflow: ellipsis;
}

.fl-photo-caption-below {
    padding-bottom: 20px;
    padding-top: 10px;
}
```

### Photo Hover Caption
```css
.fl-photo-caption-hover {
    background: rgba(0,0,0,0.7);
    bottom: 0;
    color: #fff;
    left: 0;
    opacity: 0;
    filter: alpha(opacity = 0);
    padding: 10px 15px;
    position: absolute;
    right: 0;
    -webkit-transition: opacity 0.3s ease-in;
    -moz-transition: opacity 0.3s ease-in;
    transition: opacity 0.3s ease-in;
}

.fl-photo-content:hover .fl-photo-caption-hover {
    opacity: 100;
    filter: alpha(opacity = 100);
}
```

### Video Play Icon
```css
.product-sidebar-inner .video-thumb .video-play-icon:after {
    content: "Click to Play";
    position: relative;
    font-size: 17px;
    margin: 0px -50px;
    text-shadow: 0 0 7px #000;
}
```

---

## Pagination

### Pagination Styles
```css
.fl-builder-pagination,
.fl-builder-pagination-load-more {
    padding: 40px 0;
}

.fl-builder-pagination ul.page-numbers {
    list-style: none;
    margin: 0;
    padding: 0;
    text-align: center;
}

.fl-builder-pagination li {
    display: inline-block;
    margin: 0;
}

.fl-builder-pagination li span.page-numbers {
    border: 1px solid #e6e6e6;
    display: inline-block;
    padding: 5px 10px;
    margin: 0 0 5px;
}

.fl-builder-pagination li a.page-numbers:hover,
.fl-builder-pagination li span.current {
    background: #f5f5f5;
    text-decoration: none;
}
```

---

## Modal & Lightbox

### Magnific Popup Styles
```css
/* Arrow Buttons */
.mfp-wrap button.mfp-arrow,
.mfp-wrap button.mfp-arrow:active,
.mfp-wrap button.mfp-arrow:hover,
.mfp-wrap button.mfp-arrow:focus {
    background: transparent !important;
    border: none !important;
    box-shadow: none !important;
    outline: none !important;
}

/* Close Button */
.mfp-wrap .mfp-close,
.mfp-wrap .mfp-close:active,
.mfp-wrap .mfp-close:hover,
.mfp-wrap .mfp-close:focus {
    background: transparent !important;
    border: none !important;
    box-shadow: none !important;
    outline: none !important;
}

/* Admin Bar Adjustment */
.admin-bar .mfp-wrap .mfp-close,
.admin-bar .mfp-wrap .mfp-close:active,
.admin-bar .mfp-wrap .mfp-close:hover,
.admin-bar .mfp-wrap .mfp-close:focus {
    top: 32px !important;
}

/* Preloader */
.mfp-wrap .mfp-preloader.fa {
    font-size: 30px;
}
```

---

## Rich Text Formatting

### Rich Text Styles
```css
.fl-builder-content .fl-rich-text {
    word-wrap: break-word;
}

.fl-builder-content .fl-rich-text * {
    max-width: 100%;
    word-wrap: break-word;
}

.fl-builder-content .fl-rich-text strong {
    font-weight: bold;
}
```

---

## Summary of Key Measurements

### Typography Scale
- **Extra Large:** 50px (desktop nav title)
- **Large:** 32px (h2)
- **Medium-Large:** 26px (h3)
- **Medium:** 18px (primary button)
- **Base:** 16px (body, FL button)
- **Small:** 14px (secondary button, mobile nav)
- **Extra Small:** 13px (captions)
- **Tiny:** 12px (form errors)

### Icon Scale
- **Extra Large:** 40px (stacked profits)
- **Large:** 30px (preloader)
- **Medium:** 28px (FL icons, this week)
- **Small-Medium:** 26px (training room primary)
- **Small:** 20px (training room secondary)
- **Extra Small:** 18px (navigation icons)

### Spacing Scale
- **Button Padding:** 10-12px vertical, 20-24px horizontal
- **Icon Margin:** 5px right
- **Section Padding:** 20-40px
- **Form Error Padding:** 8px top

### Border Radius Scale
- **Fully Rounded:** 9999px / 25px (pill shape)
- **Rounded:** 100% (circles)
- **Slightly Rounded:** 4px (standard buttons)

### Transition Timing
- **Fast:** 0.2s (buttons, icon animations)
- **Standard:** 0.3s (photo captions, opacity)

---

**End of Design Specifications**

This document provides comprehensive design specifications extracted from the DashboardHome implementation file. All measurements, colors, and effects are documented as they appear in the production code.
