# Course Management System

**Video Streaming & Progress Tracking**

---

## 📋 Overview

The course management system delivers video-based educational content with progress tracking, quizzes, and completion certificates.

### Key Features

- **🎥 Video Streaming** - HLS streaming via Bunny.net
- **📊 Progress Tracking** - Per-lesson completion tracking
- **📝 Quizzes** - Interactive assessments
- **🏆 Certificates** - Completion certificates
- **📱 Mobile Responsive** - Works on all devices
- **⚡ Fast Loading** - CDN-delivered content

---

## 🏗️ Architecture

### Frontend Routes

```
frontend/src/routes/dashboard/courses/
├── +page.svelte              # Course catalog
├── [slug]/+page.svelte       # Course overview
└── [slug]/[lesson]/+page.svelte # Lesson player
```

### Backend API Endpoints

```
GET  /api/member-courses              # List enrolled courses
GET  /api/member-courses/:id          # Get course details
GET  /api/member-courses/:id/lessons  # List lessons
POST /api/member-courses/:id/enroll   # Enroll in course
POST /api/member-courses/progress     # Update progress
GET  /api/member-courses/progress     # Get progress
```

### Database Schema

**Core Tables:**
- `courses` - Course metadata
- `lessons` - Lesson content
- `course_enrollments` - User enrollments
- `lesson_progress` - Completion tracking
- `quiz_attempts` - Quiz submissions

---

## 📚 Course Data Model

```typescript
interface Course {
  id: string;
  title: string;
  description: string;
  slug: string;
  thumbnail_url: string;
  instructor: string;
  duration_minutes: number;
  lesson_count: number;
  difficulty: 'beginner' | 'intermediate' | 'advanced';
  price: number;
  is_published: boolean;
  created_at: string;
  updated_at: string;
}
```

---

## 🎥 Lesson Data Model

```typescript
interface Lesson {
  id: string;
  course_id: string;
  title: string;
  description: string;
  video_url: string;
  duration_seconds: number;
  order: number;
  is_preview: boolean; // Free preview
  resources: Resource[];
  quiz?: Quiz;
  created_at: string;
}

interface Resource {
  id: string;
  title: string;
  type: 'pdf' | 'link' | 'download';
  url: string;
}
```

---

## 📊 Progress Tracking

### Progress Data Model

```typescript
interface LessonProgress {
  user_id: string;
  lesson_id: string;
  completed: boolean;
  progress_percent: number;
  last_position_seconds: number;
  completed_at?: string;
  updated_at: string;
}
```

### Update Progress

```typescript
const response = await fetch('/api/member-courses/progress', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    lesson_id: 'lesson-123',
    progress_percent: 75,
    last_position_seconds: 450,
    completed: false
  })
});
```

### Get Course Progress

```typescript
const response = await fetch('/api/member-courses/progress?course_id=course-123');
const { data } = await response.json();

// Calculate overall progress
const totalLessons = data.length;
const completedLessons = data.filter(p => p.completed).length;
const overallProgress = (completedLessons / totalLessons) * 100;
```

---

## 🎥 Video Streaming

### Bunny.net Integration

Videos are streamed via Bunny.net CDN with HLS support:

```typescript
interface VideoConfig {
  library_id: string;
  video_id: string;
  cdn_url: string;
  thumbnail_url: string;
}

// Generate video URL
const videoUrl = `https://${config.cdn_url}/${config.video_id}/playlist.m3u8`;
```

### Video Player

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  
  let { videoUrl, onProgress } = $props();
  let player: HTMLVideoElement;
  
  onMount(() => {
    // Track progress every 5 seconds
    const interval = setInterval(() => {
      if (player) {
        const progress = (player.currentTime / player.duration) * 100;
        onProgress(progress, player.currentTime);
      }
    }, 5000);
    
    return () => clearInterval(interval);
  });
</script>

<video bind:this={player} controls>
  <source src={videoUrl} type="application/x-mpegURL" />
</video>
```

---

## 📝 Quiz System

### Quiz Data Model

```typescript
interface Quiz {
  id: string;
  lesson_id: string;
  title: string;
  passing_score: number;
  questions: Question[];
}

interface Question {
  id: string;
  question: string;
  type: 'multiple_choice' | 'true_false' | 'short_answer';
  options?: string[];
  correct_answer: string | number;
  explanation?: string;
}
```

### Submit Quiz

```typescript
const response = await fetch('/api/member-courses/quiz/submit', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  },
  body: JSON.stringify({
    quiz_id: 'quiz-123',
    answers: {
      'question-1': 'A',
      'question-2': 'B',
      'question-3': 'C'
    }
  })
});

const { score, passed, feedback } = await response.json();
```

---

## 🏆 Certificates

### Certificate Generation

Certificates are generated upon course completion:

```typescript
interface Certificate {
  id: string;
  user_id: string;
  course_id: string;
  issued_at: string;
  certificate_url: string;
}

// Check if eligible for certificate
const allLessonsCompleted = progress.every(p => p.completed);
const allQuizzesPassed = quizAttempts.every(a => a.passed);

if (allLessonsCompleted && allQuizzesPassed) {
  // Generate certificate
  const cert = await generateCertificate(userId, courseId);
}
```

---

## 📱 Mobile Responsive

### Responsive Video Player

```css
.video-container {
  position: relative;
  padding-bottom: 56.25%; /* 16:9 aspect ratio */
  height: 0;
  overflow: hidden;
}

.video-container video {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}
```

---

## 🚀 Performance Optimizations

1. **HLS Streaming** - Adaptive bitrate
2. **CDN Delivery** - Global edge network
3. **Lazy Loading** - Load lessons on demand
4. **Progress Debouncing** - Save every 5 seconds
5. **Thumbnail Optimization** - WebP format

---

## 📊 Analytics

### Track Engagement

```typescript
interface CourseAnalytics {
  course_id: string;
  total_enrollments: number;
  active_students: number;
  completion_rate: number;
  average_progress: number;
  average_time_to_complete: number;
  most_watched_lesson: string;
  drop_off_points: string[];
}
```

---

## 🔐 Access Control

### Enrollment Check

```typescript
async function checkAccess(userId: string, courseId: string): Promise<boolean> {
  const enrollment = await db.query(
    'SELECT * FROM course_enrollments WHERE user_id = $1 AND course_id = $2',
    [userId, courseId]
  );
  
  return enrollment.rows.length > 0;
}
```

### Preview Lessons

Free preview lessons are accessible without enrollment:

```typescript
const lesson = await getLesson(lessonId);

if (!lesson.is_preview) {
  const hasAccess = await checkAccess(userId, lesson.course_id);
  if (!hasAccess) {
    throw new Error('Enrollment required');
  }
}
```

---

## 📊 Metrics & Monitoring

- **Video load time** - < 2s
- **Progress save latency** - < 100ms
- **Quiz submission** - < 200ms
- **Certificate generation** - < 1s
- **CDN cache hit rate** - > 90%

