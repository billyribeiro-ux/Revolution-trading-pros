# DASHBOARD SIDE-BY-SIDE COMPARISON
## Reference File 1 vs Current SvelteKit Dashboard

---

## STRUCTURE COMPARISON

### REFERENCE FILE 1 (Lines 5647-5792)
```html
<header class="dashboard__header">
  <div class="dashboard__header-left">
    <h1 class="dashboard__page-title">Member Dashboard</h1>
  </div>
  <div class="dashboard__header-right">
    <ul class="ultradingroom" style="text-align: right;list-style: none;">
      <li class="litradingroom">
        <a href="..." class="btn btn-xs btn-link" style="font-weight: 700 !important;">Trading Room Rules</a>
      </li>
      <li style="font-size: 11px;" class="btn btn-xs btn-link litradingroomhind">
        By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
      </li>
    </ul>
    <div class="dropdown display-inline-block">
      <a href="#" class="btn btn-xs btn-orange btn-tradingroom dropdown-toggle">
        <strong>Enter a Trading Room</strong>
      </a>
      <nav class="dropdown-menu dropdown-menu--full-width">
        <ul class="dropdown-menu__menu">
          <li><a href="..."><span class="st-icon-mastering-the-trade icon icon--md"></span>Mastering The Trade Room</a></li>
          <li><a href="..."><span class="st-icon-simpler-showcase icon icon--md"></span> Simpler Showcase Room</a></li>
        </ul>
      </nav>
    </div>
  </div>
</header>

<div class="dashboard__content">
  <div class="dashboard__content-main">
    
    <!-- MEMBERSHIPS SECTION -->
    <section class="dashboard__content-section">
      <h2 class="section-title">Memberships</h2>
      <div class="membership-cards row">
        <div class="col-sm-6 col-xl-4">
          <article class="membership-card membership-card--options">
            <a href="..." class="membership-card__header">
              <span class="mem_icon">
                <span class="membership-card__icon">
                  <span class="icon icon--lg st-icon-mastering-the-trade"></span>
                </span>
              </span>
              <span class="mem_div">Mastering the Trade </span>
            </a>
            <div class="membership-card__actions">
              <a href="...">Dashboard</a>
              <a href="..." target="_blank">Trading Room</a>
            </div>
          </article>
        </div>
      </div>
    </section>

    <!-- TOOLS SECTION -->
    <section class="dashboard__content-section">
      <h2 class="section-title">Tools</h2>
      <div class="membership-cards row">
        <div class="col-sm-6 col-xl-4">
          <article class="membership-card membership-card--ww">
            <a href="..." class="membership-card__header">
              <span class="mem_icon">
                <span class="membership-card__icon">
                  <span class="icon icon--md st-icon-trade-of-the-week"></span>
                </span>
              </span>
              <span class="mem_div">Weekly Watchlist </span>
            </a>
            <div class="membership-card__actions">
              <a href="...">Dashboard</a>
            </div>
          </article>
        </div>
      </div>
    </section>

    <!-- WEEKLY WATCHLIST SECTION -->
    <div class="dashboard__content-section u--background-color-white">
      <section>
        <div class="row">
          <div class="col-sm-6 col-lg-5">
            <h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
            <div class="hidden-md d-lg-none pb-2">
              <a href="...">
                <img src="..." alt="Weekly Watchlist image" class="u--border-radius">
              </a>
            </div>
            <h4 class="h5 u--font-weight-bold">Weekly Watchlist with TG Watkins</h4>
            <div class="u--hide-read-more">
              <p>Week of December 22, 2025. </p>
            </div>
            <a href="..." class="btn btn-tiny btn-default">Watch Now</a>
          </div>
          <div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
            <a href="...">
              <img src="..." alt="Weekly Watchlist image" class="u--border-radius">
            </a>
          </div>
        </div>
      </section>
    </div>

    <!-- INLINE STYLES FROM REFERENCE -->
    <style>
      .mem_icon,.mem_div{
        display: inline-block;
        vertical-align: middle;
      }
      .mem_div{
        white-space: normal;
        width: calc(100% - 43px);
      }
    </style>
    
  </div>
  
  <aside class="dashboard__content-sidebar">
    <section class="content-sidebar__section">
    </section>
  </aside>
</div>
```

---

## CRITICAL CSS FROM REFERENCE FILE 1

### Lines 5776-5785 (Inline Styles)
```css
.mem_icon,.mem_div{
    display: inline-block;
    vertical-align: middle;
}
.mem_div{
    white-space: normal;
    width: calc(100% - 43px);
}
```

### Lines 4023-4034 (Global CSS)
```css
.ultradingroom{
    max-width: 299px;
    display: none;  /* HIDDEN BY DEFAULT - JavaScript moves it */
}
.dashboard__header{
    justify-content: space-between;
}
.litradingroomhind{
    width:300px;
    float:right;
}
```

---

## MISSING ELEMENTS IN CURRENT DASHBOARD

1. ✅ `.mem_icon` and `.mem_div` inline styles
2. ✅ `.ultradingroom` with `display: none`
3. ✅ Proper dropdown structure with `dropdown-menu__menu`
4. ✅ Icon spans with `icon icon--lg` and `icon icon--md` classes
5. ✅ Weekly Watchlist section with proper grid layout
6. ✅ `section-title-alt--underline` styling
7. ✅ Responsive column classes: `col-sm-6 col-lg-5`, `col-sm-6 col-lg-7`
8. ✅ Hidden utility classes: `hidden-md d-lg-none`, `hidden-xs hidden-sm d-none d-lg-block`
9. ✅ Button classes: `btn-tiny`, `btn-default`
10. ✅ Utility classes: `u--hide-read-more`, `pb-2`

---

## ACTION PLAN

1. Add missing `.mem_icon` and `.mem_div` styles
2. Ensure `.ultradingroom` has `display: none`
3. Fix Weekly Watchlist section structure
4. Add missing utility classes
5. Add missing button classes
6. Test each section matches reference exactly
