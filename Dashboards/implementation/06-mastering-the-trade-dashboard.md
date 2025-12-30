# 6. Mastering the Trade Dashboard - Premium Trading Room

## 🎯 Purpose & Functionality
The Mastering the Trade Dashboard provides **premium members** with access to advanced trading education, live trading room access, premium video content, and sophisticated trading strategies. This dashboard is designed for **experienced traders** seeking advanced market analysis and professional-level trading insights.

## 📋 Population Logic
- **Called On:** Navigation to `/dashboard/mastering-the-trade` URL
- **Populated With:** Premium membership content, live trading room access, advanced education
- **Data Source:** WooCommerce Membership ID: 'mastering_the_trade' + custom post content
- **Trigger:** Click on navigation link or direct URL access
- **Refreshes:** Real-time trading room status, new content uploads, membership validity

## 🏗️ Content Components
- **Live Trading Room Access** (JWT authenticated)
- **Premium Video Library**
- **Advanced Trading Strategies**
- **Market Analysis Sessions**
- **Expert Trader Insights**
- **Premium Resource Downloads**
- **Community Forum Access**

## 💻 Complete Implementation Code

### PHP Template (WordPress)
```php
<?php
/**
 * Mastering the Trade Dashboard Template
 * File: dashboard-mastering-the-trade.php
 * Purpose: Premium trading membership dashboard
 */

get_header();

// Check user authentication and membership
if (!is_user_logged_in()) {
    wp_redirect(wp_login_url(get_permalink()));
    exit;
}

$user_id = get_current_user_id();

// Verify Mastering the Trade membership
if (!user_has_membership($user_id, 'mastering_the_trade')) {
    wp_redirect(home_url('/dashboard/'));
    exit;
}

// Get membership details
$membership = wc_memberships_get_user_membership($user_id, 'mastering_the_trade');
$membership_plan = $membership->get_plan();

// Get premium content
$premium_videos = get_premium_videos('mastering_the_trade');
$strategies = get_trading_strategies('mastering_the_trade');
$market_analysis = get_market_analysis('mastering_the_trade');

// Generate JWT token for trading room
$jwt_token = generate_trading_room_jwt($user_id, 'mastering_the_trade');
$trading_room_url = 'https://chat.protradingroom.com/ptr_app/sessions/v2/authUser/652754202ad80b3e7c5131e2?sl=1&jwt=' . $jwt_token;

?>

<div class="dashboard-container premium-dashboard">
    <header class="dashboard-header premium-header">
        <div class="dashboard__header-left">
            <h1 class="dashboard__page-title"><?php _e('Mastering the Trade', 'simpler-trading'); ?></h1>
            <div class="membership-badge premium">
                <span class="badge-icon">⭐</span>
                <span class="badge-text"><?php _e('Premium Member', 'simpler-trading'); ?></span>
            </div>
        </div>
        
        <div class="dashboard__header-right">
            <!-- Live Trading Room Access -->
            <div class="live-trading-room">
                <div class="live-indicator">
                    <span class="live-dot"></span>
                    <span class="live-text"><?php _e('LIVE NOW', 'simpler-trading'); ?></span>
                </div>
                <a href="<?php echo $trading_room_url; ?>" target="_blank" class="btn-trading-room-live">
                    <span class="btn-icon">📈</span>
                    <?php _e('Enter Trading Room', 'simpler-trading'); ?>
                </a>
            </div>
        </div>
    </header>
    
    <main class="dashboard-content premium-content">
        <!-- Quick Stats -->
        <section class="quick-stats">
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-value"><?php echo count($premium_videos); ?></div>
                    <div class="stat-label"><?php _e('Premium Videos', 'simpler-trading'); ?></div>
                </div>
                <div class="stat-card">
                    <div class="stat-value"><?php echo count($strategies); ?></div>
                    <div class="stat-label"><?php _e('Trading Strategies', 'simpler-trading'); ?></div>
                </div>
                <div class="stat-card">
                    <div class="stat-value"><?php echo get_trading_days_count(); ?></div>
                    <div class="stat-label"><?php _e('Trading Days', 'simpler-trading'); ?></div>
                </div>
                <div class="stat-card">
                    <div class="stat-value"><?php echo get_win_rate_percentage(); ?>%</div>
                    <div class="stat-label"><?php _e('Win Rate', 'simpler-trading'); ?></div>
                </div>
            </div>
        </section>
        
        <!-- Live Trading Room Section -->
        <section class="live-room-section">
            <h2><?php _e('Live Trading Room', 'simpler-trading'); ?></h2>
            <div class="trading-room-embed">
                <iframe src="<?php echo $trading_room_url; ?>" 
                        width="100%" 
                        height="600px" 
                        frameborder="0" 
                        allowfullscreen>
                </iframe>
            </div>
            <div class="trading-room-info">
                <div class="room-schedule">
                    <h3><?php _e('Today\'s Schedule', 'simpler-trading'); ?></h3>
                    <?php
                    $schedule = get_trading_room_schedule('mastering_the_trade');
                    foreach ($schedule as $session): ?>
                        <div class="schedule-item">
                            <span class="time"><?php echo $session['time']; ?></span>
                            <span class="session"><?php echo $session['session']; ?></span>
                            <span class="trader"><?php echo $session['trader']; ?></span>
                        </div>
                    <?php endforeach; ?>
                </div>
            </div>
        </section>
        
        <!-- Premium Video Library -->
        <section class="premium-videos">
            <h2><?php _e('Premium Video Library', 'simpler-trading'); ?></h2>
            <div class="video-filters">
                <select id="video-category" class="filter-select">
                    <option value="all"><?php _e('All Videos', 'simpler-trading'); ?></option>
                    <option value="strategy"><?php _e('Strategy', 'simpler-trading'); ?></option>
                    <option value="analysis"><?php _e('Market Analysis', 'simpler-trading'); ?></option>
                    <option value="education"><?php _e('Education', 'simpler-trading'); ?></option>
                </select>
                <select id="video-difficulty" class="filter-select">
                    <option value="all"><?php _e('All Levels', 'simpler-trading'); ?></option>
                    <option value="beginner"><?php _e('Beginner', 'simpler-trading'); ?></option>
                    <option value="intermediate"><?php _e('Intermediate', 'simpler-trading'); ?></option>
                    <option value="advanced"><?php _e('Advanced', 'simpler-trading'); ?></option>
                </select>
            </div>
            
            <div class="videos-grid">
                <?php foreach ($premium_videos as $video): ?>
                    <?php
                    $duration = get_post_meta($video->ID, 'video_duration', true);
                    $difficulty = get_post_meta($video->ID, 'video_difficulty', true);
                    $thumbnail = get_the_post_thumbnail_url($video->ID, 'medium');
                    $video_url = get_video_streaming_url($video->ID, $user_id);
                    ?>
                    
                    <div class="video-card" data-category="<?php echo get_video_category($video->ID); ?>" data-difficulty="<?php echo $difficulty; ?>">
                        <div class="video-thumbnail">
                            <img src="<?php echo $thumbnail; ?>" alt="<?php echo get_the_title($video); ?>">
                            <div class="video-duration"><?php echo $duration; ?></div>
                            <div class="play-overlay">
                                <button class="btn-play" data-video-id="<?php echo $video->ID; ?>">
                                    ▶️
                                </button>
                            </div>
                        </div>
                        
                        <div class="video-content">
                            <h3 class="video-title"><?php echo get_the_title($video); ?></h3>
                            <div class="video-meta">
                                <span class="difficulty-badge difficulty-<?php echo $difficulty; ?>">
                                    <?php echo ucfirst($difficulty); ?>
                                </span>
                                <span class="views"><?php echo get_video_views($video->ID); ?> <?php _e('views', 'simpler-trading'); ?></span>
                            </div>
                        </div>
                    </div>
                <?php endforeach; ?>
            </div>
        </section>
        
        <!-- Trading Strategies -->
        <section class="trading-strategies">
            <h2><?php _e('Advanced Trading Strategies', 'simpler-trading'); ?></h2>
            <div class="strategies-grid">
                <?php foreach ($strategies as $strategy): ?>
                    <?php
                    $success_rate = get_post_meta($strategy->ID, 'success_rate', true);
                    $risk_level = get_post_meta($strategy->ID, 'risk_level', true);
                    $thumbnail = get_the_post_thumbnail_url($strategy->ID, 'medium');
                    ?>
                    
                    <div class="strategy-card">
                        <div class="strategy-header">
                            <img src="<?php echo $thumbnail; ?>" alt="<?php echo get_the_title($strategy); ?>">
                            <div class="strategy-stats">
                                <div class="success-rate">
                                    <span class="rate-value"><?php echo $success_rate; ?>%</span>
                                    <span class="rate-label"><?php _e('Success Rate', 'simpler-trading'); ?></span>
                                </div>
                                <div class="risk-level">
                                    <span class="risk-value"><?php echo $risk_level; ?>/10</span>
                                    <span class="risk-label"><?php _e('Risk Level', 'simpler-trading'); ?></span>
                                </div>
                            </div>
                        </div>
                        
                        <div class="strategy-content">
                            <h3 class="strategy-title"><?php echo get_the_title($strategy); ?></h3>
                            <div class="strategy-description">
                                <?php echo wp_trim_words(get_the_content($strategy), 25); ?>
                            </div>
                            <div class="strategy-actions">
                                <a href="<?php echo get_permalink($strategy); ?>" class="btn-strategy">
                                    <?php _e('Learn Strategy', 'simpler-trading'); ?>
                                </a>
                            </div>
                        </div>
                    </div>
                <?php endforeach; ?>
            </div>
        </section>
        
        <!-- Market Analysis -->
        <section class="market-analysis">
            <h2><?php _e('Market Analysis & Insights', 'simpler-trading'); ?></h2>
            <div class="analysis-tabs">
                <div class="tab-nav">
                    <button class="tab-btn active" data-tab="daily"><?php _e('Daily Analysis', 'simpler-trading'); ?></button>
                    <button class="tab-btn" data-tab="weekly"><?php _e('Weekly Outlook', 'simpler-trading'); ?></button>
                    <button class="tab-btn" data-tab="monthly"><?php _e('Monthly Review', 'simpler-trading'); ?></button>
                </div>
                
                <div class="tab-content">
                    <div class="tab-pane active" id="daily">
                        <?php
                        $daily_analysis = get_market_analysis('daily');
                        foreach ($daily_analysis as $analysis): ?>
                            <div class="analysis-item">
                                <h4><?php echo get_the_title($analysis); ?></h4>
                                <div class="analysis-content">
                                    <?php echo get_the_content($analysis); ?>
                                </div>
                                <div class="analysis-meta">
                                    <span class="analyst"><?php echo get_post_meta($analysis->ID, 'analyst', true); ?></span>
                                    <span class="date"><?php echo get_the_date('M j, Y', $analysis); ?></span>
                                </div>
                            </div>
                        <?php endforeach; ?>
                    </div>
                    
                    <div class="tab-pane" id="weekly">
                        <?php
                        $weekly_analysis = get_market_analysis('weekly');
                        foreach ($weekly_analysis as $analysis): ?>
                            <div class="analysis-item">
                                <h4><?php echo get_the_title($analysis); ?></h4>
                                <div class="analysis-content">
                                    <?php echo get_the_content($analysis); ?>
                                </div>
                                <div class="analysis-meta">
                                    <span class="analyst"><?php echo get_post_meta($analysis->ID, 'analyst', true); ?></span>
                                    <span class="date"><?php echo get_the_date('M j, Y', $analysis); ?></span>
                                </div>
                            </div>
                        <?php endforeach; ?>
                    </div>
                    
                    <div class="tab-pane" id="monthly">
                        <?php
                        $monthly_analysis = get_market_analysis('monthly');
                        foreach ($monthly_analysis as $analysis): ?>
                            <div class="analysis-item">
                                <h4><?php echo get_the_title($analysis); ?></h4>
                                <div class="analysis-content">
                                    <?php echo get_the_content($analysis); ?>
                                </div>
                                <div class="analysis-meta">
                                    <span class="analyst"><?php echo get_post_meta($analysis->ID, 'analyst', true); ?></span>
                                    <span class="date"><?php echo get_the_date('M j, Y', $analysis); ?></span>
                                </div>
                            </div>
                        <?php endforeach; ?>
                    </div>
                </div>
            </div>
        </section>
    </main>
</div>

<!-- Video Player Modal -->
<div id="video-modal" class="modal">
    <div class="modal-content video-modal">
        <div class="modal-header">
            <h3><?php _e('Premium Video', 'simpler-trading'); ?></h3>
            <button class="modal-close">&times;</button>
        </div>
        <div class="modal-body">
            <div class="video-player">
                <!-- Video player loaded dynamically -->
            </div>
        </div>
    </div>
</div>

<style>
.premium-dashboard {
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    color: white;
}

.premium-header {
    background: rgba(255,255,255,0.1);
    backdrop-filter: blur(10px);
}

.membership-badge.premium {
    background: linear-gradient(45deg, #ffd700, #ffed4e);
    color: #333;
    padding: 8px 16px;
    border-radius: 20px;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
}

.live-trading-room {
    display: flex;
    align-items: center;
    gap: 15px;
}

.live-indicator {
    display: flex;
    align-items: center;
    gap: 8px;
}

.live-dot {
    width: 8px;
    height: 8px;
    background: #ff4444;
    border-radius: 50%;
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0% { opacity: 1; }
    50% { opacity: 0.5; }
    100% { opacity: 1; }
}

.btn-trading-room-live {
    background: linear-gradient(45deg, #ff6b6b, #ff8e53);
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 25px;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    transition: transform 0.2s ease;
}

.btn-trading-room-live:hover {
    transform: translateY(-2px);
}

.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 20px;
    margin-bottom: 30px;
}

.stat-card {
    background: rgba(255,255,255,0.1);
    backdrop-filter: blur(10px);
    padding: 20px;
    border-radius: 12px;
    text-align: center;
}

.stat-value {
    font-size: 2.5em;
    font-weight: 700;
    color: #ffd700;
}

.stat-label {
    color: #ccc;
    font-size: 0.9em;
    margin-top: 5px;
}
</style>

<script>
jQuery(document).ready(function($) {
    // Video filters
    $('#video-category, #video-difficulty').on('change', function() {
        var category = $('#video-category').val();
        var difficulty = $('#video-difficulty').val();
        
        $('.video-card').each(function() {
            var $card = $(this);
            var match = true;
            
            if (category !== 'all' && $card.data('category') !== category) {
                match = false;
            }
            
            if (difficulty !== 'all' && $card.data('difficulty') !== difficulty) {
                match = false;
            }
            
            $card.toggle(match);
        });
    });
    
    // Video player modal
    $('.btn-play').on('click', function() {
        var videoId = $(this).data('video-id');
        loadVideoPlayer(videoId);
        $('#video-modal').show();
    });
    
    // Tab functionality
    $('.tab-btn').on('click', function() {
        var tab = $(this).data('tab');
        
        $('.tab-btn').removeClass('active');
        $(this).addClass('active');
        
        $('.tab-pane').removeClass('active');
        $('#' + tab).addClass('active');
    });
});

function loadVideoPlayer(videoId) {
    jQuery.ajax({
        url: ajaxurl,
        type: 'POST',
        data: {
            action: 'get_video_player',
            video_id: videoId,
            nonce: '<?php echo wp_create_nonce('video_player_nonce'); ?>'
        },
        success: function(response) {
            jQuery('.video-player').html(response);
        }
    });
}
</script>

<?php get_footer(); ?>
```

## 🎨 Svelte 5 Implementation (Modern)

### Main Page Component
```svelte
<!-- /src/routes/dashboard/mastering-the-trade/+page.svelte -->
<script lang="ts">
  import type { PageData } from './$types';
  import { fade, slide } from 'svelte/transition';
  
  interface Props {
    data: PageData;
  }
  
  let { data }: Props = $props();
  
  // State management with Svelte 5 runes
  let selectedVideoCategory = $state('all');
  let selectedDifficulty = $state('all');
  let activeTab = $state('daily');
  let showVideoModal = $state(false);
  let selectedVideo = $state<any>(null);
  
  // Derived filtered videos
  let filteredVideos = $derived(
    data.premiumVideos.filter(video => {
      const categoryMatch = selectedVideoCategory === 'all' || video.category === selectedVideoCategory;
      const difficultyMatch = selectedDifficulty === 'all' || video.difficulty === selectedDifficulty;
      return categoryMatch && difficultyMatch;
    })
  );
  
  function openVideoPlayer(video: any) {
    selectedVideo = video;
    showVideoModal = true;
  }
  
  function closeVideoModal() {
    showVideoModal = false;
    selectedVideo = null;
  }
  
  function switchTab(tab: string) {
    activeTab = tab;
  }
</script>

<div class="premium-dashboard">
  <header class="premium-header">
    <div class="header-left">
      <h1>Mastering the Trade</h1>
      <div class="membership-badge premium">
        <span class="badge-icon">⭐</span>
        <span>Premium Member</span>
      </div>
    </div>
    
    <div class="header-right">
      <div class="live-indicator">
        <span class="live-dot"></span>
        <span>LIVE NOW</span>
      </div>
      <a href={data.tradingRoomUrl} target="_blank" class="btn-trading-room-live">
        <span>📈</span>
        Enter Trading Room
      </a>
    </div>
  </header>
  
  <main class="premium-content">
    <!-- Quick Stats -->
    <section class="quick-stats">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-value">{data.premiumVideos.length}</div>
          <div class="stat-label">Premium Videos</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{data.strategies.length}</div>
          <div class="stat-label">Trading Strategies</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{data.stats.tradingDays}</div>
          <div class="stat-label">Trading Days</div>
        </div>
        <div class="stat-card">
          <div class="stat-value">{data.stats.winRate}%</div>
          <div class="stat-label">Win Rate</div>
        </div>
      </div>
    </section>
    
    <!-- Live Trading Room -->
    <section class="live-room-section">
      <h2>Live Trading Room</h2>
      <div class="trading-room-embed">
        <iframe 
          src={data.tradingRoomUrl} 
          width="100%" 
          height="600" 
          frameborder="0" 
          allowfullscreen
          title="Live Trading Room"
        ></iframe>
      </div>
      
      <div class="room-schedule">
        <h3>Today's Schedule</h3>
        {#each data.schedule as session}
          <div class="schedule-item">
            <span class="time">{session.time}</span>
            <span class="session">{session.session}</span>
            <span class="trader">{session.trader}</span>
          </div>
        {/each}
      </div>
    </section>
    
    <!-- Premium Videos -->
    <section class="premium-videos">
      <h2>Premium Video Library</h2>
      
      <div class="video-filters">
        <select bind:value={selectedVideoCategory}>
          <option value="all">All Videos</option>
          <option value="strategy">Strategy</option>
          <option value="analysis">Market Analysis</option>
          <option value="education">Education</option>
        </select>
        
        <select bind:value={selectedDifficulty}>
          <option value="all">All Levels</option>
          <option value="beginner">Beginner</option>
          <option value="intermediate">Intermediate</option>
          <option value="advanced">Advanced</option>
        </select>
      </div>
      
      <div class="videos-grid">
        {#each filteredVideos as video (video.id)}
          <div class="video-card" transition:fade>
            <div class="video-thumbnail">
              <img src={video.thumbnail} alt={video.title} />
              <div class="video-duration">{video.duration}</div>
              <button class="play-overlay" onclick={() => openVideoPlayer(video)}>
                ▶️
              </button>
            </div>
            <div class="video-content">
              <h3>{video.title}</h3>
              <div class="video-meta">
                <span class="difficulty-badge difficulty-{video.difficulty}">
                  {video.difficulty}
                </span>
                <span class="views">{video.views} views</span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </section>
    
    <!-- Trading Strategies -->
    <section class="trading-strategies">
      <h2>Advanced Trading Strategies</h2>
      <div class="strategies-grid">
        {#each data.strategies as strategy (strategy.id)}
          <div class="strategy-card">
            <div class="strategy-header">
              <img src={strategy.thumbnail} alt={strategy.title} />
              <div class="strategy-stats">
                <div class="success-rate">
                  <span class="rate-value">{strategy.successRate}%</span>
                  <span class="rate-label">Success Rate</span>
                </div>
                <div class="risk-level">
                  <span class="risk-value">{strategy.riskLevel}/10</span>
                  <span class="risk-label">Risk Level</span>
                </div>
              </div>
            </div>
            <div class="strategy-content">
              <h3>{strategy.title}</h3>
              <p>{strategy.description}</p>
              <a href="/dashboard/mastering-the-trade/strategy/{strategy.slug}" class="btn-strategy">
                Learn Strategy
              </a>
            </div>
          </div>
        {/each}
      </div>
    </section>
    
    <!-- Market Analysis -->
    <section class="market-analysis">
      <h2>Market Analysis & Insights</h2>
      <div class="analysis-tabs">
        <div class="tab-nav">
          <button 
            class="tab-btn" 
            class:active={activeTab === 'daily'}
            onclick={() => switchTab('daily')}
          >
            Daily Analysis
          </button>
          <button 
            class="tab-btn" 
            class:active={activeTab === 'weekly'}
            onclick={() => switchTab('weekly')}
          >
            Weekly Outlook
          </button>
          <button 
            class="tab-btn" 
            class:active={activeTab === 'monthly'}
            onclick={() => switchTab('monthly')}
          >
            Monthly Review
          </button>
        </div>
        
        <div class="tab-content">
          {#if activeTab === 'daily'}
            <div class="tab-pane" transition:slide>
              {#each data.analysis.daily as item}
                <div class="analysis-item">
                  <h4>{item.title}</h4>
                  <div class="analysis-content">{@html item.content}</div>
                  <div class="analysis-meta">
                    <span class="analyst">{item.analyst}</span>
                    <span class="date">{item.date}</span>
                  </div>
                </div>
              {/each}
            </div>
          {:else if activeTab === 'weekly'}
            <div class="tab-pane" transition:slide>
              {#each data.analysis.weekly as item}
                <div class="analysis-item">
                  <h4>{item.title}</h4>
                  <div class="analysis-content">{@html item.content}</div>
                  <div class="analysis-meta">
                    <span class="analyst">{item.analyst}</span>
                    <span class="date">{item.date}</span>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="tab-pane" transition:slide>
              {#each data.analysis.monthly as item}
                <div class="analysis-item">
                  <h4>{item.title}</h4>
                  <div class="analysis-content">{@html item.content}</div>
                  <div class="analysis-meta">
                    <span class="analyst">{item.analyst}</span>
                    <span class="date">{item.date}</span>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    </section>
  </main>
</div>

{#if showVideoModal}
  <div class="modal-overlay" onclick={closeVideoModal} transition:fade>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h3>Premium Video</h3>
        <button class="modal-close" onclick={closeVideoModal}>&times;</button>
      </div>
      <div class="modal-body">
        {#if selectedVideo}
          <video controls width="100%">
            <source src={selectedVideo.url} type="video/mp4" />
            <track kind="captions" />
          </video>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .premium-dashboard {
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    color: white;
    min-height: 100vh;
  }
  
  .premium-header {
    background: rgba(255,255,255,0.1);
    backdrop-filter: blur(10px);
    padding: 2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .membership-badge.premium {
    background: linear-gradient(45deg, #ffd700, #ffed4e);
    color: #333;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 600;
  }
  
  .live-dot {
    width: 8px;
    height: 8px;
    background: #ff4444;
    border-radius: 50%;
    animation: pulse 2s infinite;
  }
  
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }
  
  .btn-trading-room-live {
    background: linear-gradient(45deg, #ff6b6b, #ff8e53);
    color: white;
    padding: 0.75rem 1.5rem;
    border-radius: 25px;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    text-decoration: none;
    transition: transform 0.2s ease;
  }
  
  .btn-trading-room-live:hover {
    transform: translateY(-2px);
  }
  
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
    margin: 2rem 0;
  }
  
  .stat-card {
    background: rgba(255,255,255,0.1);
    backdrop-filter: blur(10px);
    padding: 1.5rem;
    border-radius: 12px;
    text-align: center;
  }
  
  .stat-value {
    font-size: 2.5em;
    font-weight: 700;
    color: #ffd700;
  }
  
  .videos-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-top: 1.5rem;
  }
  
  .video-card {
    background: rgba(255,255,255,0.1);
    border-radius: 12px;
    overflow: hidden;
    transition: transform 0.2s ease;
  }
  
  .video-card:hover {
    transform: translateY(-4px);
  }
  
  .strategies-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 2rem;
    margin-top: 1.5rem;
  }
  
  .tab-nav {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  
  .tab-btn {
    padding: 0.75rem 1.5rem;
    background: rgba(255,255,255,0.1);
    border: none;
    border-radius: 8px;
    color: white;
    cursor: pointer;
    transition: background 0.2s ease;
  }
  
  .tab-btn.active {
    background: rgba(255,215,0,0.3);
  }
  
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  
  .modal-content {
    background: white;
    border-radius: 12px;
    max-width: 90vw;
    max-height: 90vh;
    overflow: auto;
  }
</style>
```

### Load Function
```typescript
// /src/routes/dashboard/mastering-the-trade/+page.ts
import type { Load } from '@sveltejs/kit';
import { error } from '@sveltejs/kit';

export const load: Load = async ({ fetch, parent }) => {
  const { user } = await parent();
  
  // Verify membership
  if (!user?.memberships?.includes('mastering_the_trade')) {
    throw error(403, 'Premium membership required');
  }
  
  try {
    const [videos, strategies, analysis, schedule, stats] = await Promise.all([
      fetch('/api/dashboard/mastering-the-trade/videos').then(r => r.json()),
      fetch('/api/dashboard/mastering-the-trade/strategies').then(r => r.json()),
      fetch('/api/dashboard/mastering-the-trade/analysis').then(r => r.json()),
      fetch('/api/dashboard/mastering-the-trade/schedule').then(r => r.json()),
      fetch('/api/dashboard/mastering-the-trade/stats').then(r => r.json())
    ]);
    
    // Generate JWT token for trading room
    const jwtResponse = await fetch('/api/trading-room/jwt', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ membership: 'mastering_the_trade' })
    });
    const { token } = await jwtResponse.json();
    
    const tradingRoomUrl = `https://chat.protradingroom.com/ptr_app/sessions/v2/authUser/652754202ad80b3e7c5131e2?sl=1&jwt=${token}`;
    
    return {
      premiumVideos: videos,
      strategies,
      analysis,
      schedule,
      stats,
      tradingRoomUrl
    };
  } catch (err) {
    throw error(500, 'Failed to load dashboard data');
  }
};
```

## 🔒 Security Considerations
- **JWT Authentication**: Trading room access secured with JWT tokens (exp: 24 hours)
- **Membership Validation**: Real-time verification before content display
- **Secure Video Streaming**: DRM protection for premium content
- **Access Control**: Server-side membership checking
- **Session Management**: Secure cookie-based sessions

## ⚡ Performance Optimization
- **Code Splitting**: Lazy load video player components
- **Image Optimization**: WebP format with fallbacks
- **CDN Delivery**: Static assets served via CDN
- **Caching Strategy**: Aggressive caching for video thumbnails
- **Progressive Loading**: Load above-the-fold content first

## 📱 Responsive Design
- **Desktop**: Full-width trading room embed, multi-column grids
- **Tablet**: 2-column layouts, optimized video player
- **Mobile**: Single column, touch-optimized controls

## 🎯 Key Features
- ✅ Live trading room with JWT authentication
- ✅ Premium video library with filtering
- ✅ Advanced trading strategies with success rates
- ✅ Daily/weekly/monthly market analysis
- ✅ Real-time schedule updates
- ✅ Performance statistics dashboard
- ✅ Expert trader insights
- ✅ Community forum integration

## 📊 User Experience Flow
1. Premium member navigates to Mastering the Trade
2. Dashboard validates membership status
3. Live trading room access provided with JWT authentication
4. Premium video content and strategies displayed
5. Real-time market analysis and expert insights available

---

**Membership ID**: `mastering_the_trade`  
**Access Level**: Premium  
**Trading Room**: Live with JWT authentication  
**Content Updates**: Real-time
