# ICT 11+ Schedule System - Deployment & Testing Guide

**Date:** January 4, 2026  
**System:** Database-Backed Trading Room Schedule Management  
**Status:** ✅ Code Complete - Ready for Deployment Testing

---

## 🚀 Deployment Status

### ✅ Completed
- [x] Database migration created (`013_trading_room_schedules.sql`)
- [x] Rust API endpoints implemented (`schedules.rs`)
- [x] Admin CMS interface created (`/admin/schedules`)
- [x] Frontend sidebar updated (`TradingRoomSidebar.svelte`)
- [x] Code compiled successfully
- [x] Changes committed and pushed to GitHub

### ⏳ Pending
- [ ] Run database migration on production
- [ ] Restart API server to load new routes
- [ ] Test all endpoints
- [ ] Verify frontend integration

---

## 📦 Deployment Steps

### Step 1: Run Database Migration

The migration file is located at: `api/migrations/013_trading_room_schedules.sql`

**Option A: Using psql**
```bash
psql $DATABASE_URL -f api/migrations/013_trading_room_schedules.sql
```

**Option B: Using your database management tool**
1. Connect to your Neon database
2. Execute the SQL file contents
3. Verify tables created:
   - `trading_room_schedules`
   - `schedule_exceptions`

**Verification Query:**
```sql
-- Check tables exist
SELECT table_name FROM information_schema.tables 
WHERE table_name IN ('trading_room_schedules', 'schedule_exceptions');

-- Check seeded data
SELECT COUNT(*) FROM trading_room_schedules;
-- Expected: 12 events (6 Day Trading, 3 Swing, 3 Small Account)

-- View sample schedules
SELECT 
    mp.name as room_name,
    trs.title,
    trs.trader_name,
    trs.day_of_week,
    trs.start_time,
    trs.end_time
FROM trading_room_schedules trs
JOIN membership_plans mp ON trs.plan_id = mp.id
ORDER BY mp.name, trs.day_of_week, trs.start_time;
```

---

### Step 2: Deploy Backend API

Your API should auto-deploy from GitHub. Once deployed:

1. **Verify API is running:**
   ```bash
   curl https://your-api-url.com/health
   ```

2. **Check new routes are loaded:**
   ```bash
   curl https://your-api-url.com/api/schedules/rooms
   ```

Expected response:
```json
{
  "rooms": [
    {
      "id": 1,
      "name": "Day Trading Room",
      "slug": "day-trading-room",
      "type": "trading-room"
    },
    ...
  ],
  "count": 6
}
```

---

### Step 3: Deploy Frontend

Frontend should auto-deploy via Cloudflare Pages. Verify:

1. **Check main site:**
   ```bash
   curl -I https://your-frontend-url.pages.dev
   ```

2. **Check admin page exists:**
   ```bash
   curl -I https://your-frontend-url.pages.dev/admin/schedules
   ```

3. **Check day trading room page:**
   ```bash
   curl -I https://your-frontend-url.pages.dev/dashboard/day-trading-room
   ```

---

## 🧪 Testing Guide

### Automated Testing

Use the provided test script:

```bash
# Set your URLs
export API_URL="https://your-api-url.com"
export FRONTEND_URL="https://your-frontend-url.pages.dev"

# Run tests
./test-schedule-system.sh
```

The script will test:
- ✅ All public API endpoints
- ✅ Response structure validation
- ✅ All 6 trading rooms
- ✅ Frontend accessibility
- ✅ Admin page existence

---

### Manual Testing

#### Test 1: Public API - Get Trading Rooms
```bash
curl -X GET https://your-api-url.com/api/schedules/rooms \
  -H "Content-Type: application/json" | jq
```

**Expected:** List of 6 trading rooms/services

---

#### Test 2: Public API - Get Day Trading Room Schedule
```bash
curl -X GET https://your-api-url.com/api/schedules/day-trading-room \
  -H "Content-Type: application/json" | jq
```

**Expected:** Weekly schedule with 6 events (Mon-Tue)

---

#### Test 3: Public API - Get Upcoming Events
```bash
curl -X GET "https://your-api-url.com/api/schedules/day-trading-room/upcoming?days=7" \
  -H "Content-Type: application/json" | jq
```

**Expected:** Upcoming events for next 7 days with calculated dates

---

#### Test 4: Frontend - Day Trading Room Sidebar

1. Navigate to: `https://your-frontend-url.pages.dev/dashboard/day-trading-room`
2. Check right sidebar for "Trading Room Schedule" section
3. Verify:
   - ✅ Schedule loads without errors
   - ✅ Events display with title, date, time
   - ✅ Trader names show (if present)
   - ✅ No Google Calendar API errors in console
   - ✅ Loading states work
   - ✅ Error handling works (test by blocking API)

---

#### Test 5: Admin CMS - Schedule Management

1. Navigate to: `https://your-frontend-url.pages.dev/admin/schedules`
2. Login as admin
3. Verify:
   - ✅ Room selector shows all 6 services
   - ✅ Can select each room
   - ✅ Weekly calendar grid displays
   - ✅ Existing schedules show correctly
   - ✅ Can create new event (click "+ Add Schedule Event")
   - ✅ Can edit existing event (click ✏️ icon)
   - ✅ Can delete event (click 🗑️ icon)
   - ✅ Form validation works
   - ✅ Success/error messages display

**Create Event Test:**
- Room: Day Trading Room
- Title: "Test Event"
- Trader: "Test Trader"
- Day: Wednesday
- Time: 14:00 - 15:00
- Click "Create"
- Verify event appears in Wednesday column

**Edit Event Test:**
- Click ✏️ on any event
- Change title to "Updated Event"
- Click "Update"
- Verify change appears immediately

**Delete Event Test:**
- Click 🗑️ on test event
- Confirm deletion
- Verify event removed from grid

---

## 🔍 Troubleshooting

### Issue: API returns 404 for schedule endpoints

**Cause:** Routes not loaded or API not restarted

**Solution:**
1. Verify `schedules.rs` is in `api/src/routes/`
2. Verify `mod.rs` includes schedules module
3. Restart API server
4. Check logs for compilation errors

---

### Issue: Database migration fails

**Cause:** Tables may already exist or permissions issue

**Solution:**
```sql
-- Check if tables exist
SELECT table_name FROM information_schema.tables 
WHERE table_name LIKE '%schedule%';

-- If tables exist, drop and recreate
DROP TABLE IF EXISTS schedule_exceptions CASCADE;
DROP TABLE IF EXISTS trading_room_schedules CASCADE;

-- Then run migration again
```

---

### Issue: Frontend shows "Failed to fetch schedules"

**Causes:**
1. API not running
2. CORS not configured
3. Wrong API URL in frontend

**Solution:**
1. Check API health endpoint
2. Verify CORS allows frontend domain
3. Check browser console for exact error
4. Verify API_URL environment variable

---

### Issue: Admin page shows "Unauthorized"

**Cause:** Not logged in as admin

**Solution:**
1. Login with admin credentials
2. Verify user has `admin` or `super_admin` role
3. Check session cookie is set

---

## 📊 Expected Data

### Day Trading Room (6 events)
- **Monday:**
  - Morning Market Analysis (09:20-10:00) - Taylor Horton
  - Pre-Market Prep (09:00-09:20) - Taylor Horton
  - Midday Review (12:00-12:30) - Taylor Horton
- **Tuesday:**
  - Morning Market Analysis (09:20-10:00) - Taylor Horton
  - Pre-Market Prep (09:00-09:20) - Taylor Horton
  - Midday Review (12:00-12:30) - Taylor Horton

### Swing Trading Room (3 events)
- **Monday:** Weekly Market Outlook (16:00-17:00) - John Carter
- **Wednesday:** Midweek Analysis (16:00-16:30) - John Carter
- **Friday:** Weekly Wrap-Up (15:30-16:30) - John Carter

### Small Account Mentorship (3 events)
- **Tuesday:** Small Account Strategy (18:00-19:00) - Raghee Horner
- **Thursday:** Risk Management Session (18:00-18:45) - Raghee Horner
- **Friday:** Weekly Review (17:00-18:00) - Raghee Horner

---

## ✅ Success Criteria

The system is working flawlessly when:

1. **Public API:**
   - ✅ All 6 rooms return schedules
   - ✅ Upcoming events calculate dates correctly
   - ✅ Response times < 200ms
   - ✅ No 500 errors

2. **Admin CMS:**
   - ✅ All CRUD operations work
   - ✅ Real-time updates
   - ✅ Form validation
   - ✅ Error handling

3. **Frontend Sidebar:**
   - ✅ Loads schedules from backend API
   - ✅ No Google Calendar API calls
   - ✅ Caching works (5-min TTL)
   - ✅ Graceful degradation
   - ✅ Loading states

4. **Database:**
   - ✅ All tables created
   - ✅ Seed data present
   - ✅ Audit triggers work
   - ✅ Constraints enforced

---

## 🎯 Next Steps After Testing

Once all tests pass:

1. **Add More Schedules:**
   - Use admin CMS to add schedules for:
     - Alerts Only
     - Explosive Swing
     - SPX Profit Pulse

2. **Configure Exceptions:**
   - Add holiday exceptions
   - Add special event times
   - Test exception handling

3. **Monitor Performance:**
   - Check API response times
   - Monitor database query performance
   - Review audit logs

4. **User Training:**
   - Document admin workflow
   - Create video tutorial
   - Train content managers

---

## 📝 API Endpoints Reference

### Public Endpoints (No Auth Required)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/schedules/rooms` | List all trading rooms |
| GET | `/api/schedules/:slug` | Get weekly schedule for room |
| GET | `/api/schedules/:slug/upcoming?days=N` | Get upcoming events |

### Admin Endpoints (Auth Required)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/admin/schedules` | List all schedules |
| GET | `/api/admin/schedules/plan/:id` | Get schedules for plan |
| POST | `/api/admin/schedules` | Create schedule event |
| PUT | `/api/admin/schedules/:id` | Update schedule event |
| DELETE | `/api/admin/schedules/:id` | Delete schedule event |
| POST | `/api/admin/schedules/bulk` | Bulk create/update |
| POST | `/api/admin/schedules/exceptions` | Create exception |
| DELETE | `/api/admin/schedules/exceptions/:id` | Delete exception |

---

## 🔐 Security Notes

- ✅ All admin endpoints require authentication
- ✅ AdminUser middleware validates admin role
- ✅ Comprehensive audit logging
- ✅ Input validation on all endpoints
- ✅ SQL injection prevention (parameterized queries)
- ✅ CORS configured for frontend domain

---

## 📞 Support

If you encounter issues:

1. Check this guide first
2. Review API logs for errors
3. Check browser console for frontend errors
4. Verify database connection
5. Ensure all migrations ran successfully

---

**This is the Apple Principal Engineer ICT 11+ way: comprehensive, documented, and flawless!** 🚀
