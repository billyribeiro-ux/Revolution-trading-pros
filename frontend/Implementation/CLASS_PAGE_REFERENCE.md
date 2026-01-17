# Class Page - Complete WordPress Reference

## Extracted from quick-start-precision (Simpler Trading WordPress)

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

Same as indicator page - uses DashboardBreadcrumbs component.

---

## 3. PAGE WRAPPER STRUCTURE

```html
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<!-- Class content here -->
	</div>
</div>
```

---

## 4. CLASS RECORDINGS SECTION

### HTML Structure

```html
<section class="class-section cpost-section" id="class-recordings">
	<div class="section-inner">
		<div class="class-content-block cpost-content-block">
			<div class="current-vid">
				<h2 class="class-player-header">Quickstart To Precision Trading</h2>
				<h3 class="current-title">Quickstart To Precision Trading</h3>
				<div class="class-video-description">
					<p></p>
				</div>
				<div class="class-video-container current">
					<div class="video-overlay">
						<h3>Quickstart To Precision Trading</h3>
					</div>
					<div id="quickstart-to-precision-trading" class="class-video-player"></div>
					<video id="..." controls width="100%" poster="" style="aspect-ratio: 16/9;" title="...">
						<source
							src="https://simpler-options.s3.amazonaws.com/Moxie/MoxieQuickstart_A1.mp4"
							type="video/mp4"
						/>
						Your browser does not support the video tag.
					</video>
				</div>
			</div>
		</div>
	</div>
</section>
```

### CSS

```css
/* Video Section */
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

.class-video-container video {
	width: 100%;
	display: block;
}

.video-overlay {
	background-color: rgba(0, 0, 0, 0.269);
}
```

---

## 5. CLASS DOWNLOADS SECTION

### HTML Structure

```html
<section class="class-section cpost-section" id="dl-rp-row">
	<div class="section-inner">
		<section class="class-subsection" id="class-downloads">
			<h2>Class Downloads</h2>
			<div class="class-downloads-container">
				<iframe
					src="https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list"
					width="500"
					height="400"
					frameborder="0"
					allowfullscreen
					webkitallowfullscreen
					msallowfullscreen
				>
				</iframe>
			</div>
		</section>
	</div>
</section>
```

### CSS

```css
#dl-rp-row .section-inner {
	display: flex;
}

@media (max-width: 768px) {
	#dl-rp-row .section-inner {
		flex-direction: column-reverse;
	}
}

#dl-rp-row .class-subsection {
	flex: 1 1;
	background-color: #fff;
	padding: 25px;
}

#dl-rp-row .class-subsection#class-downloads {
	margin-right: 20px;
}

@media (max-width: 768px) {
	#dl-rp-row .class-subsection#class-downloads {
		margin: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
}

#dl-rp-row .class-subsection#class-downloads .class-downloads-container {
	margin-bottom: 40px;
}

#dl-rp-row .class-subsection#class-downloads .class-downloads-container:last-child {
	margin-bottom: 0;
}
```

---

## 6. SECTION STRUCTURE

```css
.cpost-section {
	padding: 60px 0;
}

@media (max-width: 768px) {
	.cpost-section {
		padding: 40px 0;
	}
}

.cpost-section .section-inner {
	max-width: 1100px;
	margin: 0 auto;
}

@media (max-width: 1140px) {
	.cpost-section .section-inner {
		padding: 0 20px;
	}
}

.cpost-content-block.class-content-block {
	padding: 0 40px 40px;
	border-radius: 0;
}

@media (max-width: 768px) {
	.cpost-content-block.class-content-block {
		padding-right: 20px;
		padding-left: 20px;
	}
}

@media (max-width: 600px) {
	.cpost-content-block.class-content-block {
		padding-right: 20px;
		padding-left: 20px;
	}
}
```

---

## 7. CLASS SECTION SPECIFIC STYLES

```css
#class-recordings {
	padding-top: 0;
}

#class-info {
	padding-bottom: 0;
}

#class-elearning {
	padding: 0;
}
```

---

## 8. COLOR REFERENCE

| Element            | Color     | Usage                              |
| ------------------ | --------- | ---------------------------------- |
| Body Background    | `#efefef` | Page background                    |
| Content Background | `#ffffff` | Main content areas                 |
| Text Color         | `#666666` | Body text                          |
| Link Color         | `#1e73be` | Links                              |
| Link Hover         | `#000000` | Link hover state                   |
| Video Background   | `#0a2335` | current-vid background (dark blue) |
| Video Title        | `#ffffff` | class-player-header color          |
| Video Subtitle     | `#d5d5d5` | current-title color                |
| Downloads BG       | `#ffffff` | class-subsection background        |
| Border Gray        | `#999`    | Video border                       |

---

## 9. FONT REFERENCE

| Usage               | Font Family             | Weight | Size   |
| ------------------- | ----------------------- | ------ | ------ |
| Body                | "Open Sans", sans-serif | 400    | 16px   |
| Class Player Header | "Open Sans", sans-serif | 600    | varies |
| Current Title       | "Open Sans", sans-serif | 500    | varies |
| Section H2          | "Open Sans", sans-serif | 700    | varies |

---

## 10. RESPONSIVE BREAKPOINTS

| Breakpoint          | Adjustments                                  |
| ------------------- | -------------------------------------------- |
| `max-width: 1140px` | Section padding: 0 20px                      |
| `max-width: 768px`  | Section padding: 40px 0, downloads margin: 0 |
| `max-width: 600px`  | Content block padding: 20px                  |
| `max-width: 415px`  | current-vid padding: 10px                    |

---

## 11. CRITICAL IMPLEMENTATION NOTES

1. **Body Background**: Must be `#efefef` globally (or `#FFFFFF` per user preference)
2. **Video Section**: Background `#0a2335` (dark blue) - different from indicators!
3. **Section Inner**: max-width 1100px, centered
4. **Downloads**: White background, iframe embedded
5. **Breadcrumbs**: Use DashboardBreadcrumbs component
6. **Mobile-First**: Base styles for mobile, scale up with `min-width`

---

## 12. COMPLETE PAGE STRUCTURE

```html
<!DOCTYPE html>
<html lang="en-US">
	<head>
		...
	</head>
	<body>
		<!-- Navigation -->
		<nav id="site-navigation">...</nav>

		<!-- Breadcrumbs -->
		<DashboardBreadcrumbs />

		<!-- Main Content -->
		<div id="page" class="hfeed site grid-parent">
			<div id="content" class="site-content">
				<!-- Class Recordings -->
				<section class="class-section cpost-section" id="class-recordings">
					<div class="section-inner">
						<div class="class-content-block cpost-content-block">
							<div class="current-vid">
								<h2 class="class-player-header">Class Title</h2>
								<h3 class="current-title">Class Subtitle</h3>
								<div class="class-video-container current">
									<video>...</video>
								</div>
							</div>
						</div>
					</div>
				</section>

				<!-- Downloads -->
				<section class="class-section cpost-section" id="dl-rp-row">
					<div class="section-inner">
						<section class="class-subsection" id="class-downloads">
							<h2>Class Downloads</h2>
							<div class="class-downloads-container">
								<iframe>...</iframe>
							</div>
						</section>
					</div>
				</section>
			</div>
		</div>

		<!-- Have Questions Footer -->
		<HaveQuestionsSection />

		<!-- Site Footer -->
		<footer>...</footer>
	</body>
</html>
```

---

## 13. KEY DIFFERENCES FROM INDICATOR PAGE

1. **Video Background**: `#0a2335` (dark blue) vs `#f4f4f4` (light gray)
2. **No Platform Logos**: Classes don't have ThinkorSwim/TradingView sections
3. **Downloads**: Box.com iframe embed instead of download tables
4. **Simpler Structure**: No st_box, no platform notes, no orange buttons
5. **Video Headers**: White text on dark background vs centered on light

---
