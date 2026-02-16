# Content Management System (CMS)

**Enterprise-Grade Blog & Content Editor**

---

## 📋 Overview

The Revolution Trading Pros CMS is a production-grade content management system featuring 30+ block types, real-time collaboration, AI-powered content generation, and a comprehensive publishing workflow.

### Key Features

- **📝 30+ Block Types** - Paragraph, heading, image, video, code, quote, and more
- **🤝 Real-Time Collaboration** - Yjs CRDT for conflict-free editing
- **🤖 AI Content Generation** - Claude-powered writing assistance
- **📊 Virtual Scrolling** - Handle 1000+ blocks efficiently
- **🔄 Version Control** - Complete revision history
- **📅 Publishing Workflow** - 6-state workflow (draft → published)
- **🎨 Reusable Blocks** - Save and reuse content blocks
- **🔍 SEO Optimization** - Built-in SEO tools

---

## 🏗️ Architecture

### Frontend Components

```
frontend/src/routes/admin/posts/
├── +page.svelte              # Post list
├── new/+page.svelte          # Create post
├── [id]/edit/+page.svelte    # Edit post
└── [id]/preview/+page.svelte # Preview post
```

### Backend API Endpoints

```
GET    /api/posts                # List posts
POST   /api/posts                # Create post
GET    /api/posts/:id            # Get post
PUT    /api/posts/:id            # Update post
DELETE /api/posts/:id            # Delete post
POST   /api/posts/:id/publish    # Publish post
GET    /api/cms/revisions/:id    # Get revisions
POST   /api/cms/ai/generate      # AI generation
POST   /api/cms/upload/image     # Upload image
```

### Database Schema

**Core Tables:**
- `posts` - Blog posts metadata
- `cms_blocks` - Content blocks
- `cms_revisions` - Version history
- `cms_reusable_blocks` - Saved block templates

---

## 📦 Block Types

### Text Blocks

1. **Paragraph** - Standard text content
2. **Heading** - H1-H6 headings
3. **Quote** - Blockquotes with attribution
4. **List** - Ordered/unordered lists

### Media Blocks

5. **Image** - Images with captions
6. **Video** - Embedded videos (YouTube, Vimeo, Bunny.net)
7. **Audio** - Audio players
8. **Gallery** - Image galleries

### Code Blocks

9. **Code** - Syntax-highlighted code
10. **Embed** - HTML/iframe embeds

### Layout Blocks

11. **Columns** - Multi-column layouts
12. **Divider** - Horizontal rules
13. **Spacer** - Vertical spacing

### Interactive Blocks

14. **Button** - Call-to-action buttons
15. **Form** - Contact forms
16. **Table** - Data tables

### Advanced Blocks

17. **Accordion** - Collapsible content
18. **Tabs** - Tabbed content
19. **Alert** - Info/warning/error boxes
20. **Card** - Content cards

### Trading-Specific Blocks

21. **Trade Alert** - Formatted trade alerts
22. **Chart** - Trading charts
23. **Ticker** - Stock tickers
24. **Options Chain** - Options data

---

## 📝 Block Data Model

```typescript
interface Block {
  id: string;
  type: BlockType;
  content: Record<string, any>;
  order: number;
  created_at: string;
  updated_at: string;
}

// Example: Paragraph block
{
  id: 'block-123',
  type: 'paragraph',
  content: {
    text: 'This is a paragraph.',
    alignment: 'left'
  },
  order: 0
}

// Example: Image block
{
  id: 'block-456',
  type: 'image',
  content: {
    url: 'https://cdn.example.com/image.jpg',
    alt: 'Description',
    caption: 'Image caption',
    width: 800,
    height: 600
  },
  order: 1
}
```

---

## 🤝 Real-Time Collaboration

### Yjs CRDT Integration

The CMS uses Yjs (CRDT) for conflict-free real-time collaboration:

```typescript
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';

// Create shared document
const ydoc = new Y.Doc();
const yblocks = ydoc.getArray('blocks');

// Connect to WebSocket server
const provider = new WebsocketProvider(
  'wss://api.example.com/collaboration',
  'post-123',
  ydoc
);

// Listen for changes
yblocks.observe((event) => {
  console.log('Blocks updated:', event.changes);
});
```

### Presence Awareness

Track active collaborators:

```typescript
import { Awareness } from 'y-protocols/awareness';

const awareness = provider.awareness;

// Set local state
awareness.setLocalState({
  user: {
    name: 'John Doe',
    color: '#ff0000'
  },
  cursor: { blockId: 'block-123', position: 42 }
});

// Listen for changes
awareness.on('change', () => {
  const states = awareness.getStates();
  console.log('Active users:', states.size);
});
```

---

## 🤖 AI Content Generation

### Supported Operations

1. **Generate** - Create new content
2. **Improve** - Enhance existing content
3. **Summarize** - Condense text
4. **Translate** - Language translation
5. **Expand** - Add more detail

### API Usage

```typescript
const response = await fetch('/api/cms/ai/generate', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    prompt: 'Write a paragraph about trading strategies',
    model: 'claude-3-5-sonnet-20241022',
    type: 'generate',
    max_tokens: 500
  })
});

const { output, usage } = await response.json();
```

---

## 📊 Virtual Scrolling

Handle 1000+ blocks efficiently with virtual scrolling:

```typescript
import { createVirtualizer } from '@tanstack/svelte-virtual';

const virtualizer = createVirtualizer({
  count: blocks.length,
  getScrollElement: () => scrollElement,
  estimateSize: () => 100,
  overscan: 5
});

// Render only visible blocks
{#each virtualizer.getVirtualItems() as virtualRow}
  <div style="height: {virtualRow.size}px">
    <Block data={blocks[virtualRow.index]} />
  </div>
{/each}
```

---

## 🔄 Version Control

### Revision System

Every save creates a new revision:

```typescript
interface Revision {
  id: string;
  post_id: string;
  content: Block[];
  created_by: string;
  created_at: string;
  message?: string;
}
```

### Restore Previous Version

```typescript
const response = await fetch(`/api/cms/revisions/${revisionId}/restore`, {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`
  }
});
```

---

## 📅 Publishing Workflow

### Post States

1. **Draft** - Work in progress
2. **Review** - Ready for review
3. **Scheduled** - Scheduled for future publish
4. **Published** - Live on site
5. **Archived** - Removed from public view
6. **Deleted** - Soft deleted

### State Transitions

```
Draft → Review → Published
  ↓       ↓         ↓
  ↓       ↓      Archived
  ↓       ↓         ↓
  ↓       ↓      Deleted
  ↓       ↓
  ↓    Scheduled → Published
  ↓
Deleted
```

---

## 🎨 Reusable Blocks

### Save Block as Template

```typescript
const response = await fetch('/api/cms/reusable-blocks', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    name: 'Call to Action',
    description: 'Standard CTA block',
    block: {
      type: 'button',
      content: {
        text: 'Get Started',
        url: '/signup',
        style: 'primary'
      }
    }
  })
});
```

### Insert Reusable Block

```typescript
const templates = await fetch('/api/cms/reusable-blocks');
const { data } = await templates.json();

// Insert into post
blocks.push(data[0].block);
```

---

## 🔍 SEO Optimization

### Built-in SEO Tools

1. **Meta Title** - Optimized titles
2. **Meta Description** - Search descriptions
3. **Open Graph** - Social media previews
4. **Schema.org** - Structured data
5. **Readability Score** - Content analysis

### SEO Data Model

```typescript
interface PostSEO {
  title: string;
  description: string;
  keywords: string[];
  og_image: string;
  canonical_url: string;
  schema_type: 'Article' | 'BlogPosting';
}
```

---

## 📥 Image Upload

### Upload Flow

1. **Select image** - File picker
2. **Upload to R2** - Cloudflare R2 storage
3. **Generate thumbnail** - Automatic resizing
4. **Insert block** - Add to post

### API Usage

```typescript
const formData = new FormData();
formData.append('file', imageFile);

const response = await fetch('/api/cms/upload/image', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${token}`
  },
  body: formData
});

const { url, thumbnail_url } = await response.json();
```

---

## 🚀 Performance Optimizations

1. **Debounced autosave** - Save every 3 seconds
2. **Optimistic updates** - Instant UI feedback
3. **Lazy block loading** - Load blocks on demand
4. **Image optimization** - WebP conversion
5. **CDN delivery** - Cloudflare R2 + CDN

---

## 📊 Metrics & Monitoring

- **Save latency** - < 200ms
- **AI generation** - < 3s
- **Image upload** - < 2s
- **Collaboration sync** - < 100ms
- **Virtual scroll FPS** - 60fps

