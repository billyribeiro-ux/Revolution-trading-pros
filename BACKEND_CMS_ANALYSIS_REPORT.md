# Backend Headless CMS Structure Analysis
**ICT 11+ Apple Principal Engineer Report**  
**Date:** January 4, 2026  
**Objective:** Analyze backend structure for per-room schedule management system

---

## 📊 Executive Summary

**Current State:** Google Calendar API integration (single shared calendar)  
**Limitation:** All trading rooms share one calendar  
**Requirement:** Each trading room needs its own schedule, updated weekly

**Recommendation:** Implement database-backed schedule system with CMS interface

---

## 🗄️ Database Schema Analysis

### **1. Membership Plans Table**

**Location:** `api/migrations/001_initial_schema.sql:216-230`

```sql
CREATE TABLE membership_plans (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    billing_cycle VARCHAR(20) NOT NULL,
    is_active BOOLEAN DEFAULT true,
    metadata JSONB,              -- ✅ Can store schedule config
    stripe_price_id VARCHAR(255),
    features JSONB,
    trial_days INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

**Current Services (from `008_seed_membership_plans.sql`):**

**Trading Rooms:**
1. **Day Trading Room** (`day-trading-room`)
   - Type: `trading-room`
   - Price: $197/month
   - Icon: `chart-line`

2. **Swing Trading Room** (`swing-trading-room`)
   - Type: `trading-room`
   - Price: $147/month
   - Icon: `trending-up`

3. **Small Account Mentorship** (`small-account-mentorship`)
   - Type: `trading-room`
   - Price: $97/month
   - Icon: `wallet`

**Alert Services:**
4. **Alerts Only** (`alerts-only`)
   - Type: `alert-service`
   - Price: $97/month

5. **Explosive Swing** (`explosive-swing`)
   - Type: `alert-service`
   - Price: $147/month

6. **SPX Profit Pulse** (`spx-profit-pulse`)
   - Type: `alert-service`
   - Price: $197/month

---

## 🏗️ Current Architecture

### **API Endpoints Structure**

**Existing Routes (from `api/src/routes/`):**
- ✅ `/api/subscriptions/plans` - List membership plans
- ✅ `/api/subscriptions/plans/:slug` - Get plan details
- ✅ `/api/my/subscriptions` - User's subscriptions
- ✅ `/api/admin/*` - Admin endpoints (39KB file)

**Missing:**
- ❌ No schedule endpoints
- ❌ No schedule table in database
- ❌ No schedule management in admin panel

---

## 🎯 Proposed Solution: ICT 11+ Schedule System

### **Option 1: JSONB in metadata (Quick Implementation)**

**Pros:**
- ✅ No new tables needed
- ✅ Quick to implement
- ✅ Flexible structure
- ✅ Works with existing schema

**Cons:**
- ❌ Harder to query
- ❌ No referential integrity
- ❌ Limited indexing

**Structure:**
```json
{
  "type": "trading-room",
  "icon": "chart-line",
  "schedule": {
    "timezone": "America/New_York",
    "events": [
      {
        "id": "uuid",
        "title": "Morning Market Analysis",
        "trader": "Taylor Horton",
        "day_of_week": 1,
        "start_time": "09:20",
        "end_time": "10:00",
        "recurring": true,
        "active": true
      }
    ]
  }
}
```

---

### **Option 2: Dedicated Schedule Tables (Recommended - ICT 11+)**

**Pros:**
- ✅ Proper relational structure
- ✅ Easy to query and filter
- ✅ Referential integrity
- ✅ Indexable for performance
- ✅ Audit trail support
- ✅ Scalable for future features

**Cons:**
- ⚠️ Requires migration
- ⚠️ More complex implementation

**Proposed Schema:**

```sql
-- ═══════════════════════════════════════════════════════════════════════════
-- TRADING ROOM SCHEDULES
-- ═══════════════════════════════════════════════════════════════════════════

CREATE TABLE trading_room_schedules (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES membership_plans(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    trader_name VARCHAR(255),
    trader_id BIGINT REFERENCES users(id),
    
    -- Scheduling
    day_of_week INTEGER NOT NULL, -- 0=Sunday, 1=Monday, etc.
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    timezone VARCHAR(50) DEFAULT 'America/New_York',
    
    -- Recurrence
    is_recurring BOOLEAN DEFAULT true,
    start_date DATE,
    end_date DATE,
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    is_cancelled BOOLEAN DEFAULT false,
    
    -- Metadata
    metadata JSONB,
    room_url VARCHAR(500),
    
    -- Audit
    created_by BIGINT REFERENCES users(id),
    updated_by BIGINT REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT valid_day_of_week CHECK (day_of_week >= 0 AND day_of_week <= 6),
    CONSTRAINT valid_time_range CHECK (end_time > start_time)
);

CREATE INDEX idx_schedules_plan ON trading_room_schedules(plan_id);
CREATE INDEX idx_schedules_day ON trading_room_schedules(day_of_week);
CREATE INDEX idx_schedules_active ON trading_room_schedules(is_active);
CREATE INDEX idx_schedules_date_range ON trading_room_schedules(start_date, end_date);

-- Schedule exceptions (holidays, special events, cancellations)
CREATE TABLE schedule_exceptions (
    id BIGSERIAL PRIMARY KEY,
    schedule_id BIGINT NOT NULL REFERENCES trading_room_schedules(id) ON DELETE CASCADE,
    exception_date DATE NOT NULL,
    exception_type VARCHAR(20) NOT NULL, -- cancelled, rescheduled, special
    new_start_time TIME,
    new_end_time TIME,
    reason TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(schedule_id, exception_date)
);

CREATE INDEX idx_exceptions_schedule ON schedule_exceptions(schedule_id);
CREATE INDEX idx_exceptions_date ON schedule_exceptions(exception_date);
```

---

## 🔌 API Endpoints Design

### **Schedule Management Endpoints**

```typescript
// Public - Get schedule for a specific trading room
GET /api/schedules/:plan_slug
Response: {
  plan: { id, name, slug },
  timezone: "America/New_York",
  events: [
    {
      id: 1,
      title: "Morning Market Analysis",
      trader: "Taylor Horton",
      dayOfWeek: 1, // Monday
      startTime: "09:20",
      endTime: "10:00",
      isActive: true
    }
  ]
}

// Public - Get upcoming events for next N days
GET /api/schedules/:plan_slug/upcoming?days=7
Response: {
  events: [
    {
      id: 1,
      title: "Morning Market Analysis",
      trader: "Taylor Horton",
      dateTime: "2026-01-06T09:20:00-05:00",
      endDateTime: "2026-01-06T10:00:00-05:00"
    }
  ]
}

// Admin - Create schedule event
POST /api/admin/schedules
Body: {
  planId: 1,
  title: "Morning Analysis",
  traderName: "Taylor Horton",
  dayOfWeek: 1,
  startTime: "09:20",
  endTime: "10:00",
  isRecurring: true
}

// Admin - Update schedule event
PUT /api/admin/schedules/:id
Body: { ... }

// Admin - Delete schedule event
DELETE /api/admin/schedules/:id

// Admin - Bulk update (for weekly schedule changes)
POST /api/admin/schedules/bulk
Body: {
  planId: 1,
  events: [...]
}

// Admin - Add exception (holiday, cancellation)
POST /api/admin/schedules/:id/exceptions
Body: {
  exceptionDate: "2026-12-25",
  exceptionType: "cancelled",
  reason: "Christmas Holiday"
}
```

---

## 🎨 Admin CMS Interface Design

### **Schedule Management Page**

**Route:** `/admin/schedules`

**Features:**
1. **Trading Room Selector**
   - Dropdown to select which room to manage
   - Shows: Day Trading Room, Swing Trading Room, Small Account Mentorship

2. **Weekly Calendar View**
   - Visual grid showing all scheduled events
   - Drag-and-drop to reschedule
   - Color-coded by trader

3. **Event Form**
   - Title, Trader, Day, Start Time, End Time
   - Recurring toggle
   - Date range (optional)
   - Room URL (optional)

4. **Bulk Actions**
   - Copy schedule from previous week
   - Clear all events
   - Import from CSV
   - Export to CSV

5. **Exception Management**
   - Mark specific dates as cancelled
   - Add special events
   - Holiday calendar integration

---

## 📱 Frontend Integration

### **Update TradingRoomSidebar Component**

**Current:** Google Calendar API  
**New:** Backend API

```typescript
// Instead of Google Calendar API
const response = await fetch(`/api/schedules/${planSlug}/upcoming?days=7`);
const data = await response.json();
scheduleEvents = data.events;
```

**Benefits:**
- ✅ No Google API dependency
- ✅ Faster response (database query)
- ✅ Better caching
- ✅ Per-room schedules
- ✅ Admin-controlled content

---

## 🔄 Migration Strategy

### **Phase 1: Database Setup (Week 1)**
1. Create migration file for schedule tables
2. Run migration on development
3. Seed with current Google Calendar data
4. Test queries and indexes

### **Phase 2: API Development (Week 1-2)**
1. Create schedule routes in `api/src/routes/schedules.rs`
2. Implement CRUD operations
3. Add admin endpoints
4. Add validation and error handling
5. Write tests

### **Phase 3: Admin Interface (Week 2)**
1. Create schedule management page
2. Build weekly calendar view
3. Implement event forms
4. Add bulk operations
5. Test with real data

### **Phase 4: Frontend Integration (Week 2-3)**
1. Update TradingRoomSidebar component
2. Replace Google Calendar API calls
3. Add per-room schedule support
4. Test caching and performance
5. Deploy to staging

### **Phase 5: Data Migration (Week 3)**
1. Export current Google Calendar events
2. Import into database
3. Verify data accuracy
4. Update documentation

### **Phase 6: Production Deployment (Week 3-4)**
1. Deploy backend changes
2. Deploy frontend changes
3. Monitor performance
4. Train admins on new interface
5. Deprecate Google Calendar integration

---

## 💰 Cost-Benefit Analysis

### **Current System (Google Calendar)**
**Costs:**
- ❌ Single shared calendar (no per-room separation)
- ❌ External API dependency
- ❌ Rate limits and quotas
- ❌ Requires Google API credentials
- ❌ No admin control without Google Calendar access

**Benefits:**
- ✅ Already implemented
- ✅ Familiar interface (Google Calendar)

### **Proposed System (Database-backed)**
**Costs:**
- ⚠️ Development time (2-3 weeks)
- ⚠️ Additional database storage (minimal)
- ⚠️ Admin training required

**Benefits:**
- ✅ Per-room schedules
- ✅ Full admin control via CMS
- ✅ No external dependencies
- ✅ Better performance
- ✅ Easier to maintain
- ✅ Scalable for future features
- ✅ Audit trail
- ✅ Exception handling (holidays, cancellations)

---

## 🎯 Recommendation

**Implement Option 2: Dedicated Schedule Tables**

**Rationale:**
1. **Scalability:** Proper relational structure supports future growth
2. **Maintainability:** Easier to query, update, and debug
3. **Performance:** Database queries faster than external API calls
4. **Control:** Full admin control without external dependencies
5. **Flexibility:** Per-room schedules with exception handling
6. **Reliability:** No external API failures or rate limits

**Timeline:** 3-4 weeks for complete implementation

**Priority:** High (required for proper multi-room schedule management)

---

## 📝 Next Steps

1. **User Approval:** Confirm approach (Option 1 vs Option 2)
2. **Create Migration:** Write SQL migration for schedule tables
3. **Implement API:** Build schedule endpoints in Rust
4. **Build Admin UI:** Create schedule management interface
5. **Update Frontend:** Integrate new API in TradingRoomSidebar
6. **Test & Deploy:** Comprehensive testing and production deployment

---

**Ready to proceed with implementation?**
