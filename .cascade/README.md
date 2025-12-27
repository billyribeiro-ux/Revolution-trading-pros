# Cascade AI Agent Instructions

## Critical Reference Files

### Frontend Dashboard Reference Files
**Location:** `frontend/Do's/`

This directory contains HTML reference files that define the exact structure and functionality for dashboard pages. These are the **source of truth** for dashboard implementation.

**MANDATORY:** Before working on any dashboard-related code, read:
1. `frontend/Do's/README.md` - Complete documentation
2. The specific reference file for the page you're working on
3. `frontend/.ai-context.md` - AI agent context and guidelines

### Available Reference Files
- `DashboardHome` - Main dashboard landing page
- `DayTradingRoomDashboard` - Day trading room interface
- `LatestUpdates` - Latest updates page
- `Learning-Center` - Learning center main page
- `Platform-Tutorials` - Platform tutorials section
- `Premium-Daily-Videos` - Premium videos listing
- `Premium-Daily-Videos-Clicked` - Individual video view
- `wwclickememberdashboard` - Member dashboard interactions

## Architecture Documentation

**Essential reading:**
- `frontend/ARCHITECTURE.md` - Complete frontend architecture
- `frontend/CSS_ARCHITECTURE.md` - Styling guidelines
- `frontend/.ai-context.md` - Quick reference for AI agents
- `api/README.md` - Backend API documentation

## Workflow for Dashboard Changes

1. **Check Reference Files First**
   - Navigate to `frontend/Do's/`
   - Read the relevant reference file
   - Understand the expected structure, styling, and functionality

2. **Review Architecture**
   - Check `frontend/ARCHITECTURE.md` for component patterns
   - Use existing components from `$lib/components`
   - Follow design token system in `app.css`

3. **Implement Changes**
   - Match the structure from reference files
   - Reuse components where possible
   - Maintain consistency with existing code

4. **Test Thoroughly**
   - Verify against reference file expectations
   - Test responsive behavior
   - Check accessibility

## Tech Stack

- **Frontend:** SvelteKit 5, Svelte 5, Tailwind CSS 4, TypeScript 5
- **Backend:** Rust, Axum
- **Database:** Neon PostgreSQL
- **Cache:** Upstash Redis

## Key Principles

1. **Reference First:** Always consult `Do's/` directory before dashboard work
2. **Component Reuse:** Use `$lib/components` (ui, patterns, layout)
3. **Design Tokens:** Use tokens from `app.css` `@theme` directive
4. **Performance:** Code-split heavy libraries, lazy load components
5. **Consistency:** Match existing patterns and conventions

## Common Mistakes to Avoid

- ❌ Implementing dashboard pages without checking `Do's/` reference files
- ❌ Creating new components when existing ones can be reused
- ❌ Hardcoding colors instead of using design tokens
- ❌ Ignoring the architecture documentation
- ❌ Not following the established file structure

## Questions?

Refer to:
- `frontend/.ai-context.md` - Quick AI agent guide
- `frontend/ARCHITECTURE.md` - Detailed architecture
- `frontend/Do's/README.md` - Dashboard reference documentation
