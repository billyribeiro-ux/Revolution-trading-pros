# Bunny.net Stream Setup Guide (2026 UI)
## Step-by-Step Instructions for Revolution Trading Pros

This guide walks you through creating a Bunny.net Stream video library using the current 2026 interface.

---

## Step 1: Create Your Bunny.net Account

1. Go to **https://bunny.net**
2. Click **"Sign Up"** in the top right
3. Fill in your details:
   - Email address
   - Password
   - Company name (optional)
4. Click **"Create Account"**
5. Check your email and click the verification link
6. Log in to your new account

---

## Step 2: Navigate to Stream Section

Once logged in:

1. Look at the **left sidebar menu**
2. Click on **"Stream"** (has a play button icon ▶️)
3. You'll see the Stream dashboard

**What you'll see:**
- A dashboard showing "Video Libraries"
- A button that says **"+ Add Video Library"** or **"Create Library"**
- If this is your first time, you'll see an empty state with a prompt to create your first library

---

## Step 3: Create Your Video Library

1. Click the **"+ Add Video Library"** button (usually in the top right)

2. A modal/form will appear with these fields:

### Library Name
```
revolution-trading-courses
```
*This is what you'll see in your dashboard*

### Replication Regions (Storage Location)
- **Primary Region**: Select **"US East (New York)"** 
  - *(or closest to your main audience)*
- You can add additional regions later if needed
- Primary region is where videos are stored and encoded

### Player Settings (Optional - can configure later)
- **Player Branding**: Leave default for now
- **Player Controls**: Leave default (all controls enabled)
- **Watermark**: Skip for now

### Security Settings
- **Allowed Referrers**: Leave empty (or add your domain later)
- **Block Direct Access**: Leave unchecked for now
- **Token Authentication**: Leave disabled initially

3. Click **"Create Library"** or **"Add Library"** button at the bottom

---

## Step 4: Access Your Library

After creation, you'll be redirected to your library dashboard. You should see:

- **Library name** at the top: "revolution-trading-courses"
- **Navigation tabs**: Overview, Videos, Collections, Settings, API, Analytics
- **Upload button**: Usually says "Upload Video" or "+ Add Video"

---

## Step 5: Get Your API Credentials

This is the most important step for integration:

1. Click on the **"API"** tab in your library navigation

2. You'll see a section called **"API Access"** with:

### Library ID
```
Example: 123456
```
- This is a 6-digit number
- **Copy this number** - you'll need it for `BUNNY_STREAM_LIBRARY_ID`

### API Key
```
Example: a1b2c3d4-e5f6-7890-abcd-ef1234567890
```
- This is a long string with letters, numbers, and dashes
- Click the **"Show"** or **👁️ eye icon** to reveal it
- Click the **"Copy"** button to copy it
- **Copy this key** - you'll need it for `BUNNY_STREAM_API_KEY`

⚠️ **IMPORTANT**: Keep this API key secret! Don't commit it to public repositories.

---

## Step 6: Test Your Library (Optional)

To verify everything works:

1. Go back to the **"Videos"** tab
2. Click **"Upload Video"** or **"+ Add Video"**
3. Upload a small test video (any MP4 file)
4. Wait for encoding to complete (usually 1-2 minutes for short videos)
5. Click on the video thumbnail
6. Click **"Play"** to test playback

---

## Step 7: Configure Storage Zone for Downloads

Now let's set up file storage for course downloads (PDFs, worksheets, etc.):

### Create Storage Zone

1. In the left sidebar, click **"Storage"**
2. Click **"+ Add Storage Zone"** button
3. Fill in the form:

**Storage Zone Name:**
```
revolution-downloads
```

**Storage Type:**
- Select **"Standard"** (cheaper, perfect for downloads)
- *Edge storage is more expensive and unnecessary for downloads*

**Replication Region:**
- Select **"US East"** (or same as your Stream library)

**Price Class:**
- Leave as **"Standard"** 

4. Click **"Create Storage Zone"**

---

## Step 8: Get Storage API Credentials

1. Click on your newly created storage zone **"revolution-downloads"**
2. Go to **"FTP & API Access"** tab
3. You'll see:

### Storage Zone Name
```
revolution-downloads
```
*Use this for `BUNNY_STORAGE_ZONE`*

### Password / API Key
- Click **"Show"** to reveal
- Click **"Copy"** to copy
- *Use this for `BUNNY_STORAGE_API_KEY`*

### Hostname
```
Example: ny.storage.bunnycdn.com
```
*Use this for `BUNNY_STORAGE_HOSTNAME`*

---

## Step 9: Create Pull Zone (CDN for Downloads)

This creates a fast CDN URL for your downloads:

1. In the left sidebar, click **"CDN"** or **"Pull Zones"**
2. Click **"+ Add Pull Zone"**
3. Fill in:

**Pull Zone Name:**
```
revolution-downloads-cdn
```

**Origin Type:**
- Select **"Bunny Storage"**

**Storage Zone:**
- Select **"revolution-downloads"** from dropdown

**Pricing Tier:**
- Select **"Standard"** (or "Volume" if you expect high traffic)

4. Click **"Create Pull Zone"**

### Get Your CDN URL

After creation:
1. Click on your pull zone **"revolution-downloads-cdn"**
2. Look for **"CDN Hostname"** or **"Pull Zone URL"**
3. It will look like:
```
https://revolution-downloads.b-cdn.net
```
*Use this for `BUNNY_CDN_URL`*

---

## Step 10: Add Credentials to Your .env File

Now add all your credentials to `api/.env`:

```bash
# ═══════════════════════════════════════════════════════════════════════════
# BUNNY.NET STREAM (Video Hosting)
# ═══════════════════════════════════════════════════════════════════════════
BUNNY_STREAM_API_KEY=a1b2c3d4-e5f6-7890-abcd-ef1234567890
BUNNY_STREAM_LIBRARY_ID=123456

# ═══════════════════════════════════════════════════════════════════════════
# BUNNY.NET STORAGE (File Downloads)
# ═══════════════════════════════════════════════════════════════════════════
BUNNY_STORAGE_API_KEY=your-storage-password-here
BUNNY_STORAGE_ZONE=revolution-downloads
BUNNY_STORAGE_HOSTNAME=ny.storage.bunnycdn.com
BUNNY_CDN_URL=https://revolution-downloads.b-cdn.net
```

Replace the example values with your actual credentials from Steps 5, 8, and 9.

---

## Step 11: Verify Setup

Test your configuration:

1. Start your API server:
```bash
cd api
cargo run
```

2. In another terminal, test the video upload endpoint:
```bash
curl -X POST http://localhost:3000/api/admin/courses/test-course-id/video-upload \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Video"}'
```

You should get back a JSON response with:
- `video_guid`
- `tus_upload_url`
- `embed_url`

If you see these, your Bunny.net Stream is configured correctly! ✅

---

## Common Issues & Solutions

### "API Key Invalid" Error
- Double-check you copied the full API key (it's long!)
- Make sure there are no extra spaces
- Verify you're using the Stream API key, not Storage API key

### "Library Not Found" Error
- Verify your Library ID is just the numbers (e.g., `123456`)
- No letters or special characters in Library ID

### Videos Not Playing
- Check that encoding completed in Bunny dashboard
- Verify your library is in the correct region
- Check browser console for CORS errors

### Storage Upload Fails
- Verify Storage API key is correct
- Check that storage zone name matches exactly
- Ensure hostname includes region (e.g., `ny.storage.bunnycdn.com`)

---

## Pricing Breakdown (2026)

### Bunny Stream
- **Storage**: $0.01/GB/month
- **Encoding**: $1.00/hour of video
- **Streaming**: $0.01/GB delivered

### Bunny Storage + CDN
- **Storage**: $0.01/GB/month
- **Bandwidth**: $0.01/GB (first 500TB)

### Example Monthly Cost
For a course platform with:
- 100 videos (500GB total)
- 1TB video streaming/month
- 100GB downloads/month

**Total: ~$20/month**

---

## Next Steps

1. ✅ Run database migration: `sqlx migrate run`
2. ✅ Install TUS client: `npm install tus-js-client`
3. ✅ Start building courses in `/admin/courses`
4. ✅ Upload your first video using the VideoUploader component

---

## Support Resources

- **Bunny.net Dashboard**: https://dash.bunny.net
- **Documentation**: https://docs.bunny.net
- **Stream API Docs**: https://docs.bunny.net/reference/video-library
- **Support**: support@bunny.net (usually responds in <24 hours)

---

*Last Updated: January 2026 - Revolution Trading Pros*
