//! OpenAPI Documentation - Revolution Trading Pros API
//! ICT 11+ Principal Engineer Grade - Complete API Documentation

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Revolution Trading Pros API",
        version = "1.0.0",
        description = "Enterprise Trading Platform API - Rust + Axum",
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
    paths(
        // Health
        crate::routes::health::health_check,
        
        // Auth
        crate::routes::auth::register,
        crate::routes::auth::login,
        crate::routes::auth::refresh_token,
        crate::routes::auth::logout,
        crate::routes::auth::me,
        crate::routes::auth::forgot_password,
        crate::routes::auth::reset_password,
        crate::routes::auth::verify_email,
        crate::routes::auth::resend_verification,
        
        // Users
        crate::routes::users::list_users,
        crate::routes::users::get_user,
        crate::routes::users::update_user,
        crate::routes::users::delete_user,
        
        // Products
        crate::routes::products::list_products,
        crate::routes::products::get_product,
        crate::routes::products::create_product,
        crate::routes::products::update_product,
        crate::routes::products::delete_product,
        
        // Courses
        crate::routes::courses::list_courses,
        crate::routes::courses::get_course,
        crate::routes::courses::get_lessons,
        crate::routes::courses::create_course,
        
        // Posts
        crate::routes::posts::list_posts,
        crate::routes::posts::get_post,
        crate::routes::posts::create_post,
        crate::routes::posts::update_post,
        crate::routes::posts::delete_post,
        
        // Indicators
        crate::routes::indicators::list_indicators,
        crate::routes::indicators::get_indicator,
        crate::routes::indicators::create_indicator,
        
        // Payments
        crate::routes::payments::create_checkout_session,
        crate::routes::payments::webhook,
        
        // Admin
        crate::routes::admin::list_users,
        crate::routes::admin::get_user,
        crate::routes::admin::create_user,
        crate::routes::admin::update_user,
        crate::routes::admin::delete_user,
    ),
    components(
        schemas(
            // Auth models
            crate::models::CreateUser,
            crate::models::LoginUser,
            crate::models::AuthResponse,
            crate::models::UserResponse,
            crate::models::RefreshTokenRequest,
            crate::models::RefreshTokenResponse,
            crate::models::ForgotPasswordRequest,
            crate::models::ResetPasswordRequest,
            crate::models::MessageResponse,
            
            // Product models
            crate::models::CreateProduct,
            crate::models::UpdateProduct,
            
            // Course models
            crate::models::Course,
            crate::models::CreateCourse,
            crate::models::Lesson,
            
            // Post models
            crate::models::Post,
            
            // Indicator models
            crate::models::Indicator,
            
            // Order models
            crate::models::Order,
            
            // Subscription models
            crate::models::Subscription,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication and authorization"),
        (name = "users", description = "User management"),
        (name = "products", description = "Product catalog"),
        (name = "courses", description = "Course management"),
        (name = "posts", description = "Blog posts and content"),
        (name = "indicators", description = "Trading indicators"),
        (name = "payments", description = "Payment processing"),
        (name = "admin", description = "Admin operations (requires admin role)"),
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
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    )
                    .bearer_format("JWT")
                    .description(Some("JWT token for authentication")),
                ),
            );
        }
    }
}
