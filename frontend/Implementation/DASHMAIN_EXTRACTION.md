# DASHMAIN COMPLETE EXTRACTION
## Source: /frontend/Implementation/DASHMAIN (5668 lines)

---

# PART 1: KEY DASHBOARD SIDEBAR CSS - ALL #0984ae INSTANCES

## ⚠️ COLOR UPDATE REQUIRED: #0984ae → #143E59

### Instance 1: Collapsed Nav Text Color (LINE 2468-2470)
```css
.dashboard__nav-primary.is-collapsed .dash_main_links .dashboard__nav-item-text {
    color: #0984ae !important;
}
```

### Instance 2-5: Pagination Colors (LINES 2273-2305)
```css
.paging-navigation .nav-links .current {
    font-weight: 400;
    color: #fff;
    background: #0984ae;
    padding: 10px 15px;
}

.paging-navigation .nav-links .page-numbers:first-child {
    border-top-left-radius: 5px;
    border-bottom-left-radius: 5px;
    font-weight: 400;
    color: #fff;
    background: #0984ae;
    padding: 10px 15px;
}

.paging-navigation .nav-links .prev {
    border-top-left-radius: 5px;
    border-bottom-left-radius: 5px;
    font-weight: 400;
    color: #fff;
    background: #0984ae;
    padding: 10px 15px;
}

.paging-navigation .nav-links .next {
    border-top-right-radius: 5px;
    border-bottom-right-radius: 5px;
    font-weight: 400;
    color: #fff;
    background: #0984ae;
    padding: 10px 15px;
}

.paging-navigation .nav-links a {
    border-left: 1px solid #ededed;
    display: inline-block;
    font-size: 17px;
    font-weight: 600;
    margin: 0 5px;
    padding: 10px 15px;
    color: #666;
    background: #f4f4f4;
    text-align: center;
}
```

## COMPLETE MAIN NAVIGATION CSS WITH HOVER/TRANSITIONS

### Navigation Base Styles (LINES 2335-2424)
```css
.main-navigation .inside-navigation {
    max-width: 1300px !important;
}

.main-navigation .sub-menu {
    min-width: 220px;
}

.main-navigation ul>li>a {
    font-weight: 700;
}

/* Current Menu Item */
.main-navigation .main-nav ul li[class*="current-menu-"] > a {
    color: #0A2335;
    background: #ffffff;
}

/* Current Menu Item HOVER */
.main-navigation .main-nav ul li[class*="current-menu-"]:hover > a {
    color: #0A2335;
    background: #e9ebed;
}

/* Menu Item HOVER - Blue Background */
.main-navigation ul>li.menu-item>a:hover {
    background: #0E6AC4 !important;
    color: #ffffff !important;
}

/* Sub-menu Transitions */
#menu-header-menu-2023 .sub-menu li {
    transition: background 0.2s linear;
}

#menu-header-menu-2023 .sub-menu li:hover,
#menu-header-menu-2023 .sub-menu li:active {
    background: #e9ebed;
}

/* Sub-menu Link Transitions */
.main-navigation #menu-header-menu-2023 ul.sub-menu>li>a {
    transition: color 0.2s linear;
}

.main-navigation #menu-header-menu-2023 ul.sub-menu>li>a:hover {
    background: initial !important;
    color: #3BA5FF !important;
}

/* Line Height Transition */
.main-navigation .main-nav ul li a,
.menu-toggle,
.main-navigation .mobile-bar-items a {
    transition: line-height 300ms ease;
}

/* White Navigation Background */
.main-navigation {
    background-color: #fff;
    box-shadow: 0 2px 2px -2px rgba(0,0,0,.2);
}
```

### Icon Colors (LINES 2123-2128)
```css
.main-navigation ul>li.menu-item>a .blue_icon {
    color: #0f6ac4;
}

.main-navigation ul>li.menu-item>a:hover .blue_icon {
    color: #fff;
}
```

### Cart Menu Item Hover (LINES 1997-1999)
```css
li.menu-item-cart:hover,
.menu-item-cart a:hover {
    background: #0a2436 !important;
}
```

## Secondary Nav Submenu
```css
.dashboard__nav-secondary .dashboard__nav-submenu {
    z-index: 110 !important;
}
```

## Dashboard Header
```css
.dashboard__header {
    justify-content: space-between;
}
.ultradingroom { max-width: 299px; display: none; }
.litradingroomhind { width: 300px; float: right; }
```

---

# PART 2: GLOBAL CSS VARIABLES

```css
/* Elementor Kit Colors */
--e-global-color-primary: #003366;
--e-global-color-secondary: #0F6AC4;
--e-global-color-text: #4A4A4A;
--e-global-color-accent: #FDAC3B;
--e-global-color-6cd9569: #FF9400;
--e-global-color-0c86ca0: #032235;
--e-global-color-931749f: #F5F8FB;
--e-global-color-9e9d609: #45E0B1;
```

---

# PART 3: MAIN NAVIGATION CSS

```css
.main-navigation { background-color: #fff; box-shadow: 0 2px 2px -2px rgba(0,0,0,.2); }
.main-navigation, .main-navigation ul ul { background-color: #0a2335; }
.main-navigation .main-nav ul li a, .menu-toggle { color: #0A2335; }
.main-navigation ul>li>a { font-size: 14px; font-weight: 700; }

/* Hover */
.main-navigation .main-nav ul li:hover>a { color: #0a2335; background-color: #e9ebed; }
.main-navigation ul>li.menu-item>a:hover { background: #0E6AC4 !important; color: #fff !important; }

/* Current Item */
.main-navigation .main-nav ul li[class*="current-menu-"]>a { color: #0A2335; background: #fff; }
```

---

# PART 4: BUTTON STYLES

```css
.shaped-btn { border-radius: 25px; font-weight: 800; font-size: 18px; text-transform: uppercase; padding: 10px 20px; transition: all .2s ease-in-out; }
.squared-btn { border-radius: 4px; font-weight: 800; font-size: 14px; text-transform: uppercase; padding: 10px 20px; transition: all .2s ease-in-out; }
.primary-btn { background-color: #F69532; color: #fff; box-shadow: 0 2px 5px rgba(0,0,0,0.16); }
.primary-btn:hover { background-color: #dc7309; }
```

---

# PART 5: ICON CLASSES (st-icon-*)

- st-icon-home, st-icon-courses, st-icon-indicators
- st-icon-mastering-the-trade, st-icon-simpler-showcase
- st-icon-tr3ndy-spx-alerts-circle, st-icon-consistent-growth
- st-icon-trade-of-the-week, st-icon-support, st-icon-settings
- st-icon-dashboard, st-icon-daily-videos, st-icon-learning-center
- st-icon-chatroom-archive, st-icon-forum, st-icon-training-room

---

# PART 6: SIDEBAR HTML STRUCTURE

```html
<aside class="dashboard__sidebar">
  <nav class="dashboard__nav-primary is-collapsed">
    <a href="/dashboard/account" class="dashboard__profile-nav-item">
      <span class="dashboard__profile-photo"></span>
      <span class="dashboard__profile-name">User Name</span>
    </a>
    <ul>
      <ul class="dash_main_links">
        <li class="is-active">
          <a href="/dashboard/">
            <span class="dashboard__nav-item-icon st-icon-home"></span>
            <span class="dashboard__nav-item-text">Member Dashboard</span>
          </a>
        </li>
      </ul>
    </ul>
    <ul>
      <li><p class="dashboard__nav-category">memberships</p></li>
      <ul class="dash_main_links">...</ul>
    </ul>
  </nav>
  <footer class="dashboard__toggle is-collapsed">
    <button class="dashboard__toggle-button" data-toggle-dashboard-menu>
      <div class="dashboard__toggle-button-icon"><span></span><span></span><span></span></div>
    </button>
  </footer>
  <div class="dashboard__overlay" data-toggle-dashboard-menu></div>
  <nav class="dashboard__nav-secondary">...</nav>
</aside>
```

---

# PART 7: FOOTER CSS

```css
.fl-node-59adb3c86fcbf > .fl-row-content-wrap { background-color: #0e2433; }
.fl-node-59adb3c86fcbf { color: #ffffff; }
.footer-socials { display: flex; justify-content: center; }
.app-download-btn { display: flex; justify-content: space-between; max-width: 320px; margin: 0 auto; }
```

---

# PART 8: KEY JAVASCRIPT

```javascript
// Redirect after login
jQuery(window).load(function() {
    let redirect = localStorage.getItem('coming_from_url');
    if(jQuery('body').hasClass('logged-in') && redirect){
        window.location = redirect;
        localStorage.removeItem('coming_from_url');
    }
});

// Expand/Collapse accordions
jQuery('.expnd').on('click', function() {
    jQuery(this).text(jQuery(this).text() == "Expand All +" ? "Collapse All -" : "Expand All +");
    jQuery('.fl-accordion-item').toggleClass('fl-accordion-item-active');
    jQuery('.fl-accordion-content').toggle();
});
```

---

# PART 9: BREAKPOINTS

- **Small (Mobile)**: max-width: 768px
- **Medium (Tablet)**: max-width: 992px  
- **Large (Desktop)**: > 992px

---

# PART 10: EXTERNAL CSS/JS FILES REFERENCED

## CSS:
- dashboard.8f78208b.css (main dashboard styles)
- global.95a93139.css
- generate-style (GeneratePress theme)
- simpler-trading/style.css (child theme)
- wlion-dashboard.css

## JS:
- dashboard.8f78208b.js
- global.95a93139.js
- FLBuilderLayout (Beaver Builder)

---

# PART 11: FLBUILDER LAYOUT CONFIG

```javascript
var FLBuilderLayoutConfig = {
    anchorLinkAnimations: { duration: 1000, easing: 'swing', offset: 100 },
    paths: {
        pluginUrl: 'https://www.simplertrading.com/wp-content/plugins/bb-plugin/',
        wpAjaxUrl: 'https://www.simplertrading.com/cms/wp-admin/admin-ajax.php'
    },
    breakpoints: { small: 768, medium: 992 },
    waypoint: { offset: 80 }
};
```

---

# PART 12: FULL HEIGHT ROWS

```css
.fl-row-full-height .fl-row-content-wrap,
.fl-row-custom-height .fl-row-content-wrap {
    display: flex; min-height: 100vh;
}
.fl-row-full-height.fl-row-align-center .fl-row-content-wrap {
    align-items: center; justify-content: center;
}
.fl-row-full-height.fl-row-align-bottom .fl-row-content-wrap {
    align-items: flex-end; justify-content: flex-end;
}
```

---

# PART 13: EQUAL HEIGHT COLUMNS

```css
.fl-col-group-equal-height { display: flex; flex-wrap: wrap; width: 100%; }
.fl-col-group-equal-height .fl-col { flex: 1 1 auto; }
.fl-col-group-equal-height .fl-col-content { flex-direction: column; width: 100%; }
.fl-col-group-equal-height.fl-col-group-align-center .fl-col-content {
    align-items: center; justify-content: center;
}
```

---

# PART 14: VIDEO BACKGROUNDS

```css
.fl-row-bg-video .fl-bg-video { position: absolute; top: 0; right: 0; bottom: 0; left: 0; overflow: hidden; }
.fl-row-bg-video .fl-bg-video iframe {
    pointer-events: none; width: 100vw; height: 56.25vw;
    min-height: 100vh; min-width: 177.77vh;
    position: absolute; top: 50%; left: 50%;
    transform: translate(-50%, -50%);
}
.fl-bg-video-fallback { background-size: cover; position: absolute; inset: 0; }
```

---

# PART 15: ANIMATIONS

```css
.fl-animation { opacity: 0; }
.fl-animated { opacity: 1; animation-fill-mode: both; }
```

---

# PART 16: PAGE-SPECIFIC STYLES

```css
/* Hide sidebar on member dashboard page */
.page-id-401190 .dashboard__content-sidebar { display: none; }

/* Trial card min-height */
.trial-card { min-height: 290px; }

/* Dashboard page title */
h1.dashboard__page-title { font-weight: 700; }
```

---

# PART 17: NAV ICON PSEUDO-ELEMENTS

```css
.premium-newsletter-nav-item a:before {
    content: ''; background-image: url(https://cdn.simplertrading.com/images/icons/premium-newsletter-icon.svg);
    background-size: contain; width: 18px; height: 50px; float: left; margin-right: 5px;
}
.day-nav-item a:before {
    content: ''; background-image: url(https://cdn.simplertrading.com/images/icons/simpler-day-trading-room-icon.svg);
    background-size: 100%; width: 18px; height: 50px; float: left; margin-right: 5px;
}
.fa-cust-rotate-90 { transform: rotate(90deg); }
.main-navigation ul>li.menu-item>a .blue_icon { color: #0f6ac4; }
```

---

# PART 18: COOKIE CONSENT POPUP

```css
.cs-info-bar { background-color: #ffffff; border: 1px solid #333333; color: #5D5D5D; padding: 45px 80px 40px; font-size: 20px; }
.cs_action_btn[data-cs_action="allow_all"] { background-color: #E16B43; color: #ffffff; }
.cs_action_btn[data-cs_action="disable_all"] { background-color: #F0F0F0; color: #212121; }
```

---

# PART 19: TUMME FINGERPRINTING JS

```javascript
async function manageTumme() {
    const cookieName = "TUMME_";
    const newExpiresAt = new Date().getTime() + (1000 * 60 * 60 * 24);
    
    function getTummeId() {
        const tummePromise = import('https://tumme.simplertrading.com/wbJQJVyw98FmTT4R/2ZIgApdAFmR6wc1t?apiKey=qeEzRzNajGBJ2ojVBDa6')
            .then(tumme => tumme.load({
                endpoint: ["https://tumme.simplertrading.com/wbJQJVyw98FmTT4R/beLYMVI6SYXtKltF", tumme.defaultEndpoint],
                storageKey: "TUMME_"
            }));
        return tummePromise.then(fp => fp.get()).then(result => result.visitorId);
    }
    // ... cookie management logic
}
```

---

# PART 20: DATA LAYER INIT

```javascript
window.dataLayer = window.dataLayer || [];
window.dataLayer.push({
    'event': 'dataLayer-initialized',
    'user_id': 94190,
    'pageType': "Uncategorized",
    'membership_type': [],
    'content_author_id': "2",
    'content_author': "Taylor Letterman",
    'content_title': "Mastering the Trade Dashboard",
    'content_publish_date': "22nd Feb 2018",
    'post_id': 401613,
    'content_type': "Uncategorized"
});
```

---

# PART 21: BODY CLASSES

```
page-template-dashboard-member-php, page-id-401613, logged-in, theme-generatepress,
fl-builder, sticky-menu-slide, sticky-enabled, membership-content, access-granted,
member-logged-in, no-sidebar, nav-below-header, fluid-header
```

---

# PART 22: GOOGLE CALENDAR INTEGRATION

```javascript
// Trading Room Schedule
var CLIENT_ID = '656301048421-g2s2jvb2pe772mnj8j8it67eirh4jq1f.apps.googleusercontent.com';
var API_KEY = 'AIzaSyBTC-zYg65B6xD8ezr4gMWCeUNk7y2Hlrw';
var calendarId = 'simpleroptions.com_sabio00har0rd4odbrsa705904@group.calendar.google.com';
```

---

# COMPLETE BUTTON TRANSITIONS (LINES 1211-1244)

```css
.shaped-btn {
    display: block;
    border-radius: 25px;
    width: 100%;
    font-weight: 800;
    font-size: 18px;
    text-transform: uppercase;
    padding: 10px 20px;
    letter-spacing: 1.125px;
    transition: all .2s ease-in-out;
}

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

.primary-btn {
    background-color: #F69532;
    color: #fff;
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.16);
}

.primary-btn:hover {
    background-color: #dc7309;
}
```

---

# FL-BUILDER BUTTON ICON ANIMATIONS (LINES 847-870)

```css
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

.fl-button.fl-button-icon-animation i.fl-button-icon-after {
    margin-left: 0px !important;
}

.fl-button.fl-button-icon-animation:hover i.fl-button-icon-after {
    margin-left: 10px !important;
}

.fl-button.fl-button-icon-animation i.fl-button-icon-before {
    margin-right: 0 !important;
}

.fl-button.fl-button-icon-animation:hover i.fl-button-icon-before {
    margin-right: 20px !important;
    margin-left: -10px;
}
```

---

# SUMMARY OF COLOR VALUES FOUND

| Color | Usage | Lines |
|-------|-------|-------|
| `#0984ae` | **Collapsed nav text, pagination (5 instances)** | 2469, 2276, 2285, 2294, 2303 |
| `#0a2335` | Main navigation background, cart hover | 1998, 2348, 2396 |
| `#0e2433` | Footer background | 1341, 1382 |
| `#153e59` | Secondary nav background (NOT in DASHMAIN) | - |
| `#F69532` | Primary button | 1238 |
| `#dc7309` | Primary button hover | 1243 |
| `#0F6AC4` | Global secondary color, menu hover | 2124, 2356 |
| `#0E6AC4` | Menu item hover background | 2356 |
| `#3BA5FF` | Sub-menu link hover | 2373 |
| `#e9ebed` | Sub-menu hover background, current menu hover | 2109, 2353, 2365 |
| `#45E0B1` | Accent green (Elementor) | - |
| `#003366` | Global primary color (Elementor) | - |

---

# EXTRACTION SUMMARY

**Total Lines Analyzed**: 5668  
**CSS Sections**: Lines 88-2500  
**Color Instances Found**: 5x `#0984ae`  
**Transitions Found**: 6 different transition properties  
**Hover States Found**: 15+ hover selectors  

## Files to Update
1. `/frontend/Implementation/DASHMAIN` - Lines 2469, 2276, 2285, 2294, 2303
2. `/frontend/Implementation/DashboardHome` - Same line numbers (parallel file)

---

# PART 23: FLBUILDER JAVASCRIPT METHODS

```javascript
FLBuilderLayout = {
    init: function() {
        FLBuilderLayout._destroy();
        FLBuilderLayout._initClasses();
        FLBuilderLayout._initBackgrounds();
        if (0 === $('.fl-builder-edit').length) {
            FLBuilderLayout._initModuleAnimations();
            FLBuilderLayout._initAnchorLinks();
            FLBuilderLayout._initHash();
            FLBuilderLayout._initForms();
        }
    },
    
    refreshGalleries: function(element) { /* Wookmark/MosaicFlow refresh */ },
    refreshGridLayout: function(element) { /* Masonry layout */ },
    reloadSlider: function(element) { /* BxSlider reload */ },
    resizeAudio: function(element) { /* WP audio player resize */ },
    preloadAudio: function(element) { /* Preload audio in hidden elements */ },
    resizeSlideshow: function() { /* YUI resize simulation */ },
    reloadGoogleMap: function(element) { /* Reload Google Maps iframe */ },
    
    _destroy: function() { /* Unbind scroll/resize events */ },
    _isTouch: function() { return ('ontouchstart' in window); },
    _isMobile: function() { return /Mobile|Android|Silk/i.test(navigator.userAgent); },
    
    _initClasses: function() {
        // Adds: fl-builder, fl-builder-touch, fl-builder-mobile
        // fl-builder-breakpoint-small/medium/large, fl-builder-ie-11
    },
    
    _initBackgrounds: function() {
        // Parallax and video backgrounds
        if ($('.fl-row-bg-parallax').length > 0 && !FLBuilderLayout._isMobile()) {
            FLBuilderLayout._scrollParallaxBackgrounds();
            FLBuilderLayout._initParallaxBackgrounds();
            $(window).on('scroll.fl-bg-parallax', FLBuilderLayout._scrollParallaxBackgrounds);
        }
        if ($('.fl-bg-video').length > 0) {
            FLBuilderLayout._initBgVideos();
            FLBuilderLayout._resizeBgVideos();
            $(window).on('resize.fl-bg-video', FLBuilderLayout._resizeBgVideos);
        }
    },
    
    _initParallaxBackground: function() {
        // Load parallax image and set background-position on scroll
    },
    
    _initBgVideo: function() {
        // Initialize MP4/WebM/YouTube/Vimeo video backgrounds
    },
    
    _initYoutubeBgVideo: function() {
        // YouTube Player API integration
        player = new YT.Player(videoPlayer[0], {
            videoId: videoId,
            events: { onReady, onStateChange, onError },
            playerVars: { playsinline, controls: 0, showinfo: 0, rel: 0, start, end }
        });
    },
    
    _initVimeoBgVideo: function() {
        // Vimeo Player integration
        player = new Vimeo.Player(videoPlayer[0], {
            id: videoId, loop: true, background: true, muted: true
        });
    },
    
    _toggleBgVideoAudio: function(e) {
        // Mute/unmute video audio
    },
    
    _resizeBgVideos: function() {
        // Resize all video backgrounds
    },
    
    _initModuleAnimations: function() {
        // Waypoint-triggered animations
        $('.fl-animation').each(function() {
            $(this).waypoint({ offset: '80%', handler: FLBuilderLayout._doModuleAnimation });
        });
    },
    
    _doModuleAnimation: function() {
        // Add fl-animated class with delay
        var delay = parseFloat(module.data('animation-delay'));
        setTimeout(function() { module.addClass('fl-animated'); }, delay * 1000);
    },
    
    _initHash: function() {
        // Open accordion/tab based on URL hash
        var hash = window.location.hash.replace('#', '').split('/').shift();
        if (element.hasClass('fl-accordion-item')) {
            element.find('.fl-accordion-button').trigger('click');
        }
        if (element.hasClass('fl-tabs-panel')) {
            label[0].click();
        }
    },
    
    _initAnchorLinks: function() {
        // Smooth scroll for anchor links
        $('a').each(FLBuilderLayout._initAnchorLink);
    },
    
    _scrollToElement: function(element, callback) {
        var config = FLBuilderLayoutConfig.anchorLinkAnimations;
        var dest = element.offset().top - config.offset;
        $('html, body').animate({ scrollTop: dest }, config.duration, config.easing, callback);
    },
    
    _initForms: function() {
        // Placeholder fallback and error clearing
        $('.fl-form-field input').on('focus', FLBuilderLayout._clearFormFieldError);
    }
};

$(function() { FLBuilderLayout.init(); });
```

---

# PART 24: DASHBOARD SUBMENU HANDLER

```javascript
// Open Joe course in new window from dashboard subnav
let subMenuBtn = jQuery('.dashboard__nav-submenu li a');
if(subMenuBtn) {
    jQuery('.dashboard__nav-submenu li a').on('click', function(evt){
        if (jQuery(evt.target).text().trim() === 'Continued Education') {
            evt.preventDefault();
            window.open(evt.target.href, evt.target.href);
        }
    });
}
```

---

# PART 25: ACCORDION EXPAND/COLLAPSE

```javascript
// Beaver Builder accordion expand all
var expndAll = jQuery('.expnd');
if(expndAll) {
    expndAll.on('click', function() {
        var text = jQuery(this).text();
        jQuery(this).text(text == "Expand All +" ? "Collapse All -" : "Expand All +");
        jQuery('.fl-accordion-item').toggleClass('fl-accordion-item-active');
        if(jQuery('.fl-accordion-content').is(':visible')) {
            jQuery('.fl-accordion-content').css('display', 'none');
        } else {
            jQuery('.fl-accordion-content').css('display', 'block');
        }
    });
}

// Alternative expand all
$('.expandall').on('click', function() {
    if ($(this).text() == 'Expand All +') {
        $(this).text('Collapse All -');
        $(".fl-accordion-button-icon-right").removeClass('fa-plus').addClass('fa-minus');
        $(".fl-accordion-content").show();
    } else {
        $(this).text('Expand All +');
        $(".fl-accordion-button-icon-right").removeClass('fa-minus').addClass('fa-plus');
        $(".fl-accordion-content").hide();
    }
});
```

---

# PART 26: TRADING ROOM RESIZE

```javascript
function resizeroomhind() {
    if(jQuery(window).width() > 430) {
        if(jQuery(".litradingroom").length > 0){
            jQuery(".litradingroom a").removeClass("btn");
            jQuery(".litradingroomhind").removeClass("btn");
            jQuery(".dashboard__header").css("padding-bottom", '0');
            var roomul = jQuery(".ultradingroom").html();
            jQuery(".ultradingroom").html('');
            jQuery(".dashboard__header").append('<ul style="text-align: right;list-style: none;width:100%;">'+roomul+'</ul>');
        }
    } else {
        if(jQuery(".litradingroom").length > 0){
            jQuery(".litradingroom a").removeClass("btn");
            jQuery(".litradingroomhind").removeClass("btn");
            var roomul = jQuery(".ultradingroom").html();
            jQuery(".ultradingroom").html('');
            jQuery(".dashboard__header-right").append('<ul style="list-style: none;">'+roomul+'</ul>');
        }
    }
}
jQuery(document).ready(function() { resizeroomhind(); });
jQuery(window).load(function() { resizeroomhind(); });
```

---

# PART 27: COOKIE SIZE LIMITER

```javascript
// Limit rpVisitedPages cookie size
(() => {
    const MAX_LEN = 800;
    const desc = Object.getOwnPropertyDescriptor(Document.prototype, 'cookie');
    if (!desc || !desc.set) return;
    
    Object.defineProperty(document, 'cookie', {
        configurable: true,
        get() { return desc.get.call(document); },
        set(v) {
            if (/^\s*rpVisitedPages\s*=/.test(v)) {
                const m = v.match(/^\s*(rpVisitedPages)\s*=([^;]*)/);
                if (m) {
                    let value = m[2] || '';
                    try { value = decodeURIComponent(value); } catch(e){}
                    if (value.length > MAX_LEN) {
                        value = value.slice(-MAX_LEN);
                    }
                    const enc = encodeURIComponent(value);
                    v = v.replace(/^\s*rpVisitedPages\s*=[^;]*/, `rpVisitedPages=${enc}`);
                }
            }
            return desc.set.call(document, v);
        }
    });
})();
```

---

# EXTRACTION COMPLETE

**File**: `/frontend/Implementation/DASHMAIN` (5668 lines)
**Extracted**: CSS, HTML structure, JavaScript
**Key Finding**: Color `#0984ae` at line 2469 needs to change to `#143E59`
