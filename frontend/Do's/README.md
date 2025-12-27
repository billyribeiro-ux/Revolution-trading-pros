# Dashboard Reference Files - CRITICAL FOR AI AGENTS

**⚠️ IMPORTANT: All AI agents working on this project MUST reference these files before making changes to dashboard pages.**

## Purpose

This directory contains HTML reference files that define the **exact structure, styling, and functionality** for each dashboard page in the Revolution Trading Pros application. These files serve as the **source of truth** for how pages should be implemented.

## Reference Files

| File Name | Page Description | Route |
|-----------|-----------------|-------|
| `DashboardHome` | Main member dashboard landing page | `/dashboard` |
| `DayTradingRoomDashboard` | Day trading room interface | `/dashboard/day-trading-room` |
| `LatestUpdates` | Latest updates and announcements | `/dashboard/latest-updates` |
| `Learning-Center` | Learning center main page | `/dashboard/learning-center` |
| `Platform-Tutorials` | Platform tutorials section | `/dashboard/platform-tutorials` |
| `Premium-Daily-Videos` | Premium daily videos listing | `/dashboard/premium-daily-videos` |
| `Premium-Daily-Videos-Clicked` | Individual premium video view | `/dashboard/premium-daily-videos/[id]` |
| `wwclickememberdashboard` | Member dashboard click interaction | `/dashboard/member` |

## Usage Instructions for AI Agents

### Before Making Changes
1. **ALWAYS** check this directory first when working on dashboard pages
2. **READ** the corresponding reference file to understand the expected structure
3. **MATCH** the HTML structure, CSS classes, and component hierarchy
4. **PRESERVE** the original design intent and user experience

### When Implementing Features
- Use these files as the **blueprint** for component structure
- Extract CSS class names and styling patterns
- Identify required JavaScript functionality
- Note breadcrumb patterns, meta tags, and SEO requirements

### File Format
Each file contains:
- Original URL from simplertrading.com
- Target URL for our application
- Breadcrumb structure
- Complete HTML markup
- Meta tags and SEO data
- JavaScript dependencies
- CSS class references

## Integration with Codebase

These reference files should be consulted when working on:
- `/frontend/src/routes/dashboard/**` - All dashboard routes
- `/frontend/src/lib/components/dashboard/**` - Dashboard components
- Dashboard-related styling and layouts

## Maintenance

- **DO NOT** delete or modify these reference files without explicit approval
- Keep these files synchronized with any major design changes
- Add new reference files when new dashboard pages are created

---

**🤖 AI Agent Note:** This directory is your primary reference for dashboard implementation. Treat it as required reading before any dashboard-related work.
