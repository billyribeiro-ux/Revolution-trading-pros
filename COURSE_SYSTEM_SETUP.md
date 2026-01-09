# Course Management System Setup Guide
## Revolution Trading Pros - January 2026

This guide covers everything you need to set up the Ultimate Course Management System with Bunny.net video hosting.

---

## 1. Bunny.net Account Setup

### Sign Up
1. Go to **https://bunny.net** and create an account
2. Verify your email address

### Recommended Tier: **Bunny Stream** (Pay-as-you-go)

| Feature | Pricing |
|---------|---------|
| Storage | $0.01/GB/month |
| Encoding | $1.00/hour of video |
| Delivery | $0.01/GB (first 500TB) |

**For a typical course platform:**
- 100 videos (avg 30 min each) = ~50 hours encoding = **$50 one-time**
- 500GB storage = **$5/month**
- 1TB delivery/month = **$10/month**

**Total: ~$15-20/month** for a full course library

---

## 2. Create Bunny Stream Library

1. Log into **Bunny.net Dashboard**
2. Go to **Stream** → **Video Libraries**
3. Click **Add Video Library**
4. Name it: `revolution-trading-courses`
5. Select your preferred **Storage Region** (US East recommended)
6. Click **Create**

### Get Your API Keys

After creating the library:

1. Click on your library name
2. Go to **API** tab
3. Copy these values:
   - **Library ID** (e.g., `123456`)
   - **API Key** (e.g., `a1b2c3d4-e5f6-7890-abcd-ef1234567890`)

---

## 3. Create Bunny Storage Zone (for downloads)

1. Go to **Storage** → **Storage Zones**
2. Click **Add Storage Zone**
3. Name it: `revolution-downloads`
4. Select **Standard** tier
5. Select region (same as Stream library)
6. Click **Create**

### Get Storage Credentials

1. Click on your storage zone
2. Go to **FTP & API Access**
3. Copy:
   - **Storage Zone Name**: `revolution-downloads`
   - **Password/API Key**: (your storage API key)
   - **Hostname**: `ny.storage.bunnycdn.com` (or your region)

### Create Pull Zone (CDN)

1. Go to **CDN** → **Pull Zones**
2. Click **Add Pull Zone**
3. Name: `revolution-downloads-cdn`
4. Origin URL: `https://ny.storage.bunnycdn.com/revolution-downloads`
5. Click **Create**
6. Note your **Pull Zone URL**: `https://revolution-downloads.b-cdn.net`

---

## 4. Environment Variables

Add these to your `.env` file in the `api/` directory:

```bash
# ═══════════════════════════════════════════════════════════════════════════
# BUNNY.NET STREAM (Video Hosting)
# ═══════════════════════════════════════════════════════════════════════════
BUNNY_STREAM_API_KEY=your-stream-api-key-here
BUNNY_STREAM_LIBRARY_ID=123456

# ═══════════════════════════════════════════════════════════════════════════
# BUNNY.NET STORAGE (File Downloads)
# ═══════════════════════════════════════════════════════════════════════════
BUNNY_STORAGE_API_KEY=your-storage-api-key-here
BUNNY_STORAGE_ZONE=revolution-downloads
BUNNY_STORAGE_HOSTNAME=ny.storage.bunnycdn.com
BUNNY_CDN_URL=https://revolution-downloads.b-cdn.net
```

---

## 5. Database Migration

Run the migration to create course tables:

```bash
cd api
sqlx migrate run
```

Or manually apply:
```bash
psql $DATABASE_URL -f migrations/20260109_000001_course_management_system.sql
```

---

## 6. Frontend Dependencies

Install the TUS upload client for resumable uploads:

```bash
cd frontend
npm install tus-js-client
```

---

## 7. Build & Deploy

### Backend
```bash
cd api
cargo build --release
./target/release/revolution-api
```

### Frontend
```bash
cd frontend
npm run build
npm run preview  # or deploy to Cloudflare Pages
```

---

## 8. API Endpoints Reference

### Admin Endpoints (require auth)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/admin/courses` | List all courses |
| `POST` | `/api/admin/courses` | Create course |
| `GET` | `/api/admin/courses/:id` | Get course with content |
| `PUT` | `/api/admin/courses/:id` | Update course |
| `DELETE` | `/api/admin/courses/:id` | Delete course |
| `POST` | `/api/admin/courses/:id/publish` | Publish course |
| `POST` | `/api/admin/courses/:id/video-upload` | Get TUS upload URL |
| `POST` | `/api/admin/courses/:id/upload-url` | Get file upload URL |

### Module Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET/POST` | `/api/admin/courses/:id/modules` | List/create modules |
| `PUT/DELETE` | `/api/admin/courses/:id/modules/:mid` | Update/delete module |
| `PUT` | `/api/admin/courses/:id/modules/reorder` | Reorder modules |

### Lesson Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET/POST` | `/api/admin/courses/:id/lessons` | List/create lessons |
| `GET/PUT/DELETE` | `/api/admin/courses/:id/lessons/:lid` | CRUD lesson |
| `PUT` | `/api/admin/courses/:id/lessons/reorder` | Reorder lessons |

### Download Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET/POST` | `/api/admin/courses/:id/downloads` | List/create downloads |
| `PUT/DELETE` | `/api/admin/courses/:id/downloads/:did` | Update/delete download |

### Public Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/courses` | List published courses |
| `GET` | `/api/courses/:slug` | Get course detail |

### Member Endpoints (require auth)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/api/my/courses` | User's enrolled courses |
| `GET` | `/api/my/courses/:slug/player` | Course player data |
| `PUT` | `/api/my/courses/:slug/progress` | Update lesson progress |
| `GET` | `/api/my/courses/:slug/downloads` | Course downloads |

---

## 9. Frontend Routes

| Route | Description |
|-------|-------------|
| `/admin/courses` | Course management list |
| `/admin/courses/create` | Create new course |
| `/admin/courses/[id]` | Edit course |
| `/admin/courses/[id]/lessons/[lessonId]` | Edit lesson |
| `/classes` | Public course catalog |
| `/classes/[slug]` | Course detail/player |
| `/my/courses` | Member's enrolled courses |

---

## 10. Video Upload Flow

1. Admin clicks "Upload Video" in lesson editor
2. Frontend calls `POST /api/admin/courses/:id/video-upload` with title
3. Backend creates video in Bunny Stream, returns TUS URL + auth
4. Frontend uses `tus-js-client` to upload directly to Bunny
5. On success, video GUID is saved to lesson record
6. Bunny encodes video automatically
7. Video plays via Bunny iframe embed

---

## 11. File Download Flow

1. Admin uploads file via `POST /api/admin/courses/:id/upload-url`
2. Backend returns pre-signed Bunny Storage URL
3. Frontend uploads file directly to Bunny Storage
4. CDN URL is saved to `course_downloads` table
5. Members download via CDN URL (with access control)

---

## 12. Troubleshooting

### Video not playing
- Check `BUNNY_STREAM_LIBRARY_ID` is correct
- Verify video encoding completed in Bunny dashboard
- Check browser console for embed errors

### Upload fails
- Verify `BUNNY_STREAM_API_KEY` has write permissions
- Check file size < 10GB
- Ensure stable internet connection (TUS is resumable)

### Downloads not working
- Verify `BUNNY_STORAGE_API_KEY` is correct
- Check `BUNNY_CDN_URL` matches your pull zone
- Verify file exists in storage zone

---

## 13. Cost Optimization Tips

1. **Compress videos** before upload (H.264, 1080p max)
2. **Set video expiry** for old/unused content
3. **Use regional storage** closest to your users
4. **Enable caching** on pull zones (7+ days)
5. **Monitor usage** in Bunny dashboard billing

---

## Support

- **Bunny.net Docs**: https://docs.bunny.net
- **Stream API**: https://docs.bunny.net/reference/video-library
- **Storage API**: https://docs.bunny.net/reference/storage-api

---

*Generated January 2026 - Revolution Trading Pros Course System*
