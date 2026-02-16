# System Architecture

**Revolution Trading Pros - Apple Principal Engineer ICT Level 7+**

---

## 📐 Overview

Revolution Trading Pros is a modern, production-grade trading education platform built with a **monorepo architecture** featuring a SvelteKit frontend and Rust/Axum backend.

### Design Principles

1. **Type Safety** - TypeScript frontend, Rust backend
2. **Performance** - Edge deployment, Redis caching, CDN delivery
3. **Scalability** - Horizontal scaling, connection pooling, async processing
4. **Security** - OAuth, MFA, JWT tokens, rate limiting
5. **Developer Experience** - Hot reload, type checking, comprehensive testing

---

## 🏗️ High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLIENT                              │
│                    (Browser/Mobile)                         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ HTTPS
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                   CLOUDFLARE CDN                            │
│              (Edge Caching, DDoS Protection)                │
└────────────┬────────────────────────────┬───────────────────┘
             │                            │
             │ Static Assets              │ API Requests
             ▼                            ▼
┌────────────────────────┐    ┌──────────────────────────────┐
│  CLOUDFLARE PAGES      │    │      FLY.IO                  │
│  (SvelteKit Frontend)  │    │   (Rust/Axum Backend)        │
│  - SSR/SSG             │    │   - REST API                 │
│  - Edge Functions      │    │   - WebSocket                │
│  - Svelte 5 Runes      │    │   - Background Jobs          │
└────────────────────────┘    └──────────┬───────────────────┘
                                         │
                    ┌────────────────────┼────────────────────┐
                    │                    │                    │
                    ▼                    ▼                    ▼
         ┌──────────────────┐ ┌──────────────────┐ ┌─────────────────┐
         │   POSTGRESQL     │ │      REDIS       │ │  CLOUDFLARE R2  │
         │   (Fly.io)       │ │    (Upstash)     │ │   (S3 Storage)  │
         │   - User data    │ │    - Sessions    │ │   - Images      │
         │   - Content      │ │    - Cache       │ │   - Documents   │
         │   - Analytics    │ │    - Rate limit  │ │                 │
         └──────────────────┘ └──────────────────┘ └─────────────────┘
                    │
                    ▼
         ┌──────────────────┐
         │    BUNNY.NET     │
         │  (Video CDN)     │
         │  - HLS Streaming │
         │  - Transcoding   │
         └──────────────────┘
```

---

## 🎨 Frontend Architecture

### Technology Stack

- **Framework:** SvelteKit 2.x
- **UI Library:** Svelte 5 (runes)
- **Language:** TypeScript 5.x
- **Styling:** Tailwind CSS v4
- **State Management:** Svelte runes ($state, $derived, $effect)
- **Testing:** Playwright (E2E), Vitest (unit)
- **Deployment:** Cloudflare Pages

### Directory Structure

```
frontend/src/
├── lib/
│   ├── components/       # Reusable UI components
│   │   ├── dashboard/    # Dashboard-specific components
│   │   ├── marketing/    # Marketing page components
│   │   ├── nav/          # Navigation components
│   │   └── ui/           # Base UI components
│   ├── server/           # Server-only code
│   │   └── axum/         # Backend API adapters
│   ├── stores/           # Global state stores
│   ├── utils/            # Utility functions
│   ├── seo/              # SEO layer
│   └── styles/           # Global styles
├── routes/               # SvelteKit file-based routing
│   ├── (marketing)/      # Public marketing pages
│   ├── dashboard/        # Protected dashboard
│   ├── admin/            # Admin panel
│   └── api/              # API endpoints (SvelteKit)
└── app.css               # Global CSS entry point
```

### Key Patterns

#### Svelte 5 Runes

```typescript
// State management with runes
let count = $state(0);
let doubled = $derived(count * 2);

$effect(() => {
  console.log('Count changed:', count);
});

// Props with destructuring
let { data, onUpdate }: Props = $props();
```

#### Server-Side Data Loading

```typescript
// +page.server.ts
export const load: PageServerLoad = async ({ fetch, locals }) => {
  const user = locals.user;
  const data = await fetch('/api/data').then(r => r.json());
  
  return { user, data };
};
```

#### Form Actions

```typescript
// +page.server.ts
export const actions = {
  default: async ({ request, locals }) => {
    const formData = await request.formData();
    // Process form data
    return { success: true };
  }
};
```

---

## ⚙️ Backend Architecture

### Technology Stack

- **Framework:** Axum 0.7
- **Language:** Rust 1.75+
- **Database:** PostgreSQL (SQLx)
- **Cache:** Redis
- **Storage:** Cloudflare R2 (S3-compatible)
- **Payments:** Stripe
- **Deployment:** Fly.io

### Directory Structure

```
api/src/
├── routes/           # API endpoints
│   ├── auth.rs       # Authentication
│   ├── users.rs      # User management
│   ├── posts.rs      # Blog posts
│   ├── courses.rs    # Course management
│   └── ...
├── models/           # Database models
├── services/         # Business logic
│   ├── email.rs      # Email service
│   ├── storage.rs    # File storage
│   ├── stripe.rs     # Payment processing
│   └── ...
├── middleware/       # Request middleware
│   ├── auth.rs       # JWT validation
│   ├── cors.rs       # CORS handling
│   └── rate_limit.rs # Rate limiting
├── db/               # Database layer
├── cache/            # Caching layer
├── config.rs         # Configuration
└── main.rs           # Application entry point
```

### Key Patterns

#### Route Handlers

```rust
pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, ApiError> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&state.db.pool)
    .await?;
    
    Ok(Json(user))
}
```

#### Middleware

```rust
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&req)?;
    let claims = verify_jwt(&token, &state.config.jwt_secret)?;
    
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
```

---

## 🗄️ Data Layer

### Database Schema

**Core Tables:**
- `users` - User accounts, authentication
- `posts` - Blog posts and content
- `courses` - Course metadata
- `lessons` - Course lessons
- `subscriptions` - User subscriptions
- `orders` - Payment orders
- `explosive_swings_*` - Trading room data

### Caching Strategy

**L1 Cache (In-Memory):**
- Short-lived data (< 1 minute)
- Frequently accessed data
- Fallback when Redis unavailable

**L2 Cache (Redis):**
- Session data (TTL: 24 hours)
- API responses (TTL: 5 minutes)
- Rate limit counters (TTL: 1 hour)

### Storage Strategy

**Cloudflare R2:**
- User uploads
- Course materials
- Blog images

**Bunny.net:**
- Video streaming
- HLS transcoding
- CDN delivery

---

## 🔐 Security Architecture

### Authentication Flow

1. **OAuth (Google/Apple)** → JWT token
2. **Email/Password** → Argon2 hash → JWT token
3. **MFA (TOTP)** → 6-digit code verification

### Authorization

- **Role-based access control (RBAC)**
- **JWT claims validation**
- **Route-level middleware**

### Rate Limiting

- **Per-IP limits** - 100 req/min
- **Per-user limits** - 1000 req/min
- **Endpoint-specific limits** - Configurable

---

## 📊 Monitoring & Observability

### Metrics

- **Request latency** - P50, P95, P99
- **Error rates** - 4xx, 5xx responses
- **Database queries** - Slow query detection
- **Cache hit rates** - Redis performance

### Logging

- **Structured logging** - JSON format
- **Log levels** - DEBUG, INFO, WARN, ERROR
- **Correlation IDs** - Request tracing

---

## 🚀 Deployment Architecture

### Frontend (Cloudflare Pages)

- **Edge deployment** - Global CDN
- **Automatic SSL** - HTTPS everywhere
- **Preview deployments** - Per-branch previews
- **Rollback support** - Instant rollbacks

### Backend (Fly.io)

- **Multi-region** - Global deployment
- **Auto-scaling** - Based on load
- **Health checks** - Automatic recovery
- **Zero-downtime deploys** - Rolling updates

---

## 📈 Performance Optimizations

1. **Code splitting** - Route-based chunks
2. **Image optimization** - WebP, lazy loading
3. **Database indexing** - Query optimization
4. **Connection pooling** - 50 max connections
5. **Compression** - Gzip/Brotli
6. **CDN caching** - Static asset delivery

---

## 🔄 Future Enhancements

- [ ] GraphQL API layer
- [ ] Real-time collaboration (WebSocket)
- [ ] Mobile app (React Native)
- [ ] Advanced analytics (ClickHouse)
- [ ] AI-powered recommendations

