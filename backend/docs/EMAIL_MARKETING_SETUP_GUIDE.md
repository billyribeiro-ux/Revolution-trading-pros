# Email Marketing System - Setup Guide

## Overview

This guide covers the setup and configuration of the Revolution Trading Pros enterprise email marketing system. The system provides features comparable to ActiveCampaign, Klaviyo, Mailchimp, and HubSpot.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Database Setup](#database-setup)
5. [Service Providers](#service-providers)
6. [Queue Configuration](#queue-configuration)
7. [Webhook Setup](#webhook-setup)
8. [Testing](#testing)
9. [Troubleshooting](#troubleshooting)

---

## Prerequisites

- PHP 8.1+
- Laravel 10+
- Redis (for caching and queues)
- MySQL 8+ or PostgreSQL 13+
- Composer
- Node.js 18+ (for frontend)

---

## Installation

### 1. Run Migrations

```bash
php artisan migrate
```

This will create the following tables:

- `email_templates` - Email template storage
- `email_campaigns` - Campaign management
- `newsletter_subscriptions` - Subscriber management
- `email_audit_logs` - Audit logging
- `email_campaign_metrics` - Analytics
- `email_engagement_scores` - Subscriber engagement tracking
- And more...

### 2. Seed Default Templates

```bash
php artisan db:seed --class=EmailTemplatesSeeder
php artisan db:seed --class=AdditionalEmailTemplatesSeeder
```

### 3. Install Dependencies

```bash
composer require tijsverkoyen/css-to-inline-styles
composer require jenssegers/agent
```

---

## Configuration

### Environment Variables

Add to your `.env` file:

```env
# Mail Configuration
MAIL_MAILER=smtp
MAIL_HOST=smtp.mailgun.org
MAIL_PORT=587
MAIL_USERNAME=your-username
MAIL_PASSWORD=your-password
MAIL_ENCRYPTION=tls
MAIL_FROM_ADDRESS=hello@example.com
MAIL_FROM_NAME="Revolution Trading"

# Fallback Mail Providers
MAIL_FALLBACK_DRIVER=sendgrid
SENDGRID_API_KEY=your-sendgrid-key
AWS_SES_KEY=your-ses-key
AWS_SES_SECRET=your-ses-secret

# Rate Limiting
RATE_LIMIT_DEFAULT_REQUESTS=100
RATE_LIMIT_DEFAULT_WINDOW=60
RATE_LIMIT_DAILY=10000

# Suspicious Activity
SUSPICIOUS_THRESHOLD=5
BAN_DURATION=3600

# Cache
CACHE_DRIVER=redis
REDIS_HOST=127.0.0.1
REDIS_PORT=6379

# Queue
QUEUE_CONNECTION=redis
```

### Publish Configuration

```bash
php artisan vendor:publish --tag=mail-config
```

---

## Database Setup

### Analytics Tables

The analytics system uses dedicated tables for high-performance reporting:

```sql
-- Campaign metrics (hourly aggregation)
email_campaign_metrics

-- Link click tracking
email_link_clicks

-- Email open tracking
email_opens

-- Subscriber engagement scores
email_engagement_scores

-- Deliverability metrics
email_deliverability_metrics

-- Revenue attribution
email_revenue_attribution

-- Daily stats (dashboard)
email_daily_stats
```

### Indexes

Ensure indexes are created for optimal performance:

```sql
CREATE INDEX idx_campaign_metrics_hour ON email_campaign_metrics(hour, campaign_id);
CREATE INDEX idx_link_clicks_campaign ON email_link_clicks(campaign_id, clicked_at);
CREATE INDEX idx_engagement_scores ON email_engagement_scores(subscriber_id, date);
```

---

## Service Providers

### Register Services

In `app/Providers/AppServiceProvider.php`:

```php
use App\Services\Email\EmailService;
use App\Services\Email\AuditService;
use App\Services\Email\AIEmailService;
use App\Services\Email\TokenRotationService;
use App\Services\Email\CampaignWebhookService;
use App\Services\Email\TemplateCompiledCacheService;
use App\Services\Email\VisualBuilderService;

public function register(): void
{
    $this->app->singleton(EmailService::class);
    $this->app->singleton(AuditService::class);
    $this->app->singleton(AIEmailService::class);
    $this->app->singleton(TokenRotationService::class);
    $this->app->singleton(CampaignWebhookService::class);
    $this->app->singleton(TemplateCompiledCacheService::class);
    $this->app->singleton(VisualBuilderService::class);
}
```

### Register Middleware

In `app/Http/Kernel.php`:

```php
protected $routeMiddleware = [
    // ... other middleware
    'rate.limit.global' => \App\Http\Middleware\GlobalApiRateLimiter::class,
];
```

Apply to routes:

```php
Route::middleware(['rate.limit.global:email'])->group(function () {
    // Email routes
});
```

---

## Queue Configuration

### Supervisor Configuration

Create `/etc/supervisor/conf.d/email-worker.conf`:

```ini
[program:email-worker]
process_name=%(program_name)s_%(process_num)02d
command=php /path/to/artisan queue:work redis --queue=emails-high,emails-normal,emails-low --sleep=3 --tries=3
autostart=true
autorestart=true
user=www-data
numprocs=4
redirect_stderr=true
stdout_logfile=/var/log/supervisor/email-worker.log
```

### Horizon Configuration

If using Laravel Horizon:

```php
// config/horizon.php
'environments' => [
    'production' => [
        'email-supervisor' => [
            'connection' => 'redis',
            'queue' => ['emails-high', 'emails-normal', 'emails-low'],
            'balance' => 'auto',
            'minProcesses' => 1,
            'maxProcesses' => 10,
            'tries' => 3,
        ],
    ],
],
```

---

## Webhook Setup

### Configure Webhooks

Set up webhooks to receive campaign events:

```php
use App\Services\Email\CampaignWebhookService;

$webhookService = app(CampaignWebhookService::class);

$webhook = $webhookService->registerWebhook(
    'https://your-app.com/webhooks/email',
    ['campaign.completed', 'email.bounced', 'email.complained'],
    'your-secret-key'
);
```

### Webhook Verification

Verify incoming webhooks:

```php
$payload = $request->getContent();
$signature = $request->header('X-Webhook-Signature');
$expectedSignature = hash_hmac('sha256', $payload, $secret);

if (!hash_equals($expectedSignature, $signature)) {
    return response('Invalid signature', 401);
}
```

---

## Testing

### PHPUnit Tests

```bash
# Run all email tests
php artisan test --filter=Email

# Test specific service
php artisan test tests/Unit/Services/Email/EmailServiceTest.php
```

### Test Email Sending

```bash
php artisan tinker

>>> app(App\Services\Email\EmailService::class)->send([
    'to' => 'test@example.com',
    'subject' => 'Test Email',
    'template' => 'emails.test',
    'data' => ['name' => 'John']
]);
```

### Load Testing

```bash
# Using Artillery
npx artillery run email-load-test.yml
```

---

## Troubleshooting

### Common Issues

#### 1. Emails Not Sending

```bash
# Check queue status
php artisan queue:work --once

# Check failed jobs
php artisan queue:failed

# Retry failed jobs
php artisan queue:retry all
```

#### 2. Rate Limiting Issues

```bash
# Check rate limit status for IP
php artisan tinker
>>> App\Http\Middleware\GlobalApiRateLimiter::getStatistics('1.2.3.4')

# Unban IP
>>> App\Http\Middleware\GlobalApiRateLimiter::unbanIp('1.2.3.4')
```

#### 3. Cache Issues

```bash
# Clear template cache
php artisan cache:forget compiled_template_keys

# Clear all caches
php artisan cache:clear
```

#### 4. Token Issues

```bash
# Rotate all expired tokens
php artisan tinker
>>> app(App\Services\Email\TokenRotationService::class)->batchRotateTokens(1000)
```

### Logging

Enable detailed logging in `.env`:

```env
LOG_LEVEL=debug
RATE_LIMIT_LOGGING=true
```

Check logs:

```bash
tail -f storage/logs/laravel.log | grep -E "(Email|Rate|Token)"
```

---

## Maintenance

### Daily Tasks

```bash
# Scheduled in Console/Kernel.php
$schedule->command('email:cleanup-logs')->daily();
$schedule->command('email:rotate-tokens')->daily();
$schedule->command('email:aggregate-metrics')->hourly();
```

### Weekly Tasks

```bash
$schedule->call(function () {
    app(TemplateCompiledCacheService::class)->warmCache();
})->weekly();
```

---

## Security Best Practices

1. **Always use HTTPS** for webhook endpoints
2. **Rotate API keys** regularly
3. **Monitor audit logs** for suspicious activity
4. **Enable double opt-in** for all newsletter subscriptions
5. **Implement SPF, DKIM, and DMARC** for your sending domain
6. **Review rate limit logs** weekly

---

## Support

For additional support:

- Documentation: `/docs/EMAIL_MARKETING_API.md`
- API Reference: `/docs/api`
- Email: support@revolutiontrading.com
