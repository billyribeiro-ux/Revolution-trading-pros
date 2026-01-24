# Admin CMS Resources Management System - ICT 7 Implementation Plan
**Apple Principal Engineer ICT 7 Grade - January 2026**

---

## 🎯 Executive Summary

Complete overhaul of the Admin CMS for managing Trading Room Resources, Courses, and Indicators with:
- **Beautiful, modern UI/UX** inspired by Apple's design language
- **Drag & drop video upload** with automatic URL generation
- **Section-based organization** per trading room (Introduction, Latest Updates, Premium Daily Videos, Watchlist, Weekly Alerts, Learning Center)
- **Unified workflow** for Courses and Indicators with automatic card generation
- **Missing trading rooms** added to the system

---

## 🚨 Critical Issues Identified

### 1. **Missing Trading Rooms**
Current fallback only has 3 rooms:
- ✅ Day Trading Room
- ❌ **Swing Trading Room** (MISSING)
- ❌ **Small Account Mentorship** (MISSING)
- ✅ Explosive Swings
- ❌ **SPX Profit Pulse** (MISSING)
- ✅ High Octane Scanner

**Required Order:**
1. Day Trading Room
2. Swing Trading Room
3. Small Account Mentorship
4. Explosive Swings
5. SPX Profit Pulse
6. High Octane Scanner

### 2. **No Introduction Section**
Every trading room dashboard needs an **Introduction Section** for main/hero videos, but this doesn't exist in the current `content_type` options.

### 3. **Section-Based Organization Missing**
Current implementation uses generic `content_type` (tutorial, daily_video, weekly_watchlist, etc.) but doesn't map to the actual dashboard sections:
- Introduction (main videos)
- Latest Updates
- Premium Daily Videos
- Watchlist
- Weekly Alerts (Explosive Swings specific)
- Learning Center

### 4. **No Drag & Drop Video Upload**
Current implementation requires manual URL entry. Need integrated Bunny.net upload with automatic URL generation.

### 5. **Courses & Indicators Card Generation**
Need to verify automatic card generation when videos/thumbnails are uploaded.

---

## 📋 Implementation Plan

### **Phase 1: Database Schema Updates** ⏱️ 2 hours

#### 1.1 Create Trading Rooms Master Table
```sql
-- New migration: 20260115_000001_trading_rooms_master.sql

CREATE TABLE IF NOT EXISTS trading_rooms (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    type VARCHAR(50) NOT NULL DEFAULT 'trading_room',
    -- Values: 'trading_room', 'alert_service', 'mentorship'
    
    description TEXT,
    short_description TEXT,
    
    -- Visual branding
    icon VARCHAR(100),
    color VARCHAR(20),
    logo_url VARCHAR(500),
    image_url VARCHAR(500),
    
    -- Display order (critical for admin UI)
    sort_order INTEGER NOT NULL DEFAULT 0,
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    is_featured BOOLEAN DEFAULT false,
    
    -- Metadata
    features JSONB DEFAULT '[]'::jsonb,
    metadata JSONB DEFAULT '{}'::jsonb,
    
    -- Audit
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_trading_rooms_slug ON trading_rooms(slug);
CREATE INDEX idx_trading_rooms_sort_order ON trading_rooms(sort_order);
CREATE INDEX idx_trading_rooms_active ON trading_rooms(is_active) WHERE is_active = true;

-- Seed all 6 trading rooms in correct order
INSERT INTO trading_rooms (name, slug, type, sort_order, is_active, is_featured, icon, color) VALUES
('Day Trading Room', 'day-trading-room', 'trading_room', 1, true, true, 'chart-line', '#3b82f6'),
('Swing Trading Room', 'swing-trading-room', 'trading_room', 2, true, false, 'trending-up', '#10b981'),
('Small Account Mentorship', 'small-account-mentorship', 'mentorship', 3, true, false, 'wallet', '#f59e0b'),
('Explosive Swings', 'explosive-swings', 'alert_service', 4, true, false, 'rocket', '#ef4444'),
('SPX Profit Pulse', 'spx-profit-pulse', 'alert_service', 5, true, false, 'activity', '#8b5cf6'),
('High Octane Scanner', 'high-octane-scanner', 'alert_service', 6, true, false, 'radar', '#06b6d4')
ON CONFLICT (slug) DO UPDATE SET
    name = EXCLUDED.name,
    sort_order = EXCLUDED.sort_order,
    icon = EXCLUDED.icon,
    color = EXCLUDED.color,
    updated_at = NOW();
```

#### 1.2 Update room_resources Table - Add Section Field
```sql
-- Migration: 20260115_000002_add_resource_sections.sql

ALTER TABLE room_resources 
ADD COLUMN IF NOT EXISTS section VARCHAR(100);

-- Section values:
-- 'introduction' - Main/hero videos for the room
-- 'latest_updates' - Recent updates and announcements
-- 'premium_daily_videos' - Daily trading videos
-- 'watchlist' - Weekly watchlist videos
-- 'weekly_alerts' - Weekly alert videos (Explosive Swings)
-- 'learning_center' - Educational content and tutorials

CREATE INDEX idx_room_resources_section ON room_resources(section);
CREATE INDEX idx_room_resources_room_section ON room_resources(trading_room_id, section);

-- Update content_type to include 'introduction'
ALTER TABLE room_resources DROP CONSTRAINT IF EXISTS room_resources_content_type_check;
ALTER TABLE room_resources ADD CONSTRAINT room_resources_content_type_check 
CHECK (content_type IN (
    'introduction', 'tutorial', 'daily_video', 'weekly_watchlist', 
    'trade_plan', 'guide', 'chart', 'screenshot', 'template', 
    'cheat_sheet', 'weekly_alert', 'other'
));
```

#### 1.3 Add Bunny.net Upload Tracking
```sql
-- Migration: 20260115_000003_bunny_upload_tracking.sql

CREATE TABLE IF NOT EXISTS bunny_uploads (
    id BIGSERIAL PRIMARY KEY,
    
    -- Upload tracking
    upload_id VARCHAR(255) UNIQUE,
    video_guid VARCHAR(255) UNIQUE,
    library_id BIGINT NOT NULL,
    
    -- File info
    original_filename VARCHAR(500),
    file_size BIGINT,
    mime_type VARCHAR(100),
    
    -- Processing status
    status VARCHAR(50) DEFAULT 'uploading',
    -- Values: uploading, processing, ready, failed
    
    -- URLs (auto-generated after processing)
    video_url VARCHAR(1000),
    thumbnail_url VARCHAR(1000),
    
    -- Metadata
    duration INTEGER,
    width INTEGER,
    height INTEGER,
    
    -- Link to resource (nullable until assigned)
    resource_id BIGINT REFERENCES room_resources(id) ON DELETE SET NULL,
    
    -- Audit
    uploaded_by BIGINT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_bunny_uploads_status ON bunny_uploads(status);
CREATE INDEX idx_bunny_uploads_video_guid ON bunny_uploads(video_guid);
```

---

### **Phase 2: Backend API Endpoints** ⏱️ 4 hours

#### 2.1 Trading Rooms API (`api/src/routes/trading_rooms.rs`)
```rust
// GET /api/trading-rooms - List all rooms in correct order
// GET /api/trading-rooms/:slug - Get room details
// GET /api/admin/trading-rooms - Admin list with stats
// POST /api/admin/trading-rooms - Create room
// PUT /api/admin/trading-rooms/:id - Update room
// DELETE /api/admin/trading-rooms/:id - Delete room
```

#### 2.2 Room Resources API with Sections (`api/src/routes/room_resources.rs`)
```rust
// GET /api/room-resources?room_id=1&section=introduction
// GET /api/room-resources?room_id=1&section=latest_updates
// GET /api/room-resources?room_id=1&section=premium_daily_videos
// GET /api/room-resources?room_id=1&section=watchlist
// GET /api/room-resources?room_id=1&section=weekly_alerts
// GET /api/room-resources?room_id=1&section=learning_center

// Admin endpoints
// POST /api/admin/room-resources - Create with section
// PUT /api/admin/room-resources/:id - Update
// DELETE /api/admin/room-resources/:id - Delete
```

#### 2.3 Bunny.net Upload API (`api/src/routes/bunny_upload.rs`)
```rust
// POST /api/admin/bunny/upload - Initiate upload, return upload URL
// POST /api/admin/bunny/webhook - Bunny.net webhook for processing status
// GET /api/admin/bunny/status/:upload_id - Check upload status
// POST /api/admin/bunny/assign/:upload_id - Assign to resource
```

---

### **Phase 3: Frontend - Admin Resources UI Overhaul** ⏱️ 8 hours

#### 3.1 Update `/admin/resources/+page.svelte`

**Key Changes:**
1. **Fix FALLBACK_ROOMS** - Add all 6 rooms in correct order
2. **Add Section Selector** - Dropdown/tabs for Introduction, Latest Updates, etc.
3. **Drag & Drop Upload Zone** - Integrated Bunny.net uploader
4. **Beautiful Card Layout** - Apple-inspired design with smooth animations
5. **Section-Specific Fields** - Show/hide fields based on selected section

**UI Structure:**
```
┌─────────────────────────────────────────────────────────────┐
│  Trading Room Resources Management                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  [Day Trading] [Swing Trading] [Small Account] [Explosive]  │
│  [SPX Profit Pulse] [High Octane Scanner]                   │
│                                                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Section: [Introduction ▼]                                  │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  📤 Drag & Drop Video Here                         │    │
│  │     or click to browse                             │    │
│  │                                                     │    │
│  │  Supports: MP4, MOV, AVI (max 5GB)                │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  [+ Add Existing Video URL]                                 │
│                                                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Introduction Videos (2)                                    │
│                                                              │
│  ┌──────────────────────────────────────────────┐          │
│  │ 🎬 Welcome to Day Trading Room               │          │
│  │    Duration: 12:45 | Views: 1,234            │          │
│  │    [Edit] [Replace] [Delete]                 │          │
│  └──────────────────────────────────────────────┘          │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

#### 3.2 Section Definitions
```typescript
const ROOM_SECTIONS = [
    {
        id: 'introduction',
        name: 'Introduction',
        description: 'Main welcome and overview videos',
        icon: IconVideo,
        allowedTypes: ['video'],
        maxVideos: 3
    },
    {
        id: 'latest_updates',
        name: 'Latest Updates',
        description: 'Recent announcements and updates',
        icon: IconBell,
        allowedTypes: ['video', 'pdf'],
        maxVideos: null
    },
    {
        id: 'premium_daily_videos',
        name: 'Premium Daily Videos',
        description: 'Daily trading analysis and setups',
        icon: IconCalendar,
        allowedTypes: ['video'],
        maxVideos: null
    },
    {
        id: 'watchlist',
        name: 'Watchlist',
        description: 'Weekly stock watchlist videos',
        icon: IconList,
        allowedTypes: ['video', 'pdf', 'spreadsheet'],
        maxVideos: null
    },
    {
        id: 'weekly_alerts',
        name: 'Weekly Alerts',
        description: 'Weekly alert summaries (Explosive Swings)',
        icon: IconAlertTriangle,
        allowedTypes: ['video'],
        maxVideos: null,
        roomsOnly: ['explosive-swings'] // Only show for specific rooms
    },
    {
        id: 'learning_center',
        name: 'Learning Center',
        description: 'Educational tutorials and guides',
        icon: IconSchool,
        allowedTypes: ['video', 'pdf', 'document'],
        maxVideos: null
    }
];
```

#### 3.3 Drag & Drop Upload Component
```svelte
<!-- BunnyVideoUploader.svelte -->
<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import IconUpload from '@tabler/icons-svelte/icons/upload';
    import IconCheck from '@tabler/icons-svelte/icons/check';
    import IconX from '@tabler/icons-svelte/icons/x';
    
    let isDragging = $state(false);
    let isUploading = $state(false);
    let uploadProgress = $state(0);
    let uploadedVideoUrl = $state('');
    
    const dispatch = createEventDispatcher();
    
    async function handleDrop(e: DragEvent) {
        e.preventDefault();
        isDragging = false;
        
        const files = e.dataTransfer?.files;
        if (files && files.length > 0) {
            await uploadVideo(files[0]);
        }
    }
    
    async function uploadVideo(file: File) {
        isUploading = true;
        uploadProgress = 0;
        
        try {
            // 1. Get upload URL from backend
            const initResponse = await fetch('/api/admin/bunny/upload', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    filename: file.name,
                    filesize: file.size,
                    mime_type: file.type
                })
            });
            
            const { upload_url, upload_id, video_guid } = await initResponse.json();
            
            // 2. Upload to Bunny.net with progress tracking
            const xhr = new XMLHttpRequest();
            
            xhr.upload.addEventListener('progress', (e) => {
                if (e.lengthComputable) {
                    uploadProgress = (e.loaded / e.total) * 100;
                }
            });
            
            xhr.addEventListener('load', async () => {
                if (xhr.status === 200 || xhr.status === 201) {
                    // 3. Poll for processing status
                    await pollUploadStatus(upload_id);
                }
            });
            
            xhr.open('PUT', upload_url);
            xhr.setRequestHeader('Content-Type', file.type);
            xhr.send(file);
            
        } catch (error) {
            console.error('Upload failed:', error);
            isUploading = false;
        }
    }
    
    async function pollUploadStatus(uploadId: string) {
        const maxAttempts = 60; // 5 minutes max
        let attempts = 0;
        
        const interval = setInterval(async () => {
            attempts++;
            
            const response = await fetch(`/api/admin/bunny/status/${uploadId}`);
            const { status, video_url, thumbnail_url } = await response.json();
            
            if (status === 'ready') {
                clearInterval(interval);
                uploadedVideoUrl = video_url;
                isUploading = false;
                
                // Emit success event with URLs
                dispatch('upload-complete', {
                    video_url,
                    thumbnail_url,
                    upload_id: uploadId
                });
            } else if (status === 'failed' || attempts >= maxAttempts) {
                clearInterval(interval);
                isUploading = false;
                dispatch('upload-error', { message: 'Upload failed or timed out' });
            }
        }, 5000); // Check every 5 seconds
    }
</script>

<div
    class="upload-zone"
    class:dragging={isDragging}
    class:uploading={isUploading}
    ondragover={(e) => { e.preventDefault(); isDragging = true; }}
    ondragleave={() => isDragging = false}
    ondrop={handleDrop}
>
    {#if isUploading}
        <div class="upload-progress">
            <div class="progress-bar" style="width: {uploadProgress}%"></div>
            <p>Uploading... {Math.round(uploadProgress)}%</p>
        </div>
    {:else if uploadedVideoUrl}
        <div class="upload-success">
            <IconCheck size={48} stroke={1.5} />
            <p>Upload complete!</p>
            <code>{uploadedVideoUrl}</code>
        </div>
    {:else}
        <IconUpload size={48} stroke={1.5} />
        <h3>Drag & Drop Video Here</h3>
        <p>or click to browse</p>
        <small>Supports: MP4, MOV, AVI (max 5GB)</small>
    {/if}
</div>

<style>
    .upload-zone {
        border: 2px dashed #cbd5e1;
        border-radius: 12px;
        padding: 3rem;
        text-align: center;
        transition: all 0.3s ease;
        cursor: pointer;
    }
    
    .upload-zone.dragging {
        border-color: #3b82f6;
        background: #eff6ff;
    }
    
    .upload-zone.uploading {
        cursor: not-allowed;
    }
    
    .progress-bar {
        height: 8px;
        background: linear-gradient(90deg, #3b82f6, #06b6d4);
        border-radius: 4px;
        transition: width 0.3s ease;
    }
</style>
```

---

### **Phase 4: Courses & Indicators Auto-Card Generation** ⏱️ 3 hours

#### 4.1 Verify Existing Implementation
Check `/admin/courses/+page.svelte` and `/admin/indicators/+page.svelte` for:
- Automatic card generation when video/thumbnail uploaded
- Card preview in admin
- Proper metadata extraction

#### 4.2 Enhance if Needed
```typescript
// Auto-generate course card data from video upload
interface CourseCard {
    title: string;
    thumbnail_url: string;
    video_url: string;
    duration: string;
    instructor: string;
    level: 'beginner' | 'intermediate' | 'advanced';
    description: string;
}

// When video is uploaded, extract metadata and create card
async function createCourseCard(videoData: any) {
    const card: CourseCard = {
        title: videoData.title || 'Untitled Course',
        thumbnail_url: videoData.thumbnail_url || generateThumbnail(videoData.video_url),
        video_url: videoData.video_url,
        duration: formatDuration(videoData.duration),
        instructor: videoData.instructor || 'Revolution Trading Pros',
        level: videoData.level || 'intermediate',
        description: videoData.description || ''
    };
    
    // Save to database
    await coursesApi.create(card);
}
```

---

### **Phase 5: UI/UX Polish** ⏱️ 4 hours

#### 5.1 Design System
- **Colors:** Use `#143E59` (dark teal) for primary actions
- **Typography:** Inter font family, clear hierarchy
- **Spacing:** 8px grid system
- **Animations:** Smooth 300ms transitions, spring physics for drag & drop
- **Icons:** Tabler Icons throughout

#### 5.2 Responsive Design
- Mobile-first approach
- Touch-friendly targets (min 44px)
- Collapsible sections on mobile
- Swipe gestures for room switching

#### 5.3 Loading States
- Skeleton screens for content loading
- Smooth fade-in animations
- Progress indicators for uploads
- Optimistic UI updates

#### 5.4 Error Handling
- Inline validation messages
- Toast notifications for success/error
- Retry mechanisms for failed uploads
- Graceful degradation if API unavailable

---

### **Phase 6: Testing & Validation** ⏱️ 2 hours

#### 6.1 Functional Testing
- [ ] All 6 trading rooms load correctly in order
- [ ] Section selector shows correct sections per room
- [ ] Drag & drop upload works end-to-end
- [ ] Video URLs auto-generate after upload
- [ ] Resources display in correct sections on dashboard
- [ ] Course cards auto-generate from uploads
- [ ] Indicator cards auto-generate from uploads

#### 6.2 UI/UX Testing
- [ ] Smooth animations and transitions
- [ ] Responsive on mobile, tablet, desktop
- [ ] Accessible (keyboard navigation, screen readers)
- [ ] Fast performance (< 100ms interactions)

#### 6.3 Error Scenarios
- [ ] Large file upload (> 5GB) rejected
- [ ] Network failure during upload handled gracefully
- [ ] Invalid file type rejected
- [ ] API unavailable shows fallback UI

---

## 📊 Success Metrics

1. **All 6 trading rooms** visible in admin in correct order
2. **Introduction section** available for all rooms
3. **Drag & drop upload** working with automatic URL generation
4. **Section-based organization** functional for all dashboard pages
5. **Course/Indicator cards** auto-generate on upload
6. **Beautiful UI** matching Apple design standards
7. **< 3 clicks** to upload and publish a video
8. **< 5 seconds** upload initiation time
9. **100% mobile responsive**
10. **Zero console errors** in production

---

## 🗂️ File Structure

```
api/
├── migrations/
│   ├── 20260115_000001_trading_rooms_master.sql
│   ├── 20260115_000002_add_resource_sections.sql
│   └── 20260115_000003_bunny_upload_tracking.sql
├── src/
│   └── routes/
│       ├── trading_rooms.rs (enhanced)
│       ├── room_resources.rs (enhanced with sections)
│       └── bunny_upload.rs (NEW)

frontend/
├── src/
│   ├── lib/
│   │   ├── api/
│   │   │   ├── trading-rooms.ts (enhanced)
│   │   │   ├── room-resources.ts (enhanced)
│   │   │   └── bunny-upload.ts (NEW)
│   │   └── components/
│   │       └── admin/
│   │           ├── BunnyVideoUploader.svelte (NEW)
│   │           ├── ResourceSectionSelector.svelte (NEW)
│   │           └── ResourceCard.svelte (enhanced)
│   └── routes/
│       └── admin/
│           ├── resources/
│           │   └── +page.svelte (MAJOR OVERHAUL)
│           ├── courses/
│           │   └── +page.svelte (enhance auto-card)
│           └── indicators/
│               └── +page.svelte (enhance auto-card)
```

---

## ⏰ Timeline

| Phase | Duration | Priority |
|-------|----------|----------|
| Phase 1: Database Schema | 2 hours | 🔴 Critical |
| Phase 2: Backend APIs | 4 hours | 🔴 Critical |
| Phase 3: Admin UI Overhaul | 8 hours | 🔴 Critical |
| Phase 4: Auto-Card Generation | 3 hours | 🟡 High |
| Phase 5: UI/UX Polish | 4 hours | 🟡 High |
| Phase 6: Testing | 2 hours | 🟢 Medium |
| **Total** | **23 hours** | **~3 days** |

---

## 🚀 Next Steps

1. **Review and approve this plan**
2. **Create database migrations** (Phase 1)
3. **Implement backend APIs** (Phase 2)
4. **Build admin UI** (Phase 3)
5. **Test end-to-end** (Phase 6)
6. **Deploy to production**

---

**This plan ensures:**
✅ All 6 trading rooms in correct order  
✅ Introduction section for every room  
✅ Section-based video organization  
✅ Drag & drop upload with auto-URL generation  
✅ Beautiful Apple-grade UI/UX  
✅ Courses & Indicators auto-card generation  
✅ Smooth, professional workflow  
✅ Zero shortcuts, ICT 7 end-to-end quality  
