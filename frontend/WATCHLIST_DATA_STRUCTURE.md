# Weekly Watchlist Data Structure Documentation

## Critical Implementation Guide for Adding New Videos

### Data Structure Format

```typescript
interface WatchlistEntry {
	slug: string; // URL slug (format: MMDDYYYY-trader-name)
	title: string; // Full title
	trader: string; // Trader name only
	weekOf: string; // Display date (e.g., "January 3, 2026")
	poster: string; // Video thumbnail URL
	videoUrl: string; // MP4 video URL
	spreadsheetUrl?: string; // Single Google Sheets URL (fallback)
	isLatest?: boolean; // Mark as latest (optional)
	watchlistDates?: Array<{
		// Multiple date versions (RECOMMENDED)
		date: string; // Display date (e.g., "1/3/2026")
		spreadsheetUrl: string; // Google Sheets URL for this date
	}>;
}
```

### Adding a New Watchlist Video - Step by Step

#### 1. Upload Video to S3

```
URL Pattern: https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-[INITIALS]-[MMDDYYYY].mp4
Example: https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-MB-01032026.mp4
```

#### 2. Create Google Sheets Watchlist

- Create sheet in Google Sheets
- File → Share → Publish to web → Embed
- Copy the URL (format: `https://docs.google.com/spreadsheets/d/e/[SHEET_ID]/pubhtml`)

#### 3. Add Entry to watchlistEntries Array

**IMPORTANT: Add new entries at the TOP of the array (index 0)**

```typescript
const watchlistEntries = [
	// NEW ENTRY GOES HERE (NEWEST FIRST)
	{
		slug: '01122026-tg-watkins',
		title: 'Weekly Watchlist with TG Watkins',
		trader: 'TG Watkins',
		weekOf: 'January 12, 2026',
		poster:
			'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
		videoUrl: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-TG-01122026.mp4',
		isLatest: true, // Only set on the newest entry
		watchlistDates: [
			{
				date: '1/12/2026',
				spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-xxx/pubhtml'
			}
			// Add more dates if you update the watchlist mid-week
		]
	}
	// Older entries below...
];
```

### Date Switcher Functionality

#### How It Works

1. **Initial Load**: Shows first date (index 0) automatically
2. **Date Click**: User clicks a date → switches spreadsheet
3. **Arrow Navigation**:
   - `>` button → next date (index + 1)
   - `<` button → previous date (index - 1)
4. **Entry Switch**: When navigating to different watchlist → resets to index 0

#### Multiple Dates Example

```typescript
{
  slug: '01032026-melissa-beegle',
  title: 'Weekly Watchlist with Melissa Beegle',
  trader: 'Melissa Beegle',
  weekOf: 'January 3, 2026',
  poster: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
  videoUrl: 'https://cloud-streaming.s3.amazonaws.com/WeeklyWatchlist/WW-MB-01032026.mp4',
  watchlistDates: [
    {
      date: '1/3/2026',   // Most recent update
      spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0.../pubhtml'
    },
    {
      date: '5/28/2025',  // Mid-week update
      spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0.../pubhtml'
    },
    {
      date: '3/9/2025',   // Original
      spreadsheetUrl: 'https://docs.google.com/spreadsheets/d/e/2PACX-1vS0DkJXxG0.../pubhtml'
    }
  ]
}
```

### Critical Safeguards Implemented

#### 1. Index Reset on Entry Change

```typescript
// When user navigates to different watchlist, date index resets to 0
selectedDateIndex = 0;
```

#### 2. Bounds Validation

```typescript
// Prevents invalid index access
if (!currentEntry?.watchlistDates || index < 0 || index >= currentEntry.watchlistDates.length) {
	console.warn('Invalid date index:', index);
	return;
}
```

#### 3. Auto-Correction

```typescript
// If selectedDateIndex exceeds available dates, auto-correct to last valid index
const validIndex = Math.min(selectedDateIndex, currentEntry.watchlistDates.length - 1);
```

#### 4. Fallback Handling

```typescript
// Priority order:
// 1. watchlistDates array (preferred)
// 2. Single spreadsheetUrl (fallback)
// 3. Empty string with warning
```

### Navigation Logic

#### Previous/Next Watchlist

```typescript
// Array order: [newest, ..., oldest]
// Previous = older (index + 1)
// Next = newer (index - 1)

previousEntry =
	currentIndex < watchlistEntries.length - 1 ? watchlistEntries[currentIndex + 1] : null;

nextEntry = currentIndex > 0 ? watchlistEntries[currentIndex - 1] : null;
```

### Testing Checklist

When adding a new video, verify:

- [ ] Video plays correctly
- [ ] Spreadsheet loads in Watchlist tab
- [ ] Date switcher shows all dates
- [ ] Clicking dates switches spreadsheet
- [ ] Arrow buttons work (disabled at boundaries)
- [ ] Previous/Next navigation works
- [ ] Breadcrumbs show correct title
- [ ] Page title is correct
- [ ] Mobile responsive layout works

### Common Issues & Solutions

#### Issue: Date switcher not showing

**Solution**: Ensure `watchlistDates` array has more than 1 entry

#### Issue: Wrong spreadsheet loads

**Solution**: Verify `spreadsheetUrl` in correct date object

#### Issue: Navigation broken

**Solution**: Check slug format matches exactly (MMDDYYYY-trader-name)

#### Issue: Video not playing

**Solution**: Verify S3 URL is public and MP4 format

### Future Automation Plan

When ready to automate:

1. **API Endpoint**: `/api/watchlist` (GET, POST, PUT, DELETE)
2. **Database Schema**: Match this TypeScript interface
3. **Admin Panel**: Form to add video URL, Google Sheets URL, dates
4. **Auto-Latest**: Newest entry (by date) automatically marked as latest
5. **Validation**: Server-side checks for URL formats, required fields

### File Locations

- **Page Component**: `/frontend/src/routes/watchlist/[slug]/+page.svelte`
- **Data Source**: Currently in component (line 23-59)
- **Future API**: `/frontend/src/routes/api/watchlist/+server.ts` (to be created)

---

**Last Updated**: January 13, 2026  
**Status**: Production Ready - Manual Entry  
**Next Phase**: API Automation
