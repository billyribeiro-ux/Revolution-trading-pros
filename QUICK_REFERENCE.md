# Quick Reference Guide - Revolution Trading Pros

## 🚀 Development Commands

### Frontend (SvelteKit)
```bash
cd frontend
npm run dev          # Start dev server (http://localhost:5173)
npm run build        # Production build
npm run preview      # Preview production build
npm run check        # Type checking & linting
npm run test         # Run Playwright E2E tests
```

### Backend (Laravel)
```bash
cd backend
php artisan serve    # Start dev server (http://localhost:8000)
php artisan migrate  # Run migrations
php artisan db:seed  # Seed database
php artisan db:seed --class=CleanDatabaseSeeder  # Clean test data
```

## 📁 Key File Locations

### Frontend
- **Routes:** `frontend/src/routes/`
- **Components:** `frontend/src/lib/components/`
- **Stores:** `frontend/src/lib/stores/`
- **API Client:** `frontend/src/lib/api/`
- **Tests:** `frontend/tests/comprehensive-e2e.spec.ts`

### Backend
- **Controllers:** `backend/app/Http/Controllers/`
- **Models:** `backend/app/Models/`
- **Routes:** `backend/routes/api.php`
- **Database:** `backend/database/`
- **Seeders:** `backend/database/seeders/`

## 🔗 Route Structure

### Public Routes
- `/` - Home
- `/about` - About Us
- `/our-mission` - Mission Statement
- `/mentorship` - Mentorship Program
- `/blog` - Blog Listing
- `/courses` - Course Catalog
- `/indicators` - Indicator Catalog
- `/resources` - Free Resources

### Alert Services
- `/alerts/spx-profit-pulse` - SPX Profit Pulse
- `/alerts/explosive-swings` - Explosive Swings

### Trading Rooms
- `/live-trading-rooms/day-trading` - Day Trading Room
- `/live-trading-rooms/swing-trading` - Swing Trading Room
- `/live-trading-rooms/small-accounts` - Small Accounts Room

### E-commerce
- `/cart` - Shopping Cart
- `/checkout` - Checkout Process

### Authentication
- `/login` - User Login
- `/register` - User Registration
- `/forgot-password` - Password Recovery
- `/reset-password` - Password Reset

### Admin Panel
- `/admin` - Dashboard Overview
- `/admin/members` - Member Management
- `/admin/analytics` - Analytics Dashboard
- `/admin/subscriptions` - Subscription Management
- `/admin/blog` - Blog Management
- `/admin/email` - Email Campaigns
- `/admin/seo` - SEO Tools

## 🧪 Testing

### Run All E2E Tests
```bash
cd frontend
npm run test
```

### Run Specific Test
```bash
npx playwright test --grep "home page"
```

### Run in UI Mode
```bash
npx playwright test --ui
```

## 🗄️ Database Management

### Clean Test Data
```bash
cd backend
php artisan db:seed --class=CleanDatabaseSeeder
```

### Fresh Migration
```bash
php artisan migrate:fresh --seed
```

### Create New Migration
```bash
php artisan make:migration create_table_name
```

## 🔧 Common Tasks

### Add New Route
1. Create file in `frontend/src/routes/your-route/+page.svelte`
2. Add to navigation in `frontend/src/lib/components/NavBar.svelte`
3. Update tests in `frontend/tests/comprehensive-e2e.spec.ts`

### Add New API Endpoint
1. Create controller: `php artisan make:controller YourController`
2. Add route in `backend/routes/api.php`
3. Update frontend API client in `frontend/src/lib/api/`

### Fix Build Errors
```bash
# Frontend
cd frontend
npm run check

# Backend
cd backend
composer install
php artisan config:clear
php artisan cache:clear
```

## 📊 Environment Variables

### Frontend (.env)
```env
VITE_API_URL=http://localhost:8000
```

### Backend (.env)
```env
APP_URL=http://localhost:8000
DB_CONNECTION=sqlite
DB_DATABASE=/path/to/database.sqlite
```

## 🐛 Debugging

### Frontend Console Errors
- Check browser console (F12)
- Review `npm run check` output
- Verify API endpoint URLs

### Backend API Errors
- Check `backend/storage/logs/laravel.log`
- Use `dd()` for debugging
- Enable debug mode in `.env`: `APP_DEBUG=true`

## 📝 Git Workflow

### Standard Workflow
```bash
git status                    # Check changes
git add -A                    # Stage all changes
git commit -m "message"       # Commit with message
git push origin main          # Push to remote
```

### Commit Message Format
```
type: brief description

- Detail 1
- Detail 2
```

**Types:** `feat`, `fix`, `chore`, `docs`, `style`, `refactor`, `test`

## 🎯 Performance Tips

1. **Use Svelte 5 runes** - Modern reactive syntax
2. **Lazy load routes** - Split code by route
3. **Optimize images** - Use WebP format
4. **Cache API calls** - Use enhanced client caching
5. **Monitor bundle size** - Keep under 200KB initial

## 🔐 Security Checklist

- [ ] Environment variables not committed
- [ ] API keys stored securely
- [ ] CSRF protection enabled
- [ ] Input validation on all forms
- [ ] SQL injection prevention (use Eloquent)
- [ ] XSS protection (Svelte auto-escapes)

## 📚 Documentation Links

- **SvelteKit:** https://kit.svelte.dev/docs
- **Svelte 5:** https://svelte.dev/docs/svelte/overview
- **Laravel:** https://laravel.com/docs
- **Playwright:** https://playwright.dev/docs/intro
- **Tabler Icons:** https://tabler.io/icons

## 🆘 Need Help?

1. Check `CLEANUP_SUMMARY.md` for recent changes
2. Review comprehensive E2E tests for examples
3. Check Laravel logs: `backend/storage/logs/`
4. Check browser console for frontend errors
5. Run `npm run check` and `php artisan config:clear`
