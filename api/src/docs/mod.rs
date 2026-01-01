//! OpenAPI Documentation - Revolution Trading Pros API
//! ICT 11+ Principal Engineer Grade - Complete API Documentation

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Revolution Trading Pros API",
        version = "1.0.0",
        description = "Enterprise Trading Platform API - Rust + Axum\n\n## Authentication\nUse Bearer token in Authorization header: `Authorization: Bearer <token>`\n\n## Base URLs\n- Production: https://revolution-trading-pros-api.fly.dev\n- Development: http://localhost:8080",
        contact(
            name = "Revolution Trading Pros",
            email = "support@revolutiontradingpros.com"
        ),
        license(
            name = "MIT",
        )
    ),
    servers(
        (url = "https://revolution-trading-pros-api.fly.dev", description = "Production"),
        (url = "http://localhost:8080", description = "Development")
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication and authorization"),
        (name = "users", description = "User management"),
        (name = "products", description = "Product catalog"),
        (name = "courses", description = "Course management"),
        (name = "posts", description = "Blog posts and content"),
        (name = "indicators", description = "Trading indicators"),
        (name = "payments", description = "Payment processing with Stripe"),
        (name = "admin", description = "Admin operations (requires admin role)"),
        (name = "monitoring", description = "Metrics and monitoring"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token for authentication"))
                        .build(),
                ),
            );
        }
    }
}
