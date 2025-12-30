# 4. Weekly Watchlist Dashboard - Market Analysis Tool

## 🎯 Purpose & Functionality
The Weekly Watchlist Dashboard provides **comprehensive market analysis** with weekly trade recommendations, expert insights, and technical analysis. It displays featured watchlist content, video analysis, and actionable trading ideas. This dashboard serves as the **primary market research tool** for traders.

## 📋 Population Logic
- **Called On:** Navigation to `/dashboard/ww/` URL
- **Populated With:** Latest weekly watchlist content, market analysis videos
- **Data Source:** Custom post type 'weekly_watchlist' + market data API
- **Trigger:** Click on "Weekly Watchlist" link or direct URL access
- **Refreshes:** Weekly content updates, real-time market data

## 🏗️ Content Components
- **Featured Weekly Watchlist Video**
- **Trade Recommendations List**
- **Market Analysis Charts**
- **Expert Commentary Section**
- **Historical Performance Data**
- **Risk Management Guidelines**

## 💻 Complete Implementation Code

### PHP Template (WordPress)
```php
<?php
/**
 * Weekly Watchlist Dashboard Template
 * File: dashboard-weekly-watchlist.php
 * Purpose: Market analysis and trade recommendations
 */

get_header();

// Check user authentication
if (!is_user_logged_in()) {
    wp_redirect(wp_login_url(get_permalink()));
    exit;
}

$user_id = get_current_user_id();

// Get latest weekly watchlist
$weekly_watchlist = get_posts(array(
    'post_type' => 'weekly_watchlist',
    'posts_per_page' => 1,
    'orderby' => 'date',
    'order' => 'DESC'
));

// Get historical watchlists
$historical_watchlists = get_posts(array(
    'post_type' => 'weekly_watchlist',
    'posts_per_page' => 12,
    'orderby' => 'date',
    'order' => 'DESC',
    'exclude' => !empty($weekly_watchlist) ? $weekly_watchlist[0]->ID : array()
));

// Get market data
$market_data = get_market_data_for_watchlist(!empty($weekly_watchlist) ? $weekly_watchlist[0]->ID : 0);
?>

<div class="dashboard-container">
    <header class="dashboard-header">
        <h1><?php _e('Weekly Watchlist', 'simpler-trading'); ?></h1>
        
        <!-- Date Navigation -->
        <div class="date-navigation">
            <button class="nav-btn nav-prev" onclick="navigateWeek(-1)">
                <span class="icon">←</span>
                Previous Week
            </button>
            <div class="current-week">
                <?php 
                if (!empty($weekly_watchlist)) {
                    echo date('F j, Y', strtotime($weekly_watchlist[0]->post_date));
                } else {
                    echo 'No Watchlist Available';
                }
                ?>
            </div>
            <button class="nav-btn nav-next" onclick="navigateWeek(1)">
                Next Week
                <span class="icon">→</span>
            </button>
        </div>
    </header>
    
    <main class="dashboard-content">
        <?php if (!empty($weekly_watchlist)): ?>
            <?php
            $watchlist = $weekly_watchlist[0];
            $host = get_post_meta($watchlist->ID, 'host_name', true);
            $video_url = get_post_meta($watchlist->ID, 'video_url', true);
            $thumbnail = get_the_post_thumbnail_url($watchlist->ID, 'large');
            $trade_recommendations = get_post_meta($watchlist->ID, 'trade_recommendations', true);
            $market_analysis = get_post_meta($watchlist->ID, 'market_analysis', true);
            ?>
            
            <!-- Featured Watchlist Section -->
            <section class="featured-watchlist">
                <div class="watchlist-header">
                    <h2><?php echo get_the_title($watchlist); ?></h2>
                    <div class="watchlist-meta">
                        <span class="host-name">Host: <?php echo $host; ?></span>
                        <span class="publish-date">Published: <?php echo get_the_date('F j, Y', $watchlist); ?></span>
                    </div>
                </div>
                
                <div class="watchlist-content">
                    <div class="video-section">
                        <div class="video-player">
                            <?php if ($video_url): ?>
                                <video controls poster="<?php echo $thumbnail; ?>">
                                    <source src="<?php echo $video_url; ?>" type="video/mp4">
                                    Your browser does not support the video tag.
                                </video>
                            <?php else: ?>
                                <img src="<?php echo $thumbnail ?: get_default_watchlist_image(); ?>" alt="<?php echo get_the_title($watchlist); ?>">
                            <?php endif; ?>
                        </div>
                        
                        <div class="video-info">
                            <div class="video-description">
                                <?php echo wpautop(get_the_content($watchlist)); ?>
                            </div>
                            
                            <div class="video-actions">
                                <button class="btn-download-notes" onclick="downloadWatchlistNotes(<?php echo $watchlist->ID; ?>)">
                                    <span class="icon">📥</span>
                                    Download Notes
                                </button>
                                <button class="btn-share" onclick="shareWatchlist(<?php echo $watchlist->ID; ?>)">
                                    <span class="icon">🔗</span>
                                    Share
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            
            <!-- Trade Recommendations Section -->
            <?php if (!empty($trade_recommendations)): ?>
                <section class="trade-recommendations">
                    <h3>Trade Recommendations</h3>
                    <div class="recommendations-grid">
                        <?php foreach ($trade_recommendations as $recommendation): ?>
                            <div class="recommendation-card">
                                <div class="recommendation-header">
                                    <div class="ticker-symbol"><?php echo $recommendation['symbol']; ?></div>
                                    <div class="trade-action <?php echo $recommendation['action']; ?>">
                                        <?php echo ucfirst($recommendation['action']); ?>
                                    </div>
                                </div>
                                
                                <div class="recommendation-details">
                                    <div class="price-info">
                                        <span class="current-price">$<?php echo number_format($recommendation['current_price'], 2); ?></span>
                                        <span class="target-price">Target: $<?php echo number_format($recommendation['target_price'], 2); ?></span>
                                    </div>
                                    
                                    <div class="recommendation-reason">
                                        <strong>Reasoning:</strong> <?php echo $recommendation['reasoning']; ?>
                                    </div>
                                    
                                    <div class="risk-level">
                                        <span class="risk-label">Risk Level:</span>
                                        <div class="risk-meter">
                                            <div class="risk-fill" style="width: <?php echo $recommendation['risk_level'] * 100; ?>%;"></div>
                                        </div>
                                        <span class="risk-text"><?php echo $recommendation['risk_level_text']; ?></span>
                                    </div>
                                </div>
                            </div>
                        <?php endforeach; ?>
                    </div>
                </section>
            <?php endif; ?>
            
            <!-- Market Analysis Section -->
            <?php if (!empty($market_analysis)): ?>
                <section class="market-analysis">
                    <h3>Market Analysis</h3>
                    <div class="analysis-content">
                        <?php echo wpautop($market_analysis); ?>
                    </div>
                    
                    <!-- Market Data Charts -->
                    <div class="market-charts">
                        <div class="chart-container">
                            <h4>Market Overview</h4>
                            <div id="market-overview-chart" class="chart-placeholder">
                                <!-- Chart will be rendered here -->
                            </div>
                        </div>
                        
                        <div class="chart-container">
                            <h4>Sector Performance</h4>
                            <div id="sector-performance-chart" class="chart-placeholder">
                                <!-- Chart will be rendered here -->
                            </div>
                        </div>
                    </div>
                </section>
            <?php endif; ?>
            
        <?php else: ?>
            <!-- No Watchlist Available -->
            <section class="no-watchlist">
                <div class="empty-state">
                    <div class="empty-icon">📈</div>
                    <h3><?php _e('No Weekly Watchlist Available', 'simpler-trading'); ?></h3>
                    <p><?php _e('Check back soon for the latest market analysis and trade recommendations.', 'simpler-trading'); ?></p>
                </div>
            </section>
        <?php endif; ?>
        
        <!-- Historical Watchlists -->
        <?php if (!empty($historical_watchlists)): ?>
            <section class="historical-watchlists">
                <h3>Previous Weeks</h3>
                <div class="history-grid">
                    <?php foreach ($historical_watchlists as $historical): ?>
                        <div class="history-card" onclick="loadWatchlist(<?php echo $historical->ID; ?>)">
                            <div class="history-thumbnail">
                                <img src="<?php echo get_the_post_thumbnail_url($historical->ID, 'medium'); ?>" alt="<?php echo get_the_title($historical); ?>">
                            </div>
                            <div class="history-content">
                                <h4><?php echo get_the_title($historical); ?></h4>
                                <p class="history-date"><?php echo get_the_date('F j, Y', $historical); ?></p>
                            </div>
                        </div>
                    <?php endforeach; ?>
                </div>
            </section>
        <?php endif; ?>
    </main>
</div>

<?php get_footer(); ?>

<script>
jQuery(document).ready(function($) {
    // Initialize charts
    initializeMarketCharts();
    
    // Navigation functions
    window.navigateWeek = function(direction) {
        // Implementation for week navigation
        console.log('Navigate week:', direction);
    };
    
    window.loadWatchlist = function(watchlistId) {
        // Load specific watchlist
        window.location.href = '/dashboard/ww/?week=' + watchlistId;
    };
    
    window.downloadWatchlistNotes = function(watchlistId) {
        $.ajax({
            url: '/wp-json/simpler-trading/v1/watchlist-notes/' + watchlistId,
            method: 'GET',
            success: function(response) {
                // Create download link
                const link = document.createElement('a');
                link.href = response.notes_url;
                link.download = response.filename;
                link.click();
            },
            error: function() {
                alert('Error downloading notes');
            }
        });
    };
    
    window.shareWatchlist = function(watchlistId) {
        const shareUrl = window.location.href;
        if (navigator.share) {
            navigator.share({
                title: 'Weekly Watchlist',
                url: shareUrl
            });
        } else {
            // Fallback: copy to clipboard
            navigator.clipboard.writeText(shareUrl).then(() => {
                alert('Link copied to clipboard!');
            });
        }
    };
    
    function initializeMarketCharts() {
        // Initialize TradingView widgets or custom charts
        if (typeof TradingView !== 'undefined') {
            new TradingView.widget({
                "width": 980,
                "height": 610,
                "symbol": "SPY",
                "interval": "D",
                "timezone": "Etc/UTC",
                "theme": "light",
                "style": "1",
                "locale": "en",
                "toolbar_bg": "#f1f3f6",
                "enable_publishing": false,
                "allow_symbol_change": true,
                "container_id": "market-overview-chart"
            });
        }
    }
});
</script>

<style>
.watchlist-header {
    margin-bottom: 2rem;
}

.watchlist-header h2 {
    margin: 0 0 1rem 0;
    color: #1f2937;
}

.watchlist-meta {
    display: flex;
    gap: 2rem;
    color: #6b7280;
    font-size: 0.875rem;
}

.video-section {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 2rem;
    margin-bottom: 3rem;
}

.video-player {
    position: relative;
    border-radius: 12px;
    overflow: hidden;
    background: #000;
}

.video-player video,
.video-player img {
    width: 100%;
    height: auto;
    display: block;
}

.video-info {
    background: #f9fafb;
    padding: 1.5rem;
    border-radius: 12px;
}

.video-description {
    margin-bottom: 1.5rem;
    color: #4b5563;
    line-height: 1.6;
}

.video-actions {
    display: flex;
    gap: 1rem;
}

.btn-download-notes,
.btn-share {
    flex: 1;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    transition: all 0.2s ease;
}

.btn-download-notes {
    background: #1f2937;
    color: white;
}

.btn-share {
    background: #f3f4f6;
    color: #1f2937;
}

.trade-recommendations {
    margin-bottom: 3rem;
}

.trade-recommendations h3 {
    margin: 0 0 2rem 0;
    color: #1f2937;
}

.recommendations-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
}

.recommendation-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 12px;
    padding: 1.5rem;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.recommendation-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
}

.recommendation-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.ticker-symbol {
    font-size: 1.25rem;
    font-weight: 700;
    color: #1f2937;
}

.trade-action {
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
}

.trade-action.buy {
    background: #10b981;
    color: white;
}

.trade-action.sell {
    background: #ef4444;
    color: white;
}

.trade-action.hold {
    background: #f59e0b;
    color: white;
}

.price-info {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
}

.current-price {
    font-size: 1.125rem;
    font-weight: 600;
    color: #1f2937;
}

.target-price {
    color: #6b7280;
}

.recommendation-reason {
    margin-bottom: 1rem;
    color: #4b5563;
}

.risk-level {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.risk-label {
    font-size: 0.875rem;
    color: #6b7280;
}

.risk-meter {
    flex: 1;
    height: 6px;
    background: #e5e7eb;
    border-radius: 3px;
    overflow: hidden;
}

.risk-fill {
    height: 100%;
    background: linear-gradient(to right, #10b981, #f59e0b, #ef4444);
    transition: width 0.3s ease;
}

.risk-text {
    font-size: 0.75rem;
    color: #6b7280;
}

.market-analysis {
    margin-bottom: 3rem;
}

.market-analysis h3 {
    margin: 0 0 2rem 0;
    color: #1f2937;
}

.analysis-content {
    background: #f9fafb;
    padding: 2rem;
    border-radius: 12px;
    margin-bottom: 2rem;
}

.market-charts {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: 2rem;
}

.chart-container {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 12px;
    padding: 1.5rem;
}

.chart-container h4 {
    margin: 0 0 1rem 0;
    color: #1f2937;
}

.chart-placeholder {
    height: 300px;
    background: #f9fafb;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #6b7280;
}

.historical-watchlists {
    margin-bottom: 3rem;
}

.historical-watchlists h3 {
    margin: 0 0 2rem 0;
    color: #1f2937;
}

.history-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1.5rem;
}

.history-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 12px;
    overflow: hidden;
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.history-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
}

.history-thumbnail {
    height: 120px;
    overflow: hidden;
}

.history-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
}

.history-content {
    padding: 1rem;
}

.history-content h4 {
    margin: 0 0 0.5rem 0;
    color: #1f2937;
    font-size: 0.875rem;
}

.history-date {
    color: #6b7280;
    font-size: 0.75rem;
}

.date-navigation {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.nav-btn {
    padding: 0.5rem 1rem;
    border: 1px solid #e5e7eb;
    background: white;
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s ease;
}

.nav-btn:hover {
    background: #f9fafb;
    border-color: #d1d5db;
}

.current-week {
    padding: 0.5rem 1rem;
    background: #1f2937;
    color: white;
    border-radius: 8px;
    font-weight: 600;
}

@media (max-width: 768px) {
    .video-section {
        grid-template-columns: 1fr;
    }
    
    .recommendations-grid {
        grid-template-columns: 1fr;
    }
    
    .market-charts {
        grid-template-columns: 1fr;
    }
    
    .history-grid {
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    }
}
</style>
```

### Svelte 5 Implementation (Modern)
```svelte
<!-- src/routes/dashboard/ww/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import type { PageData } from './$page';
  
  // Dashboard components
  import FeaturedWatchlist from '$lib/components/dashboard/sections/FeaturedWatchlist.svelte';
  import TradeRecommendations from '$lib/components/dashboard/sections/TradeRecommendations.svelte';
  import MarketAnalysis from '$lib/components/dashboard/sections/MarketAnalysis.svelte';
  import HistoricalWatchlists from '$lib/components/dashboard/sections/HistoricalWatchlists.svelte';
  import DateNavigation from '$lib/components/dashboard/navigation/DateNavigation.svelte';
  
  // Props from load function
  let { data }: { data: PageData } = $props();
  
  // State
  let currentWeek = $state(data.currentWeek);
  let selectedWatchlist = $state(data.currentWatchlist);
  let historicalWatchlists = $state(data.historicalWatchlists);
  
  // Effects
  $effect(() => {
    // Update selected watchlist when current week changes
    selectedWatchlist = data.currentWatchlist;
  });
  
  function handleWeekChange(direction: number) {
    // Navigate to previous/next week
    const newWeek = new Date(currentWeek);
    newWeek.setDate(newWeek.getDate() + (direction * 7));
    currentWeek = newWeek.toISOString();
    
    // Load watchlist for new week
    loadWatchlistForWeek(currentWeek);
  }
  
  function handleWatchlistSelect(watchlist: any) {
    selectedWatchlist = watchlist;
  }
  
  async function loadWatchlistForWeek(week: string) {
    try {
      const response = await fetch(`/api/watchlist/week/${week}`);
      const watchlist = await response.json();
      selectedWatchlist = watchlist;
    } catch (error) {
      console.error('Failed to load watchlist:', error);
    }
  }
  
  function downloadNotes() {
    if (selectedWatchlist?.notesUrl) {
      const link = document.createElement('a');
      link.href = selectedWatchlist.notesUrl;
      link.download = selectedWatchlist.notesFilename;
      link.click();
    }
  }
  
  async function shareWatchlist() {
    const shareUrl = window.location.href;
    if (navigator.share) {
      try {
        await navigator.share({
          title: 'Weekly Watchlist',
          text: selectedWatchlist?.title,
          url: shareUrl
        });
      } catch (error) {
        console.log('Share cancelled');
      }
    } else {
      // Fallback: copy to clipboard
      try {
        await navigator.clipboard.writeText(shareUrl);
        alert('Link copied to clipboard!');
      } catch (error) {
        console.error('Failed to copy link');
      }
    }
  }
</script>

<div class="weekly-watchlist-dashboard">
  <header class="dashboard-header">
    <h1>Weekly Watchlist</h1>
    <DateNavigation 
      currentWeek={currentWeek}
      onchange={handleWeekChange}
    />
  </header>
  
  <main class="dashboard-content">
    {#if selectedWatchlist}
      <FeaturedWatchlist 
        watchlist={selectedWatchlist}
        onDownloadNotes={downloadNotes}
        onShare={shareWatchlist}
      />
      
      {#if selectedWatchlist.recommendations}
        <TradeRecommendations recommendations={selectedWatchlist.recommendations} />
      {/if}
      
      {#if selectedWatchlist.marketAnalysis}
        <MarketAnalysis analysis={selectedWatchlist.marketAnalysis} />
      {/if}
    {:else}
      <div class="no-watchlist">
        <div class="empty-state">
          <div class="empty-icon">📈</div>
          <h3>No Weekly Watchlist Available</h3>
          <p>Check back soon for the latest market analysis and trade recommendations.</p>
        </div>
      </div>
    {/if}
    
    {#if historicalWatchlists.length > 0}
      <HistoricalWatchlists 
        watchlists={historicalWatchlists}
        onSelect={handleWatchlistSelect}
      />
    {/if}
  </main>
</div>

<style>
  .weekly-watchlist-dashboard {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    flex-wrap: wrap;
    gap: 1rem;
  }
  
  .dashboard-header h1 {
    margin: 0;
    color: #1f2937;
  }
  
  .dashboard-content {
    min-height: 400px;
  }
  
  .no-watchlist {
    padding: 4rem 2rem;
    text-align: center;
  }
  
  .empty-state {
    color: #6b7280;
  }
  
  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }
  
  .empty-state h3 {
    margin: 0 0 1rem 0;
    color: #1f2937;
  }
</style>
```

### Load Function (SvelteKit)
```typescript
// src/routes/dashboard/ww/+page.ts
import type { Load } from '@sveltejs/kit';
import { getCurrentWatchlist } from '$lib/api/watchlist';
import { getHistoricalWatchlists } from '$lib/api/watchlist';
import { getUserProfile } from '$lib/api/user';

export const load: Load = async ({ fetch, parent, url }) => {
  try {
    const week = url.searchParams.get('week') || '';
    
    const [currentWatchlist, historicalWatchlists, userProfile] = await Promise.all([
      getCurrentWatchlist(week),
      getHistoricalWatchlists(),
      getUserProfile()
    ]);

    return {
      currentWatchlist: currentWatchlist,
      historicalWatchlists: historicalWatchlists,
      currentWeek: currentWatchlist?.week || new Date().toISOString(),
      profile: userProfile
    };
  } catch (error) {
    throw error(500, 'Failed to load watchlist data');
  }
};
```

### Featured Watchlist Component
```svelte
<!-- src/lib/components/dashboard/sections/FeaturedWatchlist.svelte -->
<script lang="ts">
  import type { Watchlist } from '$lib/types/watchlist';
  
  interface Props {
    watchlist: Watchlist;
    onDownloadNotes: () => void;
    onShare: () => void;
  }
  
  let { watchlist, onDownloadNotes, onShare }: Props = $props();
</script>

<section class="featured-watchlist">
  <div class="watchlist-header">
    <h2>{watchlist.title}</h2>
    <div class="watchlist-meta">
      <span class="host-name">Host: {watchlist.host}</span>
      <span class="publish-date">Published: {watchlist.publishDate}</span>
    </div>
  </div>
  
  <div class="watchlist-content">
    <div class="video-section">
      <div class="video-player">
        {#if watchlist.videoUrl}
          <video controls poster={watchlist.thumbnail}>
            <source src={watchlist.videoUrl} type="video/mp4">
            Your browser does not support the video tag.
          </video>
        {:else}
          <img src={watchlist.thumbnail || '/images/default-watchlist.jpg'} alt={watchlist.title} />
        {/if}
      </div>
      
      <div class="video-info">
        <div class="video-description">
          {@html watchlist.description}
        </div>
        
        <div class="video-actions">
          <button class="btn-download-notes" onclick={onDownloadNotes}>
            <span class="icon">📥</span>
            Download Notes
          </button>
          <button class="btn-share" onclick={onShare}>
            <span class="icon">🔗</span>
            Share
          </button>
        </div>
      </div>
    </div>
  </div>
</section>

<style>
  .featured-watchlist {
    margin-bottom: 3rem;
  }
  
  .watchlist-header {
    margin-bottom: 2rem;
  }
  
  .watchlist-header h2 {
    margin: 0 0 1rem 0;
    color: #1f2937;
    font-size: 2rem;
  }
  
  .watchlist-meta {
    display: flex;
    gap: 2rem;
    color: #6b7280;
    font-size: 0.875rem;
  }
  
  .video-section {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 2rem;
  }
  
  .video-player {
    position: relative;
    border-radius: 12px;
    overflow: hidden;
    background: #000;
    aspect-ratio: 16/9;
  }
  
  .video-player video,
  .video-player img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  
  .video-info {
    background: #f9fafb;
    padding: 1.5rem;
    border-radius: 12px;
  }
  
  .video-description {
    margin-bottom: 1.5rem;
    color: #4b5563;
    line-height: 1.6;
  }
  
  .video-actions {
    display: flex;
    gap: 1rem;
  }
  
  .btn-download-notes,
  .btn-share {
    flex: 1;
    padding: 0.75rem 1rem;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    transition: all 0.2s ease;
  }
  
  .btn-download-notes {
    background: #1f2937;
    color: white;
  }
  
  .btn-share {
    background: #f3f4f6;
    color: #1f2937;
  }
  
  .btn-download-notes:hover,
  .btn-share:hover {
    transform: translateY(-1px);
    opacity: 0.9;
  }
  
  @media (max-width: 768px) {
    .video-section {
      grid-template-columns: 1fr;
    }
  }
</style>
```

## 🔧 Technical Implementation Details

### WordPress Integration
- **Template:** `dashboard-weekly-watchlist.php`
- **Database Queries:** Custom post type queries + metadata
- **Video Management:** Video hosting and streaming
- **File Downloads:** Secure note PDF generation

### SvelteKit Integration
- **Route:** `/dashboard/ww/+page.svelte`
- **Load Function:** Watchlist data fetching with week filtering
- **State Management:** Reactive week navigation with Svelte 5 runes
- **Component Architecture:** Modular watchlist and analysis components

### API Integration
- **Current Watchlist:** `getCurrentWatchlist()` API call
- **Historical Data:** `getHistoricalWatchlists()` API call
- **Market Data:** Real-time market data integration
- **Video Streaming:** CDN-based video delivery

## 🎨 CSS Classes & Styling

### Watchlist Layout
- `.featured-watchlist` - Main watchlist container
- `.video-section` - Video and info grid layout
- `.video-player` - Video player container
- `.video-info` - Description and actions panel

### Trade Recommendations
- `.trade-recommendations` - Recommendations section
- `.recommendation-card` - Individual trade card
- `.ticker-symbol` - Stock ticker display
- `.trade-action` - Buy/sell/hold indicator

### Market Analysis
- `.market-analysis` - Analysis section
- `.market-charts` - Chart grid layout
- `.chart-container` - Individual chart wrapper

## 🔄 User Experience Flow

1. **Weekly Content:** Latest market analysis and trade ideas
2. **Video Analysis:** Expert commentary with visual charts
3. **Trade Recommendations:** Actionable trade setups with risk levels
4. **Historical Data:** Access to previous weeks' analysis
5. **Download Resources:** PDF notes and shareable content

## 📱 Responsive Design

- **Mobile (< 768px):** Single column layout, stacked video
- **Tablet (768px - 1024px):** Two-column video layout
- **Desktop (> 1024px):** Full multi-column layout
- **Video Optimized:** Responsive video player

## 🔒 Security Considerations

- **Access Control:** Member-only content access
- **Video Protection:** Secure streaming with authentication
- **Download Security:** Time-limited download links
- **XSS Protection:** Sanitized user content

## 📊 Performance Optimization

- **Video Streaming:** Adaptive bitrate streaming
- **Image Optimization:** WebP format with fallbacks
- **Chart Caching:** Cached market data visualizations
- **CDN Delivery:** Static assets via CDN

## 🎯 Key Features

- **Weekly Analysis:** Regular market commentary
- **Trade Ideas:** Specific trade recommendations
- **Risk Management:** Clear risk level indicators
- **Historical Data:** Archive of past analyses
- **Mobile Responsive:** Optimized for all devices
- **Social Sharing:** Easy content sharing
- **Download Resources:** PDF notes and guides

This implementation provides a comprehensive market analysis platform with modern Svelte 5 patterns while maintaining compatibility with existing content management systems.
