# Indicator Page - Complete WordPress Reference

## Extracted from volume-max-model (Simpler Trading WordPress)

---

## 1. PAGE BACKGROUND & BODY

```css
/* CRITICAL: Body and HTML background */
body {
	background-color: #efefef;
	color: #666666;
}

a,
a:visited {
	color: #1e73be;
}

a:hover,
a:focus,
a:active {
	color: #000000;
}

body,
button,
input,
select,
textarea {
	font-family: 'Open Sans', sans-serif;
}

body .grid-container {
	max-width: 1160px;
}
```

---

## 2. BREADCRUMBS NAVIGATION

### HTML Structure

```html
<nav id="breadcrumbs" class="breadcrumbs">
	<div class="container-fluid">
		<ul>
			<li class="item-home">
				<a
					class="breadcrumb-link breadcrumb-home"
					href="https://www.simplertrading.com"
					title="Home"
					>Home</a
				>
			</li>
			<li class="separator separator-home">/</li>
			<li class="item-cat item-custom-post-type-indicators">
				<a class="breadcrumb-cat breadcrumb-custom-post-type-indicators" href="" title="Indicators"
					>Indicators</a
				>
			</li>
			<li class="separator">/</li>
			<li class="item-current item-719939">
				<strong
					class="breadcrumb-current breadcrumb-719939"
					title="Volume Max Tool Kit (formerly VWAP)"
					>Volume Max Tool Kit (formerly VWAP)</strong
				>
			</li>
		</ul>
	</div>
</nav>
```

### CSS

```css
.breadcrumbs {
	z-index: 1;
	background-color: #efefef;
	border-bottom: 1px solid #dbdbdb;
	padding: 15px 20px;
	font-size: 12px;
	font-family: 'Open Sans', sans-serif;
}

.breadcrumbs .container-fluid {
	max-width: 1160px;
	margin: 0 auto;
}

.breadcrumbs ul {
	list-style: none;
	margin: 0;
	padding: 0;
	display: flex;
	flex-wrap: wrap;
	align-items: center;
}

.breadcrumbs li {
	display: inline;
	margin: 0;
	padding: 0;
}

.breadcrumb-link {
	color: #1e73be;
	text-decoration: none;
	font-weight: 800;
}

.breadcrumb-link:hover {
	color: #143e59;
	text-decoration: underline;
}

.breadcrumbs .separator {
	color: #999999;
	margin: 0 8px;
}

.breadcrumb-current {
	color: #666;
	font-weight: 800;
	font-size: 12px;
}
```

---

## 3. MAIN INDICATORS CONTAINER

### HTML Structure

```html
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<div class="indicators">
			<main>
				<!-- Content here -->
			</main>
		</div>
	</div>
</div>
```

### CSS (from custom-posts.css)

```css
.indicators-template {
	background-color: #fff;
	font-size: 24px;
}

.indicators-template #page {
	padding: 0;
	max-width: 100%;
}

.indicators-template .indicators {
	padding: 50px 15px;
	max-width: 1200px;
	margin: auto;
}

.indicators-template .no_permission {
	padding: 50px 15px;
}
```

---

## 4. PAGE TITLE (H1)

### HTML

```html
<h1>Volume Max Tool Kit (formerly VWAP)</h1>
```

### CSS

```css
.indicators-template h1 {
	color: #0c2434;
	font-weight: 700;
	text-align: center;
	margin-bottom: 10px;
	font-size: 54px;
}

/* From generate-style */
h1 {
	font-family: 'Open Sans Condensed', sans-serif;
	font-weight: bold;
	font-size: 44px;
	line-height: 1.1em;
}

@media (max-width: 768px) {
	h1 {
		font-size: 30px;
	}
}
```

---

## 5. PLATFORMS SECTION

### HTML

```html
<p class="platforms">
	<strong>Available Platforms:</strong>
	ThinkorSwim, TradingView
</p>
<hr />
```

### CSS

```css
.indicators-template .platforms {
	text-align: center;
	margin-bottom: 40px;
}
```

---

## 6. VIDEO SECTION (current-vid)

### HTML Structure

```html
<section id="ca-main" class="ca-section cpost-section">
	<div class="section-inner">
		<div class="ca-content-block cpost-content-block">
			<!-- First Video -->
			<div class="current-vid">
				<div class="video-container current">
					<div class="video-overlay"></div>
					<div id="1" class="video-player"></div>
					<video
						id="1"
						controls
						width="100%"
						poster="https://cdn.simplertrading.com/2020/03/25163022/simpler-geenric-video-bg-768x432.jpg"
						style="aspect-ratio: 16/9;"
						title="Indicator Setup and Q&A, with Eric Purdy"
					>
						<source src="..." type="video/mp4" />
						Your browser does not support the video tag.
					</video>
				</div>
			</div>

			<!-- Second Video (note: current-vid on video-container) -->
			<div class="">
				<div class="video-container current current-vid">
					<div class="video-overlay"></div>
					<div id="2" class="video-player"></div>
					<video id="2" controls width="100%" poster="..." style="aspect-ratio: 16/9;" title="...">
						<source src="..." type="video/mp4" />
					</video>
				</div>
			</div>
		</div>
	</div>
</section>
```

### CSS

```css
/* current-vid wrapper */
.current-vid {
	width: 100%;
	background-color: #0a2335;
	padding: 25px 25px 0;
}

@media (max-width: 415px) {
	.current-vid {
		padding: 10px 10px 0;
	}
}

.current-vid .class-player-header,
.current-vid .player-header {
	color: #fff;
}

.current-vid .current-title {
	color: #d5d5d5;
}

.current-vid .class-video-container,
.current-vid .video-container {
	border: 1px solid #999;
	cursor: pointer;
}

.current-vid .class-video-container.current,
.current-vid .video-container.current {
	width: 100%;
	display: flex;
	z-index: 1;
}

/* For indicator pages specifically */
.indicators-template #ca-main.cpost-section {
	padding-bottom: 0;
}

.indicators-template #ca-main.cpost-section .section-inner {
	max-width: 1200px;
	margin: 0 auto;
	padding: 0;
}

.indicators-template #ca-main.cpost-section .section-inner .current-vid,
.indicators-template #ca-main.cpost-section .section-inner .vid-list {
	background-color: #f4f4f4;
}

.indicators-template #ca-main.cpost-section .section-inner .current-vid .video-overlay,
.indicators-template #ca-main.cpost-section .section-inner .vid-list .video-overlay {
	background-color: rgba(0, 0, 0, 0.269);
}
```

---

## 7. ST_BOX (Platform Download Boxes)

### HTML Structure

```html
<div class="st_box thinkorswim">
	<img
		width="250"
		src="/wp-content/themes/simpler-trading/assets/images/thinkorswim.png"
		alt="ThinkorSwim"
	/>

	<table>
		<tr>
			<th>ThinkorSwim Install File:</th>
			<th></th>
		</tr>
		<tr>
			<td>Volume Max Indicator</td>
			<td class="text-right">
				<a class="orng_btn" href="/?st-download-file=452914b18dc78691e6c98731b9e094fe">
					Click to Download
				</a>
			</td>
		</tr>
		<!-- More rows... -->
	</table>
</div>

<div class="st_box tradingview">
	<img
		width="250"
		src="/wp-content/themes/simpler-trading/assets/images/tradingview.png"
		alt="TradingView"
	/>

	<div class="platform_notes">
		Please email your TradingView Username to support@simplertrading.com...
	</div>
</div>
```

### CSS

```css
.indicators-template .st_box {
	border: 24px solid #f4f4f4;
	padding: 20px;
	margin-top: 30px;
}

.indicators-template .st_box form,
.indicators-template .st_box h2,
.indicators-template .st_box img {
	margin-bottom: 20px;
}

.indicators-template .st_box table {
	margin: 0;
	border: 0;
}

.indicators-template .st_box table th {
	border: 0;
}

.indicators-template .st_box table th:first-child {
	padding-left: 0;
}

.indicators-template .st_box table td {
	border: 0;
	padding: 20px 0;
	border-top: 1px solid #f4f4f4;
}

@media (max-width: 768px) {
	.indicators-template .st_box table td {
		display: flex;
	}
}

.indicators-template .st_box .platform_notes {
	padding-top: 20px;
	border-top: 1px solid #f4f4f4;
}

.indicators-template .border-0 {
	border: 0;
}
```

---

## 8. ORANGE BUTTON (orng_btn)

### HTML

```html
<a class="orng_btn" href="/?st-download-file=...">Click to Download</a>
<a class="orng_btn" href="https://...">Click to View</a>
```

### CSS

```css
.indicators-template .orng_btn {
	background-color: #f8ac00;
	border-radius: 80px;
	font-weight: 700;
	color: #fff;
	display: inline-block;
	text-align: center;
	padding: 10px 35px;
	min-width: 250px;
	font-size: 20px;
}

.indicators-template .orng_btn:hover {
	background-color: #df9c00;
}
```

---

## 9. SUPPORTING DOCUMENTATION BOX

### HTML

```html
<div class="st_box">
	<h2>
		<strong>Supporting Documentation</strong>
	</h2>
	<table>
		<tr>
			<td>TOS Installation Guide</td>
			<td class="text-right">
				<a class="orng_btn" href="https://intercom.help/simpler-trading/en/articles/3263969">
					Click to View
				</a>
			</td>
		</tr>
		<!-- More rows... -->
	</table>
</div>
```

---

## 10. HAVE QUESTIONS? FOOTER SECTION

### HTML Structure

```html
<div class="fl-builder-content fl-builder-content-878010">
	<div class="fl-row fl-row-full-width fl-row-bg-color fl-node-5d56ce96c4c56">
		<div class="fl-row-content-wrap">
			<div class="fl-row-content fl-row-fixed-width fl-node-content">
				<div class="fl-col-group fl-node-5d56ce96c4b60">
					<div class="fl-col fl-node-5d56ce96c4b9d">
						<div class="fl-col-content fl-node-content">
							<div class="fl-module fl-module-rich-text fl-node-5d56ce96c4be0">
								<div class="fl-module-content fl-node-content">
									<div class="fl-rich-text">
										<h2><strong>Have Questions?</strong></h2>
									</div>
								</div>
							</div>
							<div class="fl-module fl-module-rich-text fl-node-5d56ce96c4c1b">
								<div class="fl-module-content fl-node-content">
									<div class="fl-rich-text">
										<p>
											Our support staff is the best by far! You can email
											<a href="mailto:support@simplertrading.com">support@simplertrading.com</a>
											or call us at <a href="tel:5122668659">(512) 266-8659</a>
										</p>
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
```

### CSS

```css
.fl-builder-content .fl-node-5d56ce96c4c56 a {
	color: #0984ae;
}

.fl-node-5d56ce96c4c56 > .fl-row-content-wrap {
	background-color: #f7f7f7;
}

.fl-node-5d56ce96c4c56 .fl-row-content {
	max-width: 800px;
}

.fl-node-5d56ce96c4c56 > .fl-row-content-wrap {
	padding-top: 60px;
	padding-bottom: 60px;
}

@media (max-width: 992px) {
	.fl-node-5d56ce96c4c56.fl-row > .fl-row-content-wrap {
		padding-top: 40px;
		padding-bottom: 40px;
	}
}

@media (max-width: 768px) {
	.fl-node-5d56ce96c4c56.fl-row > .fl-row-content-wrap {
		padding-top: 30px;
		padding-right: 20px;
		padding-bottom: 30px;
		padding-left: 20px;
	}
}

/* Have Questions heading */
.fl-builder-content .fl-node-5d56ce96c4be0 .fl-rich-text,
.fl-builder-content .fl-node-5d56ce96c4be0 .fl-rich-text *:not(b, strong) {
	font-size: 42px;
	text-align: center;
}

@media (max-width: 768px) {
	.fl-builder-content .fl-node-5d56ce96c4be0 .fl-rich-text,
	.fl-builder-content .fl-node-5d56ce96c4be0 .fl-rich-text *:not(b, strong) {
		font-size: 24px;
	}
}

/* Support text */
.fl-builder-content .fl-node-5d56ce96c4c1b .fl-rich-text,
.fl-builder-content .fl-node-5d56ce96c4c1b .fl-rich-text *:not(b, strong) {
	font-size: 22px;
	text-align: center;
}

@media (max-width: 768px) {
	.fl-builder-content .fl-node-5d56ce96c4c1b .fl-rich-text,
	.fl-builder-content .fl-node-5d56ce96c4c1b .fl-rich-text *:not(b, strong) {
		font-size: 16px;
		text-align: center;
	}
}
```

---

## 11. COLOR REFERENCE

| Element             | Color     | Usage                        |
| ------------------- | --------- | ---------------------------- |
| Body Background     | `#efefef` | Page background              |
| Content Background  | `#ffffff` | Main content areas           |
| Text Color          | `#666666` | Body text                    |
| Link Color          | `#1e73be` | Links                        |
| Link Hover          | `#000000` | Link hover state             |
| Heading Dark        | `#0c2434` | H1 headings                  |
| Video Background    | `#0a2335` | current-vid background       |
| Orange Button       | `#f8ac00` | orng_btn default             |
| Orange Button Hover | `#df9c00` | orng_btn hover               |
| Border Gray         | `#f4f4f4` | st_box border, table borders |
| Support Section BG  | `#f7f7f7` | Have Questions section       |
| Breadcrumb Link     | `#1e73be` | Breadcrumb links             |
| Breadcrumb Hover    | `#143E59` | Breadcrumb link hover        |
| Separator           | `#999999` | Breadcrumb separators        |

---

## 12. FONT REFERENCE

| Usage         | Font Family                       | Weight     | Size                  |
| ------------- | --------------------------------- | ---------- | --------------------- |
| Body          | "Open Sans", sans-serif           | 400        | 24px (indicator page) |
| H1            | "Open Sans Condensed", sans-serif | bold (700) | 54px / 44px           |
| Navigation    | "Open Sans", sans-serif           | 400        | 14px                  |
| Breadcrumb    | "Open Sans", sans-serif           | 800        | 12px                  |
| Orange Button | "Open Sans", sans-serif           | 700        | 20px                  |
| Support Title | "Open Sans", sans-serif           | -          | 42px                  |
| Support Text  | "Open Sans", sans-serif           | -          | 22px                  |

---

## 13. RESPONSIVE BREAKPOINTS

| Breakpoint         | Adjustments                        |
| ------------------ | ---------------------------------- |
| `max-width: 992px` | Footer padding reduces             |
| `max-width: 768px` | H1 reduces to 30px, mobile layouts |
| `max-width: 576px` | Mobile-specific styles             |
| `max-width: 415px` | current-vid padding: 10px          |

---

## 14. CRITICAL IMPLEMENTATION NOTES

1. **Body Background**: Must be `#efefef` globally
2. **Indicators Container**: White background `#fff`, max-width 1200px, centered
3. **st_box**: 24px solid `#f4f4f4` border, 20px padding
4. **Logo Width**: `width="250"` on platform logos
5. **Orange Button**: `#f8ac00` with `#df9c00` hover, 80px border-radius
6. **Video Section**: Background `#f4f4f4` for indicator pages (NOT `#0a2335`)
7. **Breadcrumbs**: Below nav, `#efefef` background, 1px border-bottom

---

## 15. COMPLETE PAGE STRUCTURE

```html
<!DOCTYPE html>
<html lang="en-US">
	<head>
		...
	</head>
	<body class="indicators-template">
		<!-- Navigation -->
		<nav id="site-navigation">...</nav>

		<!-- Breadcrumbs -->
		<nav id="breadcrumbs" class="breadcrumbs">
			<div class="container-fluid">
				<ul>
					<li class="item-home"><a>Home</a></li>
					<li class="separator separator-home">/</li>
					<li class="item-cat"><a>Indicators</a></li>
					<li class="separator">/</li>
					<li class="item-current"><strong>Indicator Name</strong></li>
				</ul>
			</div>
		</nav>

		<!-- Main Content -->
		<div id="page" class="hfeed site grid-parent">
			<div id="content" class="site-content">
				<div class="indicators">
					<main>
						<h1>Indicator Name</h1>
						<section>
							<p class="platforms">
								<strong>Available Platforms:</strong> ThinkorSwim, TradingView
							</p>
							<hr />

							<!-- Video Section -->
							<section id="ca-main" class="ca-section cpost-section">
								<div class="section-inner">
									<div class="ca-content-block cpost-content-block">
										<div class="current-vid">
											<div class="video-container current">
												<video>...</video>
											</div>
										</div>
									</div>
								</div>
							</section>

							<!-- Platform Downloads -->
							<div class="st_box thinkorswim">
								<img width="250" src="..." alt="ThinkorSwim" />
								<table>
									...
								</table>
							</div>

							<div class="st_box tradingview">
								<img width="250" src="..." alt="TradingView" />
								<div class="platform_notes">...</div>
							</div>

							<!-- Support Docs -->
							<div class="st_box">
								<h2><strong>Supporting Documentation</strong></h2>
								<table>
									...
								</table>
							</div>
						</section>
					</main>
				</div>
			</div>
		</div>

		<!-- Have Questions Footer -->
		<div class="fl-builder-content">
			<div class="fl-row fl-row-full-width fl-row-bg-color">
				<div class="fl-row-content-wrap" style="background-color: #f7f7f7;">
					<div class="fl-row-content fl-row-fixed-width" style="max-width: 800px;">
						<div class="fl-rich-text">
							<h2><strong>Have Questions?</strong></h2>
						</div>
						<div class="fl-rich-text">
							<p>Our support staff is the best by far!...</p>
						</div>
					</div>
				</div>
			</div>
		</div>

		<!-- Site Footer -->
		<footer>...</footer>
	</body>
</html>
```

---

**Last Updated**: January 9, 2026
**Source**: https://my.simplertrading.com/indicators/volume-max-i
