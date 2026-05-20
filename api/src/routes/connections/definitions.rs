//! Service metadata + category map (split from `connections.rs`
//! lines 190-363, R20-B maintainability pass, 2026-05-20).
//! Pure data — no DB access, no side effects.

use serde_json::json;

pub(super) fn get_service_definitions() -> Vec<serde_json::Value> {
    vec![
        // Built-in services
        json!({
            "key": "fluent_crm_pro",
            "name": "FluentCRM Pro",
            "category": "CRM",
            "description": "Advanced customer relationship management with automation, segmentation, and email sequences.",
            "icon": "crm",
            "color": "#10B981",
            "is_oauth": false,
            "is_builtin": true,
            "fields": []
        }),
        json!({
            "key": "fluent_forms_pro",
            "name": "FluentForms Pro",
            "category": "Forms",
            "description": "Drag-and-drop form builder with conditional logic, file uploads, and integrations.",
            "icon": "forms",
            "color": "#8B5CF6",
            "is_oauth": false,
            "is_builtin": true,
            "fields": []
        }),
        json!({
            "key": "fluent_smtp",
            "name": "FluentSMTP",
            "category": "Email",
            "description": "Reliable email delivery with multiple provider support and detailed logging.",
            "icon": "email",
            "color": "#F59E0B",
            "is_oauth": false,
            "is_builtin": true,
            "fields": []
        }),
        // External services
        json!({
            "key": "stripe",
            "name": "Stripe",
            "category": "Payment",
            "description": "Accept payments, manage subscriptions, and process refunds securely.",
            "icon": "stripe",
            "color": "#635BFF",
            "docs_url": "https://stripe.com/docs",
            "signup_url": "https://dashboard.stripe.com/register",
            "is_oauth": false,
            "is_builtin": false,
            "environments": ["production", "sandbox"],
            "fields": [
                {"key": "publishable_key", "label": "Publishable Key", "type": "text", "required": true, "placeholder": "pk_live_..."},
                {"key": "secret_key", "label": "Secret Key", "type": "password", "required": true, "placeholder": "sk_live_..."},
                {"key": "webhook_secret", "label": "Webhook Secret", "type": "password", "required": false, "placeholder": "whsec_..."}
            ]
        }),
        json!({
            "key": "bunny_cdn",
            "name": "Bunny.net CDN",
            "category": "Storage",
            "description": "Global CDN and video streaming with edge caching for fast content delivery.",
            "icon": "bunny",
            "color": "#FF6B00",
            "docs_url": "https://docs.bunny.net",
            "signup_url": "https://bunny.net",
            "is_oauth": false,
            "is_builtin": false,
            "fields": [
                {"key": "api_key", "label": "API Key", "type": "password", "required": true},
                {"key": "storage_zone", "label": "Storage Zone", "type": "text", "required": true},
                {"key": "library_id", "label": "Video Library ID", "type": "text", "required": false}
            ]
        }),
        json!({
            "key": "sendgrid",
            "name": "SendGrid",
            "category": "Email",
            "description": "Transactional email delivery with templates, analytics, and deliverability tools.",
            "icon": "email",
            "color": "#1A82E2",
            "docs_url": "https://docs.sendgrid.com",
            "signup_url": "https://signup.sendgrid.com",
            "is_oauth": false,
            "is_builtin": false,
            "fields": [
                {"key": "api_key", "label": "API Key", "type": "password", "required": true},
                {"key": "from_email", "label": "From Email", "type": "text", "required": true},
                {"key": "from_name", "label": "From Name", "type": "text", "required": false}
            ]
        }),
        json!({
            "key": "google_analytics",
            "name": "Google Analytics 4",
            "category": "Analytics",
            "description": "Track user behavior, conversions, and marketing performance with advanced analytics.",
            "icon": "analytics",
            "color": "#F9AB00",
            "docs_url": "https://developers.google.com/analytics",
            "signup_url": "https://analytics.google.com",
            "is_oauth": true,
            "is_builtin": false,
            "fields": [
                {"key": "measurement_id", "label": "Measurement ID", "type": "text", "required": true, "placeholder": "G-XXXXXXXXXX"},
                {"key": "api_secret", "label": "API Secret", "type": "password", "required": false}
            ]
        }),
        json!({
            "key": "openai",
            "name": "OpenAI",
            "category": "AI",
            "description": "AI-powered content generation, chat assistants, and intelligent automation.",
            "icon": "ai",
            "color": "#10A37F",
            "docs_url": "https://platform.openai.com/docs",
            "signup_url": "https://platform.openai.com/signup",
            "is_oauth": false,
            "is_builtin": false,
            "fields": [
                {"key": "api_key", "label": "API Key", "type": "password", "required": true, "placeholder": "sk-..."},
                {"key": "organization_id", "label": "Organization ID", "type": "text", "required": false}
            ]
        }),
        json!({
            "key": "cloudflare_r2",
            "name": "Cloudflare R2",
            "category": "Storage",
            "description": "S3-compatible object storage with zero egress fees and global distribution.",
            "icon": "storage",
            "color": "#F48120",
            "docs_url": "https://developers.cloudflare.com/r2",
            "signup_url": "https://dash.cloudflare.com/sign-up",
            "is_oauth": false,
            "is_builtin": false,
            "fields": [
                {"key": "account_id", "label": "Account ID", "type": "text", "required": true},
                {"key": "access_key_id", "label": "Access Key ID", "type": "text", "required": true},
                {"key": "secret_access_key", "label": "Secret Access Key", "type": "password", "required": true},
                {"key": "bucket_name", "label": "Bucket Name", "type": "text", "required": true}
            ]
        }),
        json!({
            "key": "meilisearch",
            "name": "Meilisearch",
            "category": "Search",
            "description": "Lightning-fast full-text search with typo tolerance and faceted filtering.",
            "icon": "search",
            "color": "#FF5CAA",
            "docs_url": "https://docs.meilisearch.com",
            "signup_url": "https://cloud.meilisearch.com",
            "is_oauth": false,
            "is_builtin": false,
            "fields": [
                {"key": "host", "label": "Host URL", "type": "text", "required": true},
                {"key": "api_key", "label": "API Key", "type": "password", "required": true}
            ]
        }),
    ]
}

pub(super) fn get_categories() -> serde_json::Value {
    json!({
        "Payment": {"name": "Payment", "icon": "credit-card", "services": ["stripe"]},
        "Storage": {"name": "Storage", "icon": "cloud", "services": ["bunny_cdn", "cloudflare_r2"]},
        "Email": {"name": "Email", "icon": "mail", "services": ["sendgrid", "fluent_smtp"]},
        "Analytics": {"name": "Analytics", "icon": "chart", "services": ["google_analytics"]},
        "AI": {"name": "AI", "icon": "cpu", "services": ["openai"]},
        "Search": {"name": "Search", "icon": "search", "services": ["meilisearch"]},
        "CRM": {"name": "CRM", "icon": "users", "services": ["fluent_crm_pro"]},
        "Forms": {"name": "Forms", "icon": "file-text", "services": ["fluent_forms_pro"]}
    })
}
