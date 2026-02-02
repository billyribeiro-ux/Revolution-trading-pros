# Deployment Checklist

## Pre-Deployment

### 1. Code Quality
- [ ] All tests passing (`npm run test:unit && npm run test:e2e`)
- [ ] No TypeScript errors (`npm run check`)
- [ ] No linting errors (`npm run lint`)
- [ ] Code formatted (`npm run format`)

### 2. Environment Variables
- [ ] All production env vars set in Cloudflare
- [ ] API keys configured
- [ ] Database connection string set
- [ ] Session secret generated

### 3. Database
- [ ] Migrations applied (`npx prisma migrate deploy`)
- [ ] Seeds run if needed (`npx prisma db seed`)
- [ ] Backup created

### 4. Performance
- [ ] Bundle size optimized
- [ ] Images optimized
- [ ] Code splitting verified
- [ ] Lighthouse score > 90

### 5. Security
- [ ] CSP headers configured
- [ ] CORS settings reviewed
- [ ] Authentication working
- [ ] Input sanitization tested

## Deployment Steps

### Cloudflare Pages

1. **Connect Repository**
```bash
# Login to Cloudflare
npx wrangler login
```

2. **Configure Build**
   - Build command: `npm run build`
   - Output directory: `.svelte-kit/cloudflare`
   - Node version: 20

3. **Set Environment Variables**
```bash
npx wrangler pages secret put VITE_ANTHROPIC_API_KEY
npx wrangler pages secret put DATABASE_URL
# ... etc
```

4. **Deploy**
```bash
npm run build
npx wrangler pages deploy .svelte-kit/cloudflare --project-name=revolution-trading-pros
```

### Database Migration
```bash
# Production migration
DATABASE_URL="production-url" npx prisma migrate deploy
```

### Custom Domain

1. Add domain in Cloudflare Pages
2. Update DNS records
3. Configure SSL

## Post-Deployment

### 1. Smoke Tests
- [ ] Homepage loads
- [ ] Editor works
- [ ] Block rendering correct
- [ ] File uploads work
- [ ] AI features work

### 2. Monitoring
- [ ] Sentry receiving errors
- [ ] Analytics tracking
- [ ] Performance monitoring
- [ ] Uptime monitoring

### 3. SEO
- [ ] Meta tags correct
- [ ] Sitemap generated
- [ ] robots.txt configured
- [ ] Search console verified

## Rollback Plan

If deployment fails:

1. **Cloudflare Pages**
```bash
# Rollback to previous deployment
# Use Cloudflare dashboard to revert
```

2. **Database**
```bash
# Restore from backup
pg_restore -d database_name backup.sql
```

## Monitoring

### Key Metrics
- Response time < 200ms
- Error rate < 1%
- Uptime > 99.9%

### Alerts
- Set up alerts for:
  - Error spikes
  - Performance degradation
  - High traffic

## Support

Emergency contact: support@revolutiontradingpros.com
