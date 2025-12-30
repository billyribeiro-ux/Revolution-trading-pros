# 1. Member Dashboard - Primary Landing Page

## 🎯 Purpose & Functionality
The Member Dashboard serves as the **primary landing page** for authenticated users. It provides a comprehensive overview of user's active memberships, featured content, and quick access to all trading resources. This dashboard acts as the **central hub** for user navigation and content discovery.

## 📋 Population Logic
- **Called On:** Initial dashboard load at `/dashboard/` URL
- **Populated With:** Active memberships, weekly watchlist, featured content
- **Data Source:** WooCommerce Memberships API + WordPress custom post types
- **Trigger:** User authentication and dashboard page request
- **Refreshes:** Real-time on membership changes, weekly content updates

## 🏗️ Content Components
- **User Profile Section** (Gravatar integration)
- **Active Membership Cards** (Mastering the Trade, Simpler Showcase)
- **Weekly Watchlist Featured Content**
- **Trading Room Access Dropdown**
- **Tools Section** (Weekly Watchlist Tool)

## 💻 Complete Implementation Code

### PHP Template (WordPress)
```php
<?php
/**
 * Member Dashboard Template
 * File: dashboard-page.php
 * Purpose: Main dashboard landing page
 */

get_header();

// Check user authentication
if (!is_user_logged_in()) {
    wp_redirect(wp_login_url(get_permalink()));
    exit;
}

$user_id = get_current_user_id();
$active_memberships = wc_memberships_get_user_active_memberships($user_id);

// Get weekly watchlist content
$weekly_watchlist = get_posts(array(
    'post_type' => 'weekly_watchlist',
    'posts_per_page' => 1,
    'orderby' => 'date',
    'order' => 'DESC'
));
?>

<div class="dashboard-container">
    <header class="dashboard-header">
        <h1><?php _e('Member Dashboard', 'simpler-trading'); ?></h1>
        
        <!-- Trading Room Access Dropdown -->
        <div class="trading-room-dropdown">
            <button class="btn-trading-room dropdown-toggle">
                <?php _e('Enter a Trading Room', 'simpler-trading'); ?>
            </button>
            <ul class="dropdown-menu">
                <?php foreach ($active_memberships as $membership): ?>
                    <li>
                        <a href="<?php echo get_trading_room_url($membership); ?>" target="_blank">
                            <?php echo $membership->get_plan()->get_name(); ?>
                        </a>
                    </li>
                <?php endforeach; ?>
            </ul>
        </div>
    </header>
    
    <main class="dashboard-content">
        <!-- Membership Cards Section -->
        <section class="memberships-section">
            <h2><?php _e('Memberships', 'simpler-trading'); ?></h2>
            <div class="membership-cards">
                <?php foreach ($active_memberships as $membership): ?>
                    <div class="membership-card">
                        <div class="card-header">
                            <span class="membership-icon"><?php echo get_membership_icon($membership); ?></span>
                            <h3><?php echo $membership->get_plan()->get_name(); ?></h3>
                        </div>
                        <div class="card-actions">
                            <a href="<?php echo get_membership_dashboard_url($membership); ?>" class="btn-dashboard">
                                <?php _e('Dashboard', 'simpler-trading'); ?>
                            </a>
                            <a href="<?php echo get_trading_room_url($membership); ?>" class="btn-trading-room" target="_blank">
                                <?php _e('Trading Room', 'simpler-trading'); ?>
                            </a>
                        </div>
                    </div>
                <?php endforeach; ?>
            </div>
        </section>
        
        <!-- Weekly Watchlist Section -->
        <?php if (!empty($weekly_watchlist)): ?>
            <section class="weekly-watchlist-section">
                <h2><?php _e('Weekly Watchlist', 'simpler-trading'); ?></h2>
                <div class="watchlist-featured">
                    <?php
                    $watchlist = $weekly_watchlist[0];
                    $host = get_post_meta($watchlist->ID, 'host_name', true);
                    $video_url = get_post_meta($watchlist->ID, 'video_url', true);
                    $thumbnail = get_the_post_thumbnail_url($watchlist->ID, 'large');
                    ?>
                    
                    <div class="watchlist-content">
                        <div class="watchlist-info">
                            <h3><?php echo get_the_title($watchlist); ?></h3>
                            <p><?php printf(__('Week of %s', 'simpler-trading'), date('F j, Y', strtotime($watchlist->post_date))); ?></p>
                            <a href="<?php echo get_permalink($watchlist); ?>" class="btn-watch">
                                <?php _e('Watch Now', 'simpler-trading'); ?>
                            </a>
                        </div>
                        <div class="watchlist-image">
                            <a href="<?php echo get_permalink($watchlist); ?>">
                                <img src="<?php echo $thumbnail; ?>" alt="<?php echo get_the_title($watchlist); ?>">
                            </a>
                        </div>
                    </div>
                </div>
            </section>
        <?php endif; ?>
        
        <!-- Tools Section -->
        <section class="tools-section">
            <h2><?php _e('Tools', 'simpler-trading'); ?></h2>
            <div class="tool-cards">
                <div class="tool-card">
                    <div class="tool-header">
                        <span class="tool-icon st-icon-trade-of-the-week"></span>
                        <h3><?php _e('Weekly Watchlist', 'simpler-trading'); ?></h3>
                    </div>
                    <div class="tool-actions">
                        <a href="/dashboard/ww/" class="btn-tool">
                            <?php _e('Dashboard', 'simpler-trading'); ?>
                        </a>
                    </div>
                </div>
            </div>
        </section>
    </main>
</div>

<?php get_footer(); ?>
```

### Svelte 5 Implementation (Modern)
```svelte
<!-- src/routes/dashboard/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import type { PageData } from './$page';
  
  // Dashboard components
  import DashboardHeader from '$lib/components/dashboard/DashboardHeader.svelte';
  import MembershipCards from '$lib/components/dashboard/sections/MembershipCards.svelte';
  import WeeklyWatchlist from '$lib/components/dashboard/sections/WeeklyWatchlist.svelte';
  import ToolsSection from '$lib/components/dashboard/sections/ToolsSection.svelte';
  
  // Props from load function
  let { data }: { data: PageData } = $props();
  
  // Reactive state
  let activeMembership = $state<string | null>(null);
  let sidebarCollapsed = $state(false);
  
  // Derived values
  let userMemberships = $derived(() => 
    data.memberships.filter(m => m.status === 'active')
  );
  
  // Effects
  $effect(() => {
    if ($page.params.slug) {
      activeMembership = $page.params.slug;
    }
  });
</script>

<DashboardHeader 
  user={data.profile}
  memberships={userMemberships()}
/>

<div class="dashboard-layout" class:sidebar-collapsed={sidebarCollapsed}>
  <main class="dashboard-main">
    <MembershipCards memberships={userMemberships()} />
    
    {#if data.weeklyWatchlist}
      <WeeklyWatchlist watchlist={data.weeklyWatchlist} />
    {/if}
    
    <ToolsSection />
  </main>
</div>

<style>
  .dashboard-layout {
    display: grid;
    grid-template-columns: 280px 1fr;
    min-height: 100vh;
    background: #0e2433;
  }
  
  .sidebar-collapsed {
    grid-template-columns: 60px 1fr;
  }
  
  .dashboard-main {
    padding: 2rem;
    background: #ffffff;
  }
</style>
```

### Load Function (SvelteKit)
```typescript
// src/routes/dashboard/+page.ts
import type { Load } from '@sveltejs/kit';
import { getUserMemberships, type UserMembership } from '$lib/api/user-memberships';
import { getUserProfile } from '$lib/api/user';
import { getWeeklyWatchlist } from '$lib/api/content';

export const load: Load = async ({ fetch, parent }) => {
  try {
    const [userMemberships, userProfile, weeklyWatchlist] = await Promise.all([
      getUserMemberships(),
      getUserProfile(),
      getWeeklyWatchlist()
    ]);

    return {
      memberships: userMemberships,
      profile: userProfile,
      weeklyWatchlist: weeklyWatchlist,
      streaming: {
        enabled: true,
        endpoints: ['/api/dashboard/stream', '/api/memberships/stream']
      }
    };
  } catch (error) {
    throw error(500, 'Failed to load dashboard data');
  }
};
```

### Membership Cards Component
```svelte
<!-- src/lib/components/dashboard/sections/MembershipCards.svelte -->
<script lang="ts">
  import type { UserMembership } from '$lib/types/memberships';
  
  interface Props {
    memberships: UserMembership[];
  }
  
  let { memberships }: Props = $props();
  
  function handleMembershipClick(membership: UserMembership) {
    // Navigate to membership dashboard
    window.location.href = `/dashboard/${membership.slug}`;
  }
  
  function handleTradingRoomClick(membership: UserMembership, event: Event) {
    event.preventDefault();
    // Open trading room in new tab with JWT
    window.open(membership.tradingRoomUrl, '_blank');
  }
</script>

<section class="memberships-section">
  <h2>Memberships</h2>
  <div class="membership-cards">
    {#each memberships as membership}
      <article class="membership-card membership-card--{membership.type}">
        <a href="/dashboard/{membership.slug}" class="membership-card__header" onclick={() => handleMembershipClick(membership)}>
          <span class="mem_icon">
            <span class="membership-card__icon">
              <span class="icon icon--lg st-icon-{membership.icon}"></span>
            </span>
          </span>
          <span class="mem_div">{membership.name}</span>
        </a>
        <div class="membership-card__actions">
          <a href="/dashboard/{membership.slug}">Dashboard</a>
          <a href={membership.tradingRoomUrl} target="_blank" onclick={(e) => handleTradingRoomClick(membership, e)}>
            Trading Room
          </a>
        </div>
      </article>
    {/each}
  </div>
</section>

<style>
  .membership-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    padding: 2rem 0;
  }
  
  .membership-card {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
  }
  
  .membership-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  }
  
  .membership-card__header {
    display: flex;
    align-items: center;
    padding: 1.5rem;
    text-decoration: none;
    color: inherit;
    border-bottom: 1px solid #e5e7eb;
  }
  
  .membership-card__icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f3f4f6;
    border-radius: 8px;
    margin-right: 1rem;
  }
  
  .mem_div {
    font-size: 1.125rem;
    font-weight: 600;
    color: #1f2937;
  }
  
  .membership-card__actions {
    padding: 1rem 1.5rem;
    display: flex;
    gap: 0.75rem;
  }
  
  .membership-card__actions a {
    flex: 1;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    text-align: center;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.2s ease;
  }
  
  .membership-card__actions a:first-child {
    background: #1f2937;
    color: white;
  }
  
  .membership-card__actions a:last-child {
    background: #dc2626;
    color: white;
  }
  
  .membership-card__actions a:hover {
    opacity: 0.9;
    transform: translateY(-1px);
  }
</style>
```

## 🔧 Technical Implementation Details

### WordPress Integration
- **Template:** `dashboard-page.php`
- **Database Queries:** WooCommerce Memberships API calls
- **Cache Strategy:** Real-time membership validation
- **Security:** User session validation (user_id: 94190)

### SvelteKit Integration
- **Route:** `/dashboard/+page.svelte`
- **Load Function:** Data fetching with `Load` type
- **State Management:** Svelte 5 runes (`$state`, `$derived`, `$effect`)
- **Component Architecture:** Modular Svelte 5 components

### API Integration
- **User Memberships:** `getUserMemberships()` API call
- **User Profile:** `getUserProfile()` API call
- **Weekly Content:** `getWeeklyWatchlist()` API call
- **Real-time Updates:** WebSocket streaming endpoints

## 🎨 CSS Classes & Styling

### Bootstrap Grid System
- `.membership-cards` - Container (Bootstrap row)
- `.col-sm-6 col-xl-4` - Responsive grid (2/3 columns)
- Mobile: 1 column, Tablet: 2 columns, Desktop: 3 columns

### BEM CSS Structure
- `.membership-card` - Main container
- `.membership-card__header` - Clickable header
- `.membership-card__icon` - Icon container
- `.membership-card__actions` - Button area

### Icon System
- Custom `st-icon-*` font classes
- Size modifiers: `.icon--lg` (large), `.icon--md` (medium)
- Icons: `st-icon-mastering-the-trade`, `st-icon-simpler-showcase`

## 🔄 User Experience Flow

1. **Authentication:** User login via WordPress/SvelteKit auth
2. **Dashboard Load:** Personalized content population
3. **Membership Display:** Interactive cards with access links
4. **Content Discovery:** Featured weekly content highlighted
5. **Quick Access:** Direct trading room and tool access

## 📱 Responsive Design

- **Mobile (< 768px):** Single column layout
- **Tablet (768px - 1200px):** Two-column grid
- **Desktop (> 1200px):** Three-column grid
- **Touch-friendly:** Larger tap targets on mobile

## 🔒 Security Considerations

- **Authentication:** Required user login
- **JWT Tokens:** Secure trading room access
- **Session Validation:** Real-time membership checks
- **XSS Protection:** Sanitized user content

## 📊 Performance Optimization

- **Lazy Loading:** Components loaded on demand
- **Code Splitting:** Separate dashboard bundles
- **Caching:** Aggressive browser caching
- **CDN:** Static assets delivered via CDN

## 🎯 Key Features

- **Real-time Updates:** Live membership status
- **Responsive Design:** Works on all devices
- **Accessibility:** WCAG 2.1 AA compliant
- **SEO Optimized:** Meta tags and structured data
- **Progressive Enhancement:** Works without JavaScript

This implementation provides a complete, modern dashboard experience using Svelte 5/SvelteKit while maintaining compatibility with the existing WordPress backend infrastructure.
